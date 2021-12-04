extern crate rand;

use crate::config;
use crate::movement;
use crate::ships;
use crate::Cell;
use crate::Rotation;

use rand::Rng;
use ships::ShipTracker;

pub fn set_ships(mut board: [[Cell; 10]; 10]) -> [[Cell; 10]; 10] {
	let mut ships =
		ShipTracker::new(config::SHIP_ONE_BLOCK_AMOUNT, config::SHIP_TWO_BLOCK_AMOUNT, config::SHIP_THREE_BLOCK_AMOUNT);
	let (one_block, two_block, three_block) = ships.get_ships();
	let mut remaining_ships = one_block + two_block + three_block;
	let mut this_ship = ships.get_next_unset_ship().unwrap();
	let mut ship_size = config::get_entitie_size(&this_ship);

	while remaining_ships > 0 {
		let pos_x: usize = rand::thread_rng().gen_range(0..10);
		let pos_y: usize = rand::thread_rng().gen_range(0..10);
		let rotation = match rand::thread_rng().gen_range(0..=1) {
			0 => Rotation::Horizontal,
			_ => Rotation::Vertical,
		};

		if movement::is_free_space(&board, pos_x as isize, pos_y as isize, &ship_size, &rotation) {
			board = movement::place_entity(board, pos_x, pos_y, &ship_size, &rotation, Cell::Ship);
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
