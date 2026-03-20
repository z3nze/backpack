pub struct SudokuGame {
    layout: [[u8; 9]; 9],
    area_idx: [[Vec<usize>; 9]; 9],
    areas: Vec<Vec<(usize, usize)>>,
    used: Vec<u16>,
}

impl SudokuGame {
    pub fn from_string_vec(sv: Vec<String>) -> Self {
        let mut row_iter = sv.iter();
        let layout: [[u8; 9]; 9] = std::array::from_fn(|_| {
            let row = row_iter.next().unwrap();
            let digits = row.chars().take(9).map(|x| x.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();
            let res: [u8; 9] = digits.try_into().unwrap();
            res
        });
        let rows: Vec<Vec<(usize, usize)>> = (0..9)
            .flat_map(|i| std::iter::repeat_n(i, 9))
            .zip((0..9).cycle())
            .collect::<Vec<_>>()
            .chunks_exact(9)
            .map(|c| c.to_vec())
            .collect();
        let columns: Vec<Vec<(usize, usize)>> = (0..9).cycle()
            .zip((0..9).flat_map(|i| std::iter::repeat_n(i, 9)))
            .collect::<Vec<_>>()
            .chunks_exact(9)
            .map(|c| c.to_vec())
            .collect();
        let squares: Vec<Vec<(usize, usize)>> = (0..9)
            .map(|i| (i / 3 * 3, (i % 3) * 3))
            .flat_map(|(r, c)| (0..9).map(move |i| (r + (i / 3 % 3), c + i % 3)))
            .collect::<Vec<_>>()
            .chunks_exact(9)
            .map(|c| c.to_vec())
            .collect();
        let areas = [rows, columns, squares].concat();

        let mut area_idx: [[Vec<usize>; 9]; 9] = Default::default();
        
        for (idx, area) in areas.iter().enumerate() {
            for &(r, c) in area.iter() {
                area_idx[r][c].push(idx);
            }
        }

        let mut used: Vec<u16> = vec![0; areas.len()];

        for (area, mask) in areas.iter().zip(used.iter_mut()) {
            *mask = area
                .iter()
                .map(|&(r, c)| layout[r][c])
                .filter(|&x| x != 0)
                .fold(0, |acc, x| acc | (1 << x));
        }

        SudokuGame { layout, area_idx, areas, used }
    }
    
    pub fn solve(&mut self) -> Option<[[u8; 9]; 9]> {
        let mut most_constrained: Option<(usize, usize)> = None;
        for (idx, mask) in self.used.iter().enumerate() {
            let free = (1..10).map(|x| (1 << x) & (mask)).filter(|&x| x == 0).count();
            if free == 0 {
                continue
            }
            if most_constrained.is_none() || most_constrained.unwrap().1 > free {
                most_constrained = Some((idx, free));
            }
        }

        if most_constrained.is_none() {
            return Some(self.layout);
        }

        let &(r, c) = self.areas[most_constrained.unwrap().0].iter()
            .find(|&&(r, c)| self.layout[r][c] == 0)
            .unwrap();
        for nv in 1..10 {
            let mut sat = true;
            for &ai in self.area_idx[r][c].iter() {
                sat &= (self.used[ai] & (1 << nv)) == 0;
            }
            if !sat {
                continue
            }
            self.layout[r][c] = nv;
            for &ai in self.area_idx[r][c].iter() {
                self.used[ai] ^= 1 << nv;
            }

            let res = self.solve();
            if res.is_some() {
                return res;
            }

            self.layout[r][c] = 0;
            for &ai in self.area_idx[r][c].iter() {
                self.used[ai] ^= 1 << nv;
            }
        }

        None
    }
}
