use crate::math::long_arithmetic::types::Sign;
use std::cmp::Ordering;
use std::iter::repeat;

const UNSIGNED_BASE_64: u128 = 1u128 << 64;
const SIGNED_BASE_64: i128 = 1i128 << 64;

pub(super) fn add_signed_small_to_large_bu64(
    lhs: &Vec<u64>,
    rhs: &Vec<u64>,
    sign: Sign,
) -> Vec<u64> {
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
            carry = res.div_euclid(SIGNED_BASE_64);
            **block_i = res.rem_euclid(SIGNED_BASE_64);
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

    let mut blocks: Vec<u128> = vec![0; n + m + 1];

    lhs.iter().enumerate().for_each(|(i, &ai)| {
        rhs.iter().enumerate().for_each(|(j, &bj)| {
            let uv = blocks[i + j] + (ai as u128) * (bj as u128);
            let (carry, val) = (uv / UNSIGNED_BASE_64, uv % UNSIGNED_BASE_64);
            blocks[i + j + 1] += carry;
            blocks[i + j] = val;
        })
    });

    while let Some(last) = blocks.last()
        && *last == 0
    {
        blocks.pop();
    }

    blocks.iter().map(|&x| x as u64).collect::<Vec<_>>()
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
    let a = lhs.iter().map(|&x| x as u128).collect::<Vec<_>>();
    let d = rhs as u128;
    let n = a.len();

    let mut q = vec![0; n];
    let mut carry = 0;

    for i in (0..n).rev() {
        let t = carry * UNSIGNED_BASE_64 + a[i];
        q[i] = (t / d) as u64;
        carry = t % d;
    }

    while let Some(last) = q.last()
        && *last == 0
    {
        q.pop();
    }

    (q, carry as u64)
}

pub(super) fn divide_by_multidigit_bu64(lhs: &Vec<u64>, rhs: &Vec<u64>) -> (Vec<u64>, Vec<u64>) {
    let _a = lhs.iter().map(|&x| x as u128).collect::<Vec<_>>();
    let d = rhs.iter().map(|&x| x as u128).collect::<Vec<_>>();
    let (n, m) = (lhs.len(), rhs.len());

    let f = UNSIGNED_BASE_64 / (d[m - 1] + 1);

    let a_prime = multiply_bu64(lhs, &vec![f as u64]);
    let d_prime = multiply_bu64(rhs, &vec![f as u64]);

    let mut rem = a_prime;
    let mut q = vec![0; n - m + 1];

    for j in (0..=n - m).rev() {
        let base_j = [vec![0; j], vec![1]].concat();
        let (lf, rg) = (j, j + m + 1);
        let u = &rem[lf..rg];
        let u_m = u[m] as u128;
        let u_m_1 = u[m - 1] as u128;
        let d_prime_m_1 = d_prime[m - 1] as u128;

        let mut q_hat =
            ((u_m * UNSIGNED_BASE_64 + u_m_1) / d_prime_m_1).min(UNSIGNED_BASE_64 - 1) as u64;
        let d_msd2 = vec![d_prime[m - 2], d_prime[m - 1]];
        let u_msd3 = vec![u[m - 2], u[m - 1], u[m]];
        while cmp(&multiply_bu64(&vec![q_hat], &d_msd2), &u_msd3) == Ordering::Greater {
            q_hat -= 1;
        }

        let mut scaled_divisor = multiply_bu64(&multiply_bu64(&vec![q_hat], &d_prime), &base_j);

        if cmp(&rem, &scaled_divisor) == Ordering::Less {
            q_hat -= 1;
            scaled_divisor = multiply_bu64(&multiply_bu64(&vec![q_hat], &d_prime), &base_j)
        }

        rem = add_signed_small_to_large_bu64(&rem, &scaled_divisor, Sign::NEGATIVE);

        q[j] = q_hat;
    }

    let r =
        divide_by_single_digit_bu64(&rem.iter().map(|&x| x as u64).collect::<Vec<_>>(), f as u64).0;

    (q, r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_add_signed_small_to_large_bu64() {
        let a: Vec<u64> = vec![12385590235067398802, 6574815786672150605, 58256762939999093];
        let b: Vec<u64> = vec![3120745040343772444, 127234219];
        let expected_s: Vec<u64> =
            vec![15506335275411171246, 6574815786799384824, 58256762939999093];
        let expected_d: Vec<u64> =
            vec![9264845194723626358, 6574815786544916386, 58256762939999093];

        let actual_s = add_signed_small_to_large_bu64(&a, &b, Sign::POSITIVE);
        let actual_d = add_signed_small_to_large_bu64(&a, &b, Sign::NEGATIVE);

        assert_eq!(expected_s, actual_s);
        assert_eq!(expected_d, actual_d);
    }

    #[test]
    pub fn test_multiply_bu64() {
        let a: Vec<u64> = vec![12385590235067398802, 6574815786672150605, 58256762939999093];
        let b: Vec<u64> = vec![3120745040343772444, 127234219];
        let expected_m: Vec<u64> = vec![
            11227472973818958328,
            5766268126233520549,
            12295967375701953908,
            1487040672093967956,
            401819,
        ];

        let actual_m = multiply_bu64(&a, &b);
        assert_eq!(expected_m, actual_m);
    }

    #[test]
    pub fn test_divide_by_single_digit_bu64() {
        let a: Vec<u64> = vec![12385590235067398802, 6574815786672150605, 58256762939999093];
        let b: u64 = 194632;
        let expected_q: Vec<u64> = vec![9229596213531921335, 13031283436137603541, 299317496300];
        let expected_r: u64 = 42778;

        let (actual_q, actual_r) = divide_by_single_digit_bu64(&a, b);

        assert_eq!(expected_q, actual_q);
        assert_eq!(expected_r, actual_r);
    }

    #[test]
    pub fn test_divide_by_multidigit_bu64() {
        let a: Vec<u64> = vec![12385590235067398802, 6574815786672150605, 58256762939999093];
        let b: Vec<u64> = vec![3120745040343772444, 127234219];
        let expected_q: Vec<u64> = vec![13206933958395635798, 457870243];
        let expected_r: Vec<u64> = vec![18066500957280318250, 30065207];

        let (actual_q, actual_r) = divide_by_multidigit_bu64(&a, &b);

        assert_eq!(expected_q, actual_q);
        assert_eq!(expected_r, actual_r);
    }
}
