use crate::Cell;
use Cell::{Damage, Empty, Ship, Shot};

enum Board {
	Human,
	Ai,
}

// return one line of a board and interpret states to visual styles
fn get_board_line(board: &[Cell; 10], hide_ships: Board) -> String {
	let mut output = String::new();

	for i in 0..10 {
		match board[i] {
			Empty => output += "░░░",
			Shot => output += " ◌ ",
			Ship => {
				match hide_ships {
					// we hide ships if we're looking at the AIs board
					Board::Human => output += "▓▓▓",
					Board::Ai => output += "░░░",
				}
			}
			Damage => output += "╳╳╳",
		}
	}

	output
}

pub fn draw(board_me: [[Cell; 10]; 10], board_ai: [[Cell; 10]; 10]) -> String {
	let logo = concat!(
		" ┏┓         ┏┓   ┏┓  ┏┓            ┏┓   ┏┓\r\n",
		" ┃┗━┓ ┏━━┓ ┏┛┗┓ ┏┛┗┓ ┃┃  ┏━━┓ ┏━━┓ ┃┗━┓ ┗┛ ┏━━┓\r\n",
		" ┃┏┓┃ ┃┏┓┃ ┗┓┏┛ ┗┓┏┛ ┃┃  ┃┃━┫ ┃━━┫ ┃┏┓┃ ┏┓ ┃┏┓┃\r\n",
		" ┃┗┛┃ ┃┏┓┃  ┃┗┓  ┃┗┓ ┃┗┓ ┃┃━┫ ┣━━┃ ┃┃┃┃ ┃┃ ┃┗┛┃\r\n",
		" ┗━━┛ ┗┛┗┛  ┗━┛  ┗━┛ ┗━┛ ┗━━┛ ┗━━┛ ┗┛┗┛ ┗┛ ┃┏━┛\r\n",
		"                                           ┗┛"
	);
	let names = "Me                                 ║  AI";
	let coord_top = "   1  2  3  4  5  6  7  8  9  10   ║     1  2  3  4  5  6  7  8  9  10";
	let frame_top = " ┌──────────────────────────────┐  ║   ┌──────────────────────────────┐";
	let coord_dict = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
	let frame_bottom = " └──────────────────────────────┘  ║   └──────────────────────────────┘";
	let space_bottom = "                                   ║";

	let mut output = format!("{}\r\n\r\n{}\r\n{}\r\n{}\r\n", logo, names, coord_top, frame_top);
	for i in 0..10 {
		output += coord_dict[i];
		output += "│";
		output += &get_board_line(&board_me[i], Board::Human);
		output += "│  ║  ";
		output += coord_dict[i];
		output += "│";
		output += &get_board_line(&board_ai[i], Board::Ai);
		output += "│\r\n";
	}
	output += frame_bottom;
	output += "\r\n";
	output += space_bottom;

	output
}
