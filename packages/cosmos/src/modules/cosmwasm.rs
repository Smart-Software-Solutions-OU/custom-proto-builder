pub mod wasm {
    pub mod v1 {
        include!("cosmwasm/cosmwasm.wasm.v1.rs");
    }
}
pub mod meta {
    pub const REPOSITORY: &str = "https://github.com/CosmWasm/wasmd";
    pub const TAG: &str = "v0.45.0";
}

