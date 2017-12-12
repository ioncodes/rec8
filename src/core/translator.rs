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
        // todo: implement as asm
        self.emit_debug();
    }

    pub fn je(&mut self, n1: u8, n2: u8, n3: u8) {
        let asm = vec![0x48, 0x83, 0x78, 24 + (n1 * 8), (n2 << 4) | n3]; // cmp qword ptr [rax+(24+(X*8))], NN
        self.emit_debug_symbols(
            asm.len(),
            format!("cmp qword ptr [rax+(24+(0x{:02x}*8))], 0x{}{}", n1, n2, n3),
        );
        self.emit(asm);
        self.emit_debug();
        // make new page
    }

    pub fn add(&mut self, n1: u8, n2: u8, n3: u8) {
        let asm = vec![
            0x48,
            0x81,
            0x40,
            24 + (n1 * 8),
            (n2 << 4) | n3,
            0x00,
            0x00,
            0x00,
        ]; // add qword ptr [rax+(24+(X*8))], NN
        self.emit_debug_symbols(
            asm.len(),
            format!("add qword ptr [rax+(24+(0x{:02x}*8))], 0x{}{}", n1, n2, n3),
        );
        self.emit(asm);
    }

    pub fn jmp(&mut self, _n1: u8, _n2: u8, _n3: u8) {
        // new page
        self.emit_debug();
    }

    pub fn mov_v_addr(&mut self, n1: u8, n2: u8, n3: u8) {
        let asm = vec![
            0x48,
            0xC7,
            0x40,
            24 + (n1 * 8),
            (n2 << 4) | n3,
            0x00,
            0x00,
            0x00,
        ]; // mov qword ptr [rax+(24+(X*8))], NN
        self.emit_debug_symbols(
            asm.len(),
            format!("mov qword ptr [rax+(24+(0x{:02x}*8))], 0x{}{}", n1, n2, n3),
        );
        self.emit(asm);
    }

    pub fn mov_v_v(&mut self, n1: u8, n2: u8) {
        let asm_0 = vec![0x4C, 0x8B, 0x60, 24 + (n2 * 8)]; // mov r12, qword ptr [rax+(24+(X*8))]
        let asm_1 = vec![0x4C, 0x89, 0x60, 24 + (n1 * 8)]; // mov qword ptr [rax+(24+(Y*8))], r12
        self.emit_debug_symbols(
            asm_0.len(),
            format!("mov r12, qword ptr [rax+(24+(0x{:02x}*8))]", n2),
        );
        self.emit(asm_0);
        self.emit_debug_symbols(
            asm_1.len(),
            format!("mov qword ptr [rax+(24+(0x{:02x}*8))], r12", n1),
        );
        self.emit(asm_1);
    }

    pub fn call(&mut self, _n1: u8, _n2: u8, _n3: u8) {
        // call / jump
        self.emit_debug();
    }

    pub fn draw(&mut self) {
        // draw
        self.emit_debug();
    }
}


impl fmt::Display for Translator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut w = "".to_string();
        let mut i: usize = 1;
        let mut j: usize = 0;
        let mut space_len: usize = 0;
        for &(_, ref length, _) in &self.debug_symbols {
            if space_len < *length {
                space_len = *length;
            }
        }
        space_len *= 2;
        space_len += 10; // margin
        for byte in &self.contents {
            w.push_str(&format!("{:02X}", byte));
            let &(_, ref length, ref symbol) = &self.debug_symbols[j];
            if i == *length {
                for _ in 0..space_len - (*length * 2) {
                    w.push(' ');
                }
                w.push_str(&format!(" => {}\n", symbol));
                i = 0;
                j += 1;
            }
            i += 1;
        }
        write!(f, "{}", w)
    }
}
