use crypto_ws_client::{DeribitWSClient, WSClient};
use std::sync::{Arc, Mutex};

#[macro_use]
mod utils;

#[test]
fn deribit_all_trades() {
    gen_test_code!(
        DeribitWSClient,
        subscribe,
        // https://docs.deribit.com/?javascript#trades-kind-currency-interval
        &vec![
            "trades.future.any.raw".to_string(),
            "trades.option.any.raw".to_string(),
        ]
    );
}

#[cfg(test)]
mod deribit_inverse_future {
    use crypto_ws_client::{DeribitWSClient, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    fn subscribe() {
        gen_test_code!(
            DeribitWSClient,
            subscribe,
            &vec!["trades.future.BTC.raw".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_trade,
            &vec!["BTC-25JUN21".to_string(), "BTC-25JUN21".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_ticker,
            &vec!["BTC-25JUN21".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_orderbook,
            &vec!["BTC-25JUN21".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_orderbook_snapshot,
            &vec!["BTC-25JUN21".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(DeribitWSClient, &vec!["BTC-25JUN21".to_string()], 60);
        gen_test_subscribe_candlestick!(DeribitWSClient, &vec!["BTC-25JUN21".to_string()], 86400);
    }
}

#[cfg(test)]
mod deribit_inverse_swap {
    use crypto_ws_client::{DeribitWSClient, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    fn subscribe() {
        gen_test_code!(
            DeribitWSClient,
            subscribe,
            &vec!["trades.BTC-PERPETUAL.raw".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_trade,
            &vec!["BTC-PERPETUAL".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_ticker,
            &vec!["BTC-PERPETUAL".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_orderbook,
            &vec!["BTC-PERPETUAL".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_orderbook_snapshot,
            &vec!["BTC-PERPETUAL".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(DeribitWSClient, &vec!["BTC-PERPETUAL".to_string()], 60);
        gen_test_subscribe_candlestick!(DeribitWSClient, &vec!["BTC-PERPETUAL".to_string()], 86400);
    }
}

#[cfg(test)]
mod deribit_option {
    use crypto_ws_client::{DeribitWSClient, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    fn subscribe() {
        gen_test_code!(
            DeribitWSClient,
            subscribe,
            &vec!["trades.option.any.raw".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_trade() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_trade,
            &vec![
                "BTC-2APR21-58000-C	".to_string(),
                "BTC-9APR21-60000-C".to_string(),
                "BTC-2APR21-50000-P".to_string(),
                "BTC-4JUN21-26000-P".to_string(),
            ]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_ticker,
            &vec!["BTC-4JUN21-26000-P".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_orderbook,
            &vec!["BTC-4JUN21-26000-P".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_orderbook_snapshot,
            &vec!["BTC-4JUN21-26000-P".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            DeribitWSClient,
            &vec!["BTC-4JUN21-26000-P".to_string()],
            60
        );
        gen_test_subscribe_candlestick!(
            DeribitWSClient,
            &vec!["BTC-4JUN21-26000-P".to_string()],
            86400
        );
    }
}
