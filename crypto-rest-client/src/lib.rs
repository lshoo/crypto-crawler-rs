mod error;
mod exchanges;

pub use error::Error;
pub use exchanges::binance::binance_future::BinanceFutureRestClient;
pub use exchanges::binance::binance_inverse_swap::BinanceInverseSwapRestClient;
pub use exchanges::binance::binance_linear_swap::BinanceLinearSwapRestClient;
pub use exchanges::binance::binance_option::BinanceOptionRestClient;
pub use exchanges::binance::binance_spot::BinanceSpotRestClient;
pub use exchanges::bitfinex::BitfinexRestClient;
pub use exchanges::bitget::*;
pub use exchanges::bitmex::BitmexRestClient;
pub use exchanges::bitstamp::BitstampRestClient;
pub use exchanges::bitz::*;
pub use exchanges::bybit::BybitRestClient;
pub use exchanges::coinbase_pro::CoinbaseProRestClient;
pub use exchanges::deribit::DeribitRestClient;
pub use exchanges::ftx::FtxRestClient;
pub use exchanges::huobi::huobi_future::HuobiFutureRestClient;
pub use exchanges::huobi::huobi_inverse_swap::HuobiInverseSwapRestClient;
pub use exchanges::huobi::huobi_linear_swap::HuobiLinearSwapRestClient;
pub use exchanges::huobi::huobi_option::HuobiOptionRestClient;
pub use exchanges::huobi::huobi_spot::HuobiSpotRestClient;
pub use exchanges::kraken::KrakenRestClient;
pub use exchanges::mxc::mxc_spot::MxcSpotRestClient;
pub use exchanges::mxc::mxc_swap::MxcSwapRestClient;
pub use exchanges::okex::OkexRestClient;
pub use exchanges::zbg::*;
