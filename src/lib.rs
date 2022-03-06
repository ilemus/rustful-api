
mod server {
    use std;
    use std::collections::HashMap;

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