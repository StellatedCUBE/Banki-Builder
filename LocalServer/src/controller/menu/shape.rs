#[derive(Clone, Copy)]
pub enum Shape {
	Rect(f32, f32, f32, f32),
	Circle(f32, f32, f32)
}

impl Shape {
	pub fn rect_from_pos_size(x: f32, y: f32, width: f32, height: f32) -> Self {
		Self::Rect(x, y, x + width, y + height)
	}

	pub const fn null() -> Self {
		Self::Rect(0.0, 0.0, -1.0, -1.0)
	}

	pub fn contains(self, x: f32, y: f32) -> bool {
		match self {
			Self::Rect(x_min, y_min, x_max, y_max) => x_min <= x && x <= x_max && y_min <= y && y <= y_max,
			Self::Circle(c_x, c_y, radius) => (x - c_x) * (x - c_x) + (y - c_y) * (y - c_y) <= radius * radius
		}
	}
}