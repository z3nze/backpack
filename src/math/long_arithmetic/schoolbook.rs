use crate::math::long_arithmetic::types::Sign;
use std::cmp::Ordering;
use std::iter::repeat;

pub(super) fn add_signed_small_to_large_bu64(
    lhs: &Vec<u64>,
    rhs: &Vec<u64>,
    sign: Sign,
) -> Vec<u64> {
    let base: i128 = 1i128 << 64;
    let rsign = sign as i128;
    let (n, m) = (lhs.len(), rhs.len());
    let max_len = n.max(m) + 1;

    let mut res: Vec<i128> = vec![0; max_len];
    let mut carry: i128 = 0;

    lhs.iter()
        .chain(repeat(&0))
        .take(max_len)
        .zip(rhs.iter().chain(repeat(&0)).take(max_len))
        .map(|(&ai, &bi)| (ai as i128, bi as i128))
        .zip(res.iter_mut())
        .for_each(|((ai, bi), ref mut block_i)| {
            let res = **block_i + ai + rsign * bi + carry;
            carry = res.div_euclid(base);
            **block_i = res.rem_euclid(base);
        });

    while let Some(last) = res.last()
        && *last == 0
    {
        res.pop();
    }

    res.iter().map(|&x| x as u64).collect::<Vec<_>>()
}

pub(super) fn multiply_bu64(lhs: &Vec<u64>, rhs: &Vec<u64>) -> Vec<u64> {
    let (n, m) = (lhs.len(), rhs.len());

    let mut blocks: Vec<u64> = vec![0; n * m + 1];

    lhs.iter().enumerate().for_each(|(i, &ai)| {
        rhs.iter().enumerate().for_each(|(j, &bj)| {
            let uv = blocks[i * j] as u128 + (ai as u128) * (bj as u128);
            let carry = uv >> 64;
            let val = uv as u64;
            blocks[i * j + 1] += carry as u64;
            blocks[i * j] = val;
        })
    });

    while let Some(last) = blocks.last()
        && *last == 0
    {
        blocks.pop();
    }
    blocks
}

pub(super) fn cmp(lhs: &Vec<u64>, rhs: &Vec<u64>) -> Ordering {
    let (n, m) = (lhs.len(), rhs.len());

    if n != m {
        return n.cmp(&m);
    }

    if let Some(neq) = lhs.iter().zip(rhs.iter()).find(|(x, y)| x != y) {
        return neq.0.cmp(neq.1);
    }

    Ordering::Equal
}

fn divide_by_single_digit_bu64(lhs: &Vec<u64>, rhs: u64) -> (Vec<u64>, u64) {
    let base: u128 = 1u128 << 64;
    let a = lhs.iter().map(|&x| x as u128).collect::<Vec<_>>();
    let d = rhs as u128;
    let n = a.len();

    let mut q = vec![0; n];
    let mut carry = 0;

    for i in (0..n - 1).rev() {
        let t = carry * base + a[i];
        q[i] = (t / d) as u64;
        carry = t % d;
    }

    (q, carry as u64)
}

pub(super) fn divide_bu64(lhs: &Vec<u64>, rhs: &Vec<u64>) -> (Vec<u64>, Vec<u64>) {
    let base: u128 = 1u128 << 64;
    let _a = lhs.iter().map(|&x| x as u128).collect::<Vec<_>>();
    let d = rhs.iter().map(|&x| x as u128).collect::<Vec<_>>();
    let (n, m) = (lhs.len(), rhs.len());

    let f = base.div_ceil(d[m - 1]);

    let a_prime = multiply_bu64(lhs, &vec![f as u64]);
    let d_prime = multiply_bu64(rhs, &vec![f as u64]);

    let mut rem = a_prime;
    let mut q = vec![0; n - m + 1];

    for j in (0..=n - m).rev() {
        let base_j = [vec![0; j], vec![1]].concat();
        let (lf, rg) = (j, (j + m + 1).min(n));
        let u: Vec<u128> = rem[lf..rg]
            .iter()
            .chain(repeat(&0).take(m + 1 - (rg - lf + 1)))
            .map(|&x| x as u128)
            .collect::<Vec<_>>();

        let mut q_hat = ((u[m] * base + u[m - 1]) / d[m - 1]).min(base - 1);
        while q_hat * (d[m - 1] * base + d[m - 2]) > (u[m] * base + u[m - 1]) * base + u[m - 2] {
            q_hat -= 1;
        }

        let mut scaled_divisor =
            multiply_bu64(&multiply_bu64(&vec![q_hat as u64], &d_prime), &base_j);

        if cmp(&rem, &scaled_divisor) == Ordering::Less {
            q_hat -= 1;
            scaled_divisor = multiply_bu64(&multiply_bu64(&vec![q_hat as u64], &d_prime), &base_j)
        }

        rem = add_signed_small_to_large_bu64(&rem, &scaled_divisor, Sign::NEGATIVE);

        q[j] = q_hat;
    }

    let rdiv = q.iter().map(|&x| x as u64).collect::<Vec<_>>();
    let rrem =
        divide_by_single_digit_bu64(&rem.iter().map(|&x| x as u64).collect::<Vec<_>>(), f as u64).0;

    (rdiv, rrem)
}
