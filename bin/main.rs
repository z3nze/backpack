use backpack::misc::sudoku::SudokuGame;

fn main() {
    let mut ans: u32 = 0;
    for _ in 0..50 {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("failed");

        let mut f: Vec<String> = vec![String::new(); 9];
        for row in f.iter_mut() {
            std::io::stdin().read_line(row).expect("failed");
        }

        let mut game = SudokuGame::from_string_vec(f);

        let res = game.solve().unwrap();

        ans += res[0].iter().take(3).fold(0, |acc, &x| acc * 10 + x as u32);
    }

    println!("{ans}");
}
