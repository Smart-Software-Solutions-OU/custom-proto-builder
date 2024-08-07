// This file is @generated by prost-build.
/// A TWAP record should be indexed in state by pool_id, (asset pair), timestamp
/// The asset pair assets should be lexicographically sorted.
/// Technically (pool_id, asset_0_denom, asset_1_denom, height) do not need to
/// appear in the struct however we view this as the wrong performance tradeoff
/// given SDK today. Would rather we optimize for readability and correctness,
/// than an optimal state storage format. The system bottleneck is elsewhere for
/// now.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TwapRecord {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    /// Lexicographically smaller denom of the pair
    #[prost(string, tag = "2")]
    pub asset0_denom: ::prost::alloc::string::String,
    /// Lexicographically larger denom of the pair
    #[prost(string, tag = "3")]
    pub asset1_denom: ::prost::alloc::string::String,
    /// height this record corresponds to, for debugging purposes
    #[prost(int64, tag = "4")]
    pub height: i64,
    /// This field should only exist until we have a global registry in the state
    /// machine, mapping prior block heights within {TIME RANGE} to times.
    #[prost(message, optional, tag = "5")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// We store the last spot prices in the struct, so that we can interpolate
    /// accumulator values for times between when accumulator records are stored.
    #[prost(string, tag = "6")]
    pub p0_last_spot_price: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub p1_last_spot_price: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub p0_arithmetic_twap_accumulator: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub p1_arithmetic_twap_accumulator: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub geometric_twap_accumulator: ::prost::alloc::string::String,
    /// This field contains the time in which the last spot price error occurred.
    /// It is used to alert the caller if they are getting a potentially erroneous
    /// TWAP, due to an unforeseen underlying error.
    #[prost(message, optional, tag = "11")]
    pub last_error_time: ::core::option::Option<::prost_types::Timestamp>,
}
impl ::prost::Name for TwapRecord {
    const NAME: &'static str = "TwapRecord";
    const PACKAGE: &'static str = "osmosis.twap.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "osmosis.twap.v1beta1.TwapRecord".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/osmosis.twap.v1beta1.TwapRecord".into()
    }
}
/// PruningState allows us to spread out the pruning of TWAP records over time,
/// instead of pruning all at once at the end of the epoch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PruningState {
    /// is_pruning is true if the pruning process is ongoing.
    /// This tells the module to continue pruning the TWAP records
    /// at the EndBlock.
    #[prost(bool, tag = "1")]
    pub is_pruning: bool,
    /// last_kept_time is the time of the last kept TWAP record.
    /// This is used to determine all TWAP records that are older than
    /// last_kept_time and should be pruned.
    #[prost(message, optional, tag = "2")]
    pub last_kept_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Deprecated: This field is deprecated.
    #[deprecated]
    #[prost(bytes = "vec", tag = "3")]
    pub last_key_seen: ::prost::alloc::vec::Vec<u8>,
    /// last_seen_pool_id is the pool_id that we will begin pruning in the next
    /// block. This value starts at the highest pool_id at time of epoch, and
    /// decreases until it reaches 1. When it reaches 1, the pruning
    /// process is complete.
    #[prost(uint64, tag = "4")]
    pub last_seen_pool_id: u64,
}
impl ::prost::Name for PruningState {
    const NAME: &'static str = "PruningState";
    const PACKAGE: &'static str = "osmosis.twap.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "osmosis.twap.v1beta1.PruningState".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/osmosis.twap.v1beta1.PruningState".into()
    }
}
/// Params holds parameters for the twap module
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, tag = "1")]
    pub prune_epoch_identifier: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub record_history_keep_period: ::core::option::Option<::prost_types::Duration>,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "osmosis.twap.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "osmosis.twap.v1beta1.Params".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/osmosis.twap.v1beta1.Params".into()
    }
}
/// GenesisState defines the twap module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// twaps is the collection of all twap records.
    #[prost(message, repeated, tag = "1")]
    pub twaps: ::prost::alloc::vec::Vec<TwapRecord>,
    /// params is the container of twap parameters.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "osmosis.twap.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "osmosis.twap.v1beta1.GenesisState".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/osmosis.twap.v1beta1.GenesisState".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArithmeticTwapRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub base_asset: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_asset: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "5")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
impl ::prost::Name for ArithmeticTwapRequest {
    const NAME: &'static str = "ArithmeticTwapRequest";
    const PACKAGE: &'static str = "osmosis.twap.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "osmosis.twap.v1beta1.ArithmeticTwapRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/osmosis.twap.v1beta1.ArithmeticTwapRequest".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArithmeticTwapResponse {
    #[prost(string, tag = "1")]
    pub arithmetic_twap: ::prost::alloc::string::String,
}
impl ::prost::Name for ArithmeticTwapResponse {
    const NAME: &'static str = "ArithmeticTwapResponse";
    const PACKAGE: &'static str = "osmosis.twap.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "osmosis.twap.v1beta1.ArithmeticTwapResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/osmosis.twap.v1beta1.ArithmeticTwapResponse".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArithmeticTwapToNowRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub base_asset: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_asset: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
}
impl ::prost::Name for ArithmeticTwapToNowRequest {
    const NAME: &'static str = "ArithmeticTwapToNowRequest";
    const PACKAGE: &'static str = "osmosis.twap.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "osmosis.twap.v1beta1.ArithmeticTwapToNowRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/osmosis.twap.v1beta1.ArithmeticTwapToNowRequest".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArithmeticTwapToNowResponse {
    #[prost(string, tag = "1")]
    pub arithmetic_twap: ::prost::alloc::string::String,
}
impl ::prost::Name for ArithmeticTwapToNowResponse {
    const NAME: &'static str = "ArithmeticTwapToNowResponse";
    const PACKAGE: &'static str = "osmosis.twap.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "osmosis.twap.v1beta1.ArithmeticTwapToNowResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/osmosis.twap.v1beta1.ArithmeticTwapToNowResponse".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeometricTwapRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub base_asset: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_asset: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "5")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
impl ::prost::Name for GeometricTwapRequest {
    const NAME: &'static str = "GeometricTwapRequest";
    const PACKAGE: &'static str = "osmosis.twap.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "osmosis.twap.v1beta1.GeometricTwapRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/osmosis.twap.v1beta1.GeometricTwapRequest".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeometricTwapResponse {
    #[prost(string, tag = "1")]
    pub geometric_twap: ::prost::alloc::string::String,
}
impl ::prost::Name for GeometricTwapResponse {
    const NAME: &'static str = "GeometricTwapResponse";
    const PACKAGE: &'static str = "osmosis.twap.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "osmosis.twap.v1beta1.GeometricTwapResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/osmosis.twap.v1beta1.GeometricTwapResponse".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeometricTwapToNowRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub base_asset: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_asset: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
}
impl ::prost::Name for GeometricTwapToNowRequest {
    const NAME: &'static str = "GeometricTwapToNowRequest";
    const PACKAGE: &'static str = "osmosis.twap.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "osmosis.twap.v1beta1.GeometricTwapToNowRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/osmosis.twap.v1beta1.GeometricTwapToNowRequest".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeometricTwapToNowResponse {
    #[prost(string, tag = "1")]
    pub geometric_twap: ::prost::alloc::string::String,
}
impl ::prost::Name for GeometricTwapToNowResponse {
    const NAME: &'static str = "GeometricTwapToNowResponse";
    const PACKAGE: &'static str = "osmosis.twap.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "osmosis.twap.v1beta1.GeometricTwapToNowResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/osmosis.twap.v1beta1.GeometricTwapToNowResponse".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamsRequest {}
impl ::prost::Name for ParamsRequest {
    const NAME: &'static str = "ParamsRequest";
    const PACKAGE: &'static str = "osmosis.twap.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "osmosis.twap.v1beta1.ParamsRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/osmosis.twap.v1beta1.ParamsRequest".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for ParamsResponse {
    const NAME: &'static str = "ParamsResponse";
    const PACKAGE: &'static str = "osmosis.twap.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "osmosis.twap.v1beta1.ParamsResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/osmosis.twap.v1beta1.ParamsResponse".into()
    }
}
/// Generated client implementations.
#[cfg(feature = "osmosis")]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::ParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::ParamsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.twap.v1beta1.Query/Params",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.twap.v1beta1.Query", "Params"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn arithmetic_twap(
            &mut self,
            request: impl tonic::IntoRequest<super::ArithmeticTwapRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArithmeticTwapResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.twap.v1beta1.Query/ArithmeticTwap",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.twap.v1beta1.Query", "ArithmeticTwap"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn arithmetic_twap_to_now(
            &mut self,
            request: impl tonic::IntoRequest<super::ArithmeticTwapToNowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArithmeticTwapToNowResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.twap.v1beta1.Query/ArithmeticTwapToNow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("osmosis.twap.v1beta1.Query", "ArithmeticTwapToNow"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn geometric_twap(
            &mut self,
            request: impl tonic::IntoRequest<super::GeometricTwapRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GeometricTwapResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.twap.v1beta1.Query/GeometricTwap",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.twap.v1beta1.Query", "GeometricTwap"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn geometric_twap_to_now(
            &mut self,
            request: impl tonic::IntoRequest<super::GeometricTwapToNowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GeometricTwapToNowResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.twap.v1beta1.Query/GeometricTwapToNow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("osmosis.twap.v1beta1.Query", "GeometricTwapToNow"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "osmosis")]
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QueryServer.
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        async fn params(
            &self,
            request: tonic::Request<super::ParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::ParamsResponse>, tonic::Status>;
        async fn arithmetic_twap(
            &self,
            request: tonic::Request<super::ArithmeticTwapRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArithmeticTwapResponse>,
            tonic::Status,
        >;
        async fn arithmetic_twap_to_now(
            &self,
            request: tonic::Request<super::ArithmeticTwapToNowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArithmeticTwapToNowResponse>,
            tonic::Status,
        >;
        async fn geometric_twap(
            &self,
            request: tonic::Request<super::GeometricTwapRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GeometricTwapResponse>,
            tonic::Status,
        >;
        async fn geometric_twap_to_now(
            &self,
            request: tonic::Request<super::GeometricTwapToNowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GeometricTwapToNowResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct QueryServer<T: Query> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Query> QueryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QueryServer<T>
    where
        T: Query,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/osmosis.twap.v1beta1.Query/Params" => {
                    #[allow(non_camel_case_types)]
                    struct ParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::ParamsRequest>
                    for ParamsSvc<T> {
                        type Response = super::ParamsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ParamsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::params(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ParamsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.twap.v1beta1.Query/ArithmeticTwap" => {
                    #[allow(non_camel_case_types)]
                    struct ArithmeticTwapSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::ArithmeticTwapRequest>
                    for ArithmeticTwapSvc<T> {
                        type Response = super::ArithmeticTwapResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ArithmeticTwapRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::arithmetic_twap(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ArithmeticTwapSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.twap.v1beta1.Query/ArithmeticTwapToNow" => {
                    #[allow(non_camel_case_types)]
                    struct ArithmeticTwapToNowSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::ArithmeticTwapToNowRequest>
                    for ArithmeticTwapToNowSvc<T> {
                        type Response = super::ArithmeticTwapToNowResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ArithmeticTwapToNowRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::arithmetic_twap_to_now(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ArithmeticTwapToNowSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.twap.v1beta1.Query/GeometricTwap" => {
                    #[allow(non_camel_case_types)]
                    struct GeometricTwapSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::GeometricTwapRequest>
                    for GeometricTwapSvc<T> {
                        type Response = super::GeometricTwapResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GeometricTwapRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::geometric_twap(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GeometricTwapSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.twap.v1beta1.Query/GeometricTwapToNow" => {
                    #[allow(non_camel_case_types)]
                    struct GeometricTwapToNowSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::GeometricTwapToNowRequest>
                    for GeometricTwapToNowSvc<T> {
                        type Response = super::GeometricTwapToNowResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GeometricTwapToNowRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::geometric_twap_to_now(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GeometricTwapToNowSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Query> Clone for QueryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Query> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Query> tonic::server::NamedService for QueryServer<T> {
        const NAME: &'static str = "osmosis.twap.v1beta1.Query";
    }
}
