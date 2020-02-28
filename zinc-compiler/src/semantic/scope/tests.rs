//!
//! The array tests.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::scope::error::Error as ScopeError;
use crate::semantic::Error as SemanticError;
use crate::Error;

#[test]
fn item_is_not_namespace() {
    let input = r#"
const NOT_NAMESPACE: u8 = 42;

fn main() {
    let result = NOT_NAMESPACE::UNDEFINED;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        Location::new(5, 18),
        ScopeError::ItemIsNotNamespace("NOT_NAMESPACE".to_owned()),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn item_redeclared() {
    let input = r#"
fn main() {
    let result = 42;
    {
        let result = 69;
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        Location::new(5, 9),
        ScopeError::ItemRedeclared("result".to_owned(), Location::new(3, 9)),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn item_undeclared() {
    let input = r#"
fn main() {
    result = 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        Location::new(3, 5),
        ScopeError::ItemUndeclared("result".to_owned()),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn item_undeclared_enum_variant() {
    let input = r#"
enum Jabberwocky {
    Gone = 42,
}

fn main() {
    let really = Jabberwocky::Exists;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        Location::new(7, 31),
        ScopeError::ItemUndeclared("Exists".to_owned()),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}
