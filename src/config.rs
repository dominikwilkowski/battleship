// GUI
pub const EMPTY: &str = "░░░";
pub const SHIP: &str = "▓▓▓";
pub const SHOT: &str = " ◌ ";
pub const DAMAGE: &str = "╳╳╳";

// SHIPS
pub fn get_entitie_size(kind: &str) -> usize {
	return match kind {
		"one_block" => 1,
		"two_block" => 2,
		"three_block" => 3,
		_ => 0,
	};
}

pub const SHIP_ONE_BLOCK_AMOUNT: usize = 3;
pub const SHIP_TWO_BLOCK_AMOUNT: usize = 2;
pub const SHIP_THREE_BLOCK_AMOUNT: usize = 1;
