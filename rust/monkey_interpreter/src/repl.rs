use std::io::{BufRead, Write};
use crate::lexer::{Lexer, RawMonkeyProgram};

pub fn start(mut input: impl BufRead, mut output: impl Write) -> std::io::Result<()> {
    let mut s = String::new();
    output.write(">> ".as_bytes())?;
    output.flush()?;
    while let Ok(bytes_read) = input.read_line(&mut s) {
        println!("We got some bytes {}", bytes_read);
        if bytes_read == 0 {
            break;
        }

        let p = RawMonkeyProgram::from(s.as_str());
        let mut l = Lexer::new(&p);
        while let Some(t) = l.next_token() {
            output.write(format!("{:?}\n", t).as_bytes())?;
        }
        s.clear();
        output.write(">> ".as_bytes())?;
        output.flush()?;
    }
    Ok(())
}