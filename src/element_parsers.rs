use nom::{bytes::complete::tag, sequence::separated_pair, IResult, Parser};

use crate::primitive_parsers::parse_string;

pub fn parse_key_value() -> impl FnMut(&str) -> IResult<&str, (&str, &str)> {
    |input| separated_pair(parse_string(), tag(": "), parse_string()).parse(input)
}

#[cfg(test)]
mod tests {
    use nom::Parser;

    use crate::element_parsers::parse_key_value;

    #[test]
    fn test_parse_key() {
        assert_eq!(
            parse_key_value().parse("\"name\": \"Alfredo\""),
            Ok(("", ("name", "Alfredo")))
        );

        assert_eq!(
            parse_key_value().parse("\"name\": \"Alfredo\"\n ...other key value pairs"),
            Ok(("\n ...other key value pairs", ("name", "Alfredo")))
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
