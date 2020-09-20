//!
//! The array type parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::Token;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::parser::expression::Parser as ExpressionParser;
use crate::syntax::parser::r#type::Parser as TypeParser;
use crate::syntax::tree::r#type::builder::Builder as TypeBuilder;
use crate::syntax::tree::r#type::Type;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    BracketSquareLeft,
    /// The `[` has been parsed so far.
    Type,
    /// The `[ {type}` has been parsed so far.
    Semicolon,
    /// The `[ {type} ;` has been parsed so far.
    SizeExpression,
    /// The `[ {type} ; {expression}` has been parsed so far.
    BracketSquareRight,
}

impl Default for State {
    fn default() -> Self {
        Self::BracketSquareLeft
    }
}

///
/// The array type parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The token returned from a subparser.
    next: Option<Token>,
    /// The builder of the parsed value.
    builder: TypeBuilder,
}

impl Parser {
    ///
    /// Parses an array type literal.
    ///
    /// '[u8; 16]'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(Type, Option<Token>), Error> {
        loop {
            match self.state {
                State::BracketSquareLeft => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Type;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["["],
                                lexeme,
                                None,
                            )))
                        }
                    }
                }
                State::Type => {
                    let (array_type, next) = TypeParser::default().parse(stream.clone(), None)?;
                    self.next = next;
                    self.builder.set_array_type(array_type);
                    self.state = State::Semicolon;
                }
                State::Semicolon => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        } => {
                            self.state = State::SizeExpression;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec![";"],
                                lexeme,
                                None,
                            )))
                        }
                    }
                }
                State::SizeExpression => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.set_array_size_expression(expression);
                    self.state = State::BracketSquareRight;
                }
                State::BracketSquareRight => {
                    return match crate::syntax::parser::take_or_next(self.next.take(), stream)? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        } => Ok((self.builder.finish(), self.next.take())),
                        Token { lexeme, location } => {
                            Err(Error::Syntax(SyntaxError::expected_one_of_or_operator(
                                location,
                                vec!["]"],
                                lexeme,
                                None,
                            )))
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::error::Error;
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::lexeme::literal::integer::Integer as LexicalIntegerLiteral;
    use crate::lexical::token::lexeme::symbol::Symbol;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
    use crate::syntax::tree::r#type::variant::Variant as TypeVariant;
    use crate::syntax::tree::r#type::Type;

    #[test]
    fn ok() {
        let input = r#"[field; 8]"#;

        let expected = Ok((
            Type::new(
                Location::test(1, 1),
                TypeVariant::array(
                    Type::new(Location::test(1, 2), TypeVariant::field()),
                    ExpressionTree::new(
                        Location::test(1, 9),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::test(1, 9),
                                LexicalIntegerLiteral::new_decimal("8".to_owned()),
                            ),
                        )),
                    ),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_size_expression() {
        let input = r#"[field; 4 * 4]"#;

        let expected = Ok((
            Type::new(
                Location::test(1, 1),
                TypeVariant::array(
                    Type::new(Location::test(1, 2), TypeVariant::field()),
                    ExpressionTree::new_with_leaves(
                        Location::test(1, 11),
                        ExpressionTreeNode::operator(ExpressionOperator::Multiplication),
                        Some(ExpressionTree::new(
                            Location::test(1, 9),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(1, 9),
                                    LexicalIntegerLiteral::new_decimal("4".to_owned()),
                                ),
                            )),
                        )),
                        Some(ExpressionTree::new(
                            Location::test(1, 13),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(1, 13),
                                    LexicalIntegerLiteral::new_decimal("4".to_owned()),
                                ),
                            )),
                        )),
                    ),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_nested() {
        let input = r#"[[field; 8]; 8]"#;

        let expected = Ok((
            Type::new(
                Location::test(1, 1),
                TypeVariant::array(
                    Type::new(
                        Location::test(1, 2),
                        TypeVariant::array(
                            Type::new(Location::test(1, 3), TypeVariant::field()),
                            ExpressionTree::new(
                                Location::test(1, 10),
                                ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                    IntegerLiteral::new(
                                        Location::test(1, 10),
                                        LexicalIntegerLiteral::new_decimal("8".to_owned()),
                                    ),
                                )),
                            ),
                        ),
                    ),
                    ExpressionTree::new(
                        Location::test(1, 14),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::test(1, 14),
                                LexicalIntegerLiteral::new_decimal("8".to_owned()),
                            ),
                        )),
                    ),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_semicolon() {
        let input = r#"[field, 8]"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::test(1, 7),
            vec![";"],
            Lexeme::Symbol(Symbol::Comma),
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_square_right() {
        let input = r#"[field; 8)"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_one_of_or_operator(
            Location::test(1, 10),
            vec!["]"],
            Lexeme::Symbol(Symbol::ParenthesisRight),
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
