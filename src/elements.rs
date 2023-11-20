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
    Array(Vec<ElementKind>),
    Null(Box<Option<ElementKind>>),
}

#[derive(PartialEq, Debug)]
pub struct Element {
    kind: ElementKind,
    key: String,
}

impl Element {
    fn new(key: String, kind: ElementKind) -> Element {
        Element { key, kind }
    }
}

pub fn build_element() -> impl FnMut(&str) -> IResult<&str, Element> {
    |input| {
        parse_key_value()
            .parse(input)
            .map(|(next_input, (key, value))| (next_input, Element::new(key, value)))
    }
}

#[cfg(test)]
mod tests {
    use nom::Parser;

    use crate::elements::{build_element, Element, ElementKind, NumberKind};

    #[test]
    fn test_build_element() {
        assert_eq!(
            build_element().parse("\"name\": \"Alfredo\""),
            Ok((
                "",
                Element::new(
                    "name".to_string(),
                    ElementKind::String("Alfredo".to_string())
                )
            ))
        );

        assert_eq!(
            build_element().parse("\"amount\": 5000"),
            Ok((
                "",
                Element::new(
                    "amount".to_string(),
                    ElementKind::Number(NumberKind::Integer(5000))
                )
            ))
        );

        assert_eq!(
            build_element().parse("\"temp\": -17.8"),
            Ok((
                "",
                Element::new(
                    "temp".to_string(),
                    ElementKind::Number(NumberKind::Float(-17.8))
                )
            ))
        );

        assert_eq!(
            build_element().parse("\"isActive\": false"),
            Ok((
                "",
                Element::new("isActive".to_string(), ElementKind::Boolean(false))
            ))
        );

        assert_eq!(
            build_element().parse("\"elements\": [\"array\", 123, -10.5, false]"),
            Ok((
                "",
                Element::new(
                    "elements".to_string(),
                    ElementKind::Array(vec![
                        ElementKind::String("array".to_string()),
                        ElementKind::Number(NumberKind::Integer(123)),
                        ElementKind::Number(NumberKind::Float(-10.5)),
                        ElementKind::Boolean(false),
                    ])
                )
            ))
        )
    }
}
