use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error, HttpMessage};
use tracing::{event, Level, Span};
use tracing_actix_web::RootSpanBuilder;
use uuid::Uuid;

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