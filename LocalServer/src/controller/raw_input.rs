#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RawInput {
	Key(u32),
	ControllerButton(u8),
	Mouse(u8)
}

impl Default for RawInput {
    fn default() -> Self {
        Self::Key(!0)
    }
}