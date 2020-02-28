//!
//! The array tests.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::array::error::Error as ArrayValueError;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::Error as SemanticError;
use crate::Error;

#[test]
fn pushing_invalid_type() {
    let input = r#"
fn main() {
    let array = [1, false];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 21),
        ElementError::Value(ValueError::Array(ArrayValueError::PushingInvalidType(
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            Type::boolean().to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn slice_start_out_of_range() {
    let input = r#"
fn main() {
    [1, 2, 3, 4, 5][-1 .. 1];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Value(ValueError::Array(ArrayValueError::SliceStartOutOfRange(
            "-1".to_owned(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn slice_end_out_of_range() {
    let input = r#"
fn main() {
    [1, 2, 3, 4, 5][0 .. 6];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Value(ValueError::Array(ArrayValueError::SliceEndOutOfRange(
            "6".to_owned(),
            "5".to_owned(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn slice_end_lesser_than_start() {
    let input = r#"
fn main() {
    [1, 2, 3, 4, 5][2 .. 1];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Value(ValueError::Array(ArrayValueError::SliceEndLesserThanStart(
            "2".to_owned(),
            "1".to_owned(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}
