mod http;

use std;
use std::collections::HashMap;
use http::{Method, Request, Response};


/// The primary object to manage endpoint configuration and start up a server lcoally
/// 
/// # Example
/// ```
/// use rustful_api::Api;
/// use http::{Method, Request, Response}
/// 
/// let mut api = Api::new();
/// 
/// fn handle_root(request: Request<T>) -> Response<U> {
///     println!("handled root request");
///     Response::builder().build()
/// }
/// 
/// api.add_endpoint(Method::GET, String::from("/"), )
/// ```
struct Api {
    endpoints: HashMap<String, HashMap<Method, fn (Request) -> Response >>
}
pub struct Config {
    address: String,
    port: u32
}

impl Api {
    pub fn new() -> Api {
        Api { endpoints: HashMap::new() }
    }

    pub fn add_endpoint(&mut self, method: Method, path: String,
                        handler: fn (Request) -> Response) {
        // TODO: if let?
        let existing_map = self.endpoints.get(path).get_or_insert(HashMap::new());
    }

    pub fn start(config: Config) {

    }
}