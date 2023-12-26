use crate::synacorvm::virtual_machine::VirtualMachine;
use crate::synacorvm::operations::Result;

mod synacorvm;


fn main() -> Result<()> {
    let test_bin = include_bytes!("../spec/challenge.bin");
    let mut machine = VirtualMachine::default();
    machine.load_program_from_bytes(test_bin);
    machine.run()
}
