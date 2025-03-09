use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ready, Ready, LocalBoxFuture};
use std::rc::Rc;

use crate::monitoring::{HTTP_REQUEST_COUNTER, HTTP_REQUEST_DURATION, Timer, ACTIVE_CONNECTIONS};

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