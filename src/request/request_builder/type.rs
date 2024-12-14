use crate::request::request::r#type::HttpRequest;

/// Builder pattern for constructing `Request` instances.
///
/// The `RequestBuilder` struct facilitates the creation of `Request` objects
/// through a series of method calls. It allows for flexible and clear configuration of
/// an HTTP request's components such as method, URL, headers, and body.
///
/// # Fields
/// - `http_request`: A temporary `Request` instance used to accumulate changes during
///   the construction process. It holds the current state of the builder.
/// - `builder`: A finalized `Request` instance that holds the result after the
///   builder process has been completed. It is returned when the builder is finalized.
///
/// This builder simplifies the creation of `Request` objects, ensuring thread-safety
/// and immutability of shared references, while providing a fluent API for constructing
/// HTTP requests with various configurations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestBuilder {
    pub(crate) http_request: HttpRequest,
    pub(crate) builder: HttpRequest,
}
