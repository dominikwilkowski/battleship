extern crate rand;

use crate::config;
use crate::game;
use crate::movement;
use crate::ships;
use crate::Cell;
use crate::Rotation;

use rand::Rng;
use ships::ShipTracker;

pub fn set_ships(mut board: config::Board) -> config::Board {
	let mut ships =
		ShipTracker::new(config::SHIP_ONE_BLOCK_AMOUNT, config::SHIP_TWO_BLOCK_AMOUNT, config::SHIP_THREE_BLOCK_AMOUNT);
	let (one_block, two_block, three_block) = ships.get_ships();
	let mut remaining_ships = one_block + two_block + three_block;
	let mut this_ship = ships.get_next_unset_ship().unwrap();
	let mut ship_size = config::get_entitie_size(&this_ship);

	while remaining_ships > 0 {
		let pos_x: usize = rand::thread_rng().gen_range(0..config::SIZE_X);
		let pos_y: usize = rand::thread_rng().gen_range(0..config::SIZE_Y);
		let rotation = match rand::thread_rng().gen_range(0..=1) {
			0 => Rotation::Horizontal,
			_ => Rotation::Vertical,
		};

		if movement::is_free_space(&board, pos_x as isize, pos_y as isize, ship_size, &rotation) {
			board = movement::place_entity(board, pos_x, pos_y, ship_size, &rotation, Cell::Ship);
			ships.set_ship(&this_ship);
			match ships.get_next_unset_ship() {
				Some(kind) => {
					this_ship = kind;
					ship_size = config::get_entitie_size(&this_ship);
				}
				None => {
					remaining_ships = 0;
				}
			};
		}
	}

	board
}

#[test]
fn set_ships_works() {
	let mut ships = 0;
	let board = set_ships([[Cell::Empty; config::SIZE_X]; config::SIZE_Y]);
	for row in board {
		for cell in row {
			match cell {
				Cell::ShipOne(_) | Cell::ShipTwo(_) | Cell::ShipThree(_) => {
					ships += 1;
				}
				_ => {}
			}
		}
	}

	let amount =
		config::SHIP_ONE_BLOCK_AMOUNT + (config::SHIP_TWO_BLOCK_AMOUNT * 2) + (config::SHIP_THREE_BLOCK_AMOUNT * 3);
	assert_eq!(ships, amount);
}

pub struct Attack {
	history: Vec<(usize, usize, game::HitType)>,
	todo: Vec<(usize, usize)>,
}

impl Attack {
	pub fn new() -> Self {
		Self {
			history: vec![(0, 0, game::HitType::Miss), (0, 0, game::HitType::Miss)],
			todo: vec![],
		}
	}

	pub fn shoot(&mut self, board: &config::Board) -> (usize, usize) {
		let mut pos_x: usize = 0;
		let mut pos_y: usize = 0;

		if !self.todo.is_empty() {
			let (x, y) = self.todo.pop().unwrap();
			pos_x = x;
			pos_y = y;
		} else {
			let mut valid_shot = false;

			while !valid_shot {
				pos_x = rand::thread_rng().gen_range(0..config::SIZE_X);
				pos_y = rand::thread_rng().gen_range(0..config::SIZE_Y);

				if board[pos_y][pos_x] != Cell::Shot && board[pos_y][pos_x] != Cell::Damage {
					valid_shot = true;
				}
			}
		}

		self.history.push((pos_x, pos_y, game::get_hit_type(board, board, pos_x, pos_y)));

		(pos_x, pos_y)
	}

	pub fn shoot_after_hit(&mut self, board: &config::Board) -> (usize, usize) {
		let mut possible_shots: Vec<[usize; 2]> = vec![];

		let (last_x, last_y, _) = &self.history[self.history.len() - 1];
		let (before_x, before_y, before_hit) = &self.history[self.history.len() - 2];

		let max_x = config::SIZE_X - 2;
		let max_y = config::SIZE_Y - 2;

		// we know last_hit was a HitType::Hit so we check if there is a direction already apparent
		if before_hit == &game::HitType::Hit {
			if *before_x == *last_x {
				if *last_y < max_y && board[*last_y + 1][*last_x] != Cell::Shot && board[*last_y + 1][*last_x] != Cell::Damage {
					possible_shots.push([*last_x, *last_y + 1]);
				}
				if *last_y > 0 && board[*last_y - 1][*last_x] != Cell::Shot && board[*last_y - 1][*last_x] != Cell::Damage {
					possible_shots.push([*last_x, *last_y - 1]);
				}
				if *before_y < max_y
					&& board[*before_y + 1][*last_x] != Cell::Shot
					&& board[*before_y + 1][*last_x] != Cell::Damage
				{
					possible_shots.push([*last_x, *before_y + 1]);
				}
				if *before_y > 0 && board[*before_y - 1][*last_x] != Cell::Shot && board[*before_y - 1][*last_x] != Cell::Damage
				{
					possible_shots.push([*last_x, *before_y - 1]);
				}
			} else {
				if *last_x < max_x && board[*last_y][*last_x + 1] != Cell::Shot && board[*last_y][*last_x + 1] != Cell::Damage {
					possible_shots.push([*last_x + 1, *last_y]);
				}
				if *last_x > 0 && board[*last_y][*last_x - 1] != Cell::Shot && board[*last_y][*last_x - 1] != Cell::Damage {
					possible_shots.push([*last_x - 1, *last_y]);
				}
				if *before_x < max_x
					&& board[*last_y][*before_x + 1] != Cell::Shot
					&& board[*last_y][*before_x + 1] != Cell::Damage
				{
					possible_shots.push([*before_x + 1, *last_y]);
				}
				if *before_x > 0 && board[*last_y][*before_x - 1] != Cell::Shot && board[*last_y][*before_x - 1] != Cell::Damage
				{
					possible_shots.push([*before_x - 1, *last_y]);
				}
			}
		} else {
			if *last_x < max_x && board[*last_y][*last_x + 1] != Cell::Shot && board[*last_y][*last_x + 1] != Cell::Damage {
				possible_shots.push([*last_x + 1, *last_y]);
			}
			if *last_x > 0 && board[*last_y][*last_x - 1] != Cell::Shot && board[*last_y][*last_x - 1] != Cell::Damage {
				possible_shots.push([*last_x - 1, *last_y]);
			}
			if *last_y < max_y && board[*last_y + 1][*last_x] != Cell::Shot && board[*last_y + 1][*last_x] != Cell::Damage {
				possible_shots.push([*last_x, *last_y + 1]);
			}
			if *last_y > 0 && board[*last_y - 1][*last_x] != Cell::Shot && board[*last_y - 1][*last_x] != Cell::Damage {
				possible_shots.push([*last_x, *last_y - 1]);
			}
		}

		let (next_x, next_y) = if !possible_shots.is_empty() {
			let index = rand::thread_rng().gen_range(0..possible_shots.len());
			(possible_shots[index][0], possible_shots[index][1])
		} else {
			self.shoot(board)
		};

		self.history.push((next_x, next_y, game::get_hit_type(board, board, next_x, next_y)));

		if game::get_hit_type(board, board, next_x, next_y) == game::HitType::Miss {
			for coords in &possible_shots {
				self.todo.push((coords[0], coords[1]));
			}
		}

		(next_x, next_y)
	}
}

#[test]
fn attack_works() {
	let mut attack = Attack::new();
	let mut board = [[Cell::Shot; config::SIZE_X]; config::SIZE_Y];

	board[5][5] = Cell::ShipThree([5, 5, 5, 4, 5, 3]);
	assert_eq!(attack.shoot(&board), (5, 5));
	board[5][5] = Cell::Damage;

	board[4][5] = Cell::ShipThree([5, 5, 5, 4, 5, 3]);
	assert_eq!(attack.shoot(&board), (5, 4));
	board[4][5] = Cell::Damage;

	board[3][5] = Cell::ShipThree([5, 5, 5, 4, 5, 3]);
	board[4][4] = Cell::Empty;
	board[4][6] = Cell::Empty;
	assert_eq!(attack.shoot_after_hit(&board), (5, 3));
}
