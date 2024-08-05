pub mod applications {
    pub mod fee {
        pub mod v1 {
            include!("ibc/ibc.applications.fee.v1.rs");
        }
    }
    pub mod interchain_accounts {
        pub mod controller {
            pub mod v1 {
                include!("ibc/ibc.applications.interchain_accounts.controller.v1.rs");
            }
        }
        pub mod genesis {
            pub mod v1 {
                include!("ibc/ibc.applications.interchain_accounts.genesis.v1.rs");
            }
        }
        pub mod host {
            pub mod v1 {
                include!("ibc/ibc.applications.interchain_accounts.host.v1.rs");
            }
        }
        pub mod v1 {
            include!("ibc/ibc.applications.interchain_accounts.v1.rs");
        }
    }
    pub mod transfer {
        pub mod v1 {
            include!("ibc/ibc.applications.transfer.v1.rs");
        }
        pub mod v2 {
            include!("ibc/ibc.applications.transfer.v2.rs");
        }
    }
}
pub mod core {
    pub mod channel {
        pub mod v1 {
            include!("ibc/ibc.core.channel.v1.rs");
        }
    }
    pub mod client {
        pub mod v1 {
            include!("ibc/ibc.core.client.v1.rs");
        }
    }
    pub mod commitment {
        pub mod v1 {
            include!("ibc/ibc.core.commitment.v1.rs");
        }
    }
    pub mod connection {
        pub mod v1 {
            include!("ibc/ibc.core.connection.v1.rs");
        }
    }
    pub mod types {
        pub mod v1 {
            include!("ibc/ibc.core.types.v1.rs");
        }
    }
}
pub mod meta {
    pub const REPOSITORY: &str = "https://github.com/cosmos/ibc-go";
    pub const TAG: &str = "v7.3.1";
}

