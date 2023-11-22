use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1},
    combinator::opt,
    number::complete::float,
    sequence::delimited,
    Parser,
};

use crate::prelude::{JsonValue, JsonValueParser, NumberType};

// TODO: evaluate internal string value in an stricter way
pub fn parse_string<'a>() -> JsonValueParser<'a> {
    Box::new(|input: &'a str| {
        delimited(char('"'), take_until("\""), char('"'))
            .parse(input)
            .map(|(next_input, value)| (next_input, JsonValue::String(value.to_string())))
    })
}

pub fn parse_integer<'a>() -> JsonValueParser<'a> {
    Box::new(|input: &'a str| {
        opt(char('-'))
            .parse(input)
            .and_then(|(next_input, sign)| {
                digit1(next_input).map(|(next_input, number)| {
                    let parsed_number = number.parse::<i32>().unwrap();

                    match sign {
                        Some(_) => (next_input, parsed_number * (-1)),
                        None => (next_input, parsed_number),
                    }
                })
            })
            .map(|(next_input, value)| (next_input, JsonValue::Number(NumberType::Integer(value))))
    })
}

pub fn parse_float<'a>() -> JsonValueParser<'a> {
    Box::new(|input| {
        opt(char('-')).parse(input).and_then(|(next_input, _)| {
            delimited(digit1, tag("."), digit1)
                .parse(next_input)
                .and_then(|_| float(input))
                .map(|(next_input, float)| {
                    (next_input, JsonValue::Number(NumberType::Float(float)))
                })
        })
    })
}

pub fn parse_bool<'a>() -> JsonValueParser<'a> {
    Box::new(|input| {
        alt((tag("true"), tag("false")))
            .parse(input)
            .map(|(next_input, str_bool)| match str_bool {
                "true" => (next_input, true),
                "false" => (next_input, false),

                // Another option is unreachable because parse functions have already validated given input so they only can be `true` or `false`
                // Finally, with those two options, we can match the `str_bool` to boolean values without risk
                _ => unreachable!(),
            })
            .map(|(next_input, value)| (next_input, JsonValue::Boolean(value)))
    })
}

pub fn parse_null<'a>() -> JsonValueParser<'a> {
    Box::new(|input| {
        tag("null")
            .parse(input)
            .map(|(next_input, _)| (next_input, JsonValue::Null(Box::new(None))))
    })
}

#[cfg(test)]
mod tests {

    use nom::{error, Parser};

    use crate::{
        prelude::{JsonValue, NumberType},
        primitive_parsers::parse_float,
    };

    use super::{parse_bool, parse_integer, parse_null, parse_string};

    #[test]
    fn test_parse_string() {
        assert_eq!(
            parse_string().parse("\"this is a string\""),
            Ok(("", JsonValue::String("this is a string".to_string())))
        );
        assert_eq!(
            parse_string().parse("\"other string\", ..."),
            Ok((", ...", JsonValue::String("other string".to_string())))
        );

        assert_eq!(
            parse_string().parse("not delimited string"),
            Err(nom::Err::Error(nom::error::Error::new(
                "not delimited string",
                error::ErrorKind::Char
            )))
        )
    }

    #[test]
    fn test_parse_integer() {
        assert_eq!(
            parse_integer().parse("2001"),
            Ok(("", JsonValue::Number(NumberType::Integer(2001))))
        );
        assert_eq!(
            parse_integer().parse("2001 ...continue"),
            Ok((" ...continue", JsonValue::Number(NumberType::Integer(2001))))
        );

        assert_eq!(
            parse_integer().parse("-9999 uwu"),
            Ok((" uwu", JsonValue::Number(NumberType::Integer(-9999))))
        );

        assert_eq!(
            parse_integer().parse(""),
            Err(nom::Err::Error(nom::error::Error::new(
                "",
                error::ErrorKind::Digit
            )))
        );
        assert_eq!(
            parse_integer().parse("not a number"),
            Err(nom::Err::Error(nom::error::Error::new(
                "not a number",
                error::ErrorKind::Digit
            )))
        )
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(
            parse_float().parse("2001.22"),
            Ok(("", JsonValue::Number(NumberType::Float(2001.22))))
        );
        assert_eq!(
            parse_float().parse("2001.99 ...continue"),
            Ok((
                " ...continue",
                JsonValue::Number(NumberType::Float(2001.99))
            ))
        );

        assert_eq!(
            parse_float().parse("-9999.2134 uwu"),
            Ok((" uwu", JsonValue::Number(NumberType::Float(-9999.2134))))
        );

        assert_eq!(
            parse_float().parse(""),
            Err(nom::Err::Error(nom::error::Error::new(
                "",
                error::ErrorKind::Digit
            )))
        );
        assert_eq!(
            parse_float().parse("not a number"),
            Err(nom::Err::Error(nom::error::Error::new(
                "not a number",
                error::ErrorKind::Digit
            )))
        )
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(
            parse_bool().parse("true"),
            Ok(("", JsonValue::Boolean(true)))
        );
        assert_eq!(
            parse_bool().parse("false"),
            Ok(("", JsonValue::Boolean(false)))
        );

        assert_eq!(
            parse_bool().parse("true ...and other content"),
            Ok((" ...and other content", JsonValue::Boolean(true)))
        );

        assert_eq!(
            parse_bool().parse("not a bool"),
            Err(nom::Err::Error(nom::error::Error::new(
                "not a bool",
                error::ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn test_parse_null() {
        assert_eq!(
            parse_null().parse("null"),
            Ok(("", JsonValue::Null(Box::new(None))))
        );
        assert_eq!(
            parse_null().parse("null ...another text"),
            Ok((" ...another text", JsonValue::Null(Box::new(None))))
        );

        assert_eq!(
            parse_null().parse("not null"),
            Err(nom::Err::Error(nom::error::Error::new(
                "not null",
                error::ErrorKind::Tag
            )))
        )
    }
}
