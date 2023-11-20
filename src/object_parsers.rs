use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::{IResult, Parser};

use crate::element_parsers::{parse_key_value, parse_value};
use crate::elements::JsonValue;
use crate::utils::consume_spaces;

pub fn parse_array_values() -> impl FnMut(&str) -> IResult<&str, Vec<JsonValue>> {
    |input| separated_list0(terminated(tag(", "), consume_spaces()), parse_value()).parse(input)
}

pub fn parse_array() -> impl FnMut(&str) -> IResult<&str, JsonValue> {
    |input| {
        delimited(
            terminated(char('['), consume_spaces()),
            parse_array_values(),
            preceded(consume_spaces(), char(']')),
        )
        .parse(input)
        .map(|(next_input, arr)| (next_input, JsonValue::Array(arr)))
    }
}

pub fn parse_object() -> impl FnMut(&str) -> IResult<&str, JsonValue> {
    |input| {
        delimited(
            terminated(char('{'), consume_spaces()),
            separated_list0(pair(tag(","), consume_spaces()), parse_key_value()),
            preceded(consume_spaces(), char('}')),
        )
        .parse(input)
        .map(|(next_input, elements)| (next_input, JsonValue::Object(elements)))
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use nom::Parser;

    use crate::{
        elements::{JsonValue, NumberType},
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

        assert_eq!(
            parse_object().parse(object),
            Ok((
                "",
                JsonValue::Object(vec![
                    (
                        "name".to_string(),
                        JsonValue::String("Alfredo Arvelaez".to_string())
                    ),
                    (
                        "age".to_string(),
                        JsonValue::Number(NumberType::Integer(22))
                    ),
                    (
                        "temps".to_string(),
                        JsonValue::Array(vec![
                            JsonValue::Number(NumberType::Float(15.5)),
                            JsonValue::Number(NumberType::Integer(-10)),
                            JsonValue::Number(NumberType::Float(25.5))
                        ])
                    )
                ])
            ))
        );

        let empty_object = "{}";

        assert_eq!(
            parse_object().parse(empty_object),
            Ok(("", JsonValue::Object(vec![])))
        )
    }
}
