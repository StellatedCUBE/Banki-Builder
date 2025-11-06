#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tag {
	Puzzle,
	Platforming,
	Speedrun,
	Short,
	Long,
	Music,
	// === //
	SpeedrunTechniques = 28,
	Troll = 29,
	Hax = 30,
	PuzzlePiece = 31
}

impl Tag {
	pub const DISPLAY: [Self; 9] = [
		Self::SpeedrunTechniques,
		Self::Troll,
		Self::Hax,
		Self::Puzzle,
		Self::Platforming,
		Self::Speedrun,
		Self::Short,
		Self::Long,
		Self::Music,
	];

	pub const fn bit(self) -> u32 {
		1 << self as u32
	}

	pub const fn mandatory(self) -> bool {
		self as u8 > 24
	}
}