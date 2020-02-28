//!
//! The Zinc compiler error.
//!

pub mod file;

use colored::Colorize;

use crate::lexical::Error as LexicalError;
use crate::lexical::Location;
use crate::semantic::caster::error::Error as CasterError;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::place::error::Error as PlaceError;
use crate::semantic::element::r#type::function::builtin::error::Error as BuiltInFunctionError;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::element::r#type::function::stdlib::error::Error as StandardLibraryFunctionError;
use crate::semantic::element::value::array::error::Error as ArrayValueError;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::element::value::structure::error::Error as StructureValueError;
use crate::semantic::element::value::tuple::error::Error as TupleValueError;
use crate::semantic::scope::error::Error as ScopeError;
use crate::semantic::Error as SemanticError;
use crate::syntax::Error as SyntaxError;

use self::file::Error as FileError;

#[derive(Debug, PartialEq)]
pub enum Error {
    File(FileError),
    Lexical(LexicalError),
    Syntax(SyntaxError),
    Semantic(SemanticError),
}

impl Error {
    pub fn format(self, context: &[&str]) -> String {
        match self {
            Self::File(inner) => inner.to_string(),

            Self::Lexical(LexicalError::UnterminatedBlockComment { start, end }) => {
                Self::format_range(context, "unterminated block comment", start, end, None)
            }
            Self::Lexical(LexicalError::UnterminatedDoubleQuoteString { start, end }) => {
                Self::format_range(
                    context,
                    "unterminated double quote string",
                    start,
                    end,
                    None,
                )
            }
            Self::Lexical(LexicalError::ExpectedOneOf {
                location,
                expected,
                found,
            }) => Self::format_line(
                context,
                format!("expected one of {}, found `{}`", expected, found).as_str(),
                location,
                None,
            ),
            Self::Lexical(LexicalError::ExpectedOneOfDecimal {
                location,
                expected,
                found,
            }) => Self::format_line(
                context,
                format!(
                    "expected one of decimal symbols {}, found `{}`",
                    expected, found
                )
                .as_str(),
                location,
                None,
            ),
            Self::Lexical(LexicalError::ExpectedOneOfHexadecimal {
                location,
                expected,
                found,
            }) => Self::format_line(
                context,
                format!(
                    "expected one of hexadecimal symbols {}, found `{}`",
                    expected, found
                )
                .as_str(),
                location,
                None,
            ),
            Self::Lexical(LexicalError::InvalidCharacter { location, found }) => Self::format_line(
                context,
                format!("invalid character `{}`", found).as_str(),
                location,
                None,
            ),
            Self::Lexical(LexicalError::UnexpectedEnd { location }) => {
                Self::format_line(context, "unexpected end of input", location, None)
            }

            Self::Syntax(SyntaxError::ExpectedOneOf {
                location,
                expected,
                found,
                help,
            }) => Self::format_line(
                context,
                format!("expected one of {}, found `{}`", expected, found).as_str(),
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedOneOfOrOperator {
                location,
                expected,
                found,
                help,
            }) => Self::format_line(
                context,
                format!(
                    "expected one of {} or an operator, found `{}`",
                    expected, found
                )
                .as_str(),
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedIdentifier {
                location,
                found,
                help,
            }) => Self::format_line(
                context,
                format!("expected identifier, found `{}`", found).as_str(),
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedMutOrIdentifier {
                location,
                found,
                help,
            }) => Self::format_line(
                context,
                format!("expected `mut` or identifier, found `{}`", found).as_str(),
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedFieldIdentifier {
                location,
                found,
                help,
            }) => Self::format_line(
                context,
                format!("expected field identifier, found `{}`", found).as_str(),
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedType {
                location,
                found,
                help,
            }) => Self::format_line(
                context,
                format!("expected type, found `{}`", found).as_str(),
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedTypeOrValue {
                location,
                found,
                help,
            }) => Self::format_line(
                context,
                format!(
                    "expected `:` with type or `=` with value, found `{}`",
                    found
                )
                .as_str(),
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedValue {
                location,
                found,
                help,
            }) => Self::format_line(
                context,
                format!("expected `=` with value, found `{}`", found).as_str(),
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedExpressionOrOperand { location, found }) => {
                Self::format_line(
                    context,
                    format!("expected expression or operand, found `{}`", found).as_str(),
                    location,
                    None,
                )
            }
            Self::Syntax(SyntaxError::ExpectedIntegerLiteral { location, found }) => {
                Self::format_line(
                    context,
                    format!("expected integer literal, found `{}`", found).as_str(),
                    location,
                    None,
                )
            }
            Self::Syntax(SyntaxError::ExpectedBindingPattern { location, found }) => {
                Self::format_line(
                    context,
                    format!("expected identifier or `_`, found `{}`", found).as_str(),
                    location,
                    None,
                )
            }
            Self::Syntax(SyntaxError::ExpectedMatchPattern { location, found }) => {
                Self::format_line(
                    context,
                    format!(
                        "expected identifier, boolean or integer literal, path, or `_`, found `{}`",
                        found
                    )
                    .as_str(),
                    location,
                    None,
                )
            }

            Self::Semantic(SemanticError::Element(location, ElementError::OperatorAssignmentFirstOperandExpectedPlace(found))) => {
                Self::format_line(
                    context,
                    format!(
                        "the assignment operator `=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorAssignmentSecondOperandExpectedEvaluable(found))) => {
                Self::format_line(
                    context,
                    format!(
                        "the assignment operator `=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorRangeInclusiveFirstOperandExpectedConstant(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorRangeInclusiveFirstOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the inclusive range operator `..=` expected an integer constant as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorRangeInclusiveSecondOperandExpectedConstant(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorRangeInclusiveSecondOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the inclusive range operator `..=` expected an integer constant as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorRangeFirstOperandExpectedConstant(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorRangeFirstOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the range operator `..` expected an integer constant as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorRangeSecondOperandExpectedConstant(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorRangeSecondOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the range operator `..` expected an integer constant as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorOrFirstOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorOrFirstOperandExpectedBoolean(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorOrFirstOperandExpectedBoolean(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the OR operator `||` expected a boolean as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorOrSecondOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorOrSecondOperandExpectedBoolean(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorOrSecondOperandExpectedBoolean(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the OR operator `||` expected a boolean as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorXorFirstOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorXorFirstOperandExpectedBoolean(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorXorFirstOperandExpectedBoolean(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the XOR operator `^^` expected a boolean as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorXorSecondOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorXorSecondOperandExpectedBoolean(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorXorSecondOperandExpectedBoolean(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the XOR operator `^^` expected a boolean as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorAndFirstOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorAndFirstOperandExpectedBoolean(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorAndFirstOperandExpectedBoolean(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the AND operator `&&` expected a boolean as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorAndSecondOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorAndSecondOperandExpectedBoolean(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorAndSecondOperandExpectedBoolean(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the AND operator `&&` expected a boolean as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorEqualsFirstOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorEqualsFirstOperandExpectedPrimitiveType(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorEqualsFirstOperandExpectedPrimitiveType(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the equals operator `==` expected a unit, boolean or integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorEqualsSecondOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorEqualsSecondOperandExpectedUnit(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorEqualsSecondOperandExpectedBoolean(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorEqualsSecondOperandExpectedInteger(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorEqualsSecondOperandExpectedUnit(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorEqualsSecondOperandExpectedBoolean(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorEqualsSecondOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the equals operator `==` expected a unit, boolean or integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorNotEqualsFirstOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorNotEqualsFirstOperandExpectedPrimitiveType(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorNotEqualsFirstOperandExpectedPrimitiveType(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the not equals operator `!=` expected a boolean or integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorNotEqualsSecondOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorNotEqualsSecondOperandExpectedUnit(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorNotEqualsSecondOperandExpectedBoolean(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorNotEqualsSecondOperandExpectedInteger(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorNotEqualsSecondOperandExpectedUnit(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorNotEqualsSecondOperandExpectedBoolean(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorNotEqualsSecondOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the not equals operator `!=` expected a boolean or integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorGreaterEqualsFirstOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorGreaterEqualsFirstOperandExpectedInteger(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorGreaterEqualsFirstOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the greater equals operator `>=` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorGreaterEqualsSecondOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorGreaterEqualsSecondOperandExpectedInteger(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorGreaterEqualsSecondOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the greater equals operator `>=` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorLesserEqualsFirstOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorLesserEqualsFirstOperandExpectedInteger(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorLesserEqualsFirstOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the lesser equals operator `<=` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorLesserEqualsSecondOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorLesserEqualsSecondOperandExpectedInteger(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorLesserEqualsSecondOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the lesser equals operator `<=` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorGreaterFirstOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorGreaterFirstOperandExpectedInteger(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorGreaterFirstOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the greater operator `>` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorGreaterSecondOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorGreaterSecondOperandExpectedInteger(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorGreaterSecondOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the greater operator `>` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorLesserFirstOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorLesserFirstOperandExpectedInteger(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorLesserFirstOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the lesser operator `<` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorLesserSecondOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorLesserSecondOperandExpectedInteger(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorLesserSecondOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the lesser operator `<` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorAdditionFirstOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorAdditionFirstOperandExpectedInteger(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorAdditionFirstOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the addition operator `+` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorAdditionSecondOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorAdditionSecondOperandExpectedInteger(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorAdditionSecondOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the addition operator `+` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorSubtractionFirstOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorSubtractionFirstOperandExpectedInteger(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorSubtractionFirstOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the subtraction operator `-` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorSubtractionSecondOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorSubtractionSecondOperandExpectedInteger(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorSubtractionSecondOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the subtraction operator `-` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorMultiplicationFirstOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorMultiplicationFirstOperandExpectedInteger(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorMultiplicationFirstOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the multiplication operator `*` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorMultiplicationSecondOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorMultiplicationSecondOperandExpectedInteger(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorMultiplicationSecondOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the multiplication operator `*` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorDivisionFirstOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorDivisionFirstOperandExpectedInteger(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorDivisionFirstOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the division operator `/` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorDivisionSecondOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorDivisionSecondOperandExpectedInteger(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorDivisionSecondOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the division operator `/` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorRemainderFirstOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorRemainderFirstOperandExpectedInteger(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorRemainderFirstOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the remainder operator `%` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorRemainderSecondOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorRemainderSecondOperandExpectedInteger(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorRemainderSecondOperandExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the remainder operator `%` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorCastingFirstOperandExpectedEvaluable(found))) => {
                Self::format_line(
                    context,
                    format!(
                        "the casting operator `as` expected a value as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorCastingSecondOperandExpectedType(found))) => {
                Self::format_line(
                    context,
                    format!(
                        "the casting operator `as` expected a type as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::Casting(CasterError::FromInvalidType(from, to))))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::Casting(CasterError::ToInvalidType(from, to))))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::Casting(CasterError::FromInvalidType(from, to))))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::Casting(CasterError::ToInvalidType(from, to))))) => {
                Self::format_line(
                    context,
                    format!(
                        "cannot cast from `{}` to `{}`",
                        from, to,
                    )
                        .as_str(),
                    location,
                    Some("only integer values can be casted to greater or equal bitlength"),
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::Casting(CasterError::ToLesserBitlength(from, to))))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::Casting(CasterError::ToLesserBitlength(from, to))))) => {
                Self::format_line(
                    context,
                    format!(
                        "cannot cast an integer with bitlength `{}` to an integer with bitlength `{}`",
                        from, to,
                    )
                        .as_str(),
                    location,
                    Some("integer values can only be casted to greater or equal bitlength"),
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorNegationExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorNegationExpectedInteger(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorNegationExpectedInteger(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the negation operator `-` expected an integer, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorNotExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorNotExpectedBoolean(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::OperatorNotExpectedBoolean(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the NOT operator `!` expected a boolean, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorIndexFirstOperandExpectedPlaceOrEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Place(PlaceError::OperatorIndexFirstOperandExpectedArray(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorIndexFirstOperandExpectedArray(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the index operator `[]` expected an array as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorIndexSecondOperandExpectedEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Place(PlaceError::OperatorIndexSecondOperandExpectedIntegerOrRange(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorIndexSecondOperandExpectedIntegerOrRange(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the index operator `[]` expected an integer or range as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorFieldFirstOperandExpectedPlaceOrEvaluable(found))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Place(PlaceError::OperatorFieldFirstOperandExpectedTuple(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Place(PlaceError::OperatorFieldFirstOperandExpectedStructure(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorFieldFirstOperandExpectedTuple(found)))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::OperatorFieldFirstOperandExpectedStructure(found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the field access operator `.` expected a tuple or structure as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorFieldSecondOperandExpectedMember(found))) => {
                Self::format_line(
                    context,
                    format!(
                        "the field access operator `.` expected a tuple or structure field identifier as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorPathFirstOperandExpectedPath(found))) => {
                Self::format_line(
                    context,
                    format!(
                        "the path resolution operator `::` expected an item identifier as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::OperatorPathSecondOperandExpectedMemberString(found))) => {
                Self::format_line(
                    context,
                    format!(
                        "the path resolution operator `::` expected an item identifier as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }

            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::Array(ArrayValueError::PushingInvalidType(expected, found))))) => {
                Self::format_line(
                    context,
                    format!(
                        "expected `{}`, found `{}`",
                        expected, found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::Array(ArrayValueError::SliceStartOutOfRange(left))))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Place(PlaceError::ArraySliceStartOutOfRange(left)))) => {
                Self::format_line(
                    context,
                    format!(
                        "left slice bound `{}` is negative",
                        left,
                    )
                        .as_str(),
                    location,
                    Some("slice range bounds must be within the array size"),
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::Array(ArrayValueError::SliceEndOutOfRange(right, size))))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Place(PlaceError::ArraySliceEndOutOfRange(right, size)))) => {
                Self::format_line(
                    context,
                    format!(
                        "right slice bound `{}` is out of range of the array of size {}",
                        right, size,
                    )
                        .as_str(),
                    location,
                    Some("slice range bounds must be within the array size"),
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::Array(ArrayValueError::SliceEndLesserThanStart(left, right))))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Place(PlaceError::ArraySliceEndLesserThanStart(left, right)))) => {
                Self::format_line(
                    context,
                    format!(
                        "left slice bound `{}` is greater than right slice bound `{}`",
                        left, right,
                    )
                        .as_str(),
                    location,
                    Some("left slice range bound must be lesser or equal to the right one"),
                )
            }

            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::Tuple(TupleValueError::FieldDoesNotExist(index, r#type))))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Place(PlaceError::TupleFieldDoesNotExist(index, r#type)))) => {
                Self::format_line(
                    context,
                    format!(
                        "tuple `{}` has no field with index `{}`",
                        r#type, index,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }

            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::Structure(StructureValueError::FieldAlreadyExists(field, r#type))))) => {
                Self::format_line(
                    context,
                    format!(
                        "field `{}` already exists in the structure literal of type `{}`",
                        field, r#type,
                    )
                        .as_str(),
                    location,
                    Some("each field may be specified only once"),
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::Structure(StructureValueError::FieldDoesNotExist(field, r#type))))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Place(PlaceError::StructureFieldDoesNotExist(field, r#type)))) => {
                Self::format_line(
                    context,
                    format!(
                        "field `{}` does not exist in the structure `{}` at the current position",
                        field, r#type,
                    )
                        .as_str(),
                    location,
                    Some("specify the fields in the correct order"),
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::Structure(StructureValueError::FieldInvalidType(field, r#type, expected, found))))) => {
                Self::format_line(
                    context,
                    format!(
                        "field `{}` of structure `{}` expected `{}`, found `{}`",
                        field, r#type, expected, found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::ConvertingFromType(r#type)))) => {
                Self::format_line(
                    context,
                    format!(
                        "cannot create a value of type `{}`",
                        r#type,
                    )
                        .as_str(),
                    location,
                    Some("strings, ranges, and functions cannot be used as values or stored as variables"),
                )
            }

            Self::Semantic(SemanticError::Scope(location, ScopeError::ItemRedeclared(item, reference))) => {
                Self::format_line_with_reference(
                    context,
                    format!(
                        "item `{}` redeclared here",
                        item
                    )
                        .as_str(),
                    location,
                    Some(reference),
                    Some("consider giving the latter item another name"),
                )
            }
            Self::Semantic(SemanticError::Scope(location, ScopeError::ItemUndeclared(item))) => {
                Self::format_line(
                    context,
                    format!(
                        "cannot find item `{}` in this scope",
                        item
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Scope(location, ScopeError::ItemIsNotNamespace(item))) => {
                Self::format_line(
                    context,
                    format!(
                        "item `{}` is not a namespace",
                        item
                    )
                        .as_str(),
                    location,
                    Some("only modules, structures, and enumerations can contain items within their namespaces"),
                )
            }

            Self::Semantic(SemanticError::Function(location, FunctionError::ArgumentCount(name, expected, found))) => {
                Self::format_line(
                    context,
                    format!(
                        "function `{}` expected {} arguments, found {}",
                        name, expected, found
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Function(location, FunctionError::ArgumentType(name, expected, index, field, found))) => {
                Self::format_line(
                    context,
                    format!(
                        "function `{}` expected type `{}` as the argument `{}` (#{}), found `{}`",
                        name, expected, field, index, found
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Function(location, FunctionError::ArgumentConstantness(name, index, field, found))) => {
                Self::format_line(
                    context,
                    format!(
                        "function `{}` expected a constant as the argument `{}` (#{}), found a non-constant of type `{}`",
                        name, field, index, found
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Function(location, FunctionError::ArgumentNotEvaluable(name, index, found))) => {
                Self::format_line(
                    context,
                    format!(
                        "function `{}` expected a value as the argument #{}, found `{}`",
                        name, index, found
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Function(location, FunctionError::ReturnType(name, expected, found, reference))) => {
                Self::format_line_with_reference(
                    context,
                    format!(
                        "function `{}` must return a value of type `{}`, found `{}`",
                        name, expected, found
                    )
                        .as_str(),
                    location,
                    Some(reference),
                    None,
                )
            }
            Self::Semantic(SemanticError::Function(location, FunctionError::NonCallable(name))) => {
                Self::format_line(
                    context,
                    format!(
                        "attempt to call a non-callable item `{}`",
                        name
                    )
                        .as_str(),
                    location,
                    Some("only functions may be called"),
                )
            }
            Self::Semantic(SemanticError::Function(location, FunctionError::BuiltIn(BuiltInFunctionError::Unknown(name)))) => {
                Self::format_line(
                    context,
                    format!(
                        "attempt to call a non-builtin function `{}` with `!` specifier",
                        name
                    )
                        .as_str(),
                    location,
                    Some("only built-in functions require the `!` symbol after the function name"),
                )
            }
            Self::Semantic(SemanticError::Function(location, FunctionError::BuiltIn(BuiltInFunctionError::SpecifierMissing(name)))) => {
                Self::format_line(
                    context,
                    format!(
                        "attempt to call a builtin function `{}` without `!` specifier",
                        name
                    )
                        .as_str(),
                    location,
                    Some("built-in functions require the `!` symbol after the function name"),
                )
            }
            Self::Semantic(SemanticError::Function(location, FunctionError::BuiltIn(BuiltInFunctionError::DebugArgumentCount(expected, found)))) => {
                Self::format_line(
                    context,
                    format!(
                        "the `dbg!` function expected {} arguments, but got {}",
                        expected, found,
                    )
                        .as_str(),
                    location,
                    Some("the number of `dbg!` arguments after the format string must be equal to the number of placeholders, e.g. `dbg!(\"{}, {}\", a, b)`"),
                )
            }
            Self::Semantic(SemanticError::Function(location, FunctionError::StandardLibrary(StandardLibraryFunctionError::ArrayTruncatingToBiggerSize(from, to)))) => {
                Self::format_line(
                    context,
                    format!(
                        "attempt to truncate an array from size {} to bigger size {}",
                        from, to,
                    )
                        .as_str(),
                    location,
                    Some("consider truncating the array to a smaller size"),
                )
            }
            Self::Semantic(SemanticError::Function(location, FunctionError::StandardLibrary(StandardLibraryFunctionError::ArrayPaddingToLesserSize(from, to)))) => {
                Self::format_line(
                    context,
                    format!(
                        "attempt to pad an array from size {} to lesser size {}",
                        from, to,
                    )
                        .as_str(),
                    location,
                    Some("consider padding the array to a bigger size"),
                )
            }
            Self::Semantic(SemanticError::Function(location, FunctionError::StandardLibrary(StandardLibraryFunctionError::ArrayNewLengthInvalid(value)))) => {
                Self::format_line(
                    context,
                    format!(
                        "new array length '{}' cannot act as an index",
                        value,
                    )
                        .as_str(),
                    location,
                    Some("array indexes cannot be greater than maximum of `u64`"),
                )
            }

            Self::Semantic(SemanticError::MatchNotExhausted(location)) => {
                Self::format_line(
                    context,
                    "match expression must be exhaustive",
                    location,
                    Some("consider covering all possible cases or adding an irrefutable branch with a binding or `_` wildcard"),
                )
            }
            Self::Semantic(SemanticError::MatchBranchUnreachable(location)) => {
                Self::format_line(
                    context,
                    "match expression branch is unreachable",
                    location,
                    Some("consider removing the branch or moving it above the branch with irrefutable pattern"),
                )
            }
            Self::Semantic(SemanticError::MatchBranchPatternPathExpectedEvaluable(location, item)) => {
                Self::format_line(
                    context,
                    format!("expected value, found `{}`", item).as_str(),
                    location,
                    Some("consider specifying a path to an evaluable item"),
                )
            }
            Self::Semantic(SemanticError::MatchBranchPatternInvalidType(location, expected, found, reference)) => {
                Self::format_line_with_reference(
                    context,
                    format!("expected `{}`, found `{}`", expected, found).as_str(),
                    location,
                    Some(reference),
                    Some("all branch patterns must be compatible with the type of the expression being matched"),
                )
            }
            Self::Semantic(SemanticError::MatchBranchExpressionInvalidType(location, expected, found, reference)) => {
                Self::format_line_with_reference(
                    context,
                    format!("expected `{}`, found `{}`", expected, found).as_str(),
                    location,
                    Some(reference),
                    Some("all branches must return type equal to the type of the first branch"),
                )
            }

            Self::Semantic(SemanticError::MutatingWithDifferentType(location, expected, found)) => {
                Self::format_line(
                    context,
                    format!("expected `{}`, found `{}`", expected, found).as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::MutatingImmutableMemory(location, name, reference)) => {
                Self::format_line_with_reference(
                    context,
                    format!("cannot assign twice to immutable variable `{}`", name).as_str(),
                    location,
                    reference,
                    Some(format!("make this variable mutable: `mut {}`", name).as_str()),
                )
            }

            Self::Semantic(SemanticError::LoopWhileExpectedBooleanCondition(location, r#type)) => {
                Self::format_line(
                    context,
                    format!("expected `bool`, found `{}`", r#type).as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::LoopBoundsExpectedConstantRangeExpression(location, value)) => {
                Self::format_line(
                    context,
                    format!("expected a constant range expression, found `{}`", value).as_str(),
                    location,
                    Some("only constant ranges allowed, e.g. `for i in 0..42 { ... }`"),
                )
            }

            Self::Semantic(SemanticError::ConditionalExpectedBooleanCondition(location, r#type)) => {
                Self::format_line(
                    context,
                    format!("expected `bool`, found `{}`", r#type).as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::ConditionalBranchTypesMismatch(location, first, second)) => {
                Self::format_line(
                    context,
                    format!("if and else have incompatible types `{}` and `{}`", first, second).as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::EntryPointMissing) => {
                Self::format_message(
                    "function `main` is missing",
                    Some("create the `main` function in the entry point file `main.zn`"),
                )
            }
            Self::Semantic(SemanticError::ModuleNotFound(location, name)) => {
                Self::format_line(
                    context,
                    format!(
                        "file not found for module `{}`",
                        name
                    )
                        .as_str(),
                    location,
                    Some(format!("create a file called `{}.zn` inside the 'src' directory", name).as_str()),
                )
            }
            Self::Semantic(SemanticError::UseExpectedPath(location, item)) => {
                Self::format_line(
                    context,
                    format!(
                        "`use` expected an item path, but got `{}`",
                        item
                    )
                        .as_str(),
                    location,
                    Some("consider specifying a valid path to an item to import"),
                )
            }
            Self::Semantic(SemanticError::ImplStatementExpectedStructureOrEnumeration(location, item)) => {
                Self::format_line(
                    context,
                    format!(
                        "`impl` expected a type with namespace, but got `{}`",
                        item
                    )
                        .as_str(),
                    location,
                    Some("only structures and enumerations can have an implementation"),
                )
            }
            Self::Semantic(SemanticError::TypeAliasDoesNotPointToType(location, item)) => {
                Self::format_line(
                    context,
                    format!(
                        "expected type, found `{}`",
                        item
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::TypeAliasDoesNotPointToStructure(location, item)) => {
                Self::format_line(
                    context,
                    format!(
                        "expected structure type, found `{}`",
                        item
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::ConstantExpressionHasNonConstantElement(location, _item)) => {
                Self::format_line(
                    context,
                    "attempt to use a non-constant value in a constant",
                    location,
                    None,
                )
            }

            Self::Semantic(SemanticError::Element(_, ElementError::Value(ValueError::Integer(..)))) => unimplemented!(),
            Self::Semantic(SemanticError::Element(_, ElementError::Constant(ConstantError::Integer(..)))) => unimplemented!(),
        }
    }

    fn format_message(message: &str, help: Option<&str>) -> String {
        let mut strings = Vec::with_capacity(8);
        strings.push(String::new());
        strings.push(format!(
            "{}: {}",
            "error".bright_red(),
            message.bright_white()
        ));
        if let Some(help) = help {
            strings.push(format!("{}: {}", "help".bright_white(), help.bright_blue()));
        }
        strings.push(String::new());
        strings.join("\n")
    }

    fn format_line(
        context: &[&str],
        message: &str,
        location: Location,
        help: Option<&str>,
    ) -> String {
        let line_number_length = location.line.to_string().len();

        let mut strings = Vec::with_capacity(8);
        strings.push(String::new());
        strings.push(format!(
            "{}: {}",
            "error".bright_red(),
            message.bright_white()
        ));
        strings.push(format!(" {} {}", "-->".bright_cyan(), location));
        strings.push(format!(
            "{}{}",
            " ".repeat(line_number_length + 1),
            "|".bright_cyan()
        ));
        if let Some(line) = context.get(location.line - 1) {
            strings.push(format!(
                "{}{}",
                (location.line.to_string() + " | ").bright_cyan(),
                line
            ));
        }
        strings.push(format!(
            "{}{} {}{}",
            " ".repeat(line_number_length + 1),
            "|".bright_cyan(),
            "_".repeat(location.column - 1).bright_red(),
            "^".bright_red()
        ));
        if let Some(help) = help {
            strings.push(format!("{}: {}", "help".bright_white(), help.bright_blue()));
        }
        strings.push(String::new());
        strings.join("\n")
    }

    fn format_line_with_reference(
        context: &[&str],
        message: &str,
        location: Location,
        reference: Option<Location>,
        help: Option<&str>,
    ) -> String {
        let line_number_length = location.line.to_string().len();

        let mut strings = Vec::with_capacity(11);
        strings.push(String::new());
        strings.push(format!(
            "{}: {}",
            "error".bright_red(),
            message.bright_white()
        ));

        if let Some(reference) = reference {
            let line_number_length = reference.line.to_string().len();
            strings.push(format!(
                "{}{}",
                " ".repeat(line_number_length + 1),
                "|".bright_cyan()
            ));
            if let Some(line) = context.get(reference.line - 1) {
                strings.push(format!(
                    "{}{}",
                    (reference.line.to_string() + " | ").bright_cyan(),
                    line
                ));
            }
            strings.push(format!(
                "{}{} {}{}",
                " ".repeat(line_number_length + 1),
                "|".bright_cyan(),
                "_".repeat(reference.column - 1).bright_red(),
                "^".bright_red()
            ));
        }

        strings.push(format!(" {} {}", "-->".bright_cyan(), location));

        strings.push(format!(
            "{}{}",
            " ".repeat(line_number_length + 1),
            "|".bright_cyan()
        ));
        if let Some(line) = context.get(location.line - 1) {
            strings.push(format!(
                "{}{}",
                (location.line.to_string() + " | ").bright_cyan(),
                line
            ));
        }
        strings.push(format!(
            "{}{} {}{}",
            " ".repeat(line_number_length + 1),
            "|".bright_cyan(),
            "_".repeat(location.column - 1).bright_red(),
            "^".bright_red()
        ));

        if let Some(help) = help {
            strings.push(format!("{}: {}", "help".bright_white(), help.bright_blue()));
        }
        strings.push(String::new());
        strings.join("\n")
    }

    fn format_range(
        context: &[&str],
        message: &'static str,
        start: Location,
        end: Location,
        help: Option<&str>,
    ) -> String {
        let line_number_length = end.line.to_string().len();

        let mut strings = Vec::with_capacity(8 + end.line - start.line);
        strings.push(String::new());
        strings.push(format!(
            "{}: {}",
            "error".bright_red(),
            message.bright_white()
        ));
        strings.push(format!(" {} {}", "-->".bright_cyan(), start));
        strings.push(format!(
            "{}{}",
            " ".repeat(line_number_length + 1),
            "|".bright_cyan()
        ));
        for line_number in start.line..=end.line {
            if let Some(line) = context.get(line_number - 1) {
                strings.push(format!(
                    "{}{}",
                    (line_number.to_string() + " | ").bright_cyan(),
                    line
                ));
            }
        }
        strings.push(format!(
            "{}{} {}{}",
            " ".repeat(line_number_length + 1),
            "|".bright_cyan(),
            "_".repeat(end.column - 1).bright_red(),
            "^".bright_red()
        ));
        if let Some(help) = help {
            strings.push(format!("{}: {}", "help".bright_white(), help.bright_blue()));
        }
        strings.push(String::new());
        strings.join("\n")
    }
}

impl From<FileError> for Error {
    fn from(error: FileError) -> Self {
        Self::File(error)
    }
}

impl From<LexicalError> for Error {
    fn from(error: LexicalError) -> Self {
        Self::Lexical(error)
    }
}

impl From<SyntaxError> for Error {
    fn from(error: SyntaxError) -> Self {
        Self::Syntax(error)
    }
}

impl From<SemanticError> for Error {
    fn from(error: SemanticError) -> Self {
        Self::Semantic(error)
    }
}
