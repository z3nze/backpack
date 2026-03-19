pub struct SudokuGame {
    layout: [[u8; 9]; 9],
    areas: Vec<Vec<(usize, usize)>>,
    used: Vec<u16>,
}

impl SudokuGame {
    pub fn from_string_vec(sv: Vec<String>) -> Self {
        let mut row_iter = sv.iter();
        let layout: [[u8; 9]; 9] = std::array::from_fn(|_| {
            let row = row_iter.next().unwrap();
            let digits = row.chars().map(|x| x.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();
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
            .map(|i| (i / 3 * 3, i % 3))
            .flat_map(|(r, c)| (0..9).map(move |i| (r + i / 3, c + i % 3)))
            .collect::<Vec<_>>()
            .chunks_exact(9)
            .map(|c| c.to_vec())
            .collect();
        let areas = [rows, columns, squares].concat();

        let mut used: Vec<u16> = vec![0; areas.len()];

        for (area, mask) in areas.iter().zip(used.iter_mut()) {
            *mask = area
                .iter()
                .map(|(r, c)| layout[*r][*c])
                .filter(|x| *x != 0)
                .fold(0, |acc, x| acc | (1 << x));
        }

        SudokuGame { layout, areas, used }
    }
    
    pub fn solve(&mut self) -> Option<[[u8; 9]; 9]> {
        None
    }
}
