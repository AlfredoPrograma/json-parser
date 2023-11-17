use nom::{
    bytes::complete::{take_until, take_while1},
    character::complete::char,
    combinator::opt,
    sequence::{delimited, preceded},
    IResult, Parser,
};

// TODO: evaluate internal string value in an stricter way
pub fn parse_string(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), take_until("\""), char('"')).parse(input)
}

pub fn parse_integer(input: &str) -> IResult<&str, i32> {
    opt(char('-')).parse(input).and_then(|(next, sign)| {
        take_while1(|c: char| c.is_numeric())
            .parse(next)
            .map(|(next, n)| match sign {
                Some(_) => (next, n.parse::<i32>().unwrap() * (-1)),
                None => (next, n.parse::<i32>().unwrap()),
            })
    })
}

pub fn parse_float(input: &str) -> IResult<&str, f64> {
    opt(char('-')).parse(input).and_then(|(next, sign)| {
        take_while1(|c: char| c.is_numeric())
            .parse(next)
            .and_then(|(float, integer)| {
                preceded(char('.'), take_while1(|c: char| c.is_numeric()))
                    .parse(float)
                    .map(|(next, f)| match sign {
                        Some(_) => {
                            let value = format!("{}.{}", integer, f);
                            (next, value.parse::<f64>().unwrap() * (-1.0))
                        }
                        None => {
                            let value = format!("{}.{}", integer, f);
                            (next, value.parse::<f64>().unwrap())
                        }
                    })
            })
    })
}

#[cfg(test)]
mod tests {
    use nom::error;

    use super::{parse_float, parse_integer, parse_string};

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
                error::ErrorKind::TakeWhile1
            )))
        );
        assert_eq!(
            parse_integer("not a number"),
            Err(nom::Err::Error(nom::error::Error::new(
                "not a number",
                error::ErrorKind::TakeWhile1
            )))
        )
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(parse_float("10.25"), Ok(("", 10.25)));
        assert_eq!(parse_float("-10.25"), Ok(("", -10.25)));
        assert_eq!(
            parse_float("-10.25 other text i dont use 123"),
            Ok((" other text i dont use 123", -10.25))
        );

        assert_eq!(
            parse_float(""),
            Err(nom::Err::Error(nom::error::Error::new(
                "",
                error::ErrorKind::TakeWhile1
            )))
        );

        assert_eq!(
            parse_float("not a number"),
            Err(nom::Err::Error(nom::error::Error::new(
                "not a number",
                error::ErrorKind::TakeWhile1
            )))
        )
    }
}
