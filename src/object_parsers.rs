// ["Hello", 1234, 11.2, false]

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::{IResult, Parser};

use crate::element_parsers::{parse_key_value, parse_value};
use crate::elements::ElementKind;

pub fn parse_array_values() -> impl FnMut(&str) -> IResult<&str, Vec<ElementKind>> {
    |input| separated_list0(tag(", "), parse_value()).parse(input)
}

pub fn parse_array() -> impl FnMut(&str) -> IResult<&str, ElementKind> {
    |input| {
        delimited(char('['), parse_array_values(), char(']'))
            .parse(input)
            .map(|(next_input, arr)| (next_input, ElementKind::Array(arr)))
    }
}

pub fn parse_object() -> impl FnMut(&str) -> IResult<&str, ElementKind> {
    |input| {
        delimited(
            char('{'),
            separated_list0(tag(",\n"), parse_key_value()),
            char('}'),
        )
        .parse(input)
        .map(|(next_input, elements)| (next_input, ElementKind::Object(elements)))
    }
}

#[cfg(test)]
mod tests {
    use nom::Parser;

    use crate::{
        elements::{ElementKind, NumberKind},
        object_parsers::{parse_array, parse_array_values, parse_object},
    };

    #[test]
    fn test_parse_array_values() {
        let values = "\"hello\", -100.10, 10, 25.5, false, \"world\" ...other key values";

        assert_eq!(
            parse_array_values().parse(values),
            Ok((
                " ...other key values",
                vec![
                    ElementKind::String("hello".to_string()),
                    ElementKind::Number(NumberKind::Float(-100.10)),
                    ElementKind::Number(NumberKind::Integer(10)),
                    ElementKind::Number(NumberKind::Float(25.5)),
                    ElementKind::Boolean(false),
                    ElementKind::String("world".to_string()),
                ]
            ))
        )
    }

    #[test]
    fn test_parse_array() {
        let array = "[\"hello\", -100.10, 10, 25.5, false, \"world\"]";

        assert_eq!(
            parse_array().parse(array),
            Ok((
                "",
                ElementKind::Array(vec![
                    ElementKind::String("hello".to_string()),
                    ElementKind::Number(NumberKind::Float(-100.10)),
                    ElementKind::Number(NumberKind::Integer(10)),
                    ElementKind::Number(NumberKind::Float(25.5)),
                    ElementKind::Boolean(false),
                    ElementKind::String("world".to_string()),
                ])
            ))
        );

        let invalid_array = "[\"non closed\", 123, \"array\"";

        assert_eq!(
            parse_array().parse(invalid_array),
            Err(nom::Err::Error(nom::error::Error::new(
                "",
                nom::error::ErrorKind::Char
            )))
        )
    }

    #[test]
    fn test_parse_object() {
        let object =
            "{\"name\": \"Alfredo Arvelaez\",\n\"age\": 22,\n\"temps\": [15.5, -10, 25.5]}";

        assert_eq!(
            parse_object().parse(object),
            Ok((
                "",
                ElementKind::Object(vec![
                    (
                        "name".to_string(),
                        ElementKind::String("Alfredo Arvelaez".to_string())
                    ),
                    (
                        "age".to_string(),
                        ElementKind::Number(NumberKind::Integer(22))
                    ),
                    (
                        "temps".to_string(),
                        ElementKind::Array(vec![
                            ElementKind::Number(NumberKind::Float(15.5)),
                            ElementKind::Number(NumberKind::Integer(-10)),
                            ElementKind::Number(NumberKind::Float(25.5))
                        ])
                    )
                ])
            ))
        )
    }
}
