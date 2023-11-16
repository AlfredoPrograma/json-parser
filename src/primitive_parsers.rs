use nom::{
    bytes::complete::{take_until, take_while1},
    character::complete::char,
    sequence::delimited,
    IResult, Parser,
};

// TODO: evaluate internal string value in an stricter way
pub fn parse_string(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), take_until("\""), char('"'))(input)
}

// TODO: evaluate internal number value in an stricter way. Currently is just taking positive integers
pub fn parse_number(input: &str) -> IResult<&str, i32> {
    take_while1(|c: char| c.is_numeric())
        .map(|n: &str| n.parse::<i32>().unwrap())
        .parse(input)
}

#[cfg(test)]
mod tests {
    use nom::error;

    use super::{parse_number, parse_string};

    #[test]
    fn test_parse_string() {
        assert_eq!(
            parse_string("\"this is a string\""),
            Ok(("", "this is a string"))
        );
        assert_eq!(
            parse_string("\"other string\", ..."),
            Ok((", ...", "other string"))
        );

        assert_eq!(
            parse_string("not delimited string"),
            Err(nom::Err::Error(nom::error::Error::new(
                "not delimited string",
                error::ErrorKind::Char
            )))
        )
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("2001"), Ok(("", 2001)));
        assert_eq!(parse_number("2001 ...continue"), Ok((" ...continue", 2001)));

        assert_eq!(
            parse_number(""),
            Err(nom::Err::Error(nom::error::Error::new(
                "",
                error::ErrorKind::TakeWhile1
            )))
        );
        assert_eq!(
            parse_number("not a number"),
            Err(nom::Err::Error(nom::error::Error::new(
                "not a number",
                error::ErrorKind::TakeWhile1
            )))
        )
    }
}
