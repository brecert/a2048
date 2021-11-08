use argh::FromArgs;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io;
use std::process::exit;

use termion::input::TermRead;
use termion::raw::IntoRawMode;

use a2048::game::Game;

trait Hashed {
    fn hashed(&self) -> u64;
}

impl<T: Hash> Hashed for T {
    fn hashed(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

fn print_game(game: &Game) {
    print!("score: {}", game.points);
    if game.new_points > 0 {
        print!(" (+{})", game.new_points)
    }
    print!("\n");

    let grid = &game.grid;
    let pad = grid.iter().max().unwrap().to_string().len();

    for y in 0..grid.rows() {
        print!("[");
        for el in grid.iter_row(y) {
            print!(" {:-<pad$} ", el, pad = pad)
        }
        println!("]");
    }
}

#[derive(FromArgs)]
/// A simple 2048 clone
struct A2048 {
    /// the seed to use when playing
    #[argh(option)]
    seed: Option<u64>,

    /// the width in columns of the game
    #[argh(option, default = "4")]
    width: usize,

    /// the height in rows of the game
    #[argh(option, default = "4")]
    height: usize,
}

fn main() -> io::Result<()> {
    let args: A2048 = argh::from_env();

    // let mut game = args.seed.map_or_else(Game::default, Game::from_seed);
    let mut game = Game::new(args.width, args.height, args.seed);
    game.add_random_tile();
    game.add_random_tile();

    print!(
        "{}{}{}",
        termion::cursor::Hide,
        termion::cursor::Goto(1, 1),
        termion::clear::AfterCursor
    );

    print_game(&game);

    let stdout = io::stdout().into_raw_mode()?;
    let stdin = io::stdin().keys();

    for key in stdin {
        let before = game.hashed();

        {
            use termion::event::Key;
            match key? {
                Key::Up => game.shift_top(),
                Key::Down => game.shift_bottom(),
                Key::Left => game.shift_left(),
                Key::Right => game.shift_right(),
                Key::Ctrl('c') | Key::Char('q') | Key::Esc => {
                    stdout.suspend_raw_mode()?;
                    print!("{}", termion::cursor::Show);
                    exit(0)
                }
                _ => (),
            };
        }

        if before == game.hashed() {
            continue;
        }

        game.add_random_tile();

        stdout.suspend_raw_mode()?;
        print!(
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::AfterCursor
        );
        print_game(&game);

        if game.is_game_over() {
            println!("GAME OVER!");
            print!("{}", termion::cursor::Show);
            exit(0)
        }

        stdout.activate_raw_mode()?;
    }

    Ok(())
}
