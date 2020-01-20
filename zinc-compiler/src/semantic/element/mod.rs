//!
//! The semantic analyzer element.
//!

mod access;
mod constant;
mod error;
mod path;
mod place;
mod r#type;
mod value;

pub use self::access::FieldResult as FieldAccessResult;
pub use self::access::IndexResult as IndexAccessResult;
pub use self::constant::Constant;
pub use self::constant::Error as ConstantError;
pub use self::constant::Integer as IntegerConstant;
pub use self::constant::IntegerError as IntegerConstantError;
pub use self::error::Error;
pub use self::path::Path;
pub use self::place::Error as PlaceError;
pub use self::place::Place;
pub use self::r#type::FunctionBehavior;
pub use self::r#type::Type;
pub use self::value::Array;
pub use self::value::ArrayError as ArrayValueError;
pub use self::value::Error as ValueError;
pub use self::value::Integer as IntegerValue;
pub use self::value::IntegerError as IntegerValueError;
pub use self::value::Structure;
pub use self::value::StructureError as StructureValueError;
pub use self::value::Tuple;
pub use self::value::TupleError as TupleValueError;
pub use self::value::Value;

use std::fmt;

use crate::syntax::MemberString;

#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    /// Runtime value, which is unknown at compile time (a.k.a. `lvalue`)
    Value(Value),
    /// Constant value, which is known at compile time (a.k.a. `lvalue`)
    Constant(Constant),
    /// The second operand of the casting operator
    Type(Type),
    /// The second operand of the function call operator
    ArgumentList(Vec<Self>),

    /// Path to be resolved in the scope
    Path(Path),
    /// Memory descriptor (a.k.a. `lvalue`)
    Place(Place),
    /// Tuple field index
    MemberInteger(usize),
    /// Structure field name
    MemberString(MemberString),
    /// Module scope shared reference
    Module(String),
}

impl Element {
    pub fn assign(self, other: &Self) -> Result<Place, Error> {
        match other {
            Self::Value(_) => {}
            Self::Constant(_) => {}
            element => {
                return Err(Error::OperatorAssignmentSecondOperandExpectedEvaluable(
                    element.to_string(),
                ))
            }
        }

        match self {
            Self::Place(place) => Ok(place),
            element => Err(Error::OperatorAssignmentFirstOperandExpectedPlace(
                element.to_string(),
            )),
        }
    }

    pub fn or(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => {
                value_1.or(value_2).map(Self::Value).map_err(Error::Value)
            }
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .or(&Value::from(value_2))
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(Error::OperatorOrSecondOperandExpectedEvaluable(
                element_2.to_string(),
            )),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::from(value_1)
                .or(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .or(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorOrSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorOrFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn xor(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => {
                value_1.xor(value_2).map(Self::Value).map_err(Error::Value)
            }
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .xor(&Value::from(value_2))
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorXorSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::from(value_1)
                .xor(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .xor(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorXorSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorXorFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn and(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => {
                value_1.and(value_2).map(Self::Value).map_err(Error::Value)
            }
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .and(&Value::from(value_2))
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorAndSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::from(value_1)
                .and(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .and(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorAndSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorAndFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn equals(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .equals(value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .equals(&Value::from(value_2))
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorEqualsSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::from(value_1)
                .equals(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .equals(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorEqualsSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorEqualsFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn not_equals(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .not_equals(value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .not_equals(&Value::from(value_2))
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorNotEqualsSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::from(value_1)
                .not_equals(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .not_equals(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorNotEqualsSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorNotEqualsFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn greater_equals(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .greater_equals(value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .greater_equals(&Value::from(value_2))
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorGreaterEqualsSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::from(value_1)
                .greater_equals(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .greater_equals(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorGreaterEqualsSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorGreaterEqualsFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn lesser_equals(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .lesser_equals(value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .lesser_equals(&Value::from(value_2))
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorLesserEqualsSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::from(value_1)
                .lesser_equals(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .lesser_equals(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorLesserEqualsSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorLesserEqualsFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn greater(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .greater(value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .greater(&Value::from(value_2))
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorGreaterSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::from(value_1)
                .greater(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .greater(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorGreaterSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorGreaterFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn lesser(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .lesser(value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .lesser(&Value::from(value_2))
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorLesserSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::from(value_1)
                .lesser(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .lesser(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorLesserSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorLesserFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn add(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => {
                value_1.add(value_2).map(Self::Value).map_err(Error::Value)
            }
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .add(&Value::from(value_2))
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorAdditionSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::from(value_1)
                .add(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .add(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorAdditionSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorAdditionFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn subtract(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .subtract(value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .subtract(&Value::from(value_2))
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorSubtractionSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::from(value_1)
                .subtract(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .subtract(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorSubtractionSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorSubtractionFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn multiply(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .multiply(value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .multiply(&Value::from(value_2))
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorMultiplicationSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::from(value_1)
                .multiply(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .multiply(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorMultiplicationSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorMultiplicationFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn divide(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .divide(value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .divide(&Value::from(value_2))
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorDivisionSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::from(value_1)
                .divide(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .divide(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorDivisionSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorDivisionFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn remainder(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Element::Value(value_1), Element::Value(value_2)) => value_1
                .remainder(value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(value_1), Element::Constant(value_2)) => value_1
                .remainder(&Value::from(value_2))
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Value(_), element_2) => Err(
                Error::OperatorRemainderSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (Element::Constant(value_1), Element::Value(value_2)) => Value::from(value_1)
                .remainder(&value_2)
                .map(Self::Value)
                .map_err(Error::Value),
            (Element::Constant(value_1), Element::Constant(value_2)) => value_1
                .remainder(value_2)
                .map(Self::Constant)
                .map_err(Error::Constant),
            (Element::Constant(_), element_2) => Err(
                Error::OperatorRemainderSecondOperandExpectedEvaluable(element_2.to_string()),
            ),
            (element_1, _) => Err(Error::OperatorRemainderFirstOperandExpectedEvaluable(
                element_1.to_string(),
            )),
        }
    }

    pub fn cast(&mut self, other: &Self) -> Result<Option<(bool, usize)>, Error> {
        let r#type = match other {
            Self::Type(r#type) => r#type,
            element => {
                return Err(Error::OperatorCastingSecondOperandExpectedType(
                    element.to_string(),
                ))
            }
        };

        match self {
            Element::Value(value) => value.cast(r#type).map_err(Error::Value),
            Element::Constant(constant) => constant.cast(r#type).map_err(Error::Constant),
            element => Err(Error::OperatorCastingFirstOperandExpectedEvaluable(
                element.to_string(),
            )),
        }
    }

    pub fn negate(&self) -> Result<Self, Error> {
        match self {
            Element::Value(value) => value.negate().map(Self::Value).map_err(Error::Value),
            Element::Constant(constant) => constant
                .negate()
                .map(Self::Constant)
                .map_err(Error::Constant),
            element => Err(Error::OperatorNegationExpectedEvaluable(
                element.to_string(),
            )),
        }
    }

    pub fn not(&self) -> Result<Self, Error> {
        match self {
            Element::Value(value) => value.not().map(Self::Value).map_err(Error::Value),
            Element::Constant(constant) => {
                constant.not().map(Self::Constant).map_err(Error::Constant)
            }
            element => Err(Error::OperatorNotExpectedEvaluable(element.to_string())),
        }
    }

    pub fn index(&mut self, other: &Self) -> Result<IndexAccessResult, Error> {
        match self {
            Self::Place(place) => match other {
                element @ Self::Value(_) => place.index(element).map_err(Error::Place),
                element @ Self::Constant(_) => place.index(element).map_err(Error::Place),
                element => Err(Error::OperatorIndexSecondOperandExpectedEvaluable(
                    element.to_string(),
                )),
            },
            Self::Value(value) => match other {
                Self::Value(index) => value.index_value(index).map_err(Error::Value),
                Self::Constant(index) => value.index_constant(index).map_err(Error::Value),
                element => Err(Error::OperatorIndexSecondOperandExpectedEvaluable(
                    element.to_string(),
                )),
            },
            element => Err(Error::OperatorIndexFirstOperandExpectedPlaceOrEvaluable(
                element.to_string(),
            )),
        }
    }

    pub fn field(&mut self, other: &Self) -> Result<FieldAccessResult, Error> {
        match self {
            Self::Place(place) => match other {
                Self::MemberInteger(member) => place.field_tuple(*member).map_err(Error::Place),
                Self::MemberString(member) => {
                    place.field_structure(&member.name).map_err(Error::Place)
                }
                element => Err(Error::OperatorFieldSecondOperandExpectedMember(
                    element.to_string(),
                )),
            },
            Self::Value(value) => match other {
                Self::MemberInteger(member) => value.field_tuple(*member).map_err(Error::Value),
                Self::MemberString(member) => {
                    value.field_structure(&member.name).map_err(Error::Value)
                }
                element => Err(Error::OperatorFieldSecondOperandExpectedMember(
                    element.to_string(),
                )),
            },
            element => Err(Error::OperatorFieldFirstOperandExpectedPlaceOrEvaluable(
                element.to_string(),
            )),
        }
    }

    pub fn path(&mut self, other: &Self) -> Result<(), Error> {
        let path = match self {
            Self::Path(path) => path,
            element => {
                return Err(Error::OperatorPathFirstOperandExpectedPath(
                    element.to_string(),
                ))
            }
        };

        let member = match other {
            Self::MemberString(member) => member,
            element => {
                return Err(Error::OperatorPathSecondOperandExpectedMemberString(
                    element.to_string(),
                ))
            }
        };

        path.push_element(member.to_owned());
        Ok(())
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Value(value) => write!(f, "{}", value),
            Self::Constant(constant) => write!(f, "{}", constant),
            Self::Type(r#type) => write!(f, "{}", r#type),
            Self::ArgumentList(values) => write!(
                f,
                "{}",
                values
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Path(path) => write!(f, "{}", path),
            Self::Place(place) => write!(f, "{}", place),
            Self::MemberInteger(integer) => write!(f, "{}", integer),
            Self::MemberString(member) => write!(f, "{}", member.name),
            Self::Module(name) => write!(f, "{}", name),
        }
    }
}
