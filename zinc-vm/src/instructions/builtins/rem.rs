extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::gadgets::ScalarType;
use crate::Engine;
use zinc_bytecode::instructions::Rem;

impl<E, CS> VMInstruction<E, CS> for Rem
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let condition = vm.condition_top()?;

        let (_div, unchecked_rem) =
            vm.operations()
                .div_rem_conditional(left.clone(), right.clone(), condition)?;

        let condition = vm.condition_top()?;
        let rem = vm.operations().assert_type(
            condition,
            unchecked_rem,
            ScalarType::expect_same(left.get_type(), right.get_type())?,
        )?;

        vm.push(Cell::Value(rem))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_rem() -> Result<(), TestingError> {
        let _ = env_logger::builder().is_test(true).try_init();

        VMTestRunner::new()
            .add(PushConst::new(9.into(), true, 8))
            .add(PushConst::new(4.into(), true, 8))
            .add(Rem)
            .add(PushConst::new(9.into(), true, 8))
            .add(PushConst::new((-4).into(), true, 8))
            .add(Rem)
            .add(PushConst::new((-9).into(), true, 8))
            .add(PushConst::new(4.into(), true, 8))
            .add(Rem)
            .add(PushConst::new((-9).into(), true, 8))
            .add(PushConst::new((-4).into(), true, 8))
            .add(Rem)
            .test(&[3, 3, 1, 1])
    }
}
