mod core;

use core::translator::Translator;
use core::parser::Parser;

fn main() {
    let mut parser = Parser::new("roms/MAZE".to_string());
    let mut translator = Translator::new();
    let (n1, n2, n3, n4) = parser.read();
    match (n1, n2, n3, n4) {
        (0x0A, _, _, _) => {
            translator.emit(vec![
                0x48,
                0xC7,
                0x04,
                0x25,
                0xFF,
                0xFF,
                0xFF,
                0x0F,
                (n2 << 4) | n3,
                n4,
                0x00,
                0x00,
            ]) // mov qword ptr [0xffffffff], 5
        }
        _ => panic!("Unknow instruction: {:X}{:X}{:X}{:X}", n1, n2, n3, n4),
    }

    println!("{:?}", translator.contents);
}
