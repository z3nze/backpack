use std::fmt::Debug;
use std::io::Read;
use std::str::FromStr;

#[derive(Default)]
pub struct Scanner {
    tokens: Vec<String>,
    idx: usize,
}

impl Scanner {
    pub fn read<T>(&mut self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        if self.idx == self.tokens.len() {
            let mut buf: Vec<u8> = vec![];
            buf.clear();
            std::io::stdin().lock().read_to_end(&mut buf).unwrap();
            self.tokens = String::from_utf8(buf)
                .unwrap()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            self.idx = 0;
        }
        let res = self.tokens[self.idx].parse().unwrap();
        self.idx += 1;
        res
    }
}
