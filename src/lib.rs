use anyhow::Result;
use nom::bytes::complete::tag;
use nom::character::complete::{i32, line_ending, u32};
use nom::multi::count;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;
use std::io::Write;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub type Pos = (i32, i32);

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    cells: Vec<Pos>,
    width: i32,
    height: i32,
}

#[derive(Debug)]
pub struct Config {
    filename: String,
    sleepmillis: u64,
}

impl Config {
    pub fn new(filename: String, sleepmillis: u64) -> Self {
        Self {
            filename,
            sleepmillis,
        }
    }
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
        let mut bs: Vec<Pos> = self
            .cells
            .iter()
            .flat_map(|pos| self.neighbs(pos))
            .collect();
        bs.sort();
        bs.dedup();

        bs.into_iter()
            .filter(|pos| self.is_empty(pos) && self.live_neighbs(pos) == 3)
            .collect()
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

pub fn run(config: Config) -> Result<()> {
    match open(&config.filename) {
        Err(err) => eprintln!("{}: {}", config.filename, err),
        Ok(mut file) => {
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;
            match parse_input_file(&buf) {
                Err(err) => eprintln!("{}: {}", config.filename, err),
                Ok((_, board)) => main_loop(&config, board),
            }
        }
    }
    Ok(())
}

fn main_loop(config: &Config, mut board: Board) {
    loop {
        cls();
        show_cells(&board);
        if board.is_extinct() {
            break;
        }
        board = board.next_gen();
        goto(&(board.width() + 1, board.height() + 1));
        io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(config.sleepmillis));
    }
}

fn parse_input_file(input: &str) -> IResult<&str, Board> {
    let (remaining, (width, height)) = parse_integer_pair(input)?;
    let (remaining, cell_num) = parse_integer_single(remaining)?;
    let (remaining, cells) = count(parse_integer_pair, cell_num as usize)(remaining)?;

    Ok((
        remaining,
        Board {
            cells,
            width,
            height,
        },
    ))
}

fn parse_integer_pair(input: &str) -> IResult<&str, (i32, i32)> {
    terminated(separated_pair(i32, tag(" "), i32), line_ending)(input)
}

fn parse_integer_single(input: &str) -> IResult<&str, u32> {
    terminated(u32, line_ending)(input)
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn show_cells(board: &Board) {
    board.walk(|pos| {
        write_at(&pos, 'O');
    });
}

fn write_at(pos: &Pos, c: char) {
    goto(pos);
    print!("{}", c);
}

fn goto((x, y): &Pos) {
    print!("\x1b[{};{}H", *y, *x);
}

fn cls() {
    print!("\x1b[2J");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_file() {
        let tests = vec![(
            "21 23\n4\n5 6\n7 8\n9 10\n11 12\n",
            Board {
                cells: vec![(5, 6), (7, 8), (9, 10), (11, 12)],
                width: 21,
                height: 23,
            },
        )];
        for (input, expect) in tests {
            assert_eq!(parse_input_file(input), Ok(("", expect)));
        }
    }

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
