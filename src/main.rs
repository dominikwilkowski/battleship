extern crate rand;
extern crate termion;

mod display;

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use Cell::{Damage, Empty, Ship, Shot};

#[derive(Copy, Clone, Debug)]
pub enum Cell {
	Empty,
	Shot,
	Ship,
	Damage,
}

fn main() {
	let stdin = stdin();
	let mut stdout = stdout().into_raw_mode().unwrap();

	// let mut board_me = [[Cell::Empty; 10]; 10];
	// let mut board_ai = [[Cell::Empty; 10]; 10];

	let mut board_me = [
		[Shot, Ship, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
		[Empty, Ship, Empty, Empty, Empty, Empty, Empty, Ship, Empty, Empty],
		[Empty, Ship, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
		[Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
		[Empty, Empty, Empty, Ship, Ship, Empty, Empty, Empty, Empty, Empty],
		[Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Ship],
		[Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Ship],
		[Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
		[Empty, Empty, Ship, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
		[Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Damage],
	];

	let mut board_ai = [
		[Shot, Ship, Ship, Ship, Empty, Empty, Empty, Empty, Empty, Empty],
		[Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
		[Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
		[Empty, Empty, Empty, Empty, Shot, Empty, Empty, Ship, Empty, Empty],
		[Empty, Empty, Empty, Empty, Shot, Empty, Empty, Ship, Empty, Empty],
		[Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
		[Ship, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
		[Empty, Empty, Empty, Empty, Empty, Empty, Ship, Empty, Empty, Empty],
		[Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
		[Empty, Empty, Ship, Empty, Empty, Empty, Ship, Ship, Empty, Empty],
	];

	let screen = display::draw(board_me, board_ai);

	write!(
		stdout,
		"{}{}{}{}\r\n{}",
		termion::clear::All,
		termion::cursor::Goto(1, 1),
		termion::cursor::Hide,
		screen,
		termion::cursor::Save
	)
	.unwrap();

	stdout.flush().unwrap();

	for c in stdin.keys() {
		write!(
			stdout,
			"\r\n{}{}",
			termion::cursor::Restore,
			termion::clear::CurrentLine
		)
		.unwrap();

		match c.unwrap() {
			Key::Char('q') => break,
			Key::Alt(_) => break,
			Key::Esc => break,
			Key::Left => println!("←"),
			Key::Right => println!("→"),
			Key::Up => println!("↑"),
			Key::Down => println!("↓"),
			Key::Char('\n') => println!("Enter"),
			_ => {}
		}
		stdout.flush().unwrap();
	}

	write!(stdout, "{}", termion::cursor::Show).unwrap();
}
