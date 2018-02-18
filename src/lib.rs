extern crate serde_json;

#[macro_use]
extern crate serde_derive;


mod parser;
mod node;
mod lang;
mod module;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
