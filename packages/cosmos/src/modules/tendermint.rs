pub mod abci {
    include!("tendermint/tendermint.abci.rs");
}
pub mod blocksync {
    include!("tendermint/tendermint.blocksync.rs");
}
pub mod consensus {
    include!("tendermint/tendermint.consensus.rs");
}
pub mod crypto {
    include!("tendermint/tendermint.crypto.rs");
}
pub mod libs {
    pub mod bits {
        include!("tendermint/tendermint.libs.bits.rs");
    }
}
pub mod mempool {
    include!("tendermint/tendermint.mempool.rs");
}
pub mod p2p {
    include!("tendermint/tendermint.p2p.rs");
}
pub mod privval {
    include!("tendermint/tendermint.privval.rs");
}
pub mod rpc {
    pub mod grpc {
        include!("tendermint/tendermint.rpc.grpc.rs");
    }
}
pub mod state {
    include!("tendermint/tendermint.state.rs");
}
pub mod statesync {
    include!("tendermint/tendermint.statesync.rs");
}
pub mod store {
    include!("tendermint/tendermint.store.rs");
}
pub mod types {
    include!("tendermint/tendermint.types.rs");
}
pub mod version {
    include!("tendermint/tendermint.version.rs");
}
pub mod meta {
    pub const REPOSITORY: &str = "https://github.com/cometbft/cometbft";
    pub const TAG: &str = "v0.37.2";
}

