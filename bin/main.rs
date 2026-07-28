use backpack::misc::special_subset_sums::find_special_sum_set;

fn main() {
    // let vs = vec![20, 31, 38, 39, 40, 42, 45];
    for n in 1..=7 {
        let sss = find_special_sum_set(n);
        println!(
            "{}",
            sss.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
    }
}
