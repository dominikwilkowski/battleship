use crate::Cell;
use crate::Ship;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// BOARD SIZE
// MUST BE >8 <=35
pub const SIZE_X: usize = 10;
// MUST BE >3 <=26
pub const SIZE_Y: usize = 10;
pub type BoardRow = [Cell; SIZE_X];
pub type Board = [BoardRow; SIZE_Y];

// GUI
pub const EMPTY: &str = "░░░";
pub const SHIP: &str = "▓▓▓";
pub const CROSSHAIR: &str = " ◎ ";
pub const SHOT: &str = " ◌ ";
pub const DAMAGE: &str = " ╳ ";

// SHIPS
pub fn get_entitie_size(kind: &Ship) -> usize {
	match kind {
		Ship::OneBlock => 1,
		Ship::TwoBlock => 2,
		Ship::ThreeBlock => 3,
	}
}

pub const SHIP_ONE_BLOCK_AMOUNT: usize = 3;
pub const SHIP_TWO_BLOCK_AMOUNT: usize = 2;
pub const SHIP_THREE_BLOCK_AMOUNT: usize = 1;
