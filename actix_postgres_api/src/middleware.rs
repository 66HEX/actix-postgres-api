use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ready, Ready, LocalBoxFuture};
use std::{
    rc::Rc,
    time::Instant,
};
use tracing::{event, Level, Span};
use tracing_actix_web::{RootSpanBuilder};
use uuid::Uuid;

use crate::monitoring::{HTTP_REQUEST_COUNTER, HTTP_REQUEST_DURATION, Timer, ACTIVE_CONNECTIONS};

// Custom root span builder for tracing-actix-web
pub struct CustomRootSpanBuilder;

impl RootSpanBuilder for CustomRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        let request_id = Uuid::new_v4();
        
        // Używamy własnej implementacji zamiast RequestId
        request.extensions_mut().insert(request_id);
        
        tracing::info_span!(
            "http_request",
            method = %request.method(),
            path = %request.path(),
            version = ?request.version(),
            remote_addr = %request.connection_info().peer_addr().unwrap_or("unknown"),
            request_id = %request_id,
        )
    }

    fn on_request_end<B>(span: Span, outcome: &Result<ServiceResponse<B>, Error>) {
        match outcome {
            Ok(response) => {
                let status_code = response.status();
                // Add status code to span
                span.record("status_code", status_code.as_u16());
                
                if status_code.is_success() {
                    event!(parent: &span, Level::INFO, status_code = %status_code.as_u16(), "Request succeeded");
                } else if status_code.is_client_error() {
                    event!(parent: &span, Level::WARN, status_code = %status_code.as_u16(), "Client error");
                } else if status_code.is_server_error() {
                    event!(parent: &span, Level::ERROR, status_code = %status_code.as_u16(), "Server error");
                }
            }
            Err(e) => {
                event!(parent: &span, Level::ERROR, error = %e, "Request failed");
            }
        }
    }
}

// Performance metrics middleware
pub struct PerformanceMetrics;

impl<S, B> Transform<S, ServiceRequest> for PerformanceMetrics
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = PerformanceMetricsMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PerformanceMetricsMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct PerformanceMetricsMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for PerformanceMetricsMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Increment active connections counter
        ACTIVE_CONNECTIONS.inc();
        
        let method = req.method().as_str().to_string();
        let path = req.path().to_string();
        
        let timer = Timer::new();
        let service = self.service.clone();

        Box::pin(async move {
            // Call the next service
            let res = service.call(req).await?;
            
            // Record metrics when response is received
            let status_code = res.status().as_u16().to_string();
            let duration = timer.elapsed_seconds();
            
            // Record request count and duration
            HTTP_REQUEST_COUNTER
                .with_label_values(&[&method, &path, &status_code])
                .inc();
                
            HTTP_REQUEST_DURATION
                .with_label_values(&[&method, &path, &status_code])
                .observe(duration);
            
            // Decrement active connections counter
            ACTIVE_CONNECTIONS.dec();
            
            Ok(res)
        })
    }
}