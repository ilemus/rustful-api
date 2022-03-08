
/// # `server` namespace
/// Has all the high level interactions to create and run a RESTful API server
///
/// ## Method
/// An enum of allowed operations on endpoints
///
/// ## Response
/// The response that the requester will receive (Headers, Content (Text or JSON), and parameters)
///
/// ## Request
/// The incoming request with Headers, Content (Text or JSON), and parameters
///
/// ## Endpoint
/// A combination of the operation method and a path string
///
/// ## Api
/// A collections of endpoints that can be run to start the REST server
mod server {
    use std;
    use std::collections::HashMap;

    /// # Method
    /// An enum of allowed operations on endpoints
    /// - `GET`
    /// - `PUT`
    /// - `POST`
    /// - `DELETE`
    /// - `OPTIONS`
    #[derive(Hash, PartialEq)]
    pub enum Method {
        GET,
        PUT,
        POST,
        DELETE,
        OPTIONS
    }
    pub struct Request;
    pub struct Response {
        pub code: u16
    }
    #[derive(Hash)]
    pub struct Endpoint {
        pub(crate) method: Method,
        pub(crate) path: String,
    }
    /// # Api
    /// A collections of endpoints that can be run to start the REST server
    ///
    /// ## Example
    /// ```
    /// use server;
    /// use std;
    ///
    /// let mut api = Api::new();
    /// fn handle_root_request(request: Request) -> Response {
    ///     println!("Received request on path '/'!");
    /// }
    /// api.add_endpoint(Method::GET, String::from("/"), handle_root_request);
    /// api.start();
    /// ```
    pub struct Api {
        pub(crate) endpoints: HashMap<Endpoint, fn(Request) -> Response>
    }

    impl Api {
        pub fn new() -> Api {
            return Api { endpoints : HashMap::new() };
        }

        pub fn add_endpoint(&mut self, method: Method, path: String, handler: fn(Request) -> Response) {
            let endpoint = Endpoint { method, path };
            self.endpoints.insert(endpoint, handler);
        }

        pub fn start() {

        }
    }

    impl PartialEq for Endpoint {
        fn eq(&self, other: &Self) -> bool {
            return self.method == other.method
                && self.path == other.path;
        }
    }
    impl Eq for Endpoint {}
}

#[cfg(test)]
mod tests {
    use super::*;

    fn api_test_handler(_request: server::Request) -> server::Response {
        println!("Received request");
        return server::Response { code: 200, }
    }

    #[test]
    fn test_adding_an_endpoint() {
        let mut api = server::Api::new();
        api.add_endpoint(server::Method::GET, String::from("/api/test"), api_test_handler);
        let my_model = server::Endpoint { method: server::Method::GET, path: String::from("/api/test") };
        assert!(api.endpoints.contains_key(&my_model));
    }
}