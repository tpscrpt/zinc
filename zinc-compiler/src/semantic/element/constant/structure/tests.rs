//!
//! The constant structure element tests.
//!

use crate::error::Error;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::structure::error::Error as StructureConstantError;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;
use zinc_lexical::Location;

#[test]
fn error_field_does_not_exist() {
    let input = r#"
struct Data {
    a: u8,
}

fn main() {
    const DATA: Data = Data {
        a: 0,
    }.b;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Structure(
            StructureConstantError::FieldDoesNotExist {
                location: Location::test(9, 7),
                type_identifier: "Data".to_owned(),
                field_name: "b".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_field_expected() {
    let input = r#"
struct Data {
    a: u8,
    b: u8,
}

fn main() {
    const DATA: Data = Data {
        a: 42,
        c: 64,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Structure(
            StructureConstantError::FieldExpected {
                location: Location::test(10, 9),
                type_identifier: "Data".to_owned(),
                position: 2,
                expected: "b".to_owned(),
                found: "c".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_field_invalid_type() {
    let input = r#"
struct Data {
    a: u8,
}

fn main() {
    const DATA: Data = Data {
        a: true,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Structure(
            StructureConstantError::FieldInvalidType {
                location: Location::test(8, 12),
                type_identifier: "Data".to_owned(),
                field_name: "a".to_owned(),
                expected: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
                found: Type::boolean(None).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_field_out_of_range() {
    let input = r#"
struct Data {
    a: u8,
    b: u8,
}

fn main() {
    const DATA: Data = Data {
        a: 42,
        b: 25,
        c: 64,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Structure(
            StructureConstantError::FieldOutOfRange {
                location: Location::test(11, 9),
                type_identifier: "Data".to_owned(),
                expected: 2,
                found: 3,
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
