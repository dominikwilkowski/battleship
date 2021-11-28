extern crate rand;
extern crate termion;

mod display;

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use Cell::{Damage, Empty, Placeholder, Ship, Shot};

#[derive(Copy, Clone, Debug)]
pub enum Cell {
	Empty,
	Shot,
	Ship,
	Damage,
	Placeholder,
}

fn main() {
	let stdin = stdin();
	let mut stdout = stdout().into_raw_mode().unwrap();

	let mut board_me = [[Cell::Empty; 10]; 10];
	// let mut board_ai = [[Cell::Empty; 10]; 10];

	// let mut board_me = [
	// 	[Shot, Ship, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
	// 	[Empty, Ship, Empty, Empty, Empty, Empty, Empty, Ship, Empty, Empty],
	// 	[Empty, Ship, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
	// 	[Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
	// 	[Empty, Empty, Empty, Ship, Ship, Empty, Empty, Empty, Empty, Empty],
	// 	[Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Ship],
	// 	[Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Ship],
	// 	[Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
	// 	[Empty, Empty, Ship, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
	// 	[Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Damage],
	// ];

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

	let mut pos_x = 0;
	let mut pos_y = 0;

	board_me[pos_y][pos_x] = Placeholder;

	let header = display::draw_header();
	let header_height: u16 = (header.lines().count() + 2).try_into().unwrap();
	let board = display::draw_board(board_me, board_ai);
	let board_height: u16 = (board.lines().count() + 2).try_into().unwrap();

	write!(
		stdout,
		"{}{}{}{}{}{}\r\n{}",
		termion::clear::All,
		termion::cursor::Goto(1, 2),
		termion::cursor::Hide,
		header,
		board,
		display::draw_round1_instructions(),
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
			Key::Char('h') => println!("←"),
			Key::Left => {
				board_me[pos_y][pos_x] = Empty;
				if pos_x == 0 {
					pos_x = 0;
				} else {
					pos_x -= 1;
				}
				board_me[pos_y][pos_x] = Placeholder;

				write!(
					stdout,
					"{}{}{}{}{}",
					termion::cursor::Goto(1, header_height),
					termion::clear::AfterCursor,
					display::draw_board(board_me, board_ai),
					display::draw_round1_instructions(),
					termion::cursor::Restore,
				)
				.unwrap();
			}
			Key::Right => {
				board_me[pos_y][pos_x] = Empty;
				pos_x += 1;
				if pos_x > 9 {
					pos_x = 9;
				}
				board_me[pos_y][pos_x] = Placeholder;

				write!(
					stdout,
					"{}{}{}{}{}",
					termion::cursor::Goto(1, header_height),
					termion::clear::AfterCursor,
					display::draw_board(board_me, board_ai),
					display::draw_round1_instructions(),
					termion::cursor::Restore,
				)
				.unwrap();
			}
			Key::Up => {
				board_me[pos_y][pos_x] = Empty;
				if pos_y == 0 {
					pos_y = 0;
				} else {
					pos_y -= 1;
				}
				board_me[pos_y][pos_x] = Placeholder;

				write!(
					stdout,
					"{}{}{}{}{}",
					termion::cursor::Goto(1, header_height),
					termion::clear::AfterCursor,
					display::draw_board(board_me, board_ai),
					display::draw_round1_instructions(),
					termion::cursor::Restore,
				)
				.unwrap();
			}
			Key::Down => {
				board_me[pos_y][pos_x] = Empty;
				pos_y += 1;
				if pos_y > 9 {
					pos_y = 9;
				}
				board_me[pos_y][pos_x] = Placeholder;

				write!(
					stdout,
					"{}{}{}{}{}",
					termion::cursor::Goto(1, header_height),
					termion::clear::AfterCursor,
					display::draw_board(board_me, board_ai),
					display::draw_round1_instructions(),
					termion::cursor::Restore,
				)
				.unwrap();
			}
			Key::Char('\n') => println!("Enter"),
			_ => {}
		}
		stdout.flush().unwrap();
	}

	write!(stdout, "{}", termion::cursor::Show).unwrap();
}
