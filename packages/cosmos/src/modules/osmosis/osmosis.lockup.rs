// This file is @generated by prost-build.
/// PeriodLock is a single lock unit by period defined by the x/lockup module.
/// It's a record of a locked coin at a specific time. It stores owner, duration,
/// unlock time and the number of coins locked. A state of a period lock is
/// created upon lock creation, and deleted once the lock has been matured after
/// the `duration` has passed since unbonding started.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeriodLock {
    /// ID is the unique id of the lock.
    /// The ID of the lock is decided upon lock creation, incrementing by 1 for
    /// every lock.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// Owner is the account address of the lock owner.
    /// Only the owner can modify the state of the lock.
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// Duration is the time needed for a lock to mature after unlocking has
    /// started.
    #[prost(message, optional, tag = "3")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// EndTime refers to the time at which the lock would mature and get deleted.
    /// This value is first initialized when an unlock has started for the lock,
    /// end time being block time + duration.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Coins are the tokens locked within the lock, kept in the module account.
    #[prost(message, repeated, tag = "5")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    /// Reward Receiver Address is the address that would be receiving rewards for
    /// the incentives for the lock. This is set to owner by default and can be
    /// changed via separate msg.
    #[prost(string, tag = "6")]
    pub reward_receiver_address: ::prost::alloc::string::String,
}
impl ::prost::Name for PeriodLock {
    const NAME: &'static str = "PeriodLock";
    const PACKAGE: &'static str = "osmosis.lockup";
    fn full_name() -> ::prost::alloc::string::String {
        "osmosis.lockup.PeriodLock".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/osmosis.lockup.PeriodLock".into()
    }
}
/// QueryCondition is a struct used for querying locks upon different conditions.
/// Duration field and timestamp fields could be optional, depending on the
/// LockQueryType.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCondition {
    /// LockQueryType is a type of lock query, ByLockDuration | ByLockTime
    #[prost(enumeration = "LockQueryType", tag = "1")]
    pub lock_query_type: i32,
    /// Denom represents the token denomination we are looking to lock up
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    /// Duration is used to query locks with longer duration than the specified
    /// duration. Duration field must not be nil when the lock query type is
    /// `ByLockDuration`.
    #[prost(message, optional, tag = "3")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// Timestamp is used by locks started before the specified duration.
    /// Timestamp field must not be nil when the lock query type is `ByLockTime`.
    /// Querying locks with timestamp is currently not implemented.
    #[prost(message, optional, tag = "4")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
impl ::prost::Name for QueryCondition {
    const NAME: &'static str = "QueryCondition";
    const PACKAGE: &'static str = "osmosis.lockup";
    fn full_name() -> ::prost::alloc::string::String {
        "osmosis.lockup.QueryCondition".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/osmosis.lockup.QueryCondition".into()
    }
}
/// SyntheticLock is creating virtual lockup where new denom is combination of
/// original denom and synthetic suffix. At the time of synthetic lockup creation
/// and deletion, accumulation store is also being updated and on querier side,
/// they can query as freely as native lockup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyntheticLock {
    /// Underlying Lock ID is the underlying native lock's id for this synthetic
    /// lockup. A synthetic lock MUST have an underlying lock.
    #[prost(uint64, tag = "1")]
    pub underlying_lock_id: u64,
    /// SynthDenom is the synthetic denom that is a combination of
    /// gamm share + bonding status + validator address.
    #[prost(string, tag = "2")]
    pub synth_denom: ::prost::alloc::string::String,
    /// used for unbonding synthetic lockups, for active synthetic lockups, this
    /// value is set to uninitialized value
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Duration is the duration for a synthetic lock to mature
    /// at the point of unbonding has started.
    #[prost(message, optional, tag = "4")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
impl ::prost::Name for SyntheticLock {
    const NAME: &'static str = "SyntheticLock";
    const PACKAGE: &'static str = "osmosis.lockup";
    fn full_name() -> ::prost::alloc::string::String {
        "osmosis.lockup.SyntheticLock".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/osmosis.lockup.SyntheticLock".into()
    }
}
/// LockQueryType defines the type of the lock query that can
/// either be by duration or start time of the lock.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LockQueryType {
    ByDuration = 0,
    ByTime = 1,
    NoLock = 2,
    ByGroup = 3,
}
impl LockQueryType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LockQueryType::ByDuration => "ByDuration",
            LockQueryType::ByTime => "ByTime",
            LockQueryType::NoLock => "NoLock",
            LockQueryType::ByGroup => "ByGroup",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ByDuration" => Some(Self::ByDuration),
            "ByTime" => Some(Self::ByTime),
            "NoLock" => Some(Self::NoLock),
            "ByGroup" => Some(Self::ByGroup),
            _ => None,
        }
    }
}
