extern crate termion;

use crate::config;
use crate::gui;

use termion::color;

#[derive(Debug)]
pub enum Actor {
	Ai,
	Me,
}

#[derive(Debug)]
pub struct History {
	archive: Vec<String>,
}

impl History {
	pub fn new() -> Self {
		Self {
			archive: vec![format!(
				"{}- Game started -{}",
				color::Fg(color::Cyan),
				color::Fg(color::White)
			)],
		}
	}

	pub fn set_history(&mut self, entry: &str, actor: Actor) {
		let prefix = match actor {
			Actor::Ai => format!("{}AI:{}", color::Fg(color::LightRed), color::Fg(color::White)),
			Actor::Me => format!("{}ME:{}", color::Fg(color::Green), color::Fg(color::White)),
		};

		self.archive.push(format!("{} {}", prefix, entry));
	}

	fn get_hisory_slice(&self) -> Vec<String> {
		let l = if self.archive.len() > 2 { 3 } else { self.archive.len() };

		self.archive.as_slice()[self.archive.len() - l..].to_vec()
	}

	pub fn get_history(&self) -> String {
		let padding = gui::get_padding();
		let latest = self.get_hisory_slice();
		let board_size = (config::SIZE_X * 3 * 2) + 11;
		let inner_size = (((board_size * 2) - 5) / 2) as f32;
		let size = inner_size.floor() as usize;

		let mut output = format!("{} HISTORY\r\n", padding);
		output += &format!("{} ┌{:─^width$}┐\r\n", padding, "", width = size);
		output += &format!("{} │ {:<width$} │\r\n", padding, latest[0], width = (size + 16));
		let msg2 = if latest.len() > 1 {
			latest[1].clone()
		} else {
			format!("{}{}", color::Fg(color::White), color::Fg(color::White))
		};
		output += &format!("{} │ {:<width$} │\r\n", padding, msg2, width = (size + 16));
		let msg3 = if latest.len() > 2 {
			latest[2].clone()
		} else {
			format!("{}{}", color::Fg(color::White), color::Fg(color::White))
		};
		output += &format!("{} │ {:<width$} │\r\n", padding, msg3, width = (size + 16));
		output += &format!("{} └{:─^width$}┘\r\n", padding, "", width = size);

		output
	}
}

#[test]
fn history_works() {
	let mut history = History::new();
	assert_eq!(history.get_hisory_slice()[0].contains("- Game started -"), true);
	history.set_history("Entry 1", Actor::Me);
	assert_eq!(history.get_hisory_slice()[0].contains("- Game started -"), true);
	assert_eq!(history.get_hisory_slice()[1].contains("Entry 1"), true);
	assert_eq!(history.get_hisory_slice()[1].contains("ME:"), true);
	history.set_history("Entry 2", Actor::Ai);
	assert_eq!(history.get_hisory_slice()[0].contains("- Game started -"), true);
	assert_eq!(history.get_hisory_slice()[1].contains("Entry 1"), true);
	assert_eq!(history.get_hisory_slice()[2].contains("Entry 2"), true);
	assert_eq!(history.get_hisory_slice()[2].contains("AI:"), true);
	history.set_history("Entry 3", Actor::Me);
	assert_eq!(history.get_hisory_slice()[0].contains("Entry 1"), true);
	assert_eq!(history.get_hisory_slice()[1].contains("Entry 2"), true);
	assert_eq!(history.get_hisory_slice()[2].contains("Entry 3"), true);
	assert_eq!(history.get_hisory_slice()[2].contains("ME:"), true);
	history.set_history("Entry 4", Actor::Me);
	assert_eq!(history.get_hisory_slice()[0].contains("Entry 2"), true);
	assert_eq!(history.get_hisory_slice()[1].contains("Entry 3"), true);
	assert_eq!(history.get_hisory_slice()[2].contains("Entry 4"), true);
	assert_eq!(history.get_hisory_slice()[2].contains("ME:"), true);
	history.set_history("Entry 5", Actor::Ai);
	assert_eq!(history.get_hisory_slice()[0].contains("Entry 3"), true);
	assert_eq!(history.get_hisory_slice()[1].contains("Entry 4"), true);
	assert_eq!(history.get_hisory_slice()[2].contains("Entry 5"), true);
	assert_eq!(history.get_hisory_slice()[2].contains("AI:"), true);
}
