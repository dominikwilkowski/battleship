extern crate termion;

use std::io;

use crate::config;
use crate::game;
use crate::Cell;

use termion::color;
use Cell::{Crosshair, Damage, Empty, Placeholder, Ship, ShipOne, ShipThree, ShipTwo, Shot};

pub enum Round {
	One,
	Two,
}

pub fn get_padding() -> String {
	let mut padding: f32 = 0.0;
	let size = termion::terminal_size();
	let board_size = (config::SIZE_X as f32 * 3.0 * 2.0) + 11.0;

	if let Ok((width, _)) = size {
		padding = ((width as f32 - board_size) / 2.0).floor();
	}

	format!("{:width$}", "", width = padding as usize)
}

pub fn get_header() -> String {
	let padding = get_padding();
	let reset = color::Fg(color::White);
	let board_size = (config::SIZE_X * 3 * 2) + 11;
	let inner_size = ((board_size - 46) / 2) as f32;
	let inner_padding = format!("{:width$}", "", width = inner_size.floor() as usize);

	let logo1 = format!(
		"{}{}{}┏┓         ┏┓   ┏┓  ┏┓            ┏┓   ┏┓{}\r\n",
		padding,
		inner_padding,
		color::Fg(color::White),
		reset,
	);
	let logo2 = format!(
		"{}{}{}┃┗━┓ ┏━━┓ ┏┛┗┓ ┏┛┗┓ ┃┃  ┏━━┓ ┏━━┓ ┃┗━┓ ┗┛ ┏━━┓\r\n{}",
		padding,
		inner_padding,
		color::Fg(color::White),
		reset,
	);
	let logo3 = format!(
		"{}{}{}┃┏┓┃ ┃┏┓┃ ┗┓┏┛ ┗┓┏┛ ┃┃  ┃┃━┫ ┃━━┫ ┃┏┓┃ ┏┓ ┃┏┓┃\r\n{}",
		padding,
		inner_padding,
		color::Fg(color::White),
		reset,
	);
	let logo4 = format!(
		"{}{}{}┃┗┛┃ ┃┏┓┃  ┃┗┓  ┃┗┓ ┃┗┓ ┃┃━┫ ┣━━┃ ┃┃┃┃ ┃┃ ┃┗┛┃\r\n{}",
		padding,
		inner_padding,
		color::Fg(color::Cyan),
		reset,
	);
	let logo5 = format!(
		"{}{}{}┗━━┛ ┗┛┗┛  ┗━┛  ┗━┛ ┗━┛ ┗━━┛ ┗━━┛ ┗┛┗┛ ┗┛ ┃┏━┛\r\n{}",
		padding,
		inner_padding,
		color::Fg(color::LightBlue),
		reset,
	);
	let logo6 = format!(
		"{}{}{}                                 {:>8} ┗┛{}",
		padding,
		inner_padding,
		color::Fg(color::LightBlue),
		config::VERSION,
		reset,
	);

	format!("{}{}{}{}{}{}\r\n\r\n", logo1, logo2, logo3, logo4, logo5, logo6)
}

pub fn get_score(board_me: config::Board, board_ai: config::Board, round: Round) -> String {
	let padding = get_padding();

	let score_me = match round {
		Round::One => String::from("--"),
		Round::Two => game::get_score(&board_ai),
	};

	let score_ai = match round {
		Round::One => String::from("--"),
		Round::Two => game::get_score(&board_me),
	};

	let user1 = "ME";
	let user2 = "AI";

	let inner_size = (config::SIZE_X * 3) + 5 - 15 - 7 - 2 - 3;
	let inner_padding = &format!("{:width$}", "", width = inner_size);

	format!(
		"{}{:<15}{inner_padding}{open}SCORE: {score_me}{close}   ║  {:<15}{inner_padding}{open}SCORE: {score_ai}{close}\r\n",
		padding,
		user1,
		user2,
		inner_padding=inner_padding,
		open=color::Fg(color::Magenta),
		close=color::Fg(color::White),
		score_me=score_me,
		score_ai=score_ai,
	)
}

// return one line of a board and interpret states to visual styles
fn get_board_row(
	board_row: &config::BoardRow,
	y: usize,
	pos_x: usize,
	pos_y: usize,
	cell: Cell,
	show_position: bool,
) -> String {
	let mut output = String::new();

	for (x, item) in board_row.iter().enumerate() {
		match (item, x, y) {
			(_, this_pos_x, this_pos_y) if this_pos_x == pos_x && this_pos_y == pos_y && show_position => {
				match (board_row[x], cell) {
					(Empty, Crosshair) => {
						output += &format!("{}{}{}", color::Fg(color::Green), config::CROSSHAIR, color::Fg(color::White))
					}
					(Empty, _) => output += &format!("{}{}{}", color::Fg(color::Green), config::SHIP, color::Fg(color::White)),
					(_, Crosshair) => {
						output += &format!("{}{}{}", color::Fg(color::Red), config::CROSSHAIR, color::Fg(color::White))
					}
					(_, _) => output += &format!("{}{}{}", color::Fg(color::Red), config::SHIP, color::Fg(color::White)),
				}
			}
			(Placeholder, _, _) => {
				output += &format!("{}{}{}", color::Fg(color::Green), config::SHIP, color::Fg(color::White))
			}
			(Shot, _, _) => output += config::SHOT,
			(Ship, _, _) | (ShipOne(_), _, _) | (ShipTwo(_), _, _) | (ShipThree(_), _, _) => output += config::SHIP,
			(Damage, _, _) => output += config::DAMAGE,
			(_, _, _) => match x % 2 {
				0 => output += &format!("{}{}{}", color::Fg(color::LightWhite), config::EMPTY, color::Fg(color::White)),
				_ => output += &format!("{}{}{}", color::Fg(color::LightBlack), config::EMPTY, color::Fg(color::White)),
			},
		}
	}

	output
}

pub fn get_board(
	board_me: &config::Board,
	board_ai: &config::Board,
	pos_x: usize,
	pos_y: usize,
	round: Round,
) -> String {
	let padding = get_padding();

	let mut coords = String::from(" ");
	for i in 1..=config::SIZE_X {
		coords += &format!(" {:^2}", i);
	}
	let coord_top = &format!(" {coords}   ║   {coords}", coords = coords);

	let frame_top = &format!("┌{:─^width$}┐", "", width = config::SIZE_X * 3);
	let top = &format!(" {frame_top}  ║   {frame_top}", frame_top = frame_top);

	let frame_bottom = &format!("└{:─^width$}┘", "", width = config::SIZE_X * 3);
	let bottom = &format!(" {frame_bottom}  ║   {frame_bottom}", frame_bottom = frame_bottom);
	let show_position = match round {
		Round::One => false,
		Round::Two => true,
	};

	let mut output = format!("{}{}{}\r\n{}{}\r\n", padding, color::Fg(color::White), coord_top, padding, top);
	for row in 0..config::SIZE_Y {
		output += &padding;
		output += &make_alphabetic(row);
		output += "│";
		output += &get_board_row(&board_me[row], row, pos_x, pos_y, Empty, false);
		output += "│  ║  ";
		output += &make_alphabetic(row);
		output += "│";
		output += &get_board_row(&board_ai[row], row, pos_x, pos_y, Crosshair, show_position);
		output += "│\r\n";
	}
	output += &padding;
	output += bottom;
	output += "\r\n\r\n";
	output += &format!("{}", color::Fg(color::White));

	output
}

fn make_alphabetic(y: usize) -> String {
	let alphabet = ('A'..='Z')
		.filter_map(|c| {
			let c = c as char;
			if c.is_alphabetic() {
				Some(c)
			} else {
				None
			}
		})
		.collect::<Vec<_>>();

	String::from(alphabet[y])
}

pub fn get_coord(pos_x: usize, pos_y: usize) -> String {
	format!("{}{}", make_alphabetic(pos_y), pos_x + 1)
}

#[test]
fn get_coord_works() {
	assert_eq!(get_coord(0, 0), String::from("A1"));
	assert_eq!(get_coord(1, 0), String::from("A2"));
	assert_eq!(get_coord(0, 1), String::from("B1"));
	assert_eq!(get_coord(9, 0), String::from("A10"));
	assert_eq!(get_coord(0, 9), String::from("J1"));
	assert_eq!(get_coord(9, 9), String::from("J10"));
}

pub fn get_round1_instructions() -> String {
	let padding = get_padding();
	let board_size = config::SIZE_X * 3 * 2;
	let inner_size = ((board_size + 11 - 71) / 2) as f32;
	let size = inner_size.floor() as usize;
	let inner_padding = &format!("{:width$}", "", width = size);

	format!(
		"\r\n{padding}{inner_padding}      {}PLACING ROUND - Place your ships strategically on your map{}\r\n\r\n{padding}{inner_padding} [←↑↓→] position ║ [r] rotate ║ [enter] place ║ [del] restart ║ [q] quit\r\n\r\n",
		color::Fg(color::Green),
		color::Fg(color::White),
		padding=padding,
		inner_padding=inner_padding
	)
}

pub fn get_round2_instructions() -> String {
	let padding = get_padding();
	format!(
		"\r\n{}{}   PLAY - Hit all your opponents ships and reach a score of 10 to win{}\r\n\r\n{}              [←↑↓→] position ║ [enter] shoot ║ [q] quit\r\n\r\n",
		padding,
		color::Fg(color::Green),
		color::Fg(color::White),
		padding,
	)
}

pub fn get_good_bye_msg(winner: bool) -> String {
	let padding = get_padding();

	let mut result = String::new();
	if winner {
		result += &format!("{}Congrats!\r\n", padding);
		result += &format!("{}", color::Fg(color::Green));
		result += &format!("{} ┏┓ ┏┓ ┏━━┓ ┏┓┏┓    ┏┓┏┓┏┓ ┏━━┓ ┏━┓\r\n", padding);
		result += &format!("{} ┃┗━┛┃ ┃┏┓┃ ┃┃┃┃    ┃┗┛┗┛┃ ┃┏┓┃ ┃┏┓┓\r\n", padding);
		result += &format!("{} ┗━┓┏┛ ┃┗┛┃ ┃┗┛┃    ┗┓┏┓┏┛ ┃┗┛┃ ┃┃┃┃\r\n", padding);
		result += &format!("{} ┗━━┛  ┗━━┛ ┗━━┛     ┗┛┗┛  ┗━━┛ ┗┛┗┛\r\n", padding);
		result += &format!("{}", color::Fg(color::White));
	} else {
		result += &format!("{}", color::Fg(color::Red));
		result += &format!("{}                    ┏┓             ┏┓\r\n", padding);
		result += &format!("{} ┏┓ ┏┓ ┏━━┓ ┏┓┏┓    ┃┃  ┏━━┓ ┏━━┓ ┏┛┗┓\r\n", padding);
		result += &format!("{} ┃┗━┛┃ ┃┏┓┃ ┃┃┃┃    ┃┃  ┃┏┓┃ ┃━━┫ ┗┓┏┛\r\n", padding);
		result += &format!("{} ┗━┓┏┛ ┃┗┛┃ ┃┗┛┃    ┃┗┓ ┃┗┛┃ ┣━━┃  ┃┗┓\r\n", padding);
		result += &format!("{} ┗━━┛  ┗━━┛ ┗━━┛    ┗━┛ ┗━━┛ ┗━━┛  ┗━┛\r\n", padding);
		result += &format!("{}", color::Fg(color::White));
		result += &format!("{}Try again soon.\r\n", padding);
	}

	result
}

pub fn draw(stdout: &mut dyn io::Write, score: String, board: String, history: String, instructions: String) {
	write!(
		stdout,
		"{}{}{}{}{}{}{}{}{}{}",
		termion::clear::AfterCursor,
		termion::cursor::Goto(1, 2),
		termion::color::Fg(termion::color::White),
		termion::cursor::Hide,
		get_header(),
		score,
		board,
		history,
		instructions,
		termion::cursor::Save
	)
	.unwrap();
	stdout.flush().unwrap();
}
