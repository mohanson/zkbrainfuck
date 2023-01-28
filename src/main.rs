use std::io::Read;
use zkbrainfuck::interpreter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() >= 2);
    let mut f = std::fs::File::open(&args[1])?;
    let mut c: Vec<u8> = Vec::new();
    f.read_to_end(&mut c)?;
    let mut i = interpreter::Interpreter::default();
    i.run(c)
}
