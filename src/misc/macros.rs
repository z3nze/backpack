#[macro_export]
macro_rules! read {
    () => {{
        static mut BUF: Vec<u8> = vec![];
        static mut TOKENS: Vec<String> = vec![];
        static mut IDX: usize = 0;
        unsafe {
            if IDX == TOKENS.len() {
                BUF.clear();
                std::io::stdin().lock().read_to_end(&mut BUF).unwrap();
                TOKENS = String::from_utf8_unchecked(BUF)
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                IDX = 0;
            }
            let res = TOKENS[IDX].parse().unwrap();
            IDX += 1;
            res
        }
    }};
}
