pub struct Translator {
    pub contents: Vec<u8>,
}

impl Translator {
    pub fn new() -> Translator {
        Translator { contents: Vec::<u8>::new() }
    }

    pub fn emit(&mut self, bytes: Vec<u8>) {
        self.contents.extend(bytes);
    }
}
