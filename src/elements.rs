use nom::{IResult, Parser};

use crate::element_parsers::parse_key_value;

#[derive(PartialEq, Debug)]
pub enum NumberKind {
    Integer(i32),
    Float(f32),
}

#[derive(PartialEq, Debug)]
pub enum ElementKind {
    Boolean(bool),
    Number(NumberKind),
    String(String),
}

#[derive(PartialEq, Debug)]
pub enum GenericElementKind<T> {
    Null(Option<T>),
}

#[derive(PartialEq, Debug)]
pub enum ComposedElementKind<T> {
    Generic(GenericElementKind<T>),
    Simple(ElementKind),
}

pub struct Element {
    kind: ElementKind,
    key: String,
}

impl Element {
    fn new(key: String, kind: ElementKind) -> Element {
        Element { key, kind }
    }
}

pub fn build_element<T>() -> impl FnMut(&str) -> IResult<&str, Element> {
    |input| {
        parse_key_value()
            .parse(input)
            .map(|(next_input, (key, value))| (next_input, Element::new(key, value)))
    }
}
