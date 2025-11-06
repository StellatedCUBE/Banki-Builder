
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Event {
	RoomLoad,
	MouseMove,
	MouseUp,
	MouseDown,
	FocusIn,
	ButtonDown,
	ButtonUp,
	KeyDown,
	KeyUp,
	FocusOut,
	IMData,
	GameQuit,
	GameData,
	InputFocus,
	InputUnfocus,
	SpeedrunTechniques,
	Like,
	LevelCompleteNoPP,
	LevelCompletePP,
	LevelQuit,
	ViewMove,
	GetString,
	GetReal,
	Tick,
	ClearObjectButtons,
	CreateObjectButton,
	Invalid
}

impl Event {
	pub fn from(code: u8) -> Self {
		match code {
			0 => Self::RoomLoad,
			1 => Self::MouseMove,
			2 => Self::MouseUp,
			3 => Self::MouseDown,
			4 => Self::FocusIn,
			5 => Self::ButtonDown,
			6 => Self::ButtonUp,
			7 => Self::KeyDown,
			8 => Self::KeyUp,
			9 => Self::FocusOut,
			64 => Self::IMData,
			128 => Self::GameQuit,
			129 => Self::GameData,
			130 => Self::InputFocus,
			131 => Self::InputUnfocus,
			247 => Self::SpeedrunTechniques,
			248 => Self::Like,
			249 => Self::LevelCompletePP,
			250 => Self::LevelCompleteNoPP,
			251 => Self::LevelQuit,
			252 => Self::ViewMove,
			253 => Self::GetString,
			254 => Self::GetReal,
			255 => Self::Tick,
			_ => Self::Invalid
		}
	}
}