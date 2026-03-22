use backpack::misc::anagramic_squares::AnagramicSquares;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("fail");

    let words: Vec<String> = buf
        .split(',')
        .map(|x| x.strip_prefix("\"").unwrap().strip_suffix("\"").unwrap().to_string())
        .collect();

    dbg!(&words);

    let sol = AnagramicSquares::new(words);
    sol.solve();
}
