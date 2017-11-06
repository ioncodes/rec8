use std::fmt;

/*
 * rax+0 = I
 * rax+8 = DT
 * rax+16 = ST
 * rax+24...152 = V0..VF => [rax+(24+(N*8))]
 */

pub struct Translator {
    pub contents: Vec<u8>,
    skip_next: bool,
}

impl Translator {
    pub fn new() -> Translator {
        Translator { contents: Vec::<u8>::new(), skip_next: false }
    }

    pub fn emit(&mut self, bytes: Vec<u8>) {
        self.contents.extend(bytes);
    }

    pub fn mov_i_addr(&mut self, n1: u8, n2: u8, n3: u8) {
        self.emit(vec![0x48, 0xC7, 0x00, (n1 << 4) | n2, n3, 0x00, 0x00]); // mov qword ptr [rax+0], NNN
    }

    pub fn rand_bitwise_and(&mut self, n1: u8, n2: u8, n3: u8) {
        // END, maybe use rdrand via asm?
        self.emit(vec![0x49, 0xC7, 0xC4, (n2 << 4) | n3, 0x00, 0x00, 0x00]); // mov r12, <8bit> (NN)
        self.emit(vec![0x49, 0xC7, 0xC5, 0xFF, 0x00, 0x00, 0x00]); // mov r13, <8bit> (random number)
        self.emit(vec![0x4D, 0x21, 0xE5]); // and r13, r12
        self.emit(vec![0x4C, 0x89, 0x68, 24 + (n1 * 8)]); // mov qword ptr [rax+(24+(N*8))], r13
    }

    pub fn je(&mut self, n1: u8, n2: u8, n3: u8) {
        self.emit(vec![0x48, 0x83, 0x78, 24 + (n1 * 8), (n2 << 4) | n3]); // cmp qword ptr [rax+(24+(N*8))], NN
        // END, JE
        self.skip_next = true;
    }

    pub fn je(&mut self, n1: u8, n2: u8, n3: u8) {
        self.emit(vec![0x48, 0x83, 0x78, 24 + (n1 * 8), (n2 << 4) | n3]); // cmp qword ptr [rax+(24+(N*8))], NN
        // END, JE
        self.skip_next = true;
    }
}


impl fmt::Display for Translator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut w = "[ ".to_string();
        for byte in &self.contents {
            w.push_str(&format!("{:02X}, ", byte));
        }
        let len = w.len() - 2;
        w.truncate(len);
        w.push_str(" ]");
        write!(f, "{}", w)
    }
}
