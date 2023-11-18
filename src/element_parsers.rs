use nom::{branch::alt, bytes::complete::tag, sequence::separated_pair, IResult, Parser};

use crate::{
    elements::ElementKind,
    primitive_parsers::{parse_bool, parse_float, parse_integer, parse_string},
};

pub fn parse_value() -> impl FnMut(&str) -> IResult<&str, ElementKind> {
    |input| {
        alt((
            parse_string(),
            parse_float(),
            parse_integer(),
            parse_bool(),
            // parse_null(),
        ))
        .parse(input)
    }
}

pub fn parse_key_value() -> impl FnMut(&str) -> IResult<&str, (ElementKind, ElementKind)> {
    |input| separated_pair(parse_string(), tag(": "), parse_string()).parse(input)
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
    fn test_parse_key() {
        assert_eq!(
            parse_key_value().parse("\"name\": \"Alfredo\""),
            Ok((
                "",
                (
                    ElementKind::String("name".to_string()),
                    ElementKind::String("Alfredo".to_string())
                )
            ))
        );

        assert_eq!(
            parse_key_value().parse("\"name\": \"Alfredo\"\n ...other key value pairs"),
            Ok((
                "\n ...other key value pairs",
                (
                    ElementKind::String("name".to_string()),
                    ElementKind::String("Alfredo".to_string())
                )
            ))
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
