use std::fs::File;
use std::io::Read;

pub struct Parser {
    contents: Vec<u8>,
    pc: usize,
}

impl Parser {
    pub fn new(file: String) -> Parser {
        let mut contents = Vec::<u8>::new();
        let mut f = File::open(&file).unwrap();
        let _ = f.read_to_end(&mut contents);
        Parser { contents, pc: 0 }
    }

    pub fn read(&mut self, eof: &mut bool) -> (u8, u8, u8, u8) {
        let data = (
            (self.contents[self.pc] & 0xF0) >> 4,
            self.contents[self.pc] & 0x0F,
            (self.contents[self.pc + 1] & 0xF0) >> 4,
            self.contents[self.pc + 1] & 0x0F,
        );
        self.pc += 2;
        *eof = self.pc == self.contents.len();
        data
    }
}
