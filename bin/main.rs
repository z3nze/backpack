use backpack::{io::console::Scanner, misc::special_subset_sums::special_sum_sets};

fn main() {
    let mut cin = Scanner::default();
    let n: usize = cin.read();
    println!("{:?}", special_sum_sets(n));
}
