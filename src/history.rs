#[derive(Debug)]
pub struct History {
	archive: Vec<String>,
}

impl History {
	pub fn new() -> Self {
		Self {
			archive: vec![String::from("- Game started -")],
		}
	}

	pub fn set_history(&mut self, entry: &str) {
		self.archive.push(entry.to_string());
	}

	fn get_hisory_slice(&self) -> Vec<String> {
		let l = if self.archive.len() > 2 { 3 } else { self.archive.len() };

		let mut result = self.archive.as_slice()[self.archive.len() - l..].to_vec();
		result
	}

	pub fn get_history(&self) -> String {
		let latest = self.get_hisory_slice();
		let mut output = String::from(" HISTORY\r\n");
		output += " ┌────────────────────────────────────────────────────────────────────┐\r\n";
		output += &format!(" │ {:<66} │\r\n", latest[0]);
		let msg2 = if latest.len() > 1 {
			latest[1].clone()
		} else {
			String::new()
		};
		output += &format!(" │ {:<66} │\r\n", msg2);
		let msg3 = if latest.len() > 2 {
			latest[2].clone()
		} else {
			String::new()
		};
		output += &format!(" │ {:<66} │\r\n", msg3);
		output += " └────────────────────────────────────────────────────────────────────┘\r\n";

		output
	}
}

#[test]
fn history_works() {
	let mut history = History::new();
	assert_eq!(history.get_hisory_slice(), vec![String::from("- Game started -")]);
	history.set_history("Entry 1");
	assert_eq!(history.get_hisory_slice(), vec![String::from("- Game started -"), String::from("Entry 1")]);
	history.set_history("Entry 2");
	assert_eq!(
		history.get_hisory_slice(),
		vec![
			String::from("- Game started -"),
			String::from("Entry 1"),
			String::from("Entry 2"),
		]
	);
	history.set_history("Entry 3");
	assert_eq!(
		history.get_hisory_slice(),
		vec![
			String::from("Entry 1"),
			String::from("Entry 2"),
			String::from("Entry 3"),
		]
	);
	history.set_history("Entry 4");
	assert_eq!(
		history.get_hisory_slice(),
		vec![
			String::from("Entry 2"),
			String::from("Entry 3"),
			String::from("Entry 4"),
		]
	);
	history.set_history("Entry 5");
	assert_eq!(
		history.get_hisory_slice(),
		vec![
			String::from("Entry 3"),
			String::from("Entry 4"),
			String::from("Entry 5"),
		]
	);
}
