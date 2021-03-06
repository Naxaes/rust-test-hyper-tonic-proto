/// Points are represented as latitude-longitude pairs in the E7 representation
/// (degrees multiplied by 10**7 and rounded to the nearest integer).
/// Latitudes should be in the range +/- 90 degrees and longitude should be in
/// the range +/- 180 degrees (inclusive).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    #[prost(int32, tag = "1")]
    pub latitude: i32,
    #[prost(int32, tag = "2")]
    pub longitude: i32,
}
/// A latitude-longitude rectangle, represented as two diagonally opposite
/// points "lo" and "hi".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rectangle {
    /// One corner of the rectangle.
    #[prost(message, optional, tag = "1")]
    pub lo: ::std::option::Option<Point>,
    /// The other corner of the rectangle.
    #[prost(message, optional, tag = "2")]
    pub hi: ::std::option::Option<Point>,
}
/// A feature names something at a given point.
///
/// If a feature could not be named, the name is empty.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feature {
    /// The name of the feature.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The point where the feature is detected.
    #[prost(message, optional, tag = "2")]
    pub location: ::std::option::Option<Point>,
}
/// A RouteNote is a message sent while at a given point.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteNote {
    /// The location from which the message is sent.
    #[prost(message, optional, tag = "1")]
    pub location: ::std::option::Option<Point>,
    /// The message to be sent.
    #[prost(string, tag = "2")]
    pub message: std::string::String,
}
/// A RouteSummary is received in response to a RecordRoute rpc.
///
/// It contains the number of individual points received, the number of
/// detected features, and the total distance covered as the cumulative sum of
/// the distance between each point.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteSummary {
    /// The number of points received.
    #[prost(int32, tag = "1")]
    pub point_count: i32,
    /// The number of known features passed while traversing the route.
    #[prost(int32, tag = "2")]
    pub feature_count: i32,
    /// The distance covered in metres.
    #[prost(int32, tag = "3")]
    pub distance: i32,
    /// The duration of the traversal in seconds.
    #[prost(int32, tag = "4")]
    pub elapsed_time: i32,
}
#[doc = r" Generated server implementations."]
pub mod route_guide_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with RouteGuideServer."]
    #[async_trait]
    pub trait RouteGuide: Send + Sync + 'static {
        #[doc = " Obtains the feature at a given position."]
        async fn get_feature(
            &self,
            request: tonic::Request<super::Point>,
        ) -> Result<tonic::Response<super::Feature>, tonic::Status>;
        #[doc = "Server streaming response type for the ListFeatures method."]
        type ListFeaturesStream: Stream<Item = Result<super::Feature, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Obtains the Features available within the given Rectangle.  Results are"]
        #[doc = " streamed rather than returned at once (e.g. in a response message with a"]
        #[doc = " repeated field), as the rectangle may cover a large area and contain a"]
        #[doc = " huge number of features."]
        async fn list_features(
            &self,
            request: tonic::Request<super::Rectangle>,
        ) -> Result<tonic::Response<Self::ListFeaturesStream>, tonic::Status>;
        #[doc = " Accepts a stream of Points on a route being traversed, returning a"]
        #[doc = " RouteSummary when traversal is completed."]
        async fn record_route(
            &self,
            request: tonic::Request<tonic::Streaming<super::Point>>,
        ) -> Result<tonic::Response<super::RouteSummary>, tonic::Status>;
        #[doc = "Server streaming response type for the RouteChat method."]
        type RouteChatStream: Stream<Item = Result<super::RouteNote, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Accepts a stream of RouteNotes sent while a route is being traversed,"]
        #[doc = " while receiving other RouteNotes (e.g. from other users)."]
        async fn route_chat(
            &self,
            request: tonic::Request<tonic::Streaming<super::RouteNote>>,
        ) -> Result<tonic::Response<Self::RouteChatStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct RouteGuideServer<T: RouteGuide> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: RouteGuide> RouteGuideServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for RouteGuideServer<T>
    where
        T: RouteGuide,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/route_guide.RouteGuide/GetFeature" => {
                    #[allow(non_camel_case_types)]
                    struct GetFeatureSvc<T: RouteGuide>(pub Arc<T>);
                    impl<T: RouteGuide> tonic::server::UnaryService<super::Point> for GetFeatureSvc<T> {
                        type Response = super::Feature;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Point>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_feature(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetFeatureSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/route_guide.RouteGuide/ListFeatures" => {
                    #[allow(non_camel_case_types)]
                    struct ListFeaturesSvc<T: RouteGuide>(pub Arc<T>);
                    impl<T: RouteGuide> tonic::server::ServerStreamingService<super::Rectangle> for ListFeaturesSvc<T> {
                        type Response = super::Feature;
                        type ResponseStream = T::ListFeaturesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Rectangle>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_features(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = ListFeaturesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/route_guide.RouteGuide/RecordRoute" => {
                    #[allow(non_camel_case_types)]
                    struct RecordRouteSvc<T: RouteGuide>(pub Arc<T>);
                    impl<T: RouteGuide> tonic::server::ClientStreamingService<super::Point> for RecordRouteSvc<T> {
                        type Response = super::RouteSummary;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::Point>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).record_route(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = RecordRouteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/route_guide.RouteGuide/RouteChat" => {
                    #[allow(non_camel_case_types)]
                    struct RouteChatSvc<T: RouteGuide>(pub Arc<T>);
                    impl<T: RouteGuide> tonic::server::StreamingService<super::RouteNote> for RouteChatSvc<T> {
                        type Response = super::RouteNote;
                        type ResponseStream = T::RouteChatStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::RouteNote>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).route_chat(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = RouteChatSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: RouteGuide> Clone for RouteGuideServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: RouteGuide> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RouteGuide> tonic::transport::NamedService for RouteGuideServer<T> {
        const NAME: &'static str = "route_guide.RouteGuide";
    }
}
