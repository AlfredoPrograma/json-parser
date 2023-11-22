use std::fs;

use prelude::JsonParser;

mod object_parsers;
mod prelude;
mod primitive_parsers;

const EXAMPLE_JSON_PATH: &'static str = "example.json";

fn main() {
    let parser = JsonParser::new();

    match fs::read_to_string(EXAMPLE_JSON_PATH) {
        Ok(json) => match parser.parse(&json) {
            Ok(obj) => println!("{:#?}", obj),
            Err(err) => panic!("{:#?}", err),
        },

        Err(err) => panic!("{:#?}", err),
    };
}
