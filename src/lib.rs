use primitive_types::U256;

pub struct BlockConfig {
    pub difficulty: U256,
    pub gas_limit: U256,
    pub timestamp: u64,
}

pub struct TraansactionConfig {
    pub to: Option<String>,
    pub from: Option<String>,
    pub gas_price: U256,
    pub nonce: U256,
}