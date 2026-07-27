use backpack::misc::special_subset_sums::find_special_sum_set;

fn main() {
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
