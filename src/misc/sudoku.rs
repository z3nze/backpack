pub struct SudokuGame {
    layout: [[u8; 9]; 9],
    area_idx: [[Vec<usize>; 9]; 9],
    areas: Vec<Vec<(usize, usize)>>,
    used: Vec<u16>,
}

impl SudokuGame {
    fn get_rows() -> Vec<Vec<(usize, usize)>> {
        (0..9)
            .flat_map(|i| std::iter::repeat_n(i, 9))
            .zip((0..9).cycle())
            .collect::<Vec<_>>()
            .chunks_exact(9)
            .map(|c| c.to_vec())
            .collect()
    }

    fn get_columns() -> Vec<Vec<(usize, usize)>> {
        (0..9).cycle()
            .zip((0..9).flat_map(|i| std::iter::repeat_n(i, 9)))
            .collect::<Vec<_>>()
            .chunks_exact(9)
            .map(|c| c.to_vec())
            .collect()
    }

    fn get_squares() -> Vec<Vec<(usize, usize)>> {
        (0..9)
            .map(|i| (i / 3 * 3, (i % 3) * 3))
            .flat_map(|(r, c)| (0..9).map(move |i| (r + (i / 3 % 3), c + i % 3)))
            .collect::<Vec<_>>()
            .chunks_exact(9)
            .map(|c| c.to_vec())
            .collect()
    }

    pub fn from_string_vec(sv: Vec<String>) -> Self {
        let mut row_iter = sv.iter();
        let layout: [[u8; 9]; 9] = std::array::from_fn(|_| {
            let row = row_iter.next().unwrap();
            let digits = row.chars().take(9).map(|x| x.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();
            let res: [u8; 9] = digits.try_into().unwrap();
            res
        });
        let areas = [
            Self::get_rows(),
            Self::get_columns(),
            Self::get_squares(),
        ].concat();

        let mut area_idx: [[Vec<usize>; 9]; 9] = Default::default();
        
        areas.iter().enumerate().for_each(|(idx, area)| area.iter().for_each(|&(r, c)| area_idx[r][c].push(idx)));

        let used: Vec<u16> = areas.iter()
            .map(|x| x.iter().map(|&(r, c)| layout[r][c]).filter(|&x| x != 0).fold(0, |acc, x| acc | (1 << x)))
            .collect();

        SudokuGame { layout, area_idx, areas, used }
    }

    pub fn flip(&mut self, pos: (usize, usize), val: u8) {
        let (r, c) = pos;
        self.layout[r][c] = val - self.layout[r][c];
        self.area_idx[r][c].iter().for_each(|&idx| self.used[idx] ^= 1 << val);
    }
    
    pub fn solve(&mut self) -> Option<[[u8; 9]; 9]> {
        let most_constrained_area: Option<(usize, usize)> = self.used.iter().enumerate()
            .map(|(idx, mask)| (idx, mask, (1..10).map(|x| (1 << x) & (mask)).filter(|&x| x == 0).count()))
            .filter(|&(_idx, _mask, free)| free != 0)
            .min_by(|x, y| x.2.cmp(&y.2))
            .map(|(idx, _, free)| (idx, free));

        if most_constrained_area.is_none() {
            return Some(self.layout);
        }

        let &(r, c) = self.areas[most_constrained_area.unwrap().0].iter()
            .find(|&&(r, c)| self.layout[r][c] == 0)
            .unwrap();

        let available_values = (1..10)
            .map(|x| (1 << x) - self.area_idx[r][c].iter().fold(0, |acc, &y| acc | (self.used[y] & (1 << x))))
            .fold(0, |acc, x| acc | x);

        (1..10)
            .filter(|x| available_values & (1 << x) != 0)
            .map(
                |nv| 
                {
                    self.flip((r, c), nv); 
                    if let Some(res) = self.solve() { return Some(res); };
                    self.flip((r, c), nv);
                    None
                }
            )
            .find_map(|x| x)
    }
}
