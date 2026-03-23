use std::collections::HashMap;
use super::util;

pub struct AnagramicSquares {
    words: Vec<String>
}

impl AnagramicSquares {
    pub fn new(words: Vec<String>) -> Self {
        AnagramicSquares { words }
    }

    fn issq(x: u64) -> bool {
        let rx = (x as f64).sqrt().floor() as u64;
        rx * rx == x
    }

    fn aux2(cv: &[char], ctoi: &mut HashMap<char, u8>, used: u16, w1: &String, w2: &String, ans: &mut u64) {
        if cv.is_empty() {
            let w1v: u64 = w1.chars().map(|c| ctoi.get(&c).unwrap()).fold(0, |acc, &x| acc * 10 + x as u64);
            let w2v: u64 = w2.chars().map(|c| ctoi.get(&c).unwrap()).fold(0, |acc, &x| acc * 10 + x as u64);

            if Self::issq(w1v) && Self::issq(w2v) {
                *ans = (*ans).max(w1v).max(w2v);
                dbg!(ans, w1, w2);
            }

            return;
        }

        for nv in 0..10 {
            if (1 << nv) & used == (1 << nv) {
                continue
            }
            if nv == 0 && (*cv.first().unwrap() == w1.chars().next().unwrap() || *cv.first().unwrap() == w2.chars().next().unwrap()) {
                continue
            }
            ctoi.insert(*cv.first().unwrap(), nv);
            Self::aux2(&cv[1..], ctoi, used | (1 << nv), w1, w2, ans);
            ctoi.remove(cv.first().unwrap());
        }
    }

    fn aux1(gk: String, w1: &String, w2: &String) -> u64 {
        let mut cv: Vec<char> = gk.chars().collect();
        cv.dedup();
        let mut ctoi: HashMap<char, u8> = Default::default();
        let mut ans = 0;
        Self::aux2(&cv, &mut ctoi, 0u16, w1, w2, &mut ans);
        ans
    }

    pub fn solve(&self) {
        let mut gr: HashMap<String, Vec<String>> = Default::default();

        for word in self.words.iter() {
            let mut cv: Vec<char> = word.chars().collect();
            cv.sort();
            let ws = String::from_iter(cv.iter());
            gr.entry(ws).or_default().push(word.to_string());
        }

        let ans: u64 = gr.iter()
            .filter(|(_, words)| words.len() >= 2)
            .map(|(gk, words)| {
                util::prod(words, words).iter()
                    .filter(|(w1, w2)| w1 != w2)
                    .map(|(w1, w2)| Self::aux1(gk.to_string(), w1, w2))
                    .max()
                    .unwrap_or(0)
                }
            )
            .max().unwrap();
        println!("{ans}");
    }
}
