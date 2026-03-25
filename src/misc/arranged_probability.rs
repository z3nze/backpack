pub fn sol() {
    let mut prev: i128 = 27304197;
    let start = 159140520;
    let finish = 1e13 as i128;
    let mut c = start;
    while c < finish {
        let d = c * c + (c - 1) * (c - 1);
        let rd = d.isqrt();
        if rd * rd == d {
            let blue = (rd + 1) / 2;
            println!("{blue} {c} {}", (c as f64) / (prev as f64));
            prev = c;
            c = ((c as f64) * 5.828427) as i128;
        }
        c += 1;
    }
}
