// This file is @generated by prost-build.
/// ValidatorSigningInfo defines a validator's signing info for monitoring their
/// liveness activity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSigningInfo {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Height at which validator was first a candidate OR was unjailed
    #[prost(int64, tag = "2")]
    pub start_height: i64,
    /// Index which is incremented each time the validator was a bonded
    /// in a block and may have signed a precommit or not. This in conjunction with the
    /// `SignedBlocksWindow` param determines the index in the `MissedBlocksBitArray`.
    #[prost(int64, tag = "3")]
    pub index_offset: i64,
    /// Timestamp until which the validator is jailed due to liveness downtime.
    #[prost(message, optional, tag = "4")]
    pub jailed_until: ::core::option::Option<::prost_types::Timestamp>,
    /// Whether or not a validator has been tombstoned (killed out of validator set). It is set
    /// once the validator commits an equivocation or for any other configured misbehiavor.
    #[prost(bool, tag = "5")]
    pub tombstoned: bool,
    /// A counter kept to avoid unnecessary array reads.
    /// Note that `Sum(MissedBlocksBitArray)` always equals `MissedBlocksCounter`.
    #[prost(int64, tag = "6")]
    pub missed_blocks_counter: i64,
}
impl ::prost::Name for ValidatorSigningInfo {
    const NAME: &'static str = "ValidatorSigningInfo";
    const PACKAGE: &'static str = "cosmos.slashing.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "cosmos.slashing.v1beta1.ValidatorSigningInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/cosmos.slashing.v1beta1.ValidatorSigningInfo".into()
    }
}
/// Params represents the parameters used for by the slashing module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(int64, tag = "1")]
    pub signed_blocks_window: i64,
    #[prost(bytes = "vec", tag = "2")]
    pub min_signed_per_window: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub downtime_jail_duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(bytes = "vec", tag = "4")]
    pub slash_fraction_double_sign: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub slash_fraction_downtime: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "cosmos.slashing.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "cosmos.slashing.v1beta1.Params".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/cosmos.slashing.v1beta1.Params".into()
    }
}
/// MsgUnjail defines the Msg/Unjail request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnjail {
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgUnjail {
    const NAME: &'static str = "MsgUnjail";
    const PACKAGE: &'static str = "cosmos.slashing.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "cosmos.slashing.v1beta1.MsgUnjail".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/cosmos.slashing.v1beta1.MsgUnjail".into()
    }
}
/// MsgUnjailResponse defines the Msg/Unjail response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnjailResponse {}
impl ::prost::Name for MsgUnjailResponse {
    const NAME: &'static str = "MsgUnjailResponse";
    const PACKAGE: &'static str = "cosmos.slashing.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "cosmos.slashing.v1beta1.MsgUnjailResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/cosmos.slashing.v1beta1.MsgUnjailResponse".into()
    }
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
///
/// Since: cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/slashing parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for MsgUpdateParams {
    const NAME: &'static str = "MsgUpdateParams";
    const PACKAGE: &'static str = "cosmos.slashing.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "cosmos.slashing.v1beta1.MsgUpdateParams".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/cosmos.slashing.v1beta1.MsgUpdateParams".into()
    }
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
///
/// Since: cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
impl ::prost::Name for MsgUpdateParamsResponse {
    const NAME: &'static str = "MsgUpdateParamsResponse";
    const PACKAGE: &'static str = "cosmos.slashing.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "cosmos.slashing.v1beta1.MsgUpdateParamsResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/cosmos.slashing.v1beta1.MsgUpdateParamsResponse".into()
    }
}
/// Generated client implementations.
#[cfg(feature = "cosmos")]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Msg defines the slashing Msg service.
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MsgClient<tonic::transport::Channel> {
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
    impl<T> MsgClient<T>
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
        ) -> MsgClient<InterceptedService<T, F>>
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
            MsgClient::new(InterceptedService::new(inner, interceptor))
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
        /// Unjail defines a method for unjailing a jailed validator, thus returning
        /// them into the bonded validator set, so they can begin receiving provisions
        /// and rewards again.
        pub async fn unjail(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUnjail>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUnjailResponse>,
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
                "/cosmos.slashing.v1beta1.Msg/Unjail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.slashing.v1beta1.Msg", "Unjail"));
            self.inner.unary(req, path, codec).await
        }
        /// UpdateParams defines a governance operation for updating the x/slashing module
        /// parameters. The authority defaults to the x/gov module account.
        ///
        /// Since: cosmos-sdk 0.47
        pub async fn update_params(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateParams>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateParamsResponse>,
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
                "/cosmos.slashing.v1beta1.Msg/UpdateParams",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.slashing.v1beta1.Msg", "UpdateParams"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "cosmos")]
pub mod msg_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MsgServer.
    #[async_trait]
    pub trait Msg: Send + Sync + 'static {
        /// Unjail defines a method for unjailing a jailed validator, thus returning
        /// them into the bonded validator set, so they can begin receiving provisions
        /// and rewards again.
        async fn unjail(
            &self,
            request: tonic::Request<super::MsgUnjail>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUnjailResponse>,
            tonic::Status,
        >;
        /// UpdateParams defines a governance operation for updating the x/slashing module
        /// parameters. The authority defaults to the x/gov module account.
        ///
        /// Since: cosmos-sdk 0.47
        async fn update_params(
            &self,
            request: tonic::Request<super::MsgUpdateParams>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateParamsResponse>,
            tonic::Status,
        >;
    }
    /// Msg defines the slashing Msg service.
    #[derive(Debug)]
    pub struct MsgServer<T: Msg> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Msg> MsgServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MsgServer<T>
    where
        T: Msg,
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
                "/cosmos.slashing.v1beta1.Msg/Unjail" => {
                    #[allow(non_camel_case_types)]
                    struct UnjailSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUnjail>
                    for UnjailSvc<T> {
                        type Response = super::MsgUnjailResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUnjail>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::unjail(&inner, request).await
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
                        let method = UnjailSvc(inner);
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
                "/cosmos.slashing.v1beta1.Msg/UpdateParams" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateParamsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdateParams>
                    for UpdateParamsSvc<T> {
                        type Response = super::MsgUpdateParamsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateParams>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::update_params(&inner, request).await
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
                        let method = UpdateParamsSvc(inner);
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
    impl<T: Msg> Clone for MsgServer<T> {
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
    impl<T: Msg> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Msg> tonic::server::NamedService for MsgServer<T> {
        const NAME: &'static str = "cosmos.slashing.v1beta1.Msg";
    }
}
