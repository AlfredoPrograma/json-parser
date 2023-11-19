// ["Hello", 1234, 11.2, false]

use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::{IResult, Parser};

use crate::element_parsers::parse_value;
use crate::elements::ElementKind;

pub fn parse_array_values() -> impl FnMut(&str) -> IResult<&str, Vec<ElementKind>> {
    |input| separated_list0(tag(", "), parse_value()).parse(input)
}

#[cfg(test)]
mod tests {
    use nom::Parser;

    use crate::{
        elements::{ElementKind, NumberKind},
        object_parsers::parse_array_values,
    };

    #[test]
    fn test_parse_array_values() {
        let values = "\"hello\", -100.10, 10, 25.5, false, \"world\" ...other key values";

        assert_eq!(
            parse_array_values().parse(values),
            Ok((
                " ...other key values",
                vec![
                    ElementKind::String("hello".to_string()),
                    ElementKind::Number(NumberKind::Float(-100.10)),
                    ElementKind::Number(NumberKind::Integer(10)),
                    ElementKind::Number(NumberKind::Float(25.5)),
                    ElementKind::Boolean(false),
                    ElementKind::String("world".to_string()),
                ]
            ))
        )
    }
}
