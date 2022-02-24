
mod server {
    use std;
    use std::collections::HashSet;
    use std::collections::HashMap;

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
        pub(crate) method: String,
        pub(crate) path: String,
    }
    pub struct Api {
        pub(crate) endpoints: HashSet<&'static Endpoint>,
        pub(crate) bindings: HashMap<&'static Endpoint, fn(Request) -> Response>
    }

    impl Api {
        pub fn new() -> Api {
            return Api { endpoints: HashSet::new(), bindings : HashMap::new() };
        }

        pub fn add_endpoint(&mut self, method: String, path: String, handler: fn(Request) -> Response) {
            let endpoint = Endpoint { method: method, path: path };
            self.endpoints.insert(&endpoint);
            self.bindings.insert(&endpoint, handler);
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
        api.add_endpoint(String::from("GET"), String::from("/api/test"), api_test_handler);
        let my_model = server::Endpoint { method: String::from("GET"), path: String::from("/api/test") };
        assert!(api.endpoints.contains(&my_model));
    }
}