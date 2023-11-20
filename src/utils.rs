use nom::character::complete::char;
use nom::Parser;
use nom::{branch::alt, multi::many0, IResult};

pub fn consume_spaces() -> impl FnMut(&str) -> IResult<&str, ()> {
    |input| {
        many0(alt((char('\n'), char(' '))))
            .parse(input)
            .map(|(next_input, _)| (next_input, ()))
    }
}
