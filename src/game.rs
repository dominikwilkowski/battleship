use crate::Cell;

#[derive(Debug, PartialEq)]
pub enum HitType {
	Hit,
	HitNSunk,
	Miss,
}

pub fn get_score(board: &[[Cell; 10]; 10]) -> String {
	let mut score = 0;

	for row in board {
		for cell in row {
			if cell == &Cell::Damage {
				score += 1;
			}
		}
	}

	format!("{:0>2}", score)
}

#[test]
fn get_score_works() {
	let mut board = [[Cell::Empty; 10]; 10];
	assert_eq!(get_score(&board), String::from("00"));
	board[0][0] = Cell::ShipTwo([0, 0, 0, 0]);
	assert_eq!(get_score(&board), String::from("00"));
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

pub fn get_hit_type(
	board_damage: &[[Cell; 10]; 10],
	board_ships: &[[Cell; 10]; 10],
	pos_x: usize,
	pos_y: usize,
) -> HitType {
	let mut result = HitType::Miss;

	match board_ships[pos_y][pos_x] {
		Cell::ShipOne(_) => {
			result = HitType::HitNSunk;
		}
		Cell::ShipTwo(coords) => {
			if coords[0] == pos_x && coords[1] == pos_y {
				result = match board_damage[coords[3]][coords[2]] {
					Cell::Damage => HitType::HitNSunk,
					_ => HitType::Hit,
				};
			} else {
				result = match board_damage[coords[1]][coords[0]] {
					Cell::Damage => HitType::HitNSunk,
					_ => HitType::Hit,
				};
			}
		}
		Cell::ShipThree(coords) => {
			if coords[0] == pos_x && coords[1] == pos_y {
				result = match (board_damage[coords[3]][coords[2]], board_damage[coords[5]][coords[4]]) {
					(Cell::Damage, Cell::Damage) => HitType::HitNSunk,
					(_, _) => HitType::Hit,
				};
			} else if coords[2] == pos_x && coords[3] == pos_y {
				result = match (board_damage[coords[1]][coords[0]], board_damage[coords[5]][coords[4]]) {
					(Cell::Damage, Cell::Damage) => HitType::HitNSunk,
					(_, _) => HitType::Hit,
				};
			} else {
				result = match (board_damage[coords[1]][coords[0]], board_damage[coords[3]][coords[2]]) {
					(Cell::Damage, Cell::Damage) => HitType::HitNSunk,
					(_, _) => HitType::Hit,
				};
			}
		}
		_ => {}
	};

	result
}

#[test]
fn hit_type_works() {
	let mut board = [[Cell::Empty; 10]; 10];
	assert_eq!(get_hit_type(&board, &board, 0, 0), HitType::Miss);
	board[0][0] = Cell::ShipOne([0, 0]);
	assert_eq!(get_hit_type(&board, &board, 0, 0), HitType::HitNSunk);

	board[2][2] = Cell::ShipTwo([2, 2, 3, 2]);
	board[2][3] = Cell::ShipTwo([2, 2, 3, 2]);
	assert_eq!(get_hit_type(&board, &board, 2, 2), HitType::Hit);
	board[2][2] = Cell::Damage;
	assert_eq!(get_hit_type(&board, &board, 3, 2), HitType::HitNSunk);

	board[5][5] = Cell::ShipThree([5, 5, 5, 6, 5, 7]);
	board[6][5] = Cell::ShipThree([5, 5, 5, 6, 5, 7]);
	board[7][5] = Cell::ShipThree([5, 5, 5, 6, 5, 7]);
	assert_eq!(get_hit_type(&board, &board, 5, 6), HitType::Hit);
	board[6][5] = Cell::Damage;
	assert_eq!(get_hit_type(&board, &board, 5, 7), HitType::Hit);
	board[7][5] = Cell::Damage;
	assert_eq!(get_hit_type(&board, &board, 1, 1), HitType::Miss);
	assert_eq!(get_hit_type(&board, &board, 5, 5), HitType::HitNSunk);

	board = [[Cell::Empty; 10]; 10];
	let mut board_secret = [[Cell::Empty; 10]; 10];
	board_secret[5][5] = Cell::ShipThree([5, 5, 5, 6, 5, 7]);
	board_secret[6][5] = Cell::ShipThree([5, 5, 5, 6, 5, 7]);
	board_secret[7][5] = Cell::ShipThree([5, 5, 5, 6, 5, 7]);
	assert_eq!(get_hit_type(&board, &board_secret, 5, 6), HitType::Hit);
	board[6][5] = Cell::Damage;
	assert_eq!(get_hit_type(&board, &board_secret, 5, 7), HitType::Hit);
	board[7][5] = Cell::Damage;
	assert_eq!(get_hit_type(&board, &board_secret, 5, 5), HitType::HitNSunk);
}
