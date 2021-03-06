//!
//! The `BitwiseNot` instruction.
//!

use franklin_crypto::bellman::ConstraintSystem;

use zinc_build::BitwiseNot;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for BitwiseNot {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        let scalar = vm.pop()?.try_into_value()?;

        let cs = vm.constraint_system();

        let result = gadgets::bitwise::not::bit_not(cs.namespace(|| "bit_not"), &scalar)?;

        vm.push(result.into())
    }
}
