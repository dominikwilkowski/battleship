extern crate rand;
extern crate termion;

mod ai;
pub mod config;
mod gui;
pub mod movement;
pub mod ships;

use ships::Ship;
use ships::ShipTracker;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use Cell::{Crosshair, Empty, Placeholder};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cell {
	Empty,
	Shot,
	Ship,
	Damage,
	Placeholder,
	Crosshair,
}

#[derive(Copy, Clone, Debug)]
pub enum Rotation {
	Horizontal,
	Vertical,
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
	Left,
	Right,
	Up,
	Down,
}

fn main() {
	let size = termion::terminal_size();
	if let Ok((width, height)) = size {
		let (width, height) = size.unwrap();
		if width < config::MIN_WIDTH || height < config::MIN_HEIGHT {
			panic!("\r\n\r\n{}This terminal is not big enough with width:{} height:{}.\r\nTo play Battlefield you need at least with:{} height:{}{}\r\n\r\n", termion::color::Fg(termion::color::Red), width, height, config::MIN_WIDTH, config::MIN_HEIGHT, termion::color::Fg(termion::color::Reset));
		}
	} else {
		panic!("The size of the terminal can't be determined");
	}

	let mut stdout = stdout().into_raw_mode().unwrap();

	// our boards
	let mut board_me = [[Empty; 10]; 10];
	let mut board_ai = [[Empty; 10]; 10];
	let mut board_secret = [[Empty; 10]; 10];

	// let the AI generate their own ship placements
	board_secret = ai::set_ships(board_secret);
	// board_ai = board_secret; // to visualize the ai board instantly

	// the ships to be placed
	let mut ships =
		ShipTracker::new(config::SHIP_ONE_BLOCK_AMOUNT, config::SHIP_TWO_BLOCK_AMOUNT, config::SHIP_THREE_BLOCK_AMOUNT);
	let mut this_ship = ships.get_next_unset_ship().unwrap();
	let mut ship_size = config::get_entitie_size(&this_ship);

	// rotation of our ship
	let mut rotation = Rotation::Horizontal;

	// our current position on the board
	let mut pos_x: usize = 0;
	let mut pos_y: usize = 0;

	// placing our first ship
	board_me = movement::place_entity(board_me, pos_x, pos_y, &ship_size, &rotation, Placeholder);

	// GUI
	let header = gui::get_header();
	let header_height: u16 = (header.lines().count() + 2).try_into().unwrap();
	let board = gui::get_board(board_me, board_ai);
	// let board_height: u16 = (board.lines().count() + 2).try_into().unwrap();

	write!(
		stdout,
		"{}{}{}{}{}{}{}{}",
		termion::color::Bg(termion::color::Black),
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

	let mut is_round_one_done = false;

	// FIRST ROUND setting ships
	for key in stdin().keys() {
		write!(stdout, "{}{}", termion::cursor::Restore, termion::clear::CurrentLine).unwrap();

		match key.unwrap() {
			Key::Esc | Key::Char('q') => {
				write!(stdout, "{}{}", termion::cursor::Restore, termion::cursor::Show).unwrap();
				stdout.flush().unwrap();
				termion::raw::RawTerminal::suspend_raw_mode(&stdout).unwrap();
				std::process::exit(0);
			}
			Key::Char('r') => {
				let new_rotation = match rotation {
					Rotation::Horizontal => Rotation::Vertical,
					Rotation::Vertical => Rotation::Horizontal,
				};

				if movement::is_free_space(&board_me, pos_x as isize, pos_y as isize, &ship_size, &new_rotation) {
					// reset previous placement
					board_me = movement::place_entity(board_me, pos_x, pos_y, &ship_size, &rotation, Empty);
					rotation = new_rotation;
					// now place new ship in new rotation
					board_me = movement::place_entity(board_me, pos_x, pos_y, &ship_size, &rotation, Placeholder);
				}
			}
			// PLACE SHIP
			Key::Char('\n') => {
				board_me = movement::place_entity(board_me, pos_x, pos_y, &ship_size, &rotation, Cell::Ship);

				ships.set_ship(&this_ship);
				match ships.get_next_unset_ship() {
					Some(kind) => {
						this_ship = kind;
						ship_size = config::get_entitie_size(&this_ship);
						// collision detection for new pos_x and pos_y
						let (x, y) = movement::get_next_available_coordinates(&board_me, &ship_size, &rotation);
						pos_x = x;
						pos_y = y;
						board_me = movement::place_entity(board_me, pos_x, pos_y, &ship_size, &rotation, Placeholder);
					}
					None => {
						is_round_one_done = true;
					}
				};
			}
			// MOVEMENT
			Key::Left => {
				let (board_new, pos_x_new, pos_y_new) =
					movement::move_ship(board_me, pos_x, pos_y, &rotation, &ship_size, Direction::Left);
				board_me = board_new;
				pos_x = pos_x_new;
				pos_y = pos_y_new;
			}
			Key::Right => {
				let (board_new, pos_x_new, pos_y_new) =
					movement::move_ship(board_me, pos_x, pos_y, &rotation, &ship_size, Direction::Right);
				board_me = board_new;
				pos_x = pos_x_new;
				pos_y = pos_y_new;
			}
			Key::Up => {
				let (board_new, pos_x_new, pos_y_new) =
					movement::move_ship(board_me, pos_x, pos_y, &rotation, &ship_size, Direction::Up);
				board_me = board_new;
				pos_x = pos_x_new;
				pos_y = pos_y_new;
			}
			Key::Down => {
				let (board_new, pos_x_new, pos_y_new) =
					movement::move_ship(board_me, pos_x, pos_y, &rotation, &ship_size, Direction::Down);
				board_me = board_new;
				pos_x = pos_x_new;
				pos_y = pos_y_new;
			}
			Key::Delete | Key::Backspace => {
				ships = ShipTracker::new(
					config::SHIP_ONE_BLOCK_AMOUNT,
					config::SHIP_TWO_BLOCK_AMOUNT,
					config::SHIP_THREE_BLOCK_AMOUNT,
				);
				this_ship = ships.get_next_unset_ship().unwrap();
				ship_size = config::get_entitie_size(&this_ship);
				rotation = Rotation::Horizontal;
				pos_x = 0;
				pos_y = 0;
				board_me = movement::place_entity([[Empty; 10]; 10], pos_x, pos_y, &ship_size, &rotation, Placeholder);
			}
			_ => {}
		}

		write!(
			stdout,
			"{}{}{}",
			termion::cursor::Goto(1, header_height),
			gui::get_board(board_me, board_ai),
			termion::cursor::Restore,
		)
		.unwrap();
		stdout.flush().unwrap();

		if is_round_one_done {
			break;
		}
	}

	pos_x = 0;
	pos_y = 0;
	board_ai = movement::place_entity(board_ai, pos_x, pos_y, &1, &Rotation::Horizontal, Crosshair);

	write!(
		stdout,
		"{}{}{}",
		termion::cursor::Goto(1, header_height),
		gui::get_board(board_me, board_ai),
		termion::cursor::Restore,
	)
	.unwrap();
	stdout.flush().unwrap();

	let mut is_round_two_done = false;

	// SECOND ROUND shooting turns
	for key in stdin().keys() {
		write!(stdout, "{}{}", termion::cursor::Restore, termion::clear::CurrentLine).unwrap();

		match key.unwrap() {
			Key::Esc | Key::Char('q') => {
				write!(stdout, "{}{}", termion::cursor::Restore, termion::cursor::Show).unwrap();
				stdout.flush().unwrap();
				termion::raw::RawTerminal::suspend_raw_mode(&stdout).unwrap();
				std::process::exit(0);
			}
			// SHOOT
			Key::Char('\n') => {
				// SHOOT
			}
			// MOVEMENT
			Key::Left => {
				let (board_new, pos_x_new, pos_y_new) = movement::move_crosshair(board_ai, pos_x, pos_y, Direction::Left);
				board_ai = board_new;
				pos_x = pos_x_new;
				pos_y = pos_y_new;
			}
			Key::Right => {
				let (board_new, pos_x_new, pos_y_new) = movement::move_crosshair(board_ai, pos_x, pos_y, Direction::Right);
				board_ai = board_new;
				pos_x = pos_x_new;
				pos_y = pos_y_new;
			}
			Key::Up => {
				let (board_new, pos_x_new, pos_y_new) = movement::move_crosshair(board_ai, pos_x, pos_y, Direction::Up);
				board_ai = board_new;
				pos_x = pos_x_new;
				pos_y = pos_y_new;
			}
			Key::Down => {
				let (board_new, pos_x_new, pos_y_new) = movement::move_crosshair(board_ai, pos_x, pos_y, Direction::Down);
				board_ai = board_new;
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

		if is_round_two_done {
			break;
		}
	}

	write!(stdout, "{}", termion::cursor::Show).unwrap();
}
