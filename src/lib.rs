pub type Pos = (i32, i32);

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    cells: Vec<Pos>,
    width: i32,
    height: i32,
}

impl Board {
    pub fn new(width: i32, height: i32, cells: &Vec<Pos>) -> Self {
        Self {
            cells: cells.clone(),
            width,
            height,
        }
    }

    pub fn is_extinct(&self) -> bool {
        self.cells.is_empty()
    }

    pub fn next_gen(&self) -> Board {
        let mut survivors = self.survivors();
        let births = self.births();
        survivors.extend(births);
        Self::new(self.width, self.height, &survivors)
    }

    pub fn walk(&self, callback: impl Fn(Pos)) {
        self.cells.iter().for_each(|pos| {
            callback(*pos);
        });
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    fn survivors(&self) -> Vec<Pos> {
        self.cells
            .iter()
            .filter(|pos| {
                let count = self.live_neighbs(pos);
                count == 2 || count == 3
            })
            .map(|(x, y)| (*x, *y))
            .collect()
    }

    fn live_neighbs(&self, pos: &Pos) -> i32 {
        self.neighbs(pos)
            .into_iter()
            .filter(|p| self.is_alive(p))
            .count() as i32
    }

    fn is_alive(&self, pos: &Pos) -> bool {
        self.cells.contains(pos)
    }

    fn is_empty(&self, pos: &Pos) -> bool {
        !self.is_alive(pos)
    }

    fn births(&self) -> Vec<Pos> {
        let mut new_cells = vec![];
        for x in 1..=self.width {
            for y in 1..=self.height {
                let pos = (x, y);
                if self.is_empty(&pos) && self.live_neighbs(&pos) == 3 {
                    new_cells.push(pos);
                }
            }
        }
        new_cells
    }

    fn neighbs(&self, pos: &Pos) -> Vec<Pos> {
        let (x, y) = (pos.0, pos.1);
        vec![
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
        .into_iter()
        .map(|pos| self.wrap(pos))
        .collect()
    }

    fn wrap(&self, (x, y): (i32, i32)) -> Pos {
        (
            (x - 1).rem_euclid(self.width) + 1,
            (y - 1).rem_euclid(self.height) + 1,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_next_gen() {
        let w = 5;
        let h = 5;
        let tests = vec![
            (
                Board::new(w, h, &vec![(4, 2), (2, 3), (4, 3), (3, 4), (4, 4)]),
                Board::new(w, h, &vec![(4, 3), (3, 4), (4, 4), (3, 2), (5, 3)]),
            ),
            (
                Board::new(w, h, &vec![(5, 4), (4, 5), (5, 5), (1, 4), (4, 3)]),
                Board::new(w, h, &vec![(4, 5), (5, 5), (1, 4), (1, 5), (5, 3)]),
            ),
            (
                Board::new(w, h, &vec![(4, 5), (5, 5), (1, 4), (1, 5), (5, 3)]),
                Board::new(w, h, &vec![(5, 5), (1, 4), (1, 5), (4, 4), (5, 1)]),
            ),
        ];
        for (board, expect) in tests {
            assert_eq!(board.next_gen(), expect);
        }
    }
}