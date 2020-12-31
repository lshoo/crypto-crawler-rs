use super::utils::connect_with_retry;

use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::time::{Duration, Instant};

use flate2::read::{DeflateDecoder, GzDecoder};
use log::*;
use serde_json::Value;
use tungstenite::{client::AutoStream, Error, Message, WebSocket};

pub(super) struct WSClientInternal<'a> {
    exchange: String, // Eexchange name
    url: String,      // Websocket base url
    ws_stream: WebSocket<AutoStream>,
    channels: HashSet<String>,           // subscribed channels
    on_msg: Box<dyn FnMut(String) + 'a>, // message callback
    serialize_command: fn(&[String], bool) -> Vec<String>,

    // For debugging only
    count: i64, // message received
}

impl<'a> WSClientInternal<'a> {
    pub fn new(
        exchange: &str,
        url: &str,
        on_msg: Box<dyn FnMut(String) + 'a>,
        serialize_command: fn(&[String], bool) -> Vec<String>,
    ) -> Self {
        let ws_stream = connect_with_retry(url);
        WSClientInternal {
            exchange: exchange.to_string(),
            url: url.to_string(),
            ws_stream,
            on_msg,
            channels: HashSet::new(),
            serialize_command,
            count: 0,
        }
    }

    pub fn subscribe(&mut self, channels: &[String]) {
        let mut diff = Vec::<String>::new();
        for ch in channels.iter() {
            if self.channels.insert(ch.clone()) {
                diff.push(ch.clone());
            }
        }
        if !diff.is_empty() {
            let commands = (self.serialize_command)(channels, true);
            commands.into_iter().for_each(|command| {
                let res = self.ws_stream.write_message(Message::Text(command));
                if let Err(err) = res {
                    error!("{}", err);
                }
            });
        }
    }

    pub fn unsubscribe(&mut self, channels: &[String]) {
        let mut diff = Vec::<String>::new();
        for ch in channels.iter() {
            if self.channels.remove(ch) {
                diff.push(ch.clone());
            }
        }
        if !diff.is_empty() {
            let commands = (self.serialize_command)(channels, false);
            commands.into_iter().for_each(|command| {
                let res = self.ws_stream.write_message(Message::Text(command));
                if let Err(err) = res {
                    error!("{}", err);
                }
            });
        }
    }

    // reconnect and subscribe all channels
    fn reconnect(&mut self) {
        warn!("Reconnecting to {}", &self.url);
        self.ws_stream = connect_with_retry(&self.url);
        let channels = self
            .channels
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if !channels.is_empty() {
            let commands = (self.serialize_command)(&channels, true);
            commands.into_iter().for_each(|command| {
                let resp = self.ws_stream.write_message(Message::Text(command));
                if let Err(err) = resp {
                    error!("{}", err);
                }
            });
        }
    }

    // Handle a text msg from Message::Text or Message::Binary
    fn handle_msg(&mut self, txt: &str) {
        if txt.contains("error") {
            error!("{}", txt);
            return;
        }
        self.count += 1;

        let resp = serde_json::from_str::<Value>(&txt);
        if resp.is_err() {
            error!("{} is not a JSON string", txt);
            return;
        }
        let value = resp.unwrap();

        // Exchange specific handling
        match self.exchange.as_str() {
            super::binance::EXCHANGE_NAME => {
                let obj = value.as_object().unwrap();
                if obj.contains_key("stream") || obj.contains_key("data") {
                    warn!("Received {} from {}", txt, self.url);
                    return;
                }
            }
            super::bitmex::EXCHANGE_NAME => {
                let obj = value.as_object().unwrap();
                if !obj.contains_key("table") {
                    warn!("Received {} from {}", txt, self.url);
                    return;
                }
            }
            super::bitstamp::EXCHANGE_NAME => {
                let obj = value.as_object().unwrap();
                let event = obj.get("event").unwrap().as_str().unwrap();
                match event {
                    "bts:subscription_succeeded" | "bts:unsubscription_succeeded" => {
                        debug!("Received {} from {}", txt, self.url);
                        return;
                    }
                    "bts:error" => {
                        error!("Received {} from {}", txt, self.url);
                        return;
                    }
                    "bts:request_reconnect" => {
                        warn!("Received {} from {}", txt, self.url);
                        self.reconnect();
                        return;
                    }
                    _ => (),
                }
            }
            super::huobi::EXCHANGE_NAME => {
                let obj = value.as_object().unwrap();
                if obj.contains_key("ping") {
                    let value = obj.get("ping").unwrap();
                    let mut pong_msg = HashMap::<String, &Value>::new();
                    pong_msg.insert("pong".to_string(), value);
                    let response = Message::Text(serde_json::to_string(&pong_msg).unwrap());
                    if let Err(err) = self.ws_stream.write_message(response) {
                        error!("{}", err);
                    }
                    return;
                }

                if let Some(status) = obj.get("status") {
                    if status.as_str().unwrap() != "ok" {
                        error!("Received {} from {}", txt, self.url);
                        return;
                    }
                }

                if !obj.contains_key("ch") || !obj.contains_key("ts") {
                    warn!("Received {} from {}", txt, self.url);
                    return;
                }
            }
            super::kraken::EXCHANGE_NAME => {
                if value.is_object() {
                    let obj = value.as_object().unwrap();
                    let event = obj.get("event").unwrap().as_str().unwrap();
                    match event {
                        "heartbeat" => {
                            debug!("Received {} from {}", txt, self.url);
                            let ping = r#"{
                                "event": "ping",
                                "reqid": 9527
                            }"#;
                            let _ = self
                                .ws_stream
                                .write_message(Message::Text(ping.to_string()));
                        }
                        "pong" => {
                            debug!("Received {} from {}", txt, self.url);
                        }
                        _ => {
                            warn!("Received {} from {}", txt, self.url);
                        }
                    }
                    return;
                }
            }
            super::mxc::EXCHANGE_NAME => (),
            super::okex::EXCHANGE_NAME => {
                let obj = value.as_object().unwrap();
                if let Some(event) = obj.get("event") {
                    if event.as_str().unwrap() == "error" {
                        error!("Received {} from {}", txt, self.url);
                        return;
                    }
                }

                if !obj.contains_key("table") || !obj.contains_key("data") {
                    warn!("Received {} from {}", txt, self.url);
                    return;
                }
            }
            _ => (),
        }

        (self.on_msg)(txt.to_string());
    }

    pub fn run(&mut self, duration: Option<u64>) {
        let now = Instant::now();
        loop {
            let resp = self.ws_stream.read_message();
            match resp {
                Ok(msg) => match msg {
                    Message::Text(txt) => self.handle_msg(&txt),
                    Message::Binary(binary) => {
                        let mut txt = String::new();
                        let resp = if self.url.contains("huobi.com") || self.url.contains("hbdm.com") || self.url.contains("huobi.pro") {
                            let mut decoder = GzDecoder::new(&binary[..]);
                            decoder.read_to_string(&mut txt)
                        } else if self.url.contains("okex.com") {
                            let mut decoder = DeflateDecoder::new(&binary[..]);
                            decoder.read_to_string(&mut txt)
                        } else {
                            panic!("Unknown binary format from {}", self.url)
                        };

                        match resp {
                            Ok(_) => self.handle_msg(&txt),
                            Err(err) => error!("Decompression failed, {}", err),
                        }
                    }
                    Message::Ping(resp) => {
                        let tmp = std::str::from_utf8(&resp);
                        warn!("Received a ping frame: {}", tmp.unwrap());
                    }
                    Message::Pong(resp) => {
                        let tmp = std::str::from_utf8(&resp);
                        warn!("Received a pong frame: {}", tmp.unwrap());
                    }
                    Message::Close(resp) => {
                        match resp {
                            Some(frame) => warn!("Received a Message::Close message with a CloseFrame: code: {}, reason: {}", frame.code, frame.reason),
                            None => warn!("Received a close message without CloseFrame"),
                        }
                    }
                },
                Err(err) => match err {
                    Error::ConnectionClosed => {
                        warn!("tungstenite::Error::ConnectionClosed");
                        self.reconnect();
                    }
                    _ => error!("{}", err),
                },
            }

            if let Some(seconds) = duration {
                if now.elapsed() > Duration::from_secs(seconds) {
                    break;
                }
            }
        }
    }

    pub fn close(&mut self) {
        let res = self.ws_stream.close(None);
        if let Err(err) = res {
            error!("{}", err);
        }
    }
}

/// Define exchange specific client.
macro_rules! define_client {
    ($struct_name:ident, $exchange:ident, $default_url:ident, $serialize_command:ident) => {
        impl<'a> WSClient<'a> for $struct_name<'a> {
            fn new(on_msg: Box<dyn FnMut(String) + 'a>, url: Option<&str>) -> $struct_name<'a> {
                let real_url = match url {
                    Some(endpoint) => endpoint,
                    None => $default_url,
                };
                $struct_name {
                    client: WSClientInternal::new($exchange, real_url, on_msg, $serialize_command),
                }
            }

            fn subscribe(&mut self, channels: &[String]) {
                self.client.subscribe(channels);
            }

            fn unsubscribe(&mut self, channels: &[String]) {
                self.client.unsubscribe(channels);
            }

            fn run(&mut self, duration: Option<u64>) {
                self.client.run(duration);
            }

            fn close(&mut self) {
                self.client.close();
            }
        }
    };
}
