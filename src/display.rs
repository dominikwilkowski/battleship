// return one line of a board and interpret states to visual styles
fn get_board_line(board: [u8; 10]) -> String {
	let mut output = String::new();

	for i in 0..10 {
		if board[i] == 0 {
			output += "░░░";
		} else if board[i] == 1 {
			output += " ◌ ";
		} else if board[i] == 2 {
			output += "▓▓▓";
		} else if board[i] == 3 {
			output += "╳╳╳";
		}
	}

	output
}

pub fn draw(board_me: [[u8; 10]; 10], board_ai: [[u8; 10]; 10]) {
	let logo = concat!(
		" ┏┓         ┏┓   ┏┓  ┏┓            ┏┓   ┏┓\n",
		" ┃┗━┓ ┏━━┓ ┏┛┗┓ ┏┛┗┓ ┃┃  ┏━━┓ ┏━━┓ ┃┗━┓ ┗┛ ┏━━┓\n",
		" ┃┏┓┃ ┃┏┓┃ ┗┓┏┛ ┗┓┏┛ ┃┃  ┃┃━┫ ┃━━┫ ┃┏┓┃ ┏┓ ┃┏┓┃\n",
		" ┃┗┛┃ ┃┏┓┃  ┃┗┓  ┃┗┓ ┃┗┓ ┃┃━┫ ┣━━┃ ┃┃┃┃ ┃┃ ┃┗┛┃\n",
		" ┗━━┛ ┗┛┗┛  ┗━┛  ┗━┛ ┗━┛ ┗━━┛ ┗━━┛ ┗┛┗┛ ┗┛ ┃┏━┛\n",
		"                                           ┗┛"
	);
	let names = "Me                                 ║  AI";
	let coord_top = "   1  2  3  4  5  6  7  8  9  10   ║     1  2  3  4  5  6  7  8  9  10";
	let frame_top = " ┌──────────────────────────────┐  ║   ┌──────────────────────────────┐";
	let coord_dict = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
	let frame_bottom = " └──────────────────────────────┘  ║   └──────────────────────────────┘";
	let space_bottom = "                                   ║";

	println!("{}\n\n{}\n{}\n{}", logo, names, coord_top, frame_top);
	for i in 0..10 {
		print!("{}│", coord_dict[i]);
		print!("{}", get_board_line(board_me[i]));
		print!("│  ║  {}│", coord_dict[i]);
		print!("{}", get_board_line(board_ai[i]));
		print!("│\n");
	}
	println!("{}\n{}", frame_bottom, space_bottom);
}
