use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1},
    combinator::opt,
    sequence::delimited,
    IResult, Parser,
};

// TODO: evaluate internal string value in an stricter way
pub fn parse_string(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), take_until("\""), char('"')).parse(input)
}

pub fn parse_integer(input: &str) -> IResult<&str, i32> {
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

pub fn parse_bool(input: &str) -> IResult<&str, bool> {
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

pub fn parse_null<T>(input: &str) -> IResult<&str, Option<T>> {
    tag("null")
        .parse(input)
        .map(|(next_input, _)| (next_input, None))
}

#[cfg(test)]
mod tests {

    use nom::error;

    use super::{parse_bool, parse_integer, parse_null, parse_string};

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
    fn test_parse_integer() {
        assert_eq!(parse_integer("2001"), Ok(("", 2001)));
        assert_eq!(
            parse_integer("2001 ...continue"),
            Ok((" ...continue", 2001))
        );

        assert_eq!(parse_integer("-9999 uwu"), Ok((" uwu", -9999)));

        assert_eq!(
            parse_integer(""),
            Err(nom::Err::Error(nom::error::Error::new(
                "",
                error::ErrorKind::Digit
            )))
        );
        assert_eq!(
            parse_integer("not a number"),
            Err(nom::Err::Error(nom::error::Error::new(
                "not a number",
                error::ErrorKind::Digit
            )))
        )
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(parse_bool("true"), Ok(("", true)));
        assert_eq!(parse_bool("false"), Ok(("", false)));

        assert_eq!(
            parse_bool("true ...and other content"),
            Ok((" ...and other content", true))
        );

        assert_eq!(
            parse_bool("not a bool"),
            Err(nom::Err::Error(nom::error::Error::new(
                "not a bool",
                error::ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn test_parse_null() {
        assert_eq!(parse_null::<String>("null"), Ok(("", None)));
        assert_eq!(
            parse_null::<String>("null ...another text"),
            Ok((" ...another text", None))
        );

        assert_eq!(
            parse_null::<i32>("not null"),
            Err(nom::Err::Error(nom::error::Error::new(
                "not null",
                error::ErrorKind::Tag
            )))
        )
    }
}
