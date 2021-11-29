#[derive(Debug)]
pub struct Ships {
	one_block: Vec<[u8; 3]>,
	two_block: Vec<[u8; 3]>,
	three_block: Vec<[u8; 3]>,
}

impl Ships {
	pub fn new(one_block: usize, two_block: usize, three_block: usize) -> Self {
		Self {
			one_block: vec![[0, 0, 0]; one_block],
			two_block: vec![[0, 0, 0]; two_block],
			three_block: vec![[0, 0, 0]; three_block],
		}
	}

	pub fn get_next_unset_ship(&self) -> (&str, i8) {
		let mut index: i8 = -1;
		for block in &self.one_block {
			if block[2] == 0 {
				break;
			} else {
				index += 1;
			}
		}
		if index < *&self.one_block.len() as i8 - 1 {
			return ("one_block", index + 1);
		}

		index = -1;
		for block in &self.two_block {
			if block[2] == 0 {
				break;
			} else {
				index += 1;
			}
		}
		if index < *&self.two_block.len() as i8 - 1 {
			return ("two_block", index + 1);
		}

		index = -1;
		for block in &self.three_block {
			if block[2] == 0 {
				break;
			} else {
				index += 1;
			}
		}
		if index < *&self.three_block.len() as i8 - 1 {
			return ("three_block", index + 1);
		} else {
			return ("", -1);
		}
	}

	pub fn set_ship(&mut self, kind: &str, index: usize, coord: [u8; 2]) {
		match kind {
			"one_block" => {
				if index < self.one_block.len() {
					self.one_block[index][0] = coord[0];
					self.one_block[index][1] = coord[1];
					self.one_block[index][2] = 1;
				}
			}
			"two_block" => {
				if index < self.two_block.len() {
					self.two_block[index][0] = coord[0];
					self.two_block[index][1] = coord[1];
					self.two_block[index][2] = 1;
				}
			}
			"three_block" => {
				if index < self.three_block.len() {
					self.three_block[index][0] = coord[0];
					self.three_block[index][1] = coord[1];
					self.three_block[index][2] = 1;
				}
			}
			_ => {}
		}
	}

	pub fn get_ships(&self) -> &Self {
		self
	}
}

#[test]
fn keep_track_of_ships() {
	let mut ships = Ships::new(3, 2, 1);

	let mut ship_slice = ships.get_ships();
	assert_eq!(ship_slice.one_block, vec![[0, 0, 0], [0, 0, 0], [0, 0, 0]]);
	assert_eq!(ship_slice.two_block, vec![[0, 0, 0], [0, 0, 0]]);
	assert_eq!(ship_slice.three_block, vec![[0, 0, 0]]);
	assert_eq!(ships.get_next_unset_ship(), ("one_block", 0));

	ships.set_ship("one_block", 0, [3, 8]);
	ship_slice = ships.get_ships();
	assert_eq!(ship_slice.one_block, vec![[3, 8, 1], [0, 0, 0], [0, 0, 0]]);
	assert_eq!(ship_slice.two_block, vec![[0, 0, 0], [0, 0, 0]]);
	assert_eq!(ship_slice.three_block, vec![[0, 0, 0]]);
	assert_eq!(ships.get_next_unset_ship(), ("one_block", 1));

	ships.set_ship("one_block", 1, [5, 5]);
	ship_slice = ships.get_ships();
	assert_eq!(ship_slice.one_block, vec![[3, 8, 1], [5, 5, 1], [0, 0, 0]]);
	assert_eq!(ship_slice.two_block, vec![[0, 0, 0], [0, 0, 0]]);
	assert_eq!(ship_slice.three_block, vec![[0, 0, 0]]);
	assert_eq!(ships.get_next_unset_ship(), ("one_block", 2));

	ships.set_ship("one_block", 2, [1, 1]);
	ship_slice = ships.get_ships();
	assert_eq!(ship_slice.one_block, vec![[3, 8, 1], [5, 5, 1], [1, 1, 1]]);
	assert_eq!(ship_slice.two_block, vec![[0, 0, 0], [0, 0, 0]]);
	assert_eq!(ship_slice.three_block, vec![[0, 0, 0]]);
	assert_eq!(ships.get_next_unset_ship(), ("two_block", 0));

	ships.set_ship("two_block", 0, [2, 2]);
	ship_slice = ships.get_ships();
	assert_eq!(ship_slice.one_block, vec![[3, 8, 1], [5, 5, 1], [1, 1, 1]]);
	assert_eq!(ship_slice.two_block, vec![[2, 2, 1], [0, 0, 0]]);
	assert_eq!(ship_slice.three_block, vec![[0, 0, 0]]);
	assert_eq!(ships.get_next_unset_ship(), ("two_block", 1));

	ships.set_ship("two_block", 1, [9, 9]);
	ship_slice = ships.get_ships();
	assert_eq!(ship_slice.one_block, vec![[3, 8, 1], [5, 5, 1], [1, 1, 1]]);
	assert_eq!(ship_slice.two_block, vec![[2, 2, 1], [9, 9, 1]]);
	assert_eq!(ship_slice.three_block, vec![[0, 0, 0]]);
	assert_eq!(ships.get_next_unset_ship(), ("three_block", 0));

	ships.set_ship("three_block", 0, [7, 7]);
	ship_slice = ships.get_ships();
	assert_eq!(ship_slice.one_block, vec![[3, 8, 1], [5, 5, 1], [1, 1, 1]]);
	assert_eq!(ship_slice.two_block, vec![[2, 2, 1], [9, 9, 1]]);
	assert_eq!(ship_slice.three_block, vec![[7, 7, 1]]);
	assert_eq!(ships.get_next_unset_ship(), ("", -1));

	ships = Ships::new(3, 2, 1);
	ship_slice = ships.get_ships();
	assert_eq!(ship_slice.one_block, vec![[0, 0, 0], [0, 0, 0], [0, 0, 0]]);
	assert_eq!(ship_slice.two_block, vec![[0, 0, 0], [0, 0, 0]]);
	assert_eq!(ship_slice.three_block, vec![[0, 0, 0]]);
	assert_eq!(ships.get_next_unset_ship(), ("one_block", 0));
}
