//!
//! The fn statement builder.
//!

use crate::lexical::token::location::Location;
use crate::syntax::tree::attribute::Attribute;
use crate::syntax::tree::expression::block::Expression as BlockExpression;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::pattern_binding::Pattern as BindingPattern;
use crate::syntax::tree::r#type::Type;
use crate::syntax::tree::statement::r#fn::Statement as FnStatement;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    is_public: bool,
    is_constant: bool,
    identifier: Option<Identifier>,
    argument_bindings: Vec<BindingPattern>,
    return_type: Option<Type>,
    body: Option<BlockExpression>,
    attributes: Vec<Attribute>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_is_public(&mut self) {
        self.is_public = true;
    }

    pub fn set_is_constant(&mut self) {
        self.is_constant = true;
    }

    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    pub fn set_argument_bindings(&mut self, value: Vec<BindingPattern>) {
        self.argument_bindings = value;
    }

    pub fn set_return_type(&mut self, value: Type) {
        self.return_type = Some(value);
    }

    pub fn set_body(&mut self, value: BlockExpression) {
        self.body = Some(value);
    }

    pub fn set_attributes(&mut self, value: Vec<Attribute>) {
        self.attributes = value;
    }

    pub fn finish(mut self) -> FnStatement {
        FnStatement::new(
            self.location.take().unwrap_or_else(|| {
                panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "location")
            }),
            self.is_public,
            self.is_constant,
            self.identifier.take().unwrap_or_else(|| {
                panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "identifier")
            }),
            self.argument_bindings,
            self.return_type.take(),
            self.body
                .take()
                .unwrap_or_else(|| panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "body")),
            self.attributes,
        )
    }
}
