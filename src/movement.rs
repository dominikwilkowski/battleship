use crate::Cell;
use crate::Direction;
use crate::Rotation;

use Cell::{Crosshair, Empty, Placeholder, ShipOne, ShipThree, ShipTwo};

pub fn move_ship(
	mut board: [[Cell; 10]; 10],
	mut pos_x: usize,
	mut pos_y: usize,
	rotation: &Rotation,
	ship_size: &usize,
	direction: Direction,
) -> ([[Cell; 10]; 10], usize, usize) {
	match direction {
		Direction::Left => {
			if is_free_space(&board, pos_x as isize - 1, pos_y as isize, ship_size, rotation) {
				// clear previous position
				board = place_entity(board, pos_x, pos_y, &ship_size, &rotation, Empty);
				pos_x -= 1;
				// set new position
				board = place_entity(board, pos_x, pos_y, &ship_size, &rotation, Placeholder);
			}
		}
		Direction::Right => {
			if is_free_space(&board, pos_x as isize + 1, pos_y as isize, ship_size, rotation) {
				// clear previous position
				board = place_entity(board, pos_x, pos_y, &ship_size, &rotation, Empty);
				pos_x += 1;
				// set new position
				board = place_entity(board, pos_x, pos_y, &ship_size, &rotation, Placeholder);
			}
		}
		Direction::Up => {
			if is_free_space(&board, pos_x as isize, pos_y as isize - 1, ship_size, rotation) {
				// clear previous position
				board = place_entity(board, pos_x, pos_y, &ship_size, &rotation, Empty);
				pos_y -= 1;
				// set new position
				board = place_entity(board, pos_x, pos_y, &ship_size, &rotation, Placeholder);
			}
		}
		Direction::Down => {
			if is_free_space(&board, pos_x as isize, pos_y as isize + 1, ship_size, rotation) {
				// clear previous position
				board = place_entity(board, pos_x, pos_y, &ship_size, &rotation, Empty);
				pos_y += 1;
				// set new position
				board = place_entity(board, pos_x, pos_y, &ship_size, &rotation, Placeholder);
			}
		}
	};

	(board, pos_x, pos_y)
}

#[test]
fn move_ship_works() {
	let mut board = [[Empty; 10]; 10];
	board[0][0] = Placeholder;
	let mut result = move_ship(board, 0, 0, &Rotation::Horizontal, &1, Direction::Right);
	board = [[Empty; 10]; 10];
	board[0][1] = Placeholder;
	assert_eq!(result, (board, 1, 0));

	board = [[Empty; 10]; 10];
	board[0][0] = Placeholder;
	result = move_ship(board, 0, 0, &Rotation::Horizontal, &1, Direction::Left);
	assert_eq!(result, (board, 0, 0));

	board = [[Empty; 10]; 10];
	board[0][0] = Placeholder;
	result = move_ship(board, 0, 0, &Rotation::Horizontal, &1, Direction::Up);
	assert_eq!(result, (board, 0, 0));

	board = [[Empty; 10]; 10];
	board[0][0] = Placeholder;
	result = move_ship(board, 0, 0, &Rotation::Horizontal, &1, Direction::Down);
	board = [[Empty; 10]; 10];
	board[1][0] = Placeholder;
	assert_eq!(result, (board, 0, 1));

	board = [[Empty; 10]; 10];
	board[5][5] = Placeholder;
	result = move_ship(board, 5, 5, &Rotation::Horizontal, &1, Direction::Right);
	board = [[Empty; 10]; 10];
	board[5][6] = Placeholder;
	assert_eq!(result, (board, 6, 5));

	board = [[Empty; 10]; 10];
	board[5][5] = Placeholder;
	result = move_ship(board, 5, 5, &Rotation::Horizontal, &1, Direction::Left);
	board = [[Empty; 10]; 10];
	board[5][4] = Placeholder;
	assert_eq!(result, (board, 4, 5));

	board = [[Empty; 10]; 10];
	board[5][5] = Placeholder;
	result = move_ship(board, 5, 5, &Rotation::Horizontal, &1, Direction::Up);
	board = [[Empty; 10]; 10];
	board[4][5] = Placeholder;
	assert_eq!(result, (board, 5, 4));

	board = [[Empty; 10]; 10];
	board[5][5] = Placeholder;
	result = move_ship(board, 5, 5, &Rotation::Horizontal, &1, Direction::Down);
	board = [[Empty; 10]; 10];
	board[6][5] = Placeholder;
	assert_eq!(result, (board, 5, 6));

	board = [[Empty; 10]; 10];
	board[9][9] = Placeholder;
	result = move_ship(board, 9, 9, &Rotation::Horizontal, &1, Direction::Right);
	assert_eq!(result, (board, 9, 9));

	board = [[Empty; 10]; 10];
	board[9][9] = Placeholder;
	result = move_ship(board, 9, 9, &Rotation::Horizontal, &1, Direction::Left);
	board = [[Empty; 10]; 10];
	board[9][8] = Placeholder;
	assert_eq!(result, (board, 8, 9));

	board = [[Empty; 10]; 10];
	board[9][9] = Placeholder;
	result = move_ship(board, 9, 9, &Rotation::Horizontal, &1, Direction::Up);
	board = [[Empty; 10]; 10];
	board[8][9] = Placeholder;
	assert_eq!(result, (board, 9, 8));

	board = [[Empty; 10]; 10];
	board[9][9] = Placeholder;
	result = move_ship(board, 9, 9, &Rotation::Horizontal, &1, Direction::Down);
	assert_eq!(result, (board, 9, 9));
}

pub fn move_crosshair(
	mut board: [[Cell; 10]; 10],
	mut pos_x: usize,
	mut pos_y: usize,
	direction: Direction,
) -> ([[Cell; 10]; 10], usize, usize) {
	match direction {
		Direction::Left => {
			if is_free_space(&board, pos_x as isize - 1, pos_y as isize, &1, &Rotation::Horizontal) {
				// clear previous position
				board = place_entity(board, pos_x, pos_y, &1, &Rotation::Horizontal, Empty);
				pos_x -= 1;
				// set new position
				board = place_entity(board, pos_x, pos_y, &1, &Rotation::Horizontal, Crosshair);
			}
		}
		Direction::Right => {
			if is_free_space(&board, pos_x as isize + 1, pos_y as isize, &1, &Rotation::Horizontal) {
				// clear previous position
				board = place_entity(board, pos_x, pos_y, &1, &Rotation::Horizontal, Empty);
				pos_x += 1;
				// set new position
				board = place_entity(board, pos_x, pos_y, &1, &Rotation::Horizontal, Crosshair);
			}
		}
		Direction::Up => {
			if is_free_space(&board, pos_x as isize, pos_y as isize - 1, &1, &Rotation::Horizontal) {
				// clear previous position
				board = place_entity(board, pos_x, pos_y, &1, &Rotation::Horizontal, Empty);
				pos_y -= 1;
				// set new position
				board = place_entity(board, pos_x, pos_y, &1, &Rotation::Horizontal, Crosshair);
			}
		}
		Direction::Down => {
			if is_free_space(&board, pos_x as isize, pos_y as isize + 1, &1, &Rotation::Horizontal) {
				// clear previous position
				board = place_entity(board, pos_x, pos_y, &1, &Rotation::Horizontal, Empty);
				pos_y += 1;
				// set new position
				board = place_entity(board, pos_x, pos_y, &1, &Rotation::Horizontal, Crosshair);
			}
		}
	};

	(board, pos_x, pos_y)
}

#[test]
fn move_crosshair_works() {
	let mut board = [[Empty; 10]; 10];
	board[0][0] = Crosshair;
	let mut result = move_crosshair(board, 0, 0, Direction::Right);
	board = [[Empty; 10]; 10];
	board[0][1] = Crosshair;
	assert_eq!(result, (board, 1, 0));

	board = [[Empty; 10]; 10];
	board[0][0] = Crosshair;
	result = move_crosshair(board, 0, 0, Direction::Left);
	assert_eq!(result, (board, 0, 0));

	board = [[Empty; 10]; 10];
	board[0][0] = Crosshair;
	result = move_crosshair(board, 0, 0, Direction::Up);
	assert_eq!(result, (board, 0, 0));

	board = [[Empty; 10]; 10];
	board[0][0] = Crosshair;
	result = move_crosshair(board, 0, 0, Direction::Down);
	board = [[Empty; 10]; 10];
	board[1][0] = Crosshair;
	assert_eq!(result, (board, 0, 1));

	board = [[Empty; 10]; 10];
	board[5][5] = Crosshair;
	result = move_crosshair(board, 5, 5, Direction::Right);
	board = [[Empty; 10]; 10];
	board[5][6] = Crosshair;
	assert_eq!(result, (board, 6, 5));

	board = [[Empty; 10]; 10];
	board[5][5] = Crosshair;
	result = move_crosshair(board, 5, 5, Direction::Left);
	board = [[Empty; 10]; 10];
	board[5][4] = Crosshair;
	assert_eq!(result, (board, 4, 5));

	board = [[Empty; 10]; 10];
	board[5][5] = Crosshair;
	result = move_crosshair(board, 5, 5, Direction::Up);
	board = [[Empty; 10]; 10];
	board[4][5] = Crosshair;
	assert_eq!(result, (board, 5, 4));

	board = [[Empty; 10]; 10];
	board[5][5] = Crosshair;
	result = move_crosshair(board, 5, 5, Direction::Down);
	board = [[Empty; 10]; 10];
	board[6][5] = Crosshair;
	assert_eq!(result, (board, 5, 6));

	board = [[Empty; 10]; 10];
	board[9][9] = Crosshair;
	result = move_crosshair(board, 9, 9, Direction::Right);
	assert_eq!(result, (board, 9, 9));

	board = [[Empty; 10]; 10];
	board[9][9] = Crosshair;
	result = move_crosshair(board, 9, 9, Direction::Left);
	board = [[Empty; 10]; 10];
	board[9][8] = Crosshair;
	assert_eq!(result, (board, 8, 9));

	board = [[Empty; 10]; 10];
	board[9][9] = Crosshair;
	result = move_crosshair(board, 9, 9, Direction::Up);
	board = [[Empty; 10]; 10];
	board[8][9] = Crosshair;
	assert_eq!(result, (board, 9, 8));

	board = [[Empty; 10]; 10];
	board[9][9] = Crosshair;
	result = move_crosshair(board, 9, 9, Direction::Down);
	assert_eq!(result, (board, 9, 9));
}

pub fn place_entity(
	mut board: [[Cell; 10]; 10],
	pos_x: usize,
	pos_y: usize,
	ship_size: &usize,
	rotation: &Rotation,
	cell: Cell,
) -> [[Cell; 10]; 10] {
	let mut coords: Vec<u8> = vec![];

	match rotation {
		Rotation::Horizontal => {
			for offset in 0..*ship_size {
				coords.push(pos_x as u8 + offset as u8);
				coords.push(pos_y as u8);
				// board[pos_y][pos_x + offset] = cell;
			}
		}
		Rotation::Vertical => {
			for offset in 0..*ship_size {
				coords.push(pos_x as u8);
				coords.push(pos_y as u8 + offset as u8);
				// board[pos_y + offset][pos_x] = cell;
			}
		}
	}

	let mut i = 0;
	while i < coords.len() {
		let x = coords[i] as usize;
		i += 1;
		let y = coords[i] as usize;
		i += 1;

		board[y][x] = match cell {
			Cell::Ship => match *ship_size {
				1 => ShipOne([coords[0] as usize, coords[1] as usize]),
				2 => ShipTwo([
					coords[0] as usize,
					coords[1] as usize,
					coords[2] as usize,
					coords[3] as usize,
				]),
				_ => ShipThree([
					coords[0] as usize,
					coords[1] as usize,
					coords[2] as usize,
					coords[3] as usize,
					coords[4] as usize,
					coords[5] as usize,
				]),
			},
			_ => cell,
		};
	}

	board
}

#[test]
fn place_entity_works() {
	let mut result = place_entity([[Empty; 10]; 10], 0, 0, &1, &Rotation::Horizontal, Placeholder);
	let mut board = [[Empty; 10]; 10];
	board[0][0] = Placeholder;
	assert_eq!(result, board);

	result = place_entity([[Empty; 10]; 10], 0, 0, &2, &Rotation::Horizontal, Placeholder);
	board = [[Empty; 10]; 10];
	board[0][0] = Placeholder;
	board[0][1] = Placeholder;
	assert_eq!(result, board);

	result = place_entity([[Empty; 10]; 10], 0, 0, &3, &Rotation::Horizontal, Placeholder);
	board = [[Empty; 10]; 10];
	board[0][0] = Placeholder;
	board[0][1] = Placeholder;
	board[0][2] = Placeholder;
	assert_eq!(result, board);

	result = place_entity([[Empty; 10]; 10], 0, 0, &1, &Rotation::Vertical, Placeholder);
	board = [[Empty; 10]; 10];
	board[0][0] = Placeholder;
	assert_eq!(result, board);

	result = place_entity([[Empty; 10]; 10], 0, 0, &2, &Rotation::Vertical, Placeholder);
	board = [[Empty; 10]; 10];
	board[0][0] = Placeholder;
	board[1][0] = Placeholder;
	assert_eq!(result, board);

	result = place_entity([[Empty; 10]; 10], 0, 0, &3, &Rotation::Vertical, Placeholder);
	board = [[Empty; 10]; 10];
	board[0][0] = Placeholder;
	board[1][0] = Placeholder;
	board[2][0] = Placeholder;
	assert_eq!(result, board);
}

pub fn is_free_space(
	board: &[[Cell; 10]; 10],
	temp_pos_x: isize,
	temp_pos_y: isize,
	ship_size: &usize,
	rotation: &Rotation,
) -> bool {
	let mut result = true;

	if temp_pos_x < 0 || temp_pos_y < 0 {
		return false;
	}

	let pos_x: usize = temp_pos_x as usize;
	let pos_y: usize = temp_pos_y as usize;

	match rotation {
		Rotation::Horizontal => {
			for offset in 0..*ship_size {
				let new_pos_x = pos_x + offset;
				if new_pos_x > 9 || pos_y > 9 || board[pos_y][new_pos_x] != Empty && board[pos_y][new_pos_x] != Placeholder {
					result = false;
					break;
				}
			}
		}
		Rotation::Vertical => {
			for offset in 0..*ship_size {
				let new_pos_y = pos_y + offset;
				if pos_x > 9 || new_pos_y > 9 || board[new_pos_y][pos_x] != Empty && board[new_pos_y][pos_x] != Placeholder {
					result = false;
					break;
				}
			}
		}
	};

	result
}

#[test]
fn is_free_space_works() {
	let mut board = [[Empty; 10]; 10];
	board[1][0] = Cell::Ship;
	board[2][1] = Cell::Ship;
	assert_eq!(is_free_space(&board, 0, 0, &2, &Rotation::Vertical), false);
	assert_eq!(is_free_space(&board, 0, 0, &3, &Rotation::Vertical), false);
	assert_eq!(is_free_space(&board, 1, 0, &3, &Rotation::Vertical), false);
	assert_eq!(is_free_space(&board, 0, 2, &2, &Rotation::Horizontal), false);
	assert_eq!(is_free_space(&board, 0, 2, &3, &Rotation::Horizontal), false);

	assert_eq!(is_free_space(&board, 0, 0, &1, &Rotation::Vertical), true);
	assert_eq!(is_free_space(&board, 1, 0, &2, &Rotation::Vertical), true);
	assert_eq!(is_free_space(&board, 2, 0, &3, &Rotation::Vertical), true);
	assert_eq!(is_free_space(&board, 0, 2, &1, &Rotation::Vertical), true);

	board = [[Empty; 10]; 10];
	for x in 0..10 {
		for y in 0..10 {
			assert_eq!(is_free_space(&board, x, y, &1, &Rotation::Vertical), true);
			assert_eq!(is_free_space(&board, x, y, &1, &Rotation::Horizontal), true);
		}
	}
}

pub fn get_next_available_coordinates(
	board: &[[Cell; 10]; 10],
	ship_size: &usize,
	rotation: &Rotation,
) -> (usize, usize) {
	let mut o = 0;
	let mut pos_x = 0;
	let mut pos_y = 0;

	'outer: for i in 0..10 {
		for n in 0..i {
			if is_free_space(&board, o as isize, n as isize, ship_size, rotation) {
				pos_x = o as usize;
				pos_y = n as usize;
				break 'outer;
			}

			if is_free_space(&board, n as isize, o as isize, ship_size, rotation) {
				pos_x = n as usize;
				pos_y = o as usize;
				break 'outer;
			}
		}

		if is_free_space(&board, o as isize, o as isize, ship_size, rotation) {
			pos_x = o as usize;
			pos_y = o as usize;
			break;
		}
		o += 1;
	}

	(pos_x, pos_y)
}

#[test]
fn get_next_available_coordinates_works() {
	let mut board = [[Empty; 10]; 10];
	assert_eq!(get_next_available_coordinates(&board, &1, &Rotation::Vertical), (0, 0));

	board[0][0] = Cell::Ship;
	assert_eq!(get_next_available_coordinates(&board, &1, &Rotation::Vertical), (1, 0));

	board[0][1] = Cell::Ship;
	assert_eq!(get_next_available_coordinates(&board, &1, &Rotation::Vertical), (0, 1));

	board[1][0] = Cell::Ship;
	assert_eq!(get_next_available_coordinates(&board, &1, &Rotation::Vertical), (1, 1));

	board = [[Empty; 10]; 10];
	board[1][0] = Cell::Ship;
	assert_eq!(get_next_available_coordinates(&board, &2, &Rotation::Vertical), (1, 0));
}
