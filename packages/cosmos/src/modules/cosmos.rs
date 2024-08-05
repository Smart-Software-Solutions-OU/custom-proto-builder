pub mod app {
    pub mod v1alpha1 {
        include!("cosmos/cosmos.app.v1alpha1.rs");
    }
}
pub mod auth {
    pub mod module {
        pub mod v1 {
            include!("cosmos/cosmos.auth.module.v1.rs");
        }
    }
    pub mod v1beta1 {
        include!("cosmos/cosmos.auth.v1beta1.rs");
    }
}
pub mod authz {
    pub mod module {
        pub mod v1 {
            include!("cosmos/cosmos.authz.module.v1.rs");
        }
    }
    pub mod v1beta1 {
        include!("cosmos/cosmos.authz.v1beta1.rs");
    }
}
pub mod bank {
    pub mod module {
        pub mod v1 {
            include!("cosmos/cosmos.bank.module.v1.rs");
        }
    }
    pub mod v1beta1 {
        include!("cosmos/cosmos.bank.v1beta1.rs");
    }
}
pub mod base {
    pub mod abci {
        pub mod v1beta1 {
            include!("cosmos/cosmos.base.abci.v1beta1.rs");
        }
    }
    pub mod kv {
        pub mod v1beta1 {
            include!("cosmos/cosmos.base.kv.v1beta1.rs");
        }
    }
    pub mod node {
        pub mod v1beta1 {
            include!("cosmos/cosmos.base.node.v1beta1.rs");
        }
    }
    pub mod query {
        pub mod v1beta1 {
            include!("cosmos/cosmos.base.query.v1beta1.rs");
        }
    }
    pub mod reflection {
        pub mod v1beta1 {
            include!("cosmos/cosmos.base.reflection.v1beta1.rs");
        }
        pub mod v2alpha1 {
            include!("cosmos/cosmos.base.reflection.v2alpha1.rs");
        }
    }
    pub mod snapshots {
        pub mod v1beta1 {
            include!("cosmos/cosmos.base.snapshots.v1beta1.rs");
        }
    }
    pub mod store {
        pub mod v1beta1 {
            include!("cosmos/cosmos.base.store.v1beta1.rs");
        }
    }
    pub mod tendermint {
        pub mod v1beta1 {
            include!("cosmos/cosmos.base.tendermint.v1beta1.rs");
        }
    }
    pub mod v1beta1 {
        include!("cosmos/cosmos.base.v1beta1.rs");
    }
}
pub mod consensus {
    pub mod v1 {
        include!("cosmos/cosmos.consensus.v1.rs");
    }
}
pub mod crisis {
    pub mod v1beta1 {
        include!("cosmos/cosmos.crisis.v1beta1.rs");
    }
}
pub mod crypto {
    pub mod ed25519 {
        include!("cosmos/cosmos.crypto.ed25519.rs");
    }
    pub mod hd {
        pub mod v1 {
            include!("cosmos/cosmos.crypto.hd.v1.rs");
        }
    }
    pub mod keyring {
        pub mod v1 {
            include!("cosmos/cosmos.crypto.keyring.v1.rs");
        }
    }
    pub mod multisig {
        include!("cosmos/cosmos.crypto.multisig.rs");
        pub mod v1beta1 {
            include!("cosmos/cosmos.crypto.multisig.v1beta1.rs");
        }
    }
    pub mod secp256k1 {
        include!("cosmos/cosmos.crypto.secp256k1.rs");
    }
    pub mod secp256r1 {
        include!("cosmos/cosmos.crypto.secp256r1.rs");
    }
}
pub mod distribution {
    pub mod v1beta1 {
        include!("cosmos/cosmos.distribution.v1beta1.rs");
    }
}
pub mod evidence {
    pub mod v1beta1 {
        include!("cosmos/cosmos.evidence.v1beta1.rs");
    }
}
pub mod feegrant {
    pub mod v1beta1 {
        include!("cosmos/cosmos.feegrant.v1beta1.rs");
    }
}
pub mod gov {
    pub mod v1 {
        include!("cosmos/cosmos.gov.v1.rs");
    }
    pub mod v1beta1 {
        include!("cosmos/cosmos.gov.v1beta1.rs");
    }
}
pub mod group {
    pub mod v1 {
        include!("cosmos/cosmos.group.v1.rs");
    }
}
pub mod ics23 {
    pub mod v1 {
        include!("cosmos/cosmos.ics23.v1.rs");
    }
}
pub mod mint {
    pub mod v1beta1 {
        include!("cosmos/cosmos.mint.v1beta1.rs");
    }
}
pub mod nft {
    pub mod v1beta1 {
        include!("cosmos/cosmos.nft.v1beta1.rs");
    }
}
pub mod slashing {
    pub mod v1beta1 {
        include!("cosmos/cosmos.slashing.v1beta1.rs");
    }
}
pub mod staking {
    pub mod module {
        pub mod v1 {
            include!("cosmos/cosmos.staking.module.v1.rs");
        }
    }
    pub mod v1beta1 {
        include!("cosmos/cosmos.staking.v1beta1.rs");
    }
}
pub mod tx {
    pub mod config {
        pub mod v1 {
            include!("cosmos/cosmos.tx.config.v1.rs");
        }
    }
    pub mod signing {
        pub mod v1beta1 {
            include!("cosmos/cosmos.tx.signing.v1beta1.rs");
        }
    }
    pub mod v1beta1 {
        include!("cosmos/cosmos.tx.v1beta1.rs");
    }
}
pub mod upgrade {
    pub mod v1beta1 {
        include!("cosmos/cosmos.upgrade.v1beta1.rs");
    }
}
pub mod vesting {
    pub mod v1beta1 {
        include!("cosmos/cosmos.vesting.v1beta1.rs");
    }
}
pub mod meta {
    pub const REPOSITORY: &str = "https://github.com/cosmos/ics23";
    pub const TAG: &str = "rust/v0.10.0";
}

