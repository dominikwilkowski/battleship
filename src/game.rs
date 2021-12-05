use crate::Cell;

pub fn get_score(board: &[[Cell; 10]; 10]) -> String {
	let mut score = 0;

	for row in board {
		for cell in row {
			match cell {
				Cell::ShipOne(_) | Cell::ShipTwo(_) | Cell::ShipThree(_) => {
					score += 1;
				}
				_ => {}
			}
		}
	}

	score = 10 - score;

	format!("{:0>2}", score)
}

#[test]
fn get_score_works() {
	let mut board = [[Cell::Empty; 10]; 10];
	assert_eq!(get_score(&board), String::from("10"));
	board[0][0] = Cell::Damage;
	assert_eq!(get_score(&board), String::from("10"));
	board[1][0] = Cell::ShipOne([0, 0]);
	board[1][1] = Cell::ShipOne([0, 0]);
	board[1][2] = Cell::ShipOne([0, 0]);
	board[1][3] = Cell::ShipOne([0, 0]);
	board[1][4] = Cell::ShipOne([0, 0]);
	board[1][5] = Cell::ShipOne([0, 0]);
	board[1][6] = Cell::ShipOne([0, 0]);
	board[1][7] = Cell::ShipOne([0, 0]);
	board[1][8] = Cell::ShipOne([0, 0]);
	board[1][9] = Cell::ShipOne([0, 0]);
	assert_eq!(get_score(&board), String::from("00"));
	board[1][0] = Cell::Damage;
	assert_eq!(get_score(&board), String::from("01"));
	board[1][1] = Cell::Damage;
	assert_eq!(get_score(&board), String::from("02"));
	board[1][2] = Cell::Damage;
	assert_eq!(get_score(&board), String::from("03"));
	board[1][3] = Cell::Damage;
	assert_eq!(get_score(&board), String::from("04"));
	board[1][4] = Cell::Damage;
	assert_eq!(get_score(&board), String::from("05"));
	board[1][5] = Cell::Damage;
	assert_eq!(get_score(&board), String::from("06"));
	board[1][6] = Cell::Damage;
	assert_eq!(get_score(&board), String::from("07"));
	board[1][7] = Cell::Damage;
	assert_eq!(get_score(&board), String::from("08"));
	board[1][8] = Cell::Damage;
	assert_eq!(get_score(&board), String::from("09"));
	board[1][9] = Cell::Damage;
	assert_eq!(get_score(&board), String::from("10"));
}
