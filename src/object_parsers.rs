use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::error::Error;
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated};
use nom::Parser;

use crate::prelude::{consume_spaces, JsonValue, JsonValueParser};
use crate::primitive_parsers::{parse_bool, parse_float, parse_integer, parse_null, parse_string};

pub fn parse_value<'a>() -> JsonValueParser<'a> {
    Box::new(|input| {
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
    })
}

pub fn parse_key_value<'a>() -> impl Parser<&'a str, (String, JsonValue), Error<&'a str>> {
    |input| {
        separated_pair(
            parse_string(),
            terminated(tag(":"), consume_spaces()),
            parse_value(),
        )
        .parse(input)
        .map(|(next_input, (key, value))| match key {
            JsonValue::String(k) => (next_input, (k, value)),

            // Parse string always returns a `ElementKind::String`, so other variants will never be reachable
            _ => unreachable!(),
        })
    }
}

pub fn parse_array_values<'a>() -> impl Parser<&'a str, Vec<JsonValue>, Error<&'a str>> {
    |input| separated_list0(terminated(tag(", "), consume_spaces()), parse_value()).parse(input)
}

pub fn parse_array<'a>() -> JsonValueParser<'a> {
    Box::new(|input| {
        delimited(
            terminated(char('['), consume_spaces()),
            parse_array_values(),
            preceded(consume_spaces(), char(']')),
        )
        .parse(input)
        .map(|(next_input, arr)| (next_input, JsonValue::Array(arr)))
    })
}

pub fn parse_object<'a>() -> JsonValueParser<'a> {
    Box::new(|input| {
        delimited(
            terminated(char('{'), consume_spaces()),
            separated_list0(pair(tag(","), consume_spaces()), parse_key_value()),
            preceded(consume_spaces(), char('}')),
        )
        .parse(input)
        .map(|(next_input, elements)| {
            let mut object = HashMap::new();

            elements.into_iter().for_each(|(k, v)| {
                object.insert(k, v);

                ()
            });

            (next_input, JsonValue::Object(object))
        })
    })
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, vec};

    use nom::Parser;

    use crate::{
        object_parsers::{
            parse_array, parse_array_values, parse_key_value, parse_object, parse_value,
        },
        prelude::{JsonValue, NumberType},
    };

    #[test]
    fn test_parse_array_values() {
        let values = "\"hello\", -100.10, 10, 25.5, false, \"world\" ...other key values";

        assert_eq!(
            parse_array_values().parse(values),
            Ok((
                " ...other key values",
                vec![
                    JsonValue::String("hello".to_string()),
                    JsonValue::Number(NumberType::Float(-100.10)),
                    JsonValue::Number(NumberType::Integer(10)),
                    JsonValue::Number(NumberType::Float(25.5)),
                    JsonValue::Boolean(false),
                    JsonValue::String("world".to_string()),
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
                JsonValue::Array(vec![
                    JsonValue::String("hello".to_string()),
                    JsonValue::Number(NumberType::Float(-100.10)),
                    JsonValue::Number(NumberType::Integer(10)),
                    JsonValue::Number(NumberType::Float(25.5)),
                    JsonValue::Boolean(false),
                    JsonValue::String("world".to_string()),
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
            "{\n\"name\": \"Alfredo Arvelaez\",\n\"age\": 22,\n\"temps\": [15.5, -10, 25.5]\n}";

        let mut expected = HashMap::new();

        expected.insert(
            "name".to_string(),
            JsonValue::String("Alfredo Arvelaez".to_string()),
        );
        expected.insert(
            "age".to_string(),
            JsonValue::Number(NumberType::Integer(22)),
        );
        expected.insert(
            "temps".to_string(),
            JsonValue::Array(vec![
                JsonValue::Number(NumberType::Float(15.5)),
                JsonValue::Number(NumberType::Integer(-10)),
                JsonValue::Number(NumberType::Float(25.5)),
            ]),
        );

        assert_eq!(
            parse_object().parse(object),
            Ok(("", JsonValue::Object(expected)))
        );

        let empty_object = "{}";

        assert_eq!(
            parse_object().parse(empty_object),
            Ok(("", JsonValue::Object(HashMap::new())))
        )
    }

    #[test]
    fn test_parse_value() {
        // Parsing `string` value
        assert_eq!(
            parse_value().parse("\"hello world\" ...rest"),
            Ok((" ...rest", JsonValue::String("hello world".to_string())))
        );

        // Parsing `integer` value
        assert_eq!(
            parse_value().parse("115 ...rest"),
            Ok((" ...rest", JsonValue::Number(NumberType::Integer(115))))
        );

        // Parsing `float` value
        assert_eq!(
            parse_value().parse("-10.99 ...rest"),
            Ok((" ...rest", JsonValue::Number(NumberType::Float(-10.99))))
        );

        // Parsing `boolean` value
        assert_eq!(
            parse_value().parse("true ...rest"),
            Ok((" ...rest", JsonValue::Boolean(true)))
        );

        // Parsing `array` value
        assert_eq!(
            parse_value().parse("[\"array\", 123, -10.5, false]"),
            Ok((
                "",
                JsonValue::Array(vec![
                    JsonValue::String("array".to_string()),
                    JsonValue::Number(NumberType::Integer(123)),
                    JsonValue::Number(NumberType::Float(-10.5)),
                    JsonValue::Boolean(false),
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
                ("name".to_string(), JsonValue::String("Alfredo".to_string()))
            ))
        );

        assert_eq!(
            parse_key_value().parse("\"temp\": -99\n ...other key value pairs"),
            Ok((
                "\n ...other key value pairs",
                (
                    "temp".to_string(),
                    JsonValue::Number(NumberType::Integer(-99))
                )
            ))
        );

        assert_eq!(
            parse_key_value().parse("\"price\": 25.25\n ...other key value pairs"),
            Ok((
                "\n ...other key value pairs",
                (
                    "price".to_string(),
                    JsonValue::Number(NumberType::Float(25.25))
                )
            ))
        );

        assert_eq!(
            parse_key_value().parse("\"isActive\": true\n ...other key value pairs"),
            Ok((
                "\n ...other key value pairs",
                ("isActive".to_string(), JsonValue::Boolean(true))
            ))
        );

        assert_eq!(
            parse_key_value().parse("\"elements\": [\"array\", 123, -10.5, false]"),
            Ok((
                "",
                (
                    "elements".to_string(),
                    JsonValue::Array(vec![
                        JsonValue::String("array".to_string()),
                        JsonValue::Number(NumberType::Integer(123)),
                        JsonValue::Number(NumberType::Float(-10.5)),
                        JsonValue::Boolean(false),
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
