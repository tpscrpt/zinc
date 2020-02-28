//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use crate::lexical::Location;

use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::error::Error as IntegerConstantError;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::Error as SemanticError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let value = --128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 17),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowNegation(
                BigInt::from(128),
                Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
