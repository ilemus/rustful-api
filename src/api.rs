use std::collections::HashMap;
use http::{Method, Request, Response};

struct Api {
    endpoints: HashMap<String, HashMap<Method, fn (Request<T>) -> Response<U> >>
}
struct Config {
    address: String,
    port: u32
}

impl Api {
    pub fn new() -> Api {
        Api { endpoints: HashMap::new() }
    }

    pub fn add_endpoint(&mut self, method: Method, path: String,
                        handler: fn (Request<T>) -> Response<U>) {
        let method_map = HashMap::new();
    }

    pub fn start(config: Config) {

    }
}