extern crate rand;
extern crate termion;

pub mod config;
mod gui;
mod ships;

use ships::Ships;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use Cell::{Damage, Empty, Placeholder, Ship, Shot};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cell {
	Empty,
	Shot,
	Ship,
	Damage,
	Placeholder,
}

#[derive(Copy, Clone, Debug)]
enum Rotation {
	Horizontal,
	Vertical,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
	Left,
	Right,
	Up,
	Down,
}

fn main() {
	let stdin = stdin();
	let mut stdout = stdout().into_raw_mode().unwrap();

	// our boards
	let mut board_me = [[Empty; 10]; 10];
	let mut board_ai = [[Empty; 10]; 10];

	// the ships to be placed
	let mut ships = Ships::new(
		config::SHIP_ONE_BLOCK_AMOUNT,
		config::SHIP_TWO_BLOCK_AMOUNT,
		config::SHIP_THREE_BLOCK_AMOUNT,
	);
	let (kind, index) = ships.get_next_unset_ship();
	let mut ship_size = config::get_entitie_size(kind);

	// rotation of our ship
	let mut rotation = Rotation::Horizontal;

	// our current position on the board
	let mut pos_x: usize = 0;
	let mut pos_y: usize = 0;

	// the boundary of our board depending on the size of the current ship
	let mut max_x: usize = get_max_x(&rotation, &ship_size);
	let mut max_y: usize = get_max_y(&rotation, &ship_size);

	// placing our first ship
	board_me = place_ship(board_me, pos_x, pos_y, &rotation, &ship_size, Placeholder);

	// GUI
	let header = gui::get_header();
	let header_height: u16 = (header.lines().count() + 2).try_into().unwrap();
	let board = gui::get_board(board_me, board_ai);
	let board_height: u16 = (board.lines().count() + 2).try_into().unwrap();

	write!(
		stdout,
		"{}{}{}{}{}{}{}",
		termion::clear::All,
		termion::cursor::Goto(1, 2),
		termion::cursor::Hide,
		header,
		board,
		gui::get_round1_instructions(),
		termion::cursor::Save
	)
	.unwrap();

	stdout.flush().unwrap();

	for key in stdin.keys() {
		write!(stdout, "{}{}", termion::cursor::Restore, termion::clear::CurrentLine).unwrap();

		match key.unwrap() {
			Key::Char('q') => break,
			Key::Esc => break,
			Key::Char('r') => {
				// reset previous placement
				board_me = place_ship(board_me, pos_x, pos_y, &rotation, &ship_size, Empty);
				match rotation {
					Rotation::Horizontal => {
						rotation = Rotation::Vertical;
					}
					Rotation::Vertical => {
						rotation = Rotation::Horizontal;
					}
				};

				// get new boundaries
				max_x = get_max_x(&rotation, &ship_size);
				max_y = get_max_y(&rotation, &ship_size);

				// make sure we're still within the boundaries after rotation
				if pos_x > max_x {
					pos_x = max_x;
				}
				if pos_y > max_y {
					pos_y = max_y;
				}

				board_me = place_ship(board_me, pos_x, pos_y, &rotation, &ship_size, Placeholder);
			}
			Key::Char('\n') => println!("ENTER"),
			Key::Left => {
				let (board_new, pos_x_new, pos_y_new) = move_ship(
					board_me,
					pos_x,
					pos_y,
					&max_x,
					&max_y,
					rotation,
					ship_size,
					Direction::Left,
				);
				board_me = board_new;
				pos_x = pos_x_new;
				pos_y = pos_y_new;
			}
			Key::Right => {
				let (board_new, pos_x_new, pos_y_new) = move_ship(
					board_me,
					pos_x,
					pos_y,
					&max_x,
					&max_y,
					rotation,
					ship_size,
					Direction::Right,
				);
				board_me = board_new;
				pos_x = pos_x_new;
				pos_y = pos_y_new;
			}
			Key::Up => {
				let (board_new, pos_x_new, pos_y_new) = move_ship(
					board_me,
					pos_x,
					pos_y,
					&max_x,
					&max_y,
					rotation,
					ship_size,
					Direction::Up,
				);
				board_me = board_new;
				pos_x = pos_x_new;
				pos_y = pos_y_new;
			}
			Key::Down => {
				let (board_new, pos_x_new, pos_y_new) = move_ship(
					board_me,
					pos_x,
					pos_y,
					&max_x,
					&max_y,
					rotation,
					ship_size,
					Direction::Down,
				);
				board_me = board_new;
				pos_x = pos_x_new;
				pos_y = pos_y_new;
			}
			_ => {}
		}

		write!(
			stdout,
			"{}{}{}{}{}",
			termion::cursor::Goto(1, header_height),
			termion::clear::AfterCursor,
			gui::get_board(board_me, board_ai),
			gui::get_round1_instructions(),
			termion::cursor::Restore,
		)
		.unwrap();
		stdout.flush().unwrap();
	}

	write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn move_ship<'a>(
	mut board: [[Cell; 10]; 10],
	mut pos_x: usize,
	mut pos_y: usize,
	max_x: &usize,
	max_y: &usize,
	rotation: Rotation,
	ship_size: usize,
	direction: Direction,
) -> ([[Cell; 10]; 10], usize, usize) {
	match direction {
		Direction::Left => {
			// clear previous position
			board = place_ship(board, pos_x, pos_y, &rotation, &ship_size, Empty);
			pos_x = match pos_x {
				0 => 0,
				_ => pos_x - 1,
			};
			// set new position
			board = place_ship(board, pos_x, pos_y, &rotation, &ship_size, Placeholder);
		}
		Direction::Right => {
			// clear previous position
			board = place_ship(board, pos_x, pos_y, &rotation, &ship_size, Empty);
			pos_x = match pos_x {
				pos if pos >= *max_x => *max_x,
				_ => pos_x + 1,
			};
			// set new position
			board = place_ship(board, pos_x, pos_y, &rotation, &ship_size, Placeholder);
		}
		Direction::Up => {
			// clear previous position
			board = place_ship(board, pos_x, pos_y, &rotation, &ship_size, Empty);
			pos_y = match pos_y {
				0 => 0,
				_ => pos_y - 1,
			};
			// set new position
			board = place_ship(board, pos_x, pos_y, &rotation, &ship_size, Placeholder);
		}
		Direction::Down => {
			// clear previous position
			board = place_ship(board, pos_x, pos_y, &rotation, &ship_size, Empty);
			pos_y = match pos_y {
				pos if pos >= *max_y => *max_y,
				_ => pos_y + 1,
			};
			// set new position
			board = place_ship(board, pos_x, pos_y, &rotation, &ship_size, Placeholder);
		}
	};

	(board, pos_x, pos_y)
}

fn place_ship(
	mut board: [[Cell; 10]; 10],
	mut pos_x: usize,
	mut pos_y: usize,
	rotation: &Rotation,
	ship_size: &usize,
	cell: Cell,
) -> [[Cell; 10]; 10] {
	match rotation {
		Rotation::Horizontal => {
			for offset in 0..*ship_size {
				board[pos_y][pos_x + offset] = cell;
			}
		}
		Rotation::Vertical => {
			for offset in 0..*ship_size {
				board[pos_y + offset][pos_x] = cell;
			}
		}
	}

	board
}

#[test]
fn place_ship_works() {
	let mut result = place_ship([[Empty; 10]; 10], 0, 0, &Rotation::Horizontal, &1, Placeholder);
	assert_eq!(
		result,
		[
			[
				Placeholder,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty
			],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
		]
	);

	result = place_ship([[Empty; 10]; 10], 0, 0, &Rotation::Horizontal, &2, Placeholder);
	assert_eq!(
		result,
		[
			[
				Placeholder,
				Placeholder,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty
			],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
		]
	);

	result = place_ship([[Empty; 10]; 10], 0, 0, &Rotation::Horizontal, &3, Placeholder);
	assert_eq!(
		result,
		[
			[
				Placeholder,
				Placeholder,
				Placeholder,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty
			],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
		]
	);

	result = place_ship([[Empty; 10]; 10], 0, 0, &Rotation::Vertical, &1, Placeholder);
	assert_eq!(
		result,
		[
			[
				Placeholder,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty
			],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
		]
	);

	result = place_ship([[Empty; 10]; 10], 0, 0, &Rotation::Vertical, &2, Placeholder);
	assert_eq!(
		result,
		[
			[
				Placeholder,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty
			],
			[
				Placeholder,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty
			],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
		]
	);

	result = place_ship([[Empty; 10]; 10], 0, 0, &Rotation::Vertical, &3, Placeholder);
	assert_eq!(
		result,
		[
			[
				Placeholder,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty
			],
			[
				Placeholder,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty
			],
			[
				Placeholder,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty
			],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
		]
	);
}

fn get_max_x(rotation: &Rotation, ship_size: &usize) -> usize {
	match rotation {
		Rotation::Horizontal => 9 - ship_size + 1,
		Rotation::Vertical => 9,
	}
}

#[test]
fn get_max_x_works() {
	assert_eq!(get_max_x(&Rotation::Horizontal, &1), 9);
	assert_eq!(get_max_x(&Rotation::Horizontal, &2), 8);
	assert_eq!(get_max_x(&Rotation::Horizontal, &3), 7);
	assert_eq!(get_max_x(&Rotation::Horizontal, &4), 6);
	assert_eq!(get_max_x(&Rotation::Vertical, &1), 9);
	assert_eq!(get_max_x(&Rotation::Vertical, &2), 9);
	assert_eq!(get_max_x(&Rotation::Vertical, &3), 9);
	assert_eq!(get_max_x(&Rotation::Vertical, &4), 9);
}

fn get_max_y(rotation: &Rotation, ship_size: &usize) -> usize {
	match rotation {
		Rotation::Horizontal => 9,
		Rotation::Vertical => 9 - ship_size + 1,
	}
}

#[test]
fn get_max_y_works() {
	assert_eq!(get_max_y(&Rotation::Vertical, &1), 9);
	assert_eq!(get_max_y(&Rotation::Vertical, &2), 8);
	assert_eq!(get_max_y(&Rotation::Vertical, &3), 7);
	assert_eq!(get_max_y(&Rotation::Vertical, &4), 6);
	assert_eq!(get_max_y(&Rotation::Horizontal, &1), 9);
	assert_eq!(get_max_y(&Rotation::Horizontal, &2), 9);
	assert_eq!(get_max_y(&Rotation::Horizontal, &3), 9);
	assert_eq!(get_max_y(&Rotation::Horizontal, &4), 9);
}
