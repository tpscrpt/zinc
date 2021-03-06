//!
//! The constant tuple element tests.
//!

use crate::error::Error;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::tuple::error::Error as TupleConstantError;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;
use zinc_lexical::Location;

#[test]
fn error_field_out_of_range() {
    let input = r#"
fn main() {
    const VALUE: bool = (true, true, false).5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Tuple(TupleConstantError::FieldOutOrRange {
            location: Location::test(3, 45),
            type_identifier: Type::tuple(Some(Location::test(3, 45)), vec![Type::boolean(None); 3])
                .to_string(),
            field_index: 5,
        })),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
