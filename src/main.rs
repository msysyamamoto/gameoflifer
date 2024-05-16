use std::io::{self, Write};
use std::{thread, time};

const WIDTH: i32 = 10;
const HEIGHT: i32 = 10;
const SLEEP_MILLIS: u64 = 200;

type Pos = (i32, i32);

struct Board {
    cells: Vec<Pos>,
}

impl Board {
    pub fn new(args: &[(i32, i32)]) -> Self {
        let cells = args.iter().map(|(x, y)| (*x, *y)).collect::<Vec<_>>();
        Self { cells }
    }

    pub fn from_vec(cells: &Vec<Pos>) -> Self {
        Self {
            cells: cells.clone(),
        }
    }
}

fn main() {
    let sleep_duration = time::Duration::from_millis(SLEEP_MILLIS);
    let mut board = Board::new(&[(4, 2), (2, 3), (4, 3), (3, 4), (4, 4)]);

    for _ in 0..50 {
        cls();
        show_cells(&board);
        board = next_gen(&board);
        goto(&(WIDTH + 1, HEIGHT + 1));
        io::stdout().flush().unwrap();
        thread::sleep(sleep_duration);
    }
}

fn next_gen(board: &Board) -> Board {
    let mut survivors = survivors(board);
    let births = births(board);
    survivors.extend(births);

    Board::from_vec(&survivors)
}

fn births(board: &Board) -> Vec<Pos> {
    let mut new_cells = vec![];
    for x in 1..=WIDTH {
        for y in 1..=HEIGHT {
            let pos = (x, y);
            if is_empty(board, &pos) && live_neighbs(board, &pos) == 3 {
                new_cells.push(pos);
            }
        }
    }
    new_cells
}

fn survivors(board: &Board) -> Vec<Pos> {
    board
        .cells
        .iter()
        .filter(|pos| {
            let count = live_neighbs(board, pos);
            count == 2 || count == 3
        })
        .map(|pos| (pos.0, pos.1))
        .collect()
}

fn live_neighbs(board: &Board, pos: &Pos) -> i32 {
    neighbs(pos)
        .into_iter()
        .filter(|p| is_alive(&board, p))
        .count() as i32
}

fn neighbs(pos: &Pos) -> Vec<Pos> {
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
    .map(|pos| wrap(pos))
    .collect()
}

fn wrap(pos: (i32, i32)) -> Pos {
    let (x, y) = (pos.0, pos.1);
    (
        (x - 1).rem_euclid(WIDTH) + 1,
        (y - 1).rem_euclid(HEIGHT) + 1,
    )
}

fn is_alive(board: &Board, pos: &Pos) -> bool {
    board.cells.contains(pos)
}

fn is_empty(board: &Board, pos: &Pos) -> bool {
    !is_alive(&board, pos)
}

fn show_cells(board: &Board) {
    board.cells.iter().for_each(|pos| {
        write_at(pos, 'O');
    });
}

fn write_at(pos: &Pos, c: char) {
    goto(pos);
    print!("{}", c);
}

fn goto(pos: &Pos) {
    print!("\x1b[{};{}H", pos.1, pos.0);
}

fn cls() {
    print!("\x1b[2J");
}

#[cfg(test)]
mod tests {
    use crate::{neighbs, wrap, HEIGHT, WIDTH};

    #[test]
    fn test_neighbs() {
        let tests = vec![
            (
                (2, 2),
                vec![
                    (1, 1),
                    (2, 1),
                    (3, 1),
                    (1, 2),
                    (3, 2),
                    (1, 3),
                    (2, 3),
                    (3, 3),
                ],
            ),
            (
                (1, 1),
                vec![
                    (WIDTH, HEIGHT),
                    (1, HEIGHT),
                    (2, HEIGHT),
                    (WIDTH, 1),
                    (2, 1),
                    (WIDTH, 2),
                    (1, 2),
                    (2, 2),
                ],
            ),
            (
                (WIDTH, HEIGHT),
                vec![
                    (WIDTH - 1, HEIGHT - 1),
                    (WIDTH, HEIGHT - 1),
                    (1, HEIGHT - 1),
                    (WIDTH - 1, HEIGHT),
                    (1, HEIGHT),
                    (WIDTH - 1, 1),
                    (WIDTH, 1),
                    (1, 1),
                ],
            ),
        ];

        for (pos, expect) in tests {
            assert_eq!(neighbs(&pos), expect);
        }
    }

    #[test]
    fn test_wrap() {
        let tests = vec![
            ((WIDTH, HEIGHT), (WIDTH, HEIGHT)),
            ((1, 1), (1, 1)),
            ((0, 0), (WIDTH, HEIGHT)),
            ((WIDTH + 1, HEIGHT + 1), (1, 1)),
        ];

        for (pos, expect) in tests {
            assert_eq!(wrap(pos), expect);
        }
    }
}
