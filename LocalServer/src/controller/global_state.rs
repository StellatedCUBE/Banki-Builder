use super::{level::ObjectButton, mod_input::ModInput, raw_input::RawInput};

#[derive(Default)]
pub struct ControllerGlobalState {
	pub mouse_x: f32,
	pub mouse_y: f32,
	pub mouse_dx: f32,
	pub mouse_dy: f32,
	pub recieved_real: f64,
	pub recieved_string: String,
	pub view_x: f32,
	pub view_y: f32,
	pub last_raw_input: RawInput,
	pub last_mod_input: ModInput,
	pub controller_buttons: [u64; 4],
	pub keyboard_buttons: [u64; 8],
	pub was_mouse_actually_moved: bool,
	pub mouse_raw: (i32, i32),
	pub object_button_massive_hack: Option<ObjectButton>,
	pub level_clear_time: u32,
	pub input_focused: bool,
	pub window_focused: bool,
}