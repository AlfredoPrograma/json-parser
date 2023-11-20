use nom::{
    branch::alt,
    bytes::complete::tag,
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

use crate::{
    elements::ElementKind,
    object_parsers::{parse_array, parse_object},
    primitive_parsers::{parse_bool, parse_float, parse_integer, parse_null, parse_string},
    utils::consume_spaces,
};

pub fn parse_value() -> impl FnMut(&str) -> IResult<&str, ElementKind> {
    |input| {
        alt((
            parse_string(),
            parse_float(),
            parse_integer(),
            parse_bool(),
            parse_array(),
            parse_object(),
            parse_null(),
        ))
        .parse(input)
    }
}

pub fn parse_key_value() -> impl FnMut(&str) -> IResult<&str, (String, ElementKind)> {
    |input| {
        separated_pair(
            parse_string(),
            terminated(tag(":"), consume_spaces()),
            parse_value(),
        )
        .parse(input)
        .map(|(next_input, (key, value))| match key {
            ElementKind::String(k) => (next_input, (k, value)),

            // Parse string always returns a `ElementKind::String`, so other variants will never be reachable
            _ => unreachable!(),
        })
    }
}

#[cfg(test)]
mod tests {
    use nom::Parser;

    use crate::{
        element_parsers::{parse_key_value, parse_value},
        elements::{ElementKind, NumberKind},
    };

    #[test]
    fn test_parse_value() {
        // Parsing `string` value
        assert_eq!(
            parse_value().parse("\"hello world\" ...rest"),
            Ok((" ...rest", ElementKind::String("hello world".to_string())))
        );

        // Parsing `integer` value
        assert_eq!(
            parse_value().parse("115 ...rest"),
            Ok((" ...rest", ElementKind::Number(NumberKind::Integer(115))))
        );

        // Parsing `float` value
        assert_eq!(
            parse_value().parse("-10.99 ...rest"),
            Ok((" ...rest", ElementKind::Number(NumberKind::Float(-10.99))))
        );

        // Parsing `boolean` value
        assert_eq!(
            parse_value().parse("true ...rest"),
            Ok((" ...rest", ElementKind::Boolean(true)))
        );

        // Parsing `array` value
        assert_eq!(
            parse_value().parse("[\"array\", 123, -10.5, false]"),
            Ok((
                "",
                ElementKind::Array(vec![
                    ElementKind::String("array".to_string()),
                    ElementKind::Number(NumberKind::Integer(123)),
                    ElementKind::Number(NumberKind::Float(-10.5)),
                    ElementKind::Boolean(false),
                ])
            ))
        );

        // Parsin `object` value
        assert_eq!(
            parse_value().parse("{\"name\": \"Alfredo\", \"age\": 25}"),
            Ok((
                "",
                ElementKind::Object(vec![
                    (
                        "name".to_string(),
                        ElementKind::String("Alfredo".to_string())
                    ),
                    (
                        "age".to_string(),
                        ElementKind::Number(NumberKind::Integer(25))
                    )
                ])
            ))
        );

        // TODO: Should create custom error
        // Throwing error when try to parse invalid value
        assert_eq!(
            parse_value().parse("invalid value"),
            Err(nom::Err::Error(nom::error::Error::new(
                "invalid value",
                nom::error::ErrorKind::Tag
            )))
        )
    }

    #[test]
    fn test_parse_key_value() {
        assert_eq!(
            parse_key_value().parse("\"name\": \"Alfredo\""),
            Ok((
                "",
                (
                    "name".to_string(),
                    ElementKind::String("Alfredo".to_string())
                )
            ))
        );

        assert_eq!(
            parse_key_value().parse("\"temp\": -99\n ...other key value pairs"),
            Ok((
                "\n ...other key value pairs",
                (
                    "temp".to_string(),
                    ElementKind::Number(NumberKind::Integer(-99))
                )
            ))
        );

        assert_eq!(
            parse_key_value().parse("\"price\": 25.25\n ...other key value pairs"),
            Ok((
                "\n ...other key value pairs",
                (
                    "price".to_string(),
                    ElementKind::Number(NumberKind::Float(25.25))
                )
            ))
        );

        assert_eq!(
            parse_key_value().parse("\"isActive\": true\n ...other key value pairs"),
            Ok((
                "\n ...other key value pairs",
                ("isActive".to_string(), ElementKind::Boolean(true))
            ))
        );

        assert_eq!(
            parse_key_value().parse("\"elements\": [\"array\", 123, -10.5, false]"),
            Ok((
                "",
                (
                    "elements".to_string(),
                    ElementKind::Array(vec![
                        ElementKind::String("array".to_string()),
                        ElementKind::Number(NumberKind::Integer(123)),
                        ElementKind::Number(NumberKind::Float(-10.5)),
                        ElementKind::Boolean(false),
                    ])
                )
            ))
        );

        // Throwing error when try to parse invalid value
        assert_eq!(
            parse_value().parse("invalid value"),
            Err(nom::Err::Error(nom::error::Error::new(
                "invalid value",
                nom::error::ErrorKind::Tag
            )))
        );

        assert_eq!(
            parse_key_value().parse("not a key pair value"),
            Err(nom::Err::Error(nom::error::Error::new(
                "not a key pair value",
                nom::error::ErrorKind::Char
            )))
        )
    }
}
