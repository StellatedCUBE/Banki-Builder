use std::collections::HashMap;
use std::hash::Hash;

use lazy_static::lazy_static;

use crate::controller::raw_input::RawInput;

use super::mod_input::ModInput;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum MenuIntent {
	ScrollUp,
	ScrollDown,

	SelectUp,
	SelectDown,
	SelectLeft,
	SelectRight,
	Primary,
	Secondary,
	GoBack,
}

impl MenuIntent {
	pub fn is_direction(self) -> bool {
		match self {
			Self::SelectUp |
			Self::SelectDown |
			Self::SelectLeft |
			Self::SelectRight => true,
			_ => false
		}
	}
}

pub struct ControlMap<Intent: Copy + Eq + Hash> {
	intent_to_input: HashMap<Intent, Vec<ModInput>>,
	input_to_intent: HashMap<ModInput, Intent>
}

impl<Intent: Copy + Eq + Hash> ControlMap<Intent> {
	pub fn new() -> Self {
		Self {
			intent_to_input: HashMap::new(),
			input_to_intent: HashMap::new()
		}
	}

	pub fn set_control(&mut self, intent: Intent, input: ModInput) {
		if let Some(previous_intent) = self.input_to_intent.insert(input, intent) {
			self.intent_to_input.get_mut(&previous_intent).unwrap().retain(|inp| inp != &input);
		}
		self.intent_to_input.entry(intent).or_default().push(input);
	}

	pub fn get_intent(&self, input: ModInput) -> Option<Intent> {
		self.input_to_intent.get(&input).copied()
	}
}

lazy_static!{
	pub static ref MENU_CONTROLS: ControlMap<MenuIntent> = {
		let mut controls = ControlMap::new();

		controls.set_control(MenuIntent::Primary, ModInput::from(RawInput::Mouse(1)));
		controls.set_control(MenuIntent::Secondary, ModInput::from(RawInput::Mouse(2)));

		controls.set_control(MenuIntent::ScrollUp, ModInput::from(RawInput::Mouse(4)));
		controls.set_control(MenuIntent::ScrollUp, ModInput::from_key(104));
		controls.set_control(MenuIntent::ScrollDown, ModInput::from(RawInput::Mouse(5)));
		controls.set_control(MenuIntent::ScrollDown, ModInput::from_key(98));

		controls.set_control(MenuIntent::SelectUp, ModInput::from_key(38));
		controls.set_control(MenuIntent::SelectUp, ModInput::from_key(87));
		controls.set_control(MenuIntent::SelectDown, ModInput::from_key(40));
		controls.set_control(MenuIntent::SelectDown, ModInput::from_key(83));
		controls.set_control(MenuIntent::SelectLeft, ModInput::from_key(37));
		controls.set_control(MenuIntent::SelectLeft, ModInput::from_key(65));
		controls.set_control(MenuIntent::SelectRight, ModInput::from_key(39));
		controls.set_control(MenuIntent::SelectRight, ModInput::from_key(68));
		controls.set_control(MenuIntent::Primary, ModInput::from_key(13));
		controls.set_control(MenuIntent::Primary, ModInput::from_key(89));
		controls.set_control(MenuIntent::Primary, ModInput::from_key(90));
		controls.set_control(MenuIntent::Secondary, ModInput::from_key(88));
		controls.set_control(MenuIntent::GoBack, ModInput::from_key(27));
		controls.set_control(MenuIntent::GoBack, ModInput::from(RawInput::Mouse(8)));

		controls
	};
}