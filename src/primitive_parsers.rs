use nom::{bytes::complete::take_until, character::complete::char, sequence::delimited, IResult};

// TODO: evaluate internal string value in an stricter way
pub fn string(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), take_until("\""), char('"'))(input)
}

#[cfg(test)]
mod tests {
    use nom::error;

    use super::string;

    #[test]
    fn test_string() {
        assert_eq!(string("\"this is a string\""), Ok(("", "this is a string")));
        assert_eq!(
            string("\"other string\", ..."),
            Ok((", ...", "other string"))
        );

        assert_eq!(
            string("not delimited string"),
            Err(nom::Err::Error(nom::error::Error::new(
                "not delimited string",
                error::ErrorKind::Char
            )))
        )
    }
}
