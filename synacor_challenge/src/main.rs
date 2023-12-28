use std::collections::HashMap;
use crate::synacorvm::virtual_machine::VirtualMachine;
use crate::synacorvm::operations::Result;

mod synacorvm;


fn main() -> Result<()> {
    let test_bin = include_bytes!("../spec/challenge.bin");
    let mut machine = VirtualMachine::default();
    machine.load_program_from_bytes(test_bin);
    machine.run()

    // for i in 0..=32768 {
    //     if i % 1000 == 0 {
    //         println!("i {}", i);
    //     }
    //     if heavy_func(i) == 6 {
    //         println!("IT IS {}", i);
    //         break;
    //     }
    // }
}

fn heavy_func(r7: u16) -> u16 {
    let mut memo = HashMap::new();
    f6049(4, 1, r7, &mut memo)
}

fn f6049(r0: u16, r1: u16, r7: u16, memo: &mut HashMap<(u16, u16), u16>) -> u16 {
    if let Some(r) = memo.get(&(r0, r1)) {
        return *r;
    }

    let result = if r0 == 0 {
        add(r1, 1)
    } else if r1 == 0 {
        f6049(add(r0, 32767), r7, r7, memo)
    } else {
        f6049(
            add(r0, 32767),
            f6049(r0, add(r1, 32767), r7, memo),
            r7,
            memo)
    };
    memo.insert((r0, r1), result);
    result
}

fn add(a: u16, b: u16) -> u16 {
    (a + b) % 32768
}
