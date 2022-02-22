
mod server {
    use std;
    use std::collections::HashSet;

    pub enum Method {
        GET,
        PUT,
        POST,
        DELETE,
        OPTIONS
    }
    pub struct Request;
    pub struct Model {

    }
    pub struct Api {
        models: HashSet<Model>
    }

    impl Api {
        pub fn new() -> Api {
            return Api { models: HashSet::new() };
        }

        pub fn add_endpoint(&mut self, path: &str, model: Model) {
            self.models.insert(model);
            std::println!("{}", String::from(path));
            std::println!("{}", model.to_str());
        }
    }

    impl Model {
        pub(crate) fn to_str(&self) -> String {
            return String::from("Hello");
        }

        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            // self.action.hash(state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut x = server::Api::new();
        let y = server::Model { action: |m, request| println!("received request") };
        x.add_endpoint("/api", y);
        assert_eq!(y.to_str(), "Hello");
    }
}