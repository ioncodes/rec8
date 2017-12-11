use std::fmt;

/*
 * rax+0 = I
 * rax+8 = DT
 * rax+16 = ST
 * rax+24...152 = V0..VF => [rax+(24+(N*8))]
 */

pub struct Translator {
    pub contents: Vec<u8>,
    pub debug_symbols: Vec<(usize, usize, String)>,
}

impl Translator {
    pub fn new() -> Translator {
        Translator {
            contents: Vec::<u8>::new(),
            debug_symbols: Vec::<(usize, usize, String)>::new(),
        }
    }

    pub fn emit(&mut self, bytes: Vec<u8>) {
        self.contents.extend(bytes);
    }

    fn emit_debug(&mut self) {
        self.emit_debug_symbols(16, "DEBUG!".to_string());
        self.emit(vec![0xFF; 16]);
    }

    fn emit_debug_symbols(&mut self, length: usize, symbol: String) {
        self.debug_symbols.push(
            (self.contents.len(), length, symbol),
        );
    }

    pub fn mov_i_addr(&mut self, n1: u8, n2: u8, n3: u8) {
        let asm = vec![0x48, 0xC7, 0x00, (n1 << 4) | n2, n3, 0x00, 0x00]; // mov qword ptr [rax+0], NNN
        self.emit_debug_symbols(
            asm.len(),
            format!("mov qword ptr [rax+0], 0x{}{}{}", n1, n2, n3),
        );
        self.emit(asm);
    }

    pub fn rand_bitwise_and(&mut self, _n1: u8, _n2: u8, _n3: u8) {
        // todo 1: implement as rust
        // todo 2: implement as asm
        /*
        self.emit(vec![0x49, 0xC7, 0xC4, (n2 << 4) | n3, 0x00, 0x00, 0x00]); // mov r12, <8bit> (NN)
        self.emit(vec![0x49, 0xC7, 0xC5, 0xFF, 0x00, 0x00, 0x00]); // mov r13, <8bit> (random number)
        self.emit(vec![0x4D, 0x21, 0xE5]); // and r13, r12
        self.emit(vec![0x4C, 0x89, 0x68, 24 + (n1 * 8)]); // mov qword ptr [rax+(24+(N*8))], r13
        */
        self.emit_debug();
    }

    pub fn je(&mut self, n1: u8, n2: u8, n3: u8) {
        let asm = vec![0x48, 0x83, 0x78, 24 + (n1 * 8), (n2 << 4) | n3]; // cmp qword ptr [rax+(24+(X*8))], NN
        self.emit_debug_symbols(
            asm.len(),
            format!("cmp qword ptr [rax+(24+(0x{}*8))], 0x{}{}", n1, n2, n3),
        );
        self.emit(asm);
        self.emit_debug();
        // make new page
    }

    pub fn add(&mut self, n1: u8, n2: u8, n3: u8) {
        let asm = vec![0x48, 0x81, 0x40, n1, n2 << 4 | n3, 0x00, 0x00, 0x00]; // add qword ptr [rax+(24+(X*8))], NN
        self.emit_debug_symbols(
            asm.len(),
            format!("add qword ptr [rax+(24+(0x{}*8))], 0x{}{}", n1, n2, n3),
        );
        self.emit(asm);
    }

    pub fn jmp(&mut self, _n1: u8, _n2: u8, _n3: u8) {
        // new page
        self.emit_debug();
    }

    pub fn mov_v_addr(&mut self, n1: u8, n2: u8, n3: u8) {
        let asm = vec![0x48, 0xC7, 0x40, n1, n2 << 4 | n3, 0x00, 0x00, 0x00]; // mov qword ptr [rax+(24+(X*8))], NN
        self.emit_debug_symbols(
            asm.len(),
            format!("mov qword ptr [rax+(24+(0x{}*8))], 0x{}{}", n1, n2, n3),
        );
        self.emit(asm);
    }

    pub fn draw(&mut self) {
        // draw
        self.emit_debug();
    }
}


impl fmt::Display for Translator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut w = "".to_string();
        let mut i: usize = 0;
        let mut j: usize = 0;
        for byte in &self.contents {
            w.push_str(&format!("{:02X} ", byte));
            let &(ref position, ref length, ref symbol) = &self.debug_symbols[j];
            if i == *length {
                w.push_str(&format!("\t => {}\n", symbol));
                i = 0;
                j += 1;
            }
            i += 1;
        }
        let len = w.len() - 1;
        w.truncate(len);
        // w.push_str(" ]");
        write!(f, "{}", w)
    }
}
