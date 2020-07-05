//!
//! The semantic analyzer contract value element.
//!

mod tests;

pub mod error;

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::access::dot::contract_field::ContractField as ContractFieldAccess;
use crate::semantic::element::r#type::contract::Contract as ContractType;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::structure::Structure as StructureValue;
use crate::semantic::element::value::Value;
use crate::syntax::tree::identifier::Identifier;

use self::error::Error;

///
/// Contracts are collections of named elements of different types.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Contract {
    pub location: Option<Location>,
    pub fields: Vec<(String, Option<Location>, Type)>,
    pub r#type: Option<ContractType>,
}

impl Contract {
    pub fn new(location: Option<Location>) -> Self {
        Self {
            location,
            fields: vec![],
            r#type: None,
        }
    }

    pub fn new_with_type(location: Option<Location>, r#type: ContractType) -> Self {
        Self {
            location,
            fields: r#type
                .fields
                .clone()
                .into_iter()
                .map(|(name, r#type)| (name, None, r#type))
                .collect(),
            r#type: Some(r#type),
        }
    }

    pub fn from_structure(structure: StructureValue) -> Self {
        Self {
            location: structure.location,
            fields: structure.fields,
            r#type: None,
        }
    }

    pub fn r#type(&self) -> Type {
        self.r#type
            .clone()
            .map(Type::Contract)
            .expect(crate::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS)
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.r#type == other.r#type
    }

    pub fn push(&mut self, name: String, location: Option<Location>, r#type: Type) {
        self.fields.push((name, location, r#type));
    }

    pub fn validate(&mut self, expected: ContractType) -> Result<(), Error> {
        for (index, (name, location, r#type)) in self.fields.iter().enumerate() {
            match expected.fields.get(index) {
                Some((expected_name, expected_type)) => {
                    if name != expected_name {
                        return Err(Error::FieldExpected {
                            location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                            type_identifier: expected.identifier.to_owned(),
                            position: index + 1,
                            expected: expected_name.to_owned(),
                            found: name.to_owned(),
                        });
                    }

                    if r#type != expected_type {
                        return Err(Error::FieldInvalidType {
                            location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                            type_identifier: expected.identifier.to_owned(),
                            field_name: expected_name.to_owned(),
                            expected: expected_type.to_string(),
                            found: r#type.to_string(),
                        });
                    }
                }
                None => {
                    return Err(Error::FieldOutOfRange {
                        location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                        type_identifier: expected.identifier.to_owned(),
                        expected: expected.fields.len(),
                        found: index + 1,
                    });
                }
            }
        }

        self.r#type = Some(expected);

        Ok(())
    }

    pub fn slice(self, expected: Identifier) -> Result<(Value, ContractFieldAccess), Error> {
        let mut offset = 0;
        let total_size = self.r#type().size();

        for (index, (name, _location, r#type)) in self.fields.iter().enumerate() {
            if name == expected.name.as_str() {
                let access = ContractFieldAccess::new(
                    expected.name,
                    index,
                    offset,
                    r#type.size(),
                    total_size,
                );

                let result = Value::try_from_type(r#type, self.location)
                    .expect(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS);

                return Ok((result, access));
            }
            offset += r#type.size();
        }

        Err(Error::FieldDoesNotExist {
            location: expected.location,
            type_identifier: self
                .r#type
                .expect(crate::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS)
                .identifier,
            field_name: expected.name,
        })
    }
}

impl fmt::Display for Contract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<runtime> '{}' with fields {}",
            self.r#type
                .as_ref()
                .expect(crate::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS)
                .identifier,
            self.fields
                .iter()
                .map(|(name, _location, r#type)| format!("'{}' of type '{}'", name, r#type))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}