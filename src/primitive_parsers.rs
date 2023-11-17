use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1},
    combinator::opt,
    sequence::delimited,
    IResult, Parser,
};

// TODO: evaluate internal string value in an stricter way
pub fn parse_string() -> impl FnMut(&str) -> IResult<&str, &str> {
    |input| delimited(char('"'), take_until("\""), char('"')).parse(input)
}

pub fn parse_integer() -> impl FnMut(&str) -> IResult<&str, i32> {
    |input| {
        opt(char('-')).parse(input).and_then(|(next_input, sign)| {
            digit1(next_input).map(|(next_input, number)| {
                let parsed_number = number.parse::<i32>().unwrap();

                match sign {
                    Some(_) => (next_input, parsed_number * (-1)),
                    None => (next_input, parsed_number),
                }
            })
        })
    }
}

pub fn parse_bool() -> impl FnMut(&str) -> IResult<&str, bool> {
    |input| {
        alt((tag("true"), tag("false")))
            .parse(input)
            .map(|(next_input, str_bool)| match str_bool {
                "true" => (next_input, true),
                "false" => (next_input, false),

                // Another option is unreachable because parse functions have already validated given input so they only can be `true` or `false`
                // Finally, with those two options, we can match the `str_bool` to boolean values without risk
                _ => unreachable!(),
            })
    }
}

pub fn parse_null<T>() -> impl FnMut(&str) -> IResult<&str, Option<T>> {
    |input| {
        tag("null")
            .parse(input)
            .map(|(next_input, _)| (next_input, None))
    }
}

#[cfg(test)]
mod tests {

    use nom::{error, Parser};

    use super::{parse_bool, parse_integer, parse_null, parse_string};

    #[test]
    fn test_parse_string() {
        assert_eq!(
            parse_string().parse("\"this is a string\""),
            Ok(("", "this is a string"))
        );
        assert_eq!(
            parse_string().parse("\"other string\", ..."),
            Ok((", ...", "other string"))
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
        assert_eq!(parse_integer().parse("2001"), Ok(("", 2001)));
        assert_eq!(
            parse_integer().parse("2001 ...continue"),
            Ok((" ...continue", 2001))
        );

        assert_eq!(parse_integer().parse("-9999 uwu"), Ok((" uwu", -9999)));

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
    fn test_parse_bool() {
        assert_eq!(parse_bool().parse("true"), Ok(("", true)));
        assert_eq!(parse_bool().parse("false"), Ok(("", false)));

        assert_eq!(
            parse_bool().parse("true ...and other content"),
            Ok((" ...and other content", true))
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
        assert_eq!(parse_null::<String>().parse("null"), Ok(("", None)));
        assert_eq!(
            parse_null::<String>().parse("null ...another text"),
            Ok((" ...another text", None))
        );

        assert_eq!(
            parse_null::<i32>().parse("not null"),
            Err(nom::Err::Error(nom::error::Error::new(
                "not null",
                error::ErrorKind::Tag
            )))
        )
    }
}
