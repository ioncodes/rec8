mod core;

use core::translator::Translator;
use core::parser::Parser;

fn main() {
    let mut parser = Parser::new("roms/MAZE".to_string());
    let mut translator = Translator::new();
    let mut eof = false;
    while !eof {
        let (n1, n2, n3, n4) = parser.read(&mut eof);
        match (n1, n2, n3, n4) {
            (0x0A, _, _, _) => translator.mov_i_addr(n2, n3, n4),
            (0x0C, _, _, _) => translator.rand_bitwise_and(n2, n3, n4),
            (0x03, _, _, _) => translator.je(n2, n3, n4),
            (0x0D, _, _, _) => translator.draw()
            _ => panic!("Unknow instruction: {:X}{:X}{:X}{:X}", n1, n2, n3, n4),
        }
    }

    println!("{}", translator);
}
