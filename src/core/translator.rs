use std::fmt;

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
