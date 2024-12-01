use crate::{
    body::r#type::Body,
    header::r#type::Header,
    methods::r#type::Methods,
    request::{config::r#type::Config, tmp::r#type::Tmp},
};
use std::sync::Arc;
/// Represents an HTTP request, encapsulating various components such as the method, URL, protocol,
/// headers, body, and additional metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequest {
    /// The HTTP method of the request (e.g., GET, POST, etc.).
    pub methods: Arc<Methods>,

    /// The target URL of the request.
    pub url: Arc<String>,

    /// The headers included in the request.
    pub header: Arc<Header>,

    /// The type of the body, specifying whether it is text or JSON.
    pub body: Arc<Body>,

    /// Represents the configuration settings for the HTTP request.
    pub config: Config,

    /// Stores temporary data during the HTTP request process.
    pub tmp: Tmp,
}
