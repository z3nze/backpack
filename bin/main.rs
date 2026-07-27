use backpack::{io::console::Scanner, misc::special_subset_sums::is_special_sum_set};

fn main() {
    let mut sc = Scanner::default();
    let mut res: usize = 0;
    for _ in 0..100 {
        let set_string: String = sc.read();
        let candidate_set = set_string
            .trim()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        if is_special_sum_set(&candidate_set) {
            res += candidate_set.iter().sum::<usize>();
        }
    }
    println!("{}", res);
}
