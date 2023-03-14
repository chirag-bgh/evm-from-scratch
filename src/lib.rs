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

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

pub fn evm(_code: impl AsRef<[u8]>) -> EvmResult {
    let stack: Vec<U256> = Vec::new();
    let mut pc = 0;

    let code = _code.as_ref();

    while pc < code.len() {
        let opcode = code[pc];
        pc += 1;

        if opcode == 0x00 {
            // STOP
            return EvmResult {
                stack: stack,
                success: true,
            };
        }
    }

    // TODO: Implement me

    return EvmResult {
        stack: stack,
        success: true,
    };
}
