#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Ship {
	OneBlock,
	TwoBlock,
	ThreeBlock,
}

#[derive(Debug)]
pub struct ShipTracker {
	one_block: usize,
	two_block: usize,
	three_block: usize,
}

impl ShipTracker {
	pub fn new(one_block: usize, two_block: usize, three_block: usize) -> Self {
		Self {
			one_block,
			two_block,
			three_block,
		}
	}

	pub fn get_next_unset_ship(&self) -> Option<Ship> {
		if self.one_block > 0 {
			Some(Ship::OneBlock)
		} else if self.two_block > 0 {
			Some(Ship::TwoBlock)
		} else if self.three_block > 0 {
			Some(Ship::ThreeBlock)
		} else {
			None
		}
	}

	pub fn set_ship(&mut self, kind: &Ship) {
		match kind {
			Ship::OneBlock => {
				self.one_block -= 1;
			}
			Ship::TwoBlock => {
				self.two_block -= 1;
			}
			Ship::ThreeBlock => {
				self.three_block -= 1;
			}
		}
	}

	// we use this function for the tests
	#[allow(dead_code)]
	pub fn get_ships(&self) -> &Self {
		self
	}
}

#[test]
fn keep_track_of_ships() {
	let mut ships = ShipTracker::new(3, 2, 1);

	let mut ship_slice = ships.get_ships();
	assert_eq!(ship_slice.one_block, 3);
	assert_eq!(ship_slice.two_block, 2);
	assert_eq!(ship_slice.three_block, 1);
	let mut this_ship = ships.get_next_unset_ship().unwrap();
	assert_eq!(this_ship, Ship::OneBlock);
	ships.set_ship(&this_ship);

	ship_slice = ships.get_ships();
	assert_eq!(ship_slice.one_block, 2);
	assert_eq!(ship_slice.two_block, 2);
	assert_eq!(ship_slice.three_block, 1);
	this_ship = ships.get_next_unset_ship().unwrap();
	assert_eq!(this_ship, Ship::OneBlock);
	ships.set_ship(&this_ship);

	ship_slice = ships.get_ships();
	assert_eq!(ship_slice.one_block, 1);
	assert_eq!(ship_slice.two_block, 2);
	assert_eq!(ship_slice.three_block, 1);
	this_ship = ships.get_next_unset_ship().unwrap();
	assert_eq!(this_ship, Ship::OneBlock);
	ships.set_ship(&this_ship);

	ship_slice = ships.get_ships();
	assert_eq!(ship_slice.one_block, 0);
	assert_eq!(ship_slice.two_block, 2);
	assert_eq!(ship_slice.three_block, 1);
	this_ship = ships.get_next_unset_ship().unwrap();
	assert_eq!(this_ship, Ship::TwoBlock);
	ships.set_ship(&this_ship);

	ship_slice = ships.get_ships();
	assert_eq!(ship_slice.one_block, 0);
	assert_eq!(ship_slice.two_block, 1);
	assert_eq!(ship_slice.three_block, 1);
	this_ship = ships.get_next_unset_ship().unwrap();
	assert_eq!(this_ship, Ship::TwoBlock);
	ships.set_ship(&this_ship);

	ship_slice = ships.get_ships();
	assert_eq!(ship_slice.one_block, 0);
	assert_eq!(ship_slice.two_block, 0);
	assert_eq!(ship_slice.three_block, 1);
	this_ship = ships.get_next_unset_ship().unwrap();
	assert_eq!(this_ship, Ship::ThreeBlock);
	ships.set_ship(&this_ship);

	ship_slice = ships.get_ships();
	assert_eq!(ship_slice.one_block, 0);
	assert_eq!(ship_slice.two_block, 0);
	assert_eq!(ship_slice.three_block, 0);
	assert!(ships.get_next_unset_ship().is_none())
}
