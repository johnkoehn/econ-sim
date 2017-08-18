use village::resource::ResourceType;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum TradeType {
    Buy,
    Sell,
}

pub struct TradeRequest {
    pub trade_type : TradeType,
    pub resource_type : ResourceType,
    pub amount : u32,
    pub success : bool,
}

impl TradeRequest {
    pub fn new(trade_type: TradeType, amount: u32, resource_type: ResourceType) -> TradeRequest{
        TradeRequest {
            trade_type: trade_type,
            amount: amount,
            resource_type: resource_type,
            success: false,
        }
    }
}