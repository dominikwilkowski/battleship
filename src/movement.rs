use crate::Cell;
use crate::Direction;
use crate::Rotation;

use Cell::{Empty, Placeholder};

pub fn move_ship(
	mut board: [[Cell; 10]; 10],
	mut pos_x: usize,
	mut pos_y: usize,
	rotation: Rotation,
	ship_size: usize,
	direction: Direction,
) -> ([[Cell; 10]; 10], usize, usize) {
	match direction {
		Direction::Left => {
			if is_valid_move(&board, pos_x, pos_y, &rotation, &ship_size, &direction) {
				// clear previous position
				board = place_ship(board, pos_x, pos_y, &rotation, &ship_size, Empty);
				pos_x -= 1;
				// set new position
				board = place_ship(board, pos_x, pos_y, &rotation, &ship_size, Placeholder);
			}
		}
		Direction::Right => {
			if is_valid_move(&board, pos_x, pos_y, &rotation, &ship_size, &direction) {
				// clear previous position
				board = place_ship(board, pos_x, pos_y, &rotation, &ship_size, Empty);
				pos_x += 1;
				// set new position
				board = place_ship(board, pos_x, pos_y, &rotation, &ship_size, Placeholder);
			}
		}
		Direction::Up => {
			if is_valid_move(&board, pos_x, pos_y, &rotation, &ship_size, &direction) {
				// clear previous position
				board = place_ship(board, pos_x, pos_y, &rotation, &ship_size, Empty);
				pos_y -= 1;
				// set new position
				board = place_ship(board, pos_x, pos_y, &rotation, &ship_size, Placeholder);
			}
		}
		Direction::Down => {
			if is_valid_move(&board, pos_x, pos_y, &rotation, &ship_size, &direction) {
				// clear previous position
				board = place_ship(board, pos_x, pos_y, &rotation, &ship_size, Empty);
				pos_y += 1;
				// set new position
				board = place_ship(board, pos_x, pos_y, &rotation, &ship_size, Placeholder);
			}
		}
	};

	(board, pos_x, pos_y)
}

pub fn place_ship(
	mut board: [[Cell; 10]; 10],
	pos_x: usize,
	pos_y: usize,
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

pub fn is_valid_move(
	board: &[[Cell; 10]; 10],
	pos_x: usize,
	pos_y: usize,
	rotation: &Rotation,
	ship_size: &usize,
	direction: &Direction,
) -> bool {
	let mut result = true;

	match rotation {
		Rotation::Horizontal => match direction {
			Direction::Right => {
				if pos_x + *ship_size > 9 || board[pos_y][pos_x + *ship_size] != Empty {
					result = false;
				}
			}
			Direction::Left => {
				if pos_x == 0 || board[pos_y][pos_x - 1] != Empty {
					result = false;
				}
			}
			Direction::Up => {
				for offset in 0..*ship_size {
					if pos_y == 0 || pos_x + offset > 9 || board[pos_y - 1][pos_x + offset] != Empty {
						result = false;
						break;
					}
				}
			}
			Direction::Down => {
				for offset in 0..*ship_size {
					if pos_y + 1 > 9 || pos_x + offset > 9 || board[pos_y + 1][pos_x + offset] != Empty {
						result = false;
						break;
					}
				}
			}
		},
		Rotation::Vertical => match direction {
			Direction::Right => {
				for offset in 0..*ship_size {
					if pos_x + 1 > 9 || pos_y + offset > 9 || board[pos_y + offset][pos_x + 1] != Empty {
						result = false;
						break;
					}
				}
			}
			Direction::Left => {
				for offset in 0..*ship_size {
					if pos_x == 0 || pos_y + offset > 9 || board[pos_y + offset][pos_x - 1] != Empty {
						result = false;
						break;
					}
				}
			}
			Direction::Up => {
				if pos_y == 0 || board[pos_y - 1][pos_x] != Empty {
					result = false;
				}
			}
			Direction::Down => {
				if pos_y + *ship_size > 9 || board[pos_y + *ship_size][pos_x] != Empty {
					result = false;
				}
			}
		},
	}

	result
}

#[test]
fn is_valid_move_works() {
	let board = [[Empty; 10]; 10];

	use Direction::{Down, Left, Right, Up};
	use Rotation::{Horizontal, Vertical};

	for n in 0..10 {
		// HORIZONTAL
		// one_block
		assert_eq!(is_valid_move(&board, 0, n, &Horizontal, &1, &Right), true, "Size: 1, Horizontal, Right, Pos: 0,{}", n);
		assert_eq!(is_valid_move(&board, 1, n, &Horizontal, &1, &Right), true, "Size: 1, Horizontal, Right, Pos: 1,{}", n);
		assert_eq!(is_valid_move(&board, 2, n, &Horizontal, &1, &Right), true, "Size: 1, Horizontal, Right, Pos: 2,{}", n);
		assert_eq!(is_valid_move(&board, 7, n, &Horizontal, &1, &Right), true, "Size: 1, Horizontal, Right, Pos: 7,{}", n);
		assert_eq!(is_valid_move(&board, 8, n, &Horizontal, &1, &Right), true, "Size: 1, Horizontal, Right, Pos: 8,{}", n);
		assert_eq!(is_valid_move(&board, 9, n, &Horizontal, &1, &Right), false, "Size: 1, Horizontal, Right, Pos: 9,{}", n);

		assert_eq!(is_valid_move(&board, 0, n, &Horizontal, &1, &Left), false, "Size: 1, Horizontal, Left, Pos: 0,{}", n);
		assert_eq!(is_valid_move(&board, 1, n, &Horizontal, &1, &Left), true, "Size: 1, Horizontal, Left, Pos: 1,{}", n);
		assert_eq!(is_valid_move(&board, 2, n, &Horizontal, &1, &Left), true, "Size: 1, Horizontal, Left, Pos: 2,{}", n);
		assert_eq!(is_valid_move(&board, 7, n, &Horizontal, &1, &Left), true, "Size: 1, Horizontal, Left, Pos: 7,{}", n);
		assert_eq!(is_valid_move(&board, 8, n, &Horizontal, &1, &Left), true, "Size: 1, Horizontal, Left, Pos: 8,{}", n);
		assert_eq!(is_valid_move(&board, 9, n, &Horizontal, &1, &Left), true, "Size: 1, Horizontal, Left, Pos: 9,{}", n);

		assert_eq!(is_valid_move(&board, n, 0, &Horizontal, &1, &Up), false, "Size: 1, Horizontal, Up, Pos: {},0", n);
		assert_eq!(is_valid_move(&board, n, 1, &Horizontal, &1, &Up), true, "Size: 1, Horizontal, Up, Pos: {},1", n);
		assert_eq!(is_valid_move(&board, n, 2, &Horizontal, &1, &Up), true, "Size: 1, Horizontal, Up, Pos: {},2", n);
		assert_eq!(is_valid_move(&board, n, 7, &Horizontal, &1, &Up), true, "Size: 1, Horizontal, Up, Pos: {},7", n);
		assert_eq!(is_valid_move(&board, n, 8, &Horizontal, &1, &Up), true, "Size: 1, Horizontal, Up, Pos: {},8", n);
		assert_eq!(is_valid_move(&board, n, 9, &Horizontal, &1, &Up), true, "Size: 1, Horizontal, Up, Pos: {},9", n);

		assert_eq!(is_valid_move(&board, n, 0, &Horizontal, &1, &Down), true, "Size: 1, Horizontal, Down, Pos: {},0", n);
		assert_eq!(is_valid_move(&board, n, 1, &Horizontal, &1, &Down), true, "Size: 1, Horizontal, Down, Pos: {},1", n);
		assert_eq!(is_valid_move(&board, n, 2, &Horizontal, &1, &Down), true, "Size: 1, Horizontal, Down, Pos: {},2", n);
		assert_eq!(is_valid_move(&board, n, 7, &Horizontal, &1, &Down), true, "Size: 1, Horizontal, Down, Pos: {},7", n);
		assert_eq!(is_valid_move(&board, n, 8, &Horizontal, &1, &Down), true, "Size: 1, Horizontal, Down, Pos: {},8", n);
		assert_eq!(is_valid_move(&board, n, 9, &Horizontal, &1, &Down), false, "Size: 1, Horizontal, Down, Pos: {},9", n);

		// VERTICAL
		// one_block
		assert_eq!(is_valid_move(&board, 0, n, &Vertical, &1, &Right), true, "Size: 1, Vertical, Right, Pos: 0,{}", n);
		assert_eq!(is_valid_move(&board, 1, n, &Vertical, &1, &Right), true, "Size: 1, Vertical, Right, Pos: 1,{}", n);
		assert_eq!(is_valid_move(&board, 2, n, &Vertical, &1, &Right), true, "Size: 1, Vertical, Right, Pos: 2,{}", n);
		assert_eq!(is_valid_move(&board, 7, n, &Vertical, &1, &Right), true, "Size: 1, Vertical, Right, Pos: 7,{}", n);
		assert_eq!(is_valid_move(&board, 8, n, &Vertical, &1, &Right), true, "Size: 1, Vertical, Right, Pos: 8,{}", n);
		assert_eq!(is_valid_move(&board, 9, n, &Vertical, &1, &Right), false, "Size: 1, Vertical, Right, Pos: 9,{}", n);

		assert_eq!(is_valid_move(&board, 0, n, &Vertical, &1, &Left), false, "Size: 1, Vertical, Left, Pos: 0,{}", n);
		assert_eq!(is_valid_move(&board, 1, n, &Vertical, &1, &Left), true, "Size: 1, Vertical, Left, Pos: 1,{}", n);
		assert_eq!(is_valid_move(&board, 2, n, &Vertical, &1, &Left), true, "Size: 1, Vertical, Left, Pos: 2,{}", n);
		assert_eq!(is_valid_move(&board, 7, n, &Vertical, &1, &Left), true, "Size: 1, Vertical, Left, Pos: 7,{}", n);
		assert_eq!(is_valid_move(&board, 8, n, &Vertical, &1, &Left), true, "Size: 1, Vertical, Left, Pos: 8,{}", n);
		assert_eq!(is_valid_move(&board, 9, n, &Vertical, &1, &Left), true, "Size: 1, Vertical, Left, Pos: 9,{}", n);

		assert_eq!(is_valid_move(&board, n, 0, &Vertical, &1, &Up), false, "Size: 1, Vertical, Up, Pos: {},0", n);
		assert_eq!(is_valid_move(&board, n, 1, &Vertical, &1, &Up), true, "Size: 1, Vertical, Up, Pos: {},1", n);
		assert_eq!(is_valid_move(&board, n, 2, &Vertical, &1, &Up), true, "Size: 1, Vertical, Up, Pos: {},2", n);
		assert_eq!(is_valid_move(&board, n, 7, &Vertical, &1, &Up), true, "Size: 1, Vertical, Up, Pos: {},7", n);
		assert_eq!(is_valid_move(&board, n, 8, &Vertical, &1, &Up), true, "Size: 1, Vertical, Up, Pos: {},8", n);
		assert_eq!(is_valid_move(&board, n, 9, &Vertical, &1, &Up), true, "Size: 1, Vertical, Up, Pos: {},9", n);

		assert_eq!(is_valid_move(&board, n, 0, &Vertical, &1, &Down), true, "Size: 1, Vertical, Down, Pos: {},0", n);
		assert_eq!(is_valid_move(&board, n, 1, &Vertical, &1, &Down), true, "Size: 1, Vertical, Down, Pos: {},1", n);
		assert_eq!(is_valid_move(&board, n, 2, &Vertical, &1, &Down), true, "Size: 1, Vertical, Down, Pos: {},2", n);
		assert_eq!(is_valid_move(&board, n, 7, &Vertical, &1, &Down), true, "Size: 1, Vertical, Down, Pos: {},7", n);
		assert_eq!(is_valid_move(&board, n, 8, &Vertical, &1, &Down), true, "Size: 1, Vertical, Down, Pos: {},8", n);
		assert_eq!(is_valid_move(&board, n, 9, &Vertical, &1, &Down), false, "Size: 1, Vertical, Down, Pos: {},9", n);
	}

	// Horizontal
	// two_block
	assert_eq!(is_valid_move(&board, 0, 0, &Horizontal, &2, &Right), true, "Size: 2, Horizontal, Right, Pos: 0,0");
	assert_eq!(is_valid_move(&board, 1, 0, &Horizontal, &2, &Right), true, "Size: 2, Horizontal, Right, Pos: 1,0");
	assert_eq!(is_valid_move(&board, 2, 0, &Horizontal, &2, &Right), true, "Size: 2, Horizontal, Right, Pos: 2,0");
	assert_eq!(is_valid_move(&board, 7, 0, &Horizontal, &2, &Right), true, "Size: 2, Horizontal, Right, Pos: 7,0");
	assert_eq!(is_valid_move(&board, 8, 0, &Horizontal, &2, &Right), false, "Size: 2, Horizontal, Right, Pos: 8,0");
	assert_eq!(is_valid_move(&board, 9, 0, &Horizontal, &2, &Right), false, "Size: 2, Horizontal, Right, Pos: 9,0");

	assert_eq!(is_valid_move(&board, 0, 0, &Horizontal, &2, &Left), false, "Size: 2, Horizontal, Left, Pos: 0,0");
	assert_eq!(is_valid_move(&board, 1, 0, &Horizontal, &2, &Left), true, "Size: 2, Horizontal, Left, Pos: 1,0");
	assert_eq!(is_valid_move(&board, 2, 0, &Horizontal, &2, &Left), true, "Size: 2, Horizontal, Left, Pos: 2,0");
	assert_eq!(is_valid_move(&board, 7, 0, &Horizontal, &2, &Left), true, "Size: 2, Horizontal, Left, Pos: 7,0");
	assert_eq!(is_valid_move(&board, 8, 0, &Horizontal, &2, &Left), true, "Size: 2, Horizontal, Left, Pos: 8,0");
	assert_eq!(is_valid_move(&board, 9, 0, &Horizontal, &2, &Left), true, "Size: 2, Horizontal, Left, Pos: 9,0");

	assert_eq!(is_valid_move(&board, 0, 0, &Horizontal, &2, &Up), false, "Size: 2, Horizontal, Up, Pos: 0,0");
	assert_eq!(is_valid_move(&board, 0, 1, &Horizontal, &2, &Up), true, "Size: 2, Horizontal, Up, Pos: 0,1");
	assert_eq!(is_valid_move(&board, 0, 2, &Horizontal, &2, &Up), true, "Size: 2, Horizontal, Up, Pos: 0,2");
	assert_eq!(is_valid_move(&board, 0, 7, &Horizontal, &2, &Up), true, "Size: 2, Horizontal, Up, Pos: 0,7");
	assert_eq!(is_valid_move(&board, 0, 8, &Horizontal, &2, &Up), true, "Size: 2, Horizontal, Up, Pos: 0,8");
	assert_eq!(is_valid_move(&board, 0, 9, &Horizontal, &2, &Up), true, "Size: 2, Horizontal, Up, Pos: 0,9");

	assert_eq!(is_valid_move(&board, 0, 0, &Horizontal, &2, &Down), true, "Size: 2, Horizontal, Down, Pos: 0,0");
	assert_eq!(is_valid_move(&board, 0, 1, &Horizontal, &2, &Down), true, "Size: 2, Horizontal, Down, Pos: 0,1");
	assert_eq!(is_valid_move(&board, 0, 2, &Horizontal, &2, &Down), true, "Size: 2, Horizontal, Down, Pos: 0,2");
	assert_eq!(is_valid_move(&board, 0, 7, &Horizontal, &2, &Down), true, "Size: 2, Horizontal, Down, Pos: 0,7");
	assert_eq!(is_valid_move(&board, 0, 8, &Horizontal, &2, &Down), true, "Size: 2, Horizontal, Down, Pos: 0,8");
	assert_eq!(is_valid_move(&board, 0, 9, &Horizontal, &2, &Down), false, "Size: 2, Horizontal, Down, Pos: 0,9");

	// three_block
	assert_eq!(is_valid_move(&board, 0, 0, &Horizontal, &3, &Right), true, "Size: 3, Horizontal, Right, Pos: 0,0");
	assert_eq!(is_valid_move(&board, 1, 0, &Horizontal, &3, &Right), true, "Size: 3, Horizontal, Right, Pos: 1,0");
	assert_eq!(is_valid_move(&board, 2, 0, &Horizontal, &3, &Right), true, "Size: 3, Horizontal, Right, Pos: 2,0");
	assert_eq!(is_valid_move(&board, 7, 0, &Horizontal, &3, &Right), false, "Size: 3, Horizontal, Right, Pos: 7,0");
	assert_eq!(is_valid_move(&board, 8, 0, &Horizontal, &3, &Right), false, "Size: 3, Horizontal, Right, Pos: 8,0");
	assert_eq!(is_valid_move(&board, 9, 0, &Horizontal, &3, &Right), false, "Size: 3, Horizontal, Right, Pos: 9,0");

	assert_eq!(is_valid_move(&board, 0, 0, &Horizontal, &3, &Left), false, "Size: 3, Horizontal, Left, Pos: 0,0");
	assert_eq!(is_valid_move(&board, 1, 0, &Horizontal, &3, &Left), true, "Size: 3, Horizontal, Left, Pos: 1,0");
	assert_eq!(is_valid_move(&board, 2, 0, &Horizontal, &3, &Left), true, "Size: 3, Horizontal, Left, Pos: 2,0");
	assert_eq!(is_valid_move(&board, 7, 0, &Horizontal, &3, &Left), true, "Size: 3, Horizontal, Left, Pos: 7,0");
	assert_eq!(is_valid_move(&board, 8, 0, &Horizontal, &3, &Left), true, "Size: 3, Horizontal, Left, Pos: 8,0");
	assert_eq!(is_valid_move(&board, 9, 0, &Horizontal, &3, &Left), true, "Size: 3, Horizontal, Left, Pos: 9,0");

	assert_eq!(is_valid_move(&board, 0, 0, &Horizontal, &3, &Up), false, "Size: 3, Horizontal, Up, Pos: 0,0");
	assert_eq!(is_valid_move(&board, 0, 1, &Horizontal, &3, &Up), true, "Size: 3, Horizontal, Up, Pos: 0,1");
	assert_eq!(is_valid_move(&board, 0, 2, &Horizontal, &3, &Up), true, "Size: 3, Horizontal, Up, Pos: 0,2");
	assert_eq!(is_valid_move(&board, 0, 7, &Horizontal, &3, &Up), true, "Size: 3, Horizontal, Up, Pos: 0,7");
	assert_eq!(is_valid_move(&board, 0, 8, &Horizontal, &3, &Up), true, "Size: 3, Horizontal, Up, Pos: 0,8");
	assert_eq!(is_valid_move(&board, 0, 9, &Horizontal, &3, &Up), true, "Size: 3, Horizontal, Up, Pos: 0,9");

	assert_eq!(is_valid_move(&board, 0, 0, &Horizontal, &3, &Down), true, "Size: 3, Horizontal, Down, Pos: 0,0");
	assert_eq!(is_valid_move(&board, 0, 1, &Horizontal, &3, &Down), true, "Size: 3, Horizontal, Down, Pos: 0,1");
	assert_eq!(is_valid_move(&board, 0, 2, &Horizontal, &3, &Down), true, "Size: 3, Horizontal, Down, Pos: 0,2");
	assert_eq!(is_valid_move(&board, 0, 7, &Horizontal, &3, &Down), true, "Size: 3, Horizontal, Down, Pos: 0,7");
	assert_eq!(is_valid_move(&board, 0, 8, &Horizontal, &3, &Down), true, "Size: 3, Horizontal, Down, Pos: 0,8");
	assert_eq!(is_valid_move(&board, 0, 9, &Horizontal, &3, &Down), false, "Size: 3, Horizontal, Down, Pos: 0,9");

	// VERTICAL
	// two_block
	assert_eq!(is_valid_move(&board, 0, 0, &Vertical, &2, &Right), true, "Size: 2, Vertical, Right, Pos: 0,0");
	assert_eq!(is_valid_move(&board, 1, 0, &Vertical, &2, &Right), true, "Size: 2, Vertical, Right, Pos: 1,0");
	assert_eq!(is_valid_move(&board, 2, 0, &Vertical, &2, &Right), true, "Size: 2, Vertical, Right, Pos: 2,0");
	assert_eq!(is_valid_move(&board, 7, 0, &Vertical, &2, &Right), true, "Size: 2, Vertical, Right, Pos: 7,0");
	assert_eq!(is_valid_move(&board, 8, 0, &Vertical, &2, &Right), true, "Size: 2, Vertical, Right, Pos: 8,0");
	assert_eq!(is_valid_move(&board, 9, 0, &Vertical, &2, &Right), false, "Size: 2, Vertical, Right, Pos: 9,0");

	assert_eq!(is_valid_move(&board, 0, 0, &Vertical, &2, &Left), false, "Size: 2, Vertical, Left, Pos: 0,0");
	assert_eq!(is_valid_move(&board, 1, 0, &Vertical, &2, &Left), true, "Size: 2, Vertical, Left, Pos: 1,0");
	assert_eq!(is_valid_move(&board, 2, 0, &Vertical, &2, &Left), true, "Size: 2, Vertical, Left, Pos: 2,0");
	assert_eq!(is_valid_move(&board, 7, 0, &Vertical, &2, &Left), true, "Size: 2, Vertical, Left, Pos: 7,0");
	assert_eq!(is_valid_move(&board, 8, 0, &Vertical, &2, &Left), true, "Size: 2, Vertical, Left, Pos: 8,0");
	assert_eq!(is_valid_move(&board, 9, 0, &Vertical, &2, &Left), true, "Size: 2, Vertical, Left, Pos: 9,0");

	assert_eq!(is_valid_move(&board, 0, 0, &Vertical, &2, &Up), false, "Size: 2, Vertical, Up, Pos: 0,0");
	assert_eq!(is_valid_move(&board, 0, 1, &Vertical, &2, &Up), true, "Size: 2, Vertical, Up, Pos: 0,1");
	assert_eq!(is_valid_move(&board, 0, 2, &Vertical, &2, &Up), true, "Size: 2, Vertical, Up, Pos: 0,2");
	assert_eq!(is_valid_move(&board, 0, 7, &Vertical, &2, &Up), true, "Size: 2, Vertical, Up, Pos: 0,7");
	assert_eq!(is_valid_move(&board, 0, 8, &Vertical, &2, &Up), true, "Size: 2, Vertical, Up, Pos: 0,8");
	assert_eq!(is_valid_move(&board, 0, 9, &Vertical, &2, &Up), true, "Size: 2, Vertical, Up, Pos: 0,9");

	assert_eq!(is_valid_move(&board, 0, 0, &Vertical, &2, &Down), true, "Size: 2, Vertical, Down, Pos: 0,0");
	assert_eq!(is_valid_move(&board, 0, 1, &Vertical, &2, &Down), true, "Size: 2, Vertical, Down, Pos: 0,1");
	assert_eq!(is_valid_move(&board, 0, 2, &Vertical, &2, &Down), true, "Size: 2, Vertical, Down, Pos: 0,2");
	assert_eq!(is_valid_move(&board, 0, 7, &Vertical, &2, &Down), true, "Size: 2, Vertical, Down, Pos: 0,7");
	assert_eq!(is_valid_move(&board, 0, 8, &Vertical, &2, &Down), false, "Size: 2, Vertical, Down, Pos: 0,8");
	assert_eq!(is_valid_move(&board, 0, 9, &Vertical, &2, &Down), false, "Size: 2, Vertical, Down, Pos: 0,9");

	// three_block
	assert_eq!(is_valid_move(&board, 0, 0, &Vertical, &3, &Right), true, "Size: 3, Vertical, Right, Pos: 0,0");
	assert_eq!(is_valid_move(&board, 1, 0, &Vertical, &3, &Right), true, "Size: 3, Vertical, Right, Pos: 1,0");
	assert_eq!(is_valid_move(&board, 2, 0, &Vertical, &3, &Right), true, "Size: 3, Vertical, Right, Pos: 2,0");
	assert_eq!(is_valid_move(&board, 7, 0, &Vertical, &3, &Right), true, "Size: 3, Vertical, Right, Pos: 7,0");
	assert_eq!(is_valid_move(&board, 8, 0, &Vertical, &3, &Right), true, "Size: 3, Vertical, Right, Pos: 8,0");
	assert_eq!(is_valid_move(&board, 9, 0, &Vertical, &3, &Right), false, "Size: 3, Vertical, Right, Pos: 9,0");

	assert_eq!(is_valid_move(&board, 0, 0, &Vertical, &3, &Left), false, "Size: 3, Vertical, Left, Pos: 0,0");
	assert_eq!(is_valid_move(&board, 1, 0, &Vertical, &3, &Left), true, "Size: 3, Vertical, Left, Pos: 1,0");
	assert_eq!(is_valid_move(&board, 2, 0, &Vertical, &3, &Left), true, "Size: 3, Vertical, Left, Pos: 2,0");
	assert_eq!(is_valid_move(&board, 7, 0, &Vertical, &3, &Left), true, "Size: 3, Vertical, Left, Pos: 7,0");
	assert_eq!(is_valid_move(&board, 8, 0, &Vertical, &3, &Left), true, "Size: 3, Vertical, Left, Pos: 8,0");
	assert_eq!(is_valid_move(&board, 9, 0, &Vertical, &3, &Left), true, "Size: 3, Vertical, Left, Pos: 9,0");

	assert_eq!(is_valid_move(&board, 0, 0, &Vertical, &3, &Up), false, "Size: 3, Vertical, Up, Pos: 0,0");
	assert_eq!(is_valid_move(&board, 0, 1, &Vertical, &3, &Up), true, "Size: 3, Vertical, Up, Pos: 0,1");
	assert_eq!(is_valid_move(&board, 0, 2, &Vertical, &3, &Up), true, "Size: 3, Vertical, Up, Pos: 0,2");
	assert_eq!(is_valid_move(&board, 0, 7, &Vertical, &3, &Up), true, "Size: 3, Vertical, Up, Pos: 0,7");
	assert_eq!(is_valid_move(&board, 0, 8, &Vertical, &3, &Up), true, "Size: 3, Vertical, Up, Pos: 0,8");
	assert_eq!(is_valid_move(&board, 0, 9, &Vertical, &3, &Up), true, "Size: 3, Vertical, Up, Pos: 0,9");

	assert_eq!(is_valid_move(&board, 0, 0, &Vertical, &3, &Down), true, "Size: 3, Vertical, Down, Pos: 0,0");
	assert_eq!(is_valid_move(&board, 0, 1, &Vertical, &3, &Down), true, "Size: 3, Vertical, Down, Pos: 0,1");
	assert_eq!(is_valid_move(&board, 0, 2, &Vertical, &3, &Down), true, "Size: 3, Vertical, Down, Pos: 0,2");
	assert_eq!(is_valid_move(&board, 0, 7, &Vertical, &3, &Down), false, "Size: 3, Vertical, Down, Pos: 0,7");
	assert_eq!(is_valid_move(&board, 0, 8, &Vertical, &3, &Down), false, "Size: 3, Vertical, Down, Pos: 0,8");
	assert_eq!(is_valid_move(&board, 0, 9, &Vertical, &3, &Down), false, "Size: 3, Vertical, Down, Pos: 0,9");
}

#[test]
fn is_valid_move_detects_collisions() {
	use Direction::{Down, Left, Right, Up};
	use Rotation::{Horizontal, Vertical};

	let mut board = [
		[Empty; 10],
		[
			Empty,
			Cell::Ship,
			Empty,
			Empty,
			Empty,
			Empty,
			Empty,
			Empty,
			Empty,
			Empty,
		],
		[Empty; 10],
		[Empty; 10],
		[Empty; 10],
		[
			Empty,
			Empty,
			Empty,
			Empty,
			Empty,
			Cell::Ship,
			Empty,
			Empty,
			Empty,
			Empty,
		],
		[Empty; 10],
		[Empty; 10],
		[Empty; 10],
		[Empty; 10],
	];
	assert_eq!(is_valid_move(&board, 0, 0, &Vertical, &1, &Down), true, "Should not find a collision");
	assert_eq!(is_valid_move(&board, 0, 0, &Vertical, &1, &Right), true, "Should not find a collision");

	assert_eq!(is_valid_move(&board, 1, 0, &Vertical, &1, &Down), false, "Should find a collision");
	assert_eq!(is_valid_move(&board, 2, 1, &Vertical, &1, &Left), false, "Should find a collision");
	assert_eq!(is_valid_move(&board, 1, 2, &Vertical, &1, &Up), false, "Should find a collision");
	assert_eq!(is_valid_move(&board, 0, 1, &Vertical, &1, &Right), false, "Should find a collision");

	assert_eq!(is_valid_move(&board, 0, 0, &Horizontal, &2, &Down), false, "Should find a collision");
	assert_eq!(is_valid_move(&board, 1, 0, &Horizontal, &2, &Down), false, "Should find a collision");
	assert_eq!(is_valid_move(&board, 2, 0, &Horizontal, &2, &Down), true, "Should not find a collision");
	assert_eq!(is_valid_move(&board, 2, 1, &Horizontal, &2, &Left), false, "Should find a collision");
	assert_eq!(is_valid_move(&board, 1, 2, &Horizontal, &2, &Up), false, "Should find a collision");
	assert_eq!(is_valid_move(&board, 0, 2, &Horizontal, &2, &Up), false, "Should find a collision");
	assert_eq!(is_valid_move(&board, 3, 5, &Horizontal, &2, &Right), false, "Should find a collision");

	assert_eq!(is_valid_move(&board, 5, 3, &Vertical, &2, &Down), false, "Should find a collision");
	assert_eq!(is_valid_move(&board, 5, 6, &Vertical, &2, &Up), false, "Should find a collision");
	assert_eq!(is_valid_move(&board, 4, 4, &Vertical, &2, &Right), false, "Should find a collision");
	assert_eq!(is_valid_move(&board, 4, 5, &Vertical, &2, &Right), false, "Should find a collision");
	assert_eq!(is_valid_move(&board, 6, 4, &Vertical, &2, &Left), false, "Should find a collision");
	assert_eq!(is_valid_move(&board, 6, 5, &Vertical, &2, &Left), false, "Should find a collision");
}
