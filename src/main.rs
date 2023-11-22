use std::fs;

use object_parsers::parse_object;

mod object_parsers;
mod prelude;
mod primitive_parsers;

const EXAMPLE_JSON_PATH: &'static str = "example.json";

fn main() {
    match fs::read_to_string(EXAMPLE_JSON_PATH) {
        Ok(json) => match parse_object().parse(&json) {
            Ok((_, obj)) => println!("{:#?}", obj),
            Err(err) => panic!("{:#?}", err),
        },

        Err(err) => panic!("{:#?}", err),
    };
}
