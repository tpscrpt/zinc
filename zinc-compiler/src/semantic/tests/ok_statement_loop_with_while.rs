//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::Add;
use zinc_bytecode::Call;
use zinc_bytecode::EndIf;
use zinc_bytecode::Exit;
use zinc_bytecode::If;
use zinc_bytecode::Instruction;
use zinc_bytecode::LoadPush;
use zinc_bytecode::LoopBegin;
use zinc_bytecode::LoopEnd;
use zinc_bytecode::Lt;
use zinc_bytecode::Not;
use zinc_bytecode::PopStore;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;

use crate::semantic::BinaryAnalyzer;
use crate::syntax::Parser;

#[test]
fn test() {
    let input = r#"
fn main() {
    let mut sum = 0;
    for i in 0..=10 while i < 5 {
        sum = sum + i;
    }
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(0), false, 8)),
        Instruction::PopStore(PopStore::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(0), false, 8)),
        Instruction::PopStore(PopStore::new(1)),
        Instruction::PushConst(PushConst::new(BigInt::from(1), false, 8)),
        Instruction::PopStore(PopStore::new(2)),
        Instruction::LoopBegin(LoopBegin::new(11)),
        Instruction::PushConst(PushConst::new(BigInt::from(5), false, 8)),
        Instruction::LoadPush(LoadPush::new(1)),
        Instruction::Lt(Lt),
        Instruction::Not(Not),
        Instruction::If(If),
        Instruction::PushConst(PushConst::new(BigInt::from(0), false, 8)),
        Instruction::PopStore(PopStore::new(2)),
        Instruction::EndIf(EndIf),
        Instruction::LoadPush(LoadPush::new(2)),
        Instruction::If(If),
        Instruction::LoadPush(LoadPush::new(1)),
        Instruction::LoadPush(LoadPush::new(0)),
        Instruction::Add(Add),
        Instruction::PopStore(PopStore::new(0)),
        Instruction::EndIf(EndIf),
        Instruction::PushConst(PushConst::new(BigInt::from(1), false, 8)),
        Instruction::LoadPush(LoadPush::new(1)),
        Instruction::Add(Add),
        Instruction::PopStore(PopStore::new(1)),
        Instruction::LoopEnd(LoopEnd),
        Instruction::Return(Return::new(0)),
    ]);

    let result = BinaryAnalyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect(super::PANIC_SYNTAX_ERROR),
    );

    assert_eq!(expected, result);
}
