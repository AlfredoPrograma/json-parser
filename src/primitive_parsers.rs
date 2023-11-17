use nom::{
    bytes::complete::take_until,
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

#[cfg(test)]
mod tests {
    use nom::error;

    use super::{parse_integer, parse_string};

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
}
