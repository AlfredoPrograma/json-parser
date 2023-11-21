use std::collections::HashMap;

use nom::character::complete::char;
use nom::{branch::alt, multi::many0, IResult};
use nom::{Map, Parser};

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

pub fn consume_spaces() -> impl FnMut(&str) -> IResult<&str, ()> {
    |input| {
        many0(alt((char('\n'), char(' '))))
            .parse(input)
            .map(|(next_input, _)| (next_input, ()))
    }
}
