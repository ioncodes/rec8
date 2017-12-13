use std::fmt;

/*
 * rax+0 = I
 * rax+8 = DT
 * rax+16 = ST
 * rax+24...152 = V0..VF => [rax+(24+(N*8))]
 * rax+160... = MEMORY
 * rbx = memory page base address
 */

/*
 * Relative jump in Keystone:
 * je +(2+BYTES)
 */

pub struct Translator {
    pub contents: Vec<u8>,
    pub debug_symbols: Vec<(usize, usize, String)>,
    pub create_jump: bool,
    pub instruction_list: Vec<usize>,
}

impl Translator {
    pub fn new() -> Translator {
        Translator {
            contents: Vec::<u8>::new(),
            debug_symbols: Vec::<(usize, usize, String)>::new(),
            create_jump: false,
            instruction_list: Vec::<usize>::new(),
        }
    }

    fn emit(&mut self, bytes: Vec<u8>) {
        self.contents.extend(bytes);
    }

    fn emit_debug(&mut self, asm: String) {
        self.process_jump(16);
        self.emit_debug_symbols(16, asm);
        self.emit(vec![0xFF; 16]);
        self.cache_size(16);
    }

    fn emit_debug_symbols(&mut self, length: usize, symbol: String) {
        self.debug_symbols.push(
            (self.contents.len(), length, symbol),
        );
    }

    fn process_jump(&mut self, length: usize) {
        if self.create_jump {
            let asm = vec![0x74, (0x02 + length) as u8]; // je 2+BYTES
            self.emit_debug_symbols(asm.len(), format!("je 0x02+0x{:02x}", length));
            self.emit(asm);
            self.create_jump = false;
            self.fix_jump_cache();
        }
    }

    fn parse_addr(&self, n1: u8, n2: u8, n3: u8) -> u16 {
        (((n1 as u16) << 8) | ((n2 << 4) | n3) as u16) as u16
    }

    fn get_byte(&self, bytes: u16, position: usize) -> u8 {
        ((bytes >> 8 * position) & 0xFF) as u8
    }

    fn cache_size(&mut self, size: usize) {
        self.instruction_list.push(size);
    }

    fn fix_jump_cache(&mut self) {
        *self.instruction_list.last_mut().unwrap() += 2;
    }

    fn get_x64_address(&self, address: u16) -> u16 {
        let amount_instructions = ((address - 0x200) / 2) as usize;
        let mut x64_addr = 0;
        for i in 0..amount_instructions {
            x64_addr += self.instruction_list[i] as u16;
        }
        x64_addr
    }

    pub fn mov_i_addr(&mut self, n1: u8, n2: u8, n3: u8) {
        let asm = vec![0x48, 0xC7, 0x00, (n1 << 4) | n2, n3, 0x00, 0x00]; // mov qword ptr [rax+0], NNN
        let len = asm.len();
        self.process_jump(asm.len());
        self.emit_debug_symbols(len, format!("mov qword ptr [rax+0], 0x{}{}{}", n1, n2, n3));
        self.emit(asm);
        self.cache_size(len);
    }

    pub fn rand_bitwise_and(&mut self, _n1: u8, _n2: u8, _n3: u8) {
        // todo: implement as asm
        self.emit_debug("RND".to_string());
    }

    pub fn je(&mut self, n1: u8, n2: u8, n3: u8) {
        let asm = vec![0x48, 0x83, 0x78, 24 + (n1 * 8), (n2 << 4) | n3]; // cmp qword ptr [rax+(24+(X*8))], NN
        let len = asm.len();
        self.emit_debug_symbols(
            len,
            format!("cmp qword ptr [rax+(24+(0x{:02x}*8))], 0x{}{}", n1, n2, n3),
        );
        self.emit(asm);
        self.create_jump = true;
        self.cache_size(len);
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
        let len = asm.len();
        self.process_jump(asm.len());
        self.emit_debug_symbols(
            len,
            format!("add qword ptr [rax+(24+(0x{:02x}*8))], 0x{}{}", n1, n2, n3),
        );
        self.emit(asm);
        self.cache_size(len);
    }

    pub fn jmp(&mut self, n1: u8, n2: u8, n3: u8) {
        let address = self.parse_addr(n1, n2, n3);
        let addr = self.get_x64_address(address);
        let asm_0 = vec![0x67, 0x4C, 0x8B, 0x23]; // mov r12, qword ptr [ebx]
        let asm_1 = vec![
            0x49,
            0x81,
            0xC4,
            self.get_byte(addr, 0),
            self.get_byte(addr, 1),
            0x00,
            0x00,
        ]; // add r12, NNN
        let asm_2 = vec![0x41, 0xFF, 0xE4]; // jmp r12
        let len = asm_0.len() + asm_1.len() + asm_2.len();
        self.process_jump(asm_0.len() + asm_1.len() + asm_2.len());
        self.emit_debug_symbols(asm_0.len(), "mov r12, qword ptr [ebx]".to_string());
        self.emit(asm_0);
        self.emit_debug_symbols(asm_1.len(), format!("add r12, 0x{:04x}", addr));
        self.emit(asm_1);
        self.emit_debug_symbols(asm_2.len(), "jmp r12".to_string());
        self.emit(asm_2);
        self.cache_size(len);
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
        let len = asm.len();
        self.process_jump(len);
        self.emit_debug_symbols(
            len,
            format!("mov qword ptr [rax+(24+(0x{:02x}*8))], 0x{}{}", n1, n2, n3),
        );
        self.emit(asm);
        self.cache_size(len);
    }

    pub fn mov_v_v(&mut self, n1: u8, n2: u8) {
        let asm_0 = vec![0x4C, 0x8B, 0x60, 24 + (n2 * 8)]; // mov r12, qword ptr [rax+(24+(X*8))]
        let asm_1 = vec![0x4C, 0x89, 0x60, 24 + (n1 * 8)]; // mov qword ptr [rax+(24+(Y*8))], r12
        let len = asm_0.len() + asm_1.len();
        self.process_jump(len);
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
        self.cache_size(len);
    }

    pub fn call(&mut self, n1: u8, n2: u8, n3: u8) {
        let address = self.parse_addr(n1, n2, n3);
        if address < 0x200 {
            println!(
                "Call to weird location found: 0x{:04x}. Ignoring...",
                address
            );
            return;
        }
        let addr = self.get_x64_address(address);
        let asm_0 = vec![0x68, 0x00, 0x00, 0x00, 0x00]; // push .
        let asm_1 = vec![0x67, 0x4C, 0x8B, 0x23]; // mov r12, qword ptr [ebx]
        let asm_2 = vec![
            0x49,
            0x81,
            0xC4,
            self.get_byte(addr, 0),
            self.get_byte(addr, 1),
            0x00,
            0x00,
        ]; // add r12, NNN
        let asm_3 = vec![0x41, 0xFF, 0xE4]; // jmp r12
        let len = asm_0.len() + asm_1.len() + asm_2.len() + asm_3.len();
        self.process_jump(len);
        self.emit_debug_symbols(asm_0.len(), "push .".to_string());
        self.emit(asm_0);
        self.emit_debug_symbols(asm_1.len(), "mov r12, qword ptr [ebx]".to_string());
        self.emit(asm_1);
        self.emit_debug_symbols(asm_2.len(), format!("add r12, 0x{:04x}", addr));
        self.emit(asm_2);
        self.emit_debug_symbols(asm_3.len(), "jmp r12".to_string());
        self.emit(asm_3);
        self.cache_size(len);
    }

    pub fn draw(&mut self) {
        // draw
        self.emit_debug("DRW".to_string());
    }
}


impl fmt::Display for Translator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut w = "0x0000: ".to_string();
        let mut i: usize = 1;
        let mut j: usize = 0;
        let mut k: usize = 0;
        let mut space_len: usize = 0;
        for &(_, ref length, _) in &self.debug_symbols {
            if space_len < *length {
                space_len = *length;
            }
        }
        space_len *= 2;
        space_len += 8;
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
                k += *length;
                w.push_str(&format!("0x{:04x}: ", k));
            }
            i += 1;
        }
        let len = w.len() - 8;
        w.truncate(len);
        write!(f, "{}", w)
    }
}
