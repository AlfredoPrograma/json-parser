use nom::{branch::alt, bytes::complete::tag, sequence::separated_pair, IResult, Parser};

use crate::{
    elements::ElementKind,
    primitive_parsers::{parse_bool, parse_float, parse_integer, parse_string},
};

pub fn parse_value<'a, T>() -> impl FnMut(&str) -> IResult<&str, ElementKind> {
    |input| {
        alt((
            parse_string(),
            parse_integer(),
            parse_float(),
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

    use crate::{element_parsers::parse_key_value, elements::ElementKind};

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
