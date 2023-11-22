use std::collections::HashMap;

use nom::character::complete::char;
use nom::error::Error;
use nom::Parser;
use nom::{branch::alt, multi::many0};

use crate::object_parsers::parse_object;

#[derive(PartialEq, Debug)]
pub enum NumberType {
    Integer(i32),
    Float(f32),
}

#[derive(PartialEq, Debug)]
pub enum JsonValue {
    Boolean(bool),
    Number(NumberType),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
    Null(Box<Option<JsonValue>>),
}

pub type JsonValueParser<'a> = Box<dyn Parser<&'a str, JsonValue, Error<&'a str>>>;

pub fn consume_spaces<'a>() -> impl Parser<&'a str, (), Error<&'a str>> {
    |input| {
        many0(alt((char('\n'), char(' '))))
            .parse(input)
            .map(|(next_input, _)| (next_input, ()))
    }
}

pub struct JsonParser {}

impl JsonParser {
    pub fn new() -> Self {
        JsonParser {}
    }

    pub fn parse(self, json: &str) -> Result<JsonValue, String> {
        match parse_object().parse(json) {
            Ok((_, object)) => Ok(object),
            Err(err) => Err(err.to_string()),
        }
    }
}
