use super::raw_input::RawInput;

const MAX_MODIFIERS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ModInput {
	pub main: RawInput,
	pub modifiers: [u32; MAX_MODIFIERS]
}

impl ModInput {
	pub fn new<const N: usize>(main: RawInput, bitmask: &[u64; N]) -> Self {
		let main_num = match main {
			RawInput::ControllerButton(button) => button as u32,
			RawInput::Key(key) => key,
			_ => panic!("Bad input type to add modifiers")
		};

		let mut modifiers = [!0u32; MAX_MODIFIERS];
		let mut modifier_count = 0;
		let mut i = 0;
		for submask in bitmask {
			let mut submask = *submask;
			for _ in 0..64 {
				if i != main_num && (submask & 1) == 1 {
					modifiers[modifier_count] = i;
					modifier_count += 1;
					if modifier_count == MAX_MODIFIERS {
						break;
					}
				}

				i += 1;
				submask >>= 1;
			}
		}

		Self {
			main,
			modifiers
		}
	}

	pub const fn from(main: RawInput) -> Self {
		Self {
			main,
			modifiers: [!0u32; MAX_MODIFIERS]
		}
	}

	pub const fn from_key(key: u32) -> Self {
		Self::from(RawInput::Key(key))
	}

	pub const fn from_key_ctrl(key: u32) -> Self {
		let mut this = Self::from_key(key);
		this.modifiers[0] = 17;
		this
	}

	pub const fn from_key_ctrl_shift(key: u32) -> Self {
		let mut this = Self::from_key(key);
		this.modifiers[0] = 16;
		this.modifiers[1] = 17;
		this
	}
}