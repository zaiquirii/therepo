use crate::synacorvm::operations::UnknownOpcode;
use crate::synacorvm::virtual_machine::VirtualMachine;

mod synacorvm;


fn main() -> Result<(), UnknownOpcode> {
    let test_bin = include_bytes!("../spec/challenge.bin");
    let mut machine = VirtualMachine::default();
    machine.load_program_from_bytes(test_bin);
    machine.run()
}
