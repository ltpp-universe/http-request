use std::sync::Arc;

use super::r#type::HttpRequestBuilder;
use crate::{
    global_type::r#type::{Body, Header},
    request::http_request::r#type::HttpRequest,
    Methods,
};

/// Provides a builder pattern implementation for constructing `HttpRequest` instances.
///
/// The `HttpRequestBuilder` struct is used to create and configure `HttpRequest` objects
/// through a series of method calls, enabling a flexible and clear way to construct
/// requests.
///
/// # Traits Implemented
/// - `Default`: Provides a default instance of the builder, initializing all fields
///   with default values.
///
/// # Methods
/// - `new`: Creates a new instance of the builder with default values.
/// - `set_methods`: Sets the HTTP method for the request (e.g., GET, POST).
/// - `set_url`: Sets the target URL of the request.
/// - `set_header`: Updates the headers of the request. Existing headers may be merged with
///   the provided ones.
/// - `set_body`: Updates the body of the request. Existing body data may be merged with
///   the provided data.
/// - `builder`: Finalizes the configuration and returns a fully constructed `HttpRequest`
///   instance. Resets the builder's temporary state for subsequent use.
///
/// This builder simplifies the construction of `HttpRequest` objects while maintaining
/// thread safety and ensuring immutability for shared references where applicable.
impl Default for HttpRequestBuilder {
    fn default() -> HttpRequestBuilder {
        HttpRequestBuilder {
            tmp: HttpRequest::default(),
            builder: HttpRequest::default(),
        }
    }
}

impl HttpRequestBuilder {
    /// Creates a new instance of the builder with default values.
    ///
    /// This method initializes the `HttpRequestBuilder` with default values for all
    /// fields.
    ///
    /// # Returns
    /// Returns a new instance of `HttpRequestBuilder`.
    pub fn new() -> Self {
        HttpRequestBuilder::default()
    }

    /// Sets the HTTP method for the request.
    ///
    /// This method allows you to specify the HTTP method (e.g., GET, POST) for the
    /// request being built.
    ///
    /// # Arguments
    /// - `methods`: The HTTP method to be set for the request.
    ///
    /// # Returns
    /// Returns a mutable reference to the `HttpRequestBuilder` to allow method chaining.
    pub fn set_methods(&mut self, methods: Methods) -> &mut Self {
        self.tmp.methods = Arc::new(methods);
        self
    }

    /// Sets the target URL of the request.
    ///
    /// This method allows you to specify the URL for the request being built.
    ///
    /// # Arguments
    /// - `url`: The target URL of the request.
    ///
    /// # Returns
    /// Returns a mutable reference to the `HttpRequestBuilder` to allow method chaining.
    pub fn set_url(&mut self, url: &str) -> &mut Self {
        self.tmp.url = Arc::new(url.to_owned());
        self
    }

    /// Sets the headers for the request.
    ///
    /// This method allows you to specify the headers for the request being built.
    /// Existing headers may be merged with the provided ones.
    ///
    /// # Arguments
    /// - `header`: The headers to be set for the request.
    ///
    /// # Returns
    /// Returns a mutable reference to the `HttpRequestBuilder` to allow method chaining.
    pub fn set_header(&mut self, header: &Header) -> &mut Self {
        if let Some(tmp_header) = Arc::get_mut(&mut self.tmp.header) {
            for (key, value) in header {
                tmp_header.insert(key.clone(), value.clone());
            }
        }
        self
    }

    /// Sets the body of the request.
    ///
    /// This method allows you to specify the body content of the request being built.
    /// Existing body data may be merged with the provided data.
    ///
    /// # Arguments
    /// - `body`: The body data to be set for the request.
    ///
    /// # Returns
    /// Returns a mutable reference to the `HttpRequestBuilder` to allow method chaining.
    pub fn set_body(&mut self, body: &Body) -> &mut Self {
        if let Some(tmp_body) = Arc::get_mut(&mut self.tmp.body) {
            for (key, value) in body {
                tmp_body.insert(key.clone(), value.clone());
            }
        }
        self
    }

    /// Finalizes the builder and returns a fully constructed `HttpRequest` instance.
    ///
    /// This method takes the current configuration stored in `tmp`, creates a new
    /// `HttpRequest` instance with the configuration, and resets the builder's temporary
    /// state for further use.
    ///
    /// # Returns
    /// Returns a fully constructed `HttpRequest` instance based on the current builder state.
    pub fn builder(&mut self) -> HttpRequest {
        self.builder = self.tmp.clone();
        self.tmp = HttpRequest::default();
        self.builder.clone()
    }
}
