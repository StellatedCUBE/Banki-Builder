use crate::controller::{control_settings::ControlMap, mod_input::ModInput, raw_input::RawInput};

use super::tool;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum EditorIntent {
	CursorLeft,
	CursorRight,
	CursorUp,
	CursorDown,
	PanLeft,
	PanRight,
	PanUp,
	PanDown,

	Primary,
	Play,
	SetTool(usize),
	SetToolFromRow(usize),
	SetToolRow(usize),
	NextToolRow,
	PrevToolRow,
	ToggleToolRowSelectDropdown,
	UseTool(usize),
	Save,
	SelectAll,
	MassDelete,
	ToggleSettingsMenu,
	Exit,
	ContextMenu,
	ZoomIn,
	ZoomOut,
	ZoomToggle,
	Copy,
	Paste,
	Cut,
}

impl EditorIntent {
	pub fn discriminant(&self) -> usize {
		unsafe { *<*const _>::from(self).cast::<usize>() }
	}
}

fn default() -> ControlMap<EditorIntent> {
	let mut map = ControlMap::new();

	map.set_control(EditorIntent::Primary, ModInput::from(RawInput::Mouse(1)));
	map.set_control(EditorIntent::Primary, ModInput::from_key(89)); // y
	map.set_control(EditorIntent::Primary, ModInput::from_key(90)); // z

	map.set_control(EditorIntent::CursorUp, ModInput::from_key(38));
	map.set_control(EditorIntent::CursorDown, ModInput::from_key(40));
	map.set_control(EditorIntent::CursorLeft, ModInput::from_key(37));
	map.set_control(EditorIntent::CursorRight, ModInput::from_key(39));

	map.set_control(EditorIntent::PanUp, ModInput::from_key(104));
	map.set_control(EditorIntent::PanDown, ModInput::from_key(98));
	map.set_control(EditorIntent::PanLeft, ModInput::from_key(100));
	map.set_control(EditorIntent::PanRight, ModInput::from_key(102));

	map.set_control(EditorIntent::PanUp, ModInput::from_key(75)); // k
	map.set_control(EditorIntent::PanDown, ModInput::from_key(74)); // j
	map.set_control(EditorIntent::PanLeft, ModInput::from_key(72)); // h
	map.set_control(EditorIntent::PanRight, ModInput::from_key(76)); // l

	map.set_control(EditorIntent::Play, ModInput::from_key(80)); // p

	for i in 1..11 {
		map.set_control(EditorIntent::SetToolFromRow(i - 1), ModInput::from_key(48 + i as u32 % 10)); // 1 - 9, 0
	}

	for i in 0..tool::ROW_COUNT {
		map.set_control(EditorIntent::SetToolRow(i), ModInput::from_key(i as u32 + 112));
	}

	map.set_control(EditorIntent::SetTool(0), ModInput::from_key(81)); // q
	map.set_control(EditorIntent::SetTool(1), ModInput::from_key(87)); // w
	map.set_control(EditorIntent::SetTool(2), ModInput::from_key(65)); // a
	map.set_control(EditorIntent::SetTool(3), ModInput::from_key(83)); // s

	map.set_control(EditorIntent::PrevToolRow, ModInput::from_key(33)); // pgup
	map.set_control(EditorIntent::NextToolRow, ModInput::from_key(34)); // pgdn
	map.set_control(EditorIntent::ToggleToolRowSelectDropdown, ModInput::from_key(9)); // tab

	map.set_control(EditorIntent::UseTool(1), ModInput::from(RawInput::Mouse(3)));
	map.set_control(EditorIntent::ContextMenu, ModInput::from(RawInput::Mouse(2)));
	//map.set_control(EditorIntent::UseTool(3), ModInput::from(RawInput::Mouse(2)));

	//map.set_control(EditorIntent::UseTool(3), ModInput::from_key_ctrl(88));
	map.set_control(EditorIntent::UseTool(4), ModInput::from_key_ctrl(90)); // ^z
	map.set_control(EditorIntent::UseTool(4), ModInput::from(RawInput::Mouse(8)));
	map.set_control(EditorIntent::UseTool(5), ModInput::from_key_ctrl(89)); // ^y
	map.set_control(EditorIntent::UseTool(5), ModInput::from_key_ctrl_shift(90)); // ^Z
	map.set_control(EditorIntent::UseTool(5), ModInput::from(RawInput::Mouse(9)));

	map.set_control(EditorIntent::Save, ModInput::from_key_ctrl(83)); // ^s
	map.set_control(EditorIntent::SelectAll, ModInput::from_key_ctrl(65)); // ^a
	map.set_control(EditorIntent::MassDelete, ModInput::from_key(46)); // del

	map.set_control(EditorIntent::ToggleSettingsMenu, ModInput::from_key(27)); // esc

	map.set_control(EditorIntent::ZoomIn, ModInput::from_key(187)); // =
	map.set_control(EditorIntent::ZoomIn, ModInput::from(RawInput::Mouse(4)));
	map.set_control(EditorIntent::ZoomOut, ModInput::from_key(189)); // -
	map.set_control(EditorIntent::ZoomOut, ModInput::from(RawInput::Mouse(5)));
	map.set_control(EditorIntent::ZoomToggle, ModInput::from_key(192)); // `

	map.set_control(EditorIntent::Copy, ModInput::from_key_ctrl(67)); // ^c
	map.set_control(EditorIntent::Copy, ModInput::from_key_ctrl_shift(67)); // ^C
	map.set_control(EditorIntent::Paste, ModInput::from_key_ctrl(86)); // ^v
	map.set_control(EditorIntent::Paste, ModInput::from_key_ctrl_shift(86)); // ^V
	map.set_control(EditorIntent::Cut, ModInput::from_key_ctrl(88)); // ^x

	map
}

pub fn get() -> ControlMap<EditorIntent> {
	default()
}