use argh::FromArgs;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{execute, terminal};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io;
use std::process::exit;

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

    execute!(
        io::stdout(),
        crossterm::cursor::Hide,
        crossterm::cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::FromCursorDown)
    )?;

    fn quit_game() -> crossterm::Result<()> {
        disable_raw_mode()?;
        execute!(io::stdout(), crossterm::cursor::Show)?;
        exit(0)
    }

    print_game(&game);

    loop {
        enable_raw_mode()?;

        let before = game.hashed();

        {
            use crossterm::event::{self, Event, KeyCode as Key, KeyModifiers};
            match event::read()? {
                Event::Key(key) => match key.code {
                    Key::Up => game.shift_top(),
                    Key::Down => game.shift_bottom(),
                    Key::Left => game.shift_left(),
                    Key::Right => game.shift_right(),
                    Key::Char('q') | Key::Esc => quit_game()?,
                    Key::Char('c') if key.modifiers == KeyModifiers::CONTROL => quit_game()?,
                    _ => (),
                },
                _ => (),
            }
        }

        if before == game.hashed() {
            continue;
        }

        game.add_random_tile();

        disable_raw_mode()?;
        execute!(
            io::stdout(),
            crossterm::cursor::MoveTo(0, 0),
            terminal::Clear(terminal::ClearType::FromCursorDown)
        )?;
        print_game(&game);

        if game.is_game_over() {
            println!("GAME OVER!");
            quit_game()?;
        }

        enable_raw_mode()?;
    }
}
