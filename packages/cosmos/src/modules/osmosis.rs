pub mod accum {
    pub mod v1beta1 {
        include!("osmosis/osmosis.accum.v1beta1.rs");
    }
}
pub mod concentratedliquidity {
    include!("osmosis/osmosis.concentratedliquidity.rs");
    pub mod poolmodel {
        pub mod concentrated {
            pub mod v1beta1 {
                include!("osmosis/osmosis.concentratedliquidity.poolmodel.concentrated.v1beta1.rs");
            }
        }
    }
    pub mod v1beta1 {
        include!("osmosis/osmosis.concentratedliquidity.v1beta1.rs");
    }
}
pub mod cosmwasmpool {
    pub mod v1beta1 {
        include!("osmosis/osmosis.cosmwasmpool.v1beta1.rs");
    }
}
pub mod gamm {
    pub mod poolmodels {
        pub mod balancer {
            pub mod v1beta1 {
                include!("osmosis/osmosis.gamm.poolmodels.balancer.v1beta1.rs");
            }
        }
        pub mod stableswap {
            pub mod v1beta1 {
                include!("osmosis/osmosis.gamm.poolmodels.stableswap.v1beta1.rs");
            }
        }
    }
    pub mod v1beta1 {
        include!("osmosis/osmosis.gamm.v1beta1.rs");
    }
    pub mod v2 {
        include!("osmosis/osmosis.gamm.v2.rs");
    }
}
pub mod lockup {
    include!("osmosis/osmosis.lockup.rs");
}
pub mod poolmanager {
    pub mod v1beta1 {
        include!("osmosis/osmosis.poolmanager.v1beta1.rs");
    }
    pub mod v2 {
        include!("osmosis/osmosis.poolmanager.v2.rs");
    }
}
pub mod tokenfactory {
    pub mod v1beta1 {
        include!("osmosis/osmosis.tokenfactory.v1beta1.rs");
    }
}
pub mod twap {
    pub mod v1beta1 {
        include!("osmosis/osmosis.twap.v1beta1.rs");
    }
}
pub mod txfees {
    pub mod v1beta1 {
        include!("osmosis/osmosis.txfees.v1beta1.rs");
    }
}
pub mod meta {
    pub const REPOSITORY: &str = "https://github.com/osmosis-labs/osmosis";
    pub const TAG: &str = "v25.0.0";
}

