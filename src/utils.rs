pub trait StringLine {
    fn push_str_line(&mut self, string: &str);
}

impl StringLine for String {
    fn push_str_line(&mut self, string: &str) {
        self.push_str(string);
        self.push_str("\n");
    }
}