use clap::Parser;
use gameoflifer::{Board, Pos};
use std::{
    io::{self, Write},
    thread, time,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, disable_help_flag = true)]
struct Args {
    /// Board width
    #[arg(short, long, default_value_t = 10)]
    width: i32,

    /// Board height
    #[arg(short, long, default_value_t = 10)]
    height: i32,

    /// Sleep milliseconds
    #[arg(short, long, default_value_t = 100)]
    sleepmillis: u64,

    /// Print help
    #[arg(long, action = clap::ArgAction::Help)]
    help: Option<bool>,
}

fn main() {
    let args = Args::parse();

    let sleep_duration = time::Duration::from_millis(args.sleepmillis);
    let mut board = glider(args.width, args.height);

    loop {
        cls();
        show_cells(&board);
        if board.is_extinct() {
            break;
        }
        board = board.next_gen();
        goto(&(board.width() + 1, board.height() + 1));
        io::stdout().flush().unwrap();
        thread::sleep(sleep_duration);
    }
}

fn glider(width: i32, height: i32) -> Board {
    Board::new(width, height, &vec![(4, 2), (2, 3), (4, 3), (3, 4), (4, 4)])
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
