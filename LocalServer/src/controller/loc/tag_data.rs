use crate::controller::level::tag::Tag;

use super::MaybeLocalized;

pub fn tag_name(tag: Tag) -> &'static str {
	match tag {
		Tag::Puzzle => MaybeLocalized::Localized("Puzzle", "", ""),
		Tag::Platforming => MaybeLocalized::Localized("Platforming", "", ""),
		Tag::Speedrun => MaybeLocalized::Localized("Speedrun", "", ""),
		Tag::Short => MaybeLocalized::Localized("Short", "", ""),
		Tag::Long => MaybeLocalized::Localized("Long", "", ""),
		Tag::Music => MaybeLocalized::Localized("Music", "", ""),

		Tag::SpeedrunTechniques => MaybeLocalized::Localized("Advanced Techniques", "", ""),
		Tag::Troll => MaybeLocalized::Localized("Troll", "", ""),
		Tag::Hax => MaybeLocalized::Localized("Editor Hacks Used", "", ""),

		Tag::PuzzlePiece => MaybeLocalized::Localized("", "", ""),
	}.for_current_locale_static()
}