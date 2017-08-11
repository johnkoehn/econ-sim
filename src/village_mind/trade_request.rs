#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum TradeType {
    Buy,
    Sell,
}

pub struct TradeRequest {
    pub trade_type : TradeType,
    pub amount : f64,
}

impl TradeRequest {
    pub fn new(trade_type : TradeType, amount : f64) -> TradeRequest{
        TradeRequest {
            trade_type: trade_type,
            amount: amount,
        }
    }
}