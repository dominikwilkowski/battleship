use crate::Ship;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// SIZE
pub const MIN_WIDTH: u16 = 80;
pub const MIN_HEIGHT: u16 = 35;

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
