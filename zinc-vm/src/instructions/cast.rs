extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::{Cell, VirtualMachine, VMInstruction};
use crate::core::{RuntimeError};

use crate::{gadgets};
use zinc_bytecode::instructions::Cast;

impl<VM: VirtualMachine> VMInstruction<VM> for Cast {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let old_value = vm.pop()?.value()?;

        let condition = vm.condition_top()?;
        let cs = vm.constraint_system();
        let new_value = gadgets::conditional_type_check(
            cs.namespace(|| "type check"),
            &condition,
            &old_value,
            self.scalar_type,
        )?;

        vm.push(Cell::Value(new_value))
    }
}

#[cfg(test)]
mod test {
    use crate::instructions::testing_utils::TestingError;

    #[test]
    fn test_cast() -> Result<(), TestingError> {
        Ok(())
    }
}
