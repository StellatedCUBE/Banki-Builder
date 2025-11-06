use smallvec::{smallvec, smallvec_inline, SmallVec};

use super::{tile_manager::GMTileData, LevelTheme};

pub fn get(theme: LevelTheme, variant: u32) -> SmallVec<[GMTileData; 1]> {
	match (theme, variant) {
		(LevelTheme::BambooForest, 1) |
		(LevelTheme::BambooForest, 2) |
		(LevelTheme::DreamScraps, 1) |
		(LevelTheme::DreamScraps, 2) |
		(LevelTheme::TheDepths, 1) |
		(LevelTheme::TheDepths, 2) |
		(LevelTheme::DancingStars, 1) |
		(LevelTheme::DancingStars, 2) |
		(LevelTheme::UltramarineRain, 1) |
		(LevelTheme::UltramarineRain, 2) |
		(LevelTheme::AzureWinter, 1) |
		(LevelTheme::Cirno, 1) |
		(LevelTheme::Entrance, 1) |
		(LevelTheme::ForestOfMagic, 1) |
		(LevelTheme::Purple, 1) |
		(LevelTheme::DreamFields, 1) |
		(LevelTheme::DreamFields, 2) => smallvec_inline![GMTileData {
			x: 0, y: -1,
			width: 1, height: 1,
			tx: 224, ty: 0
		}],
		(LevelTheme::Purple, 2) |
		(LevelTheme::BambooForest, 3) |
		(LevelTheme::BambooForest, 4) |
		(LevelTheme::DreamScraps, 3) |
		(LevelTheme::UltramarineRain, 3) |
		(LevelTheme::UltramarineRain, 4) |
		(LevelTheme::TheDepths, 3) |
		(LevelTheme::TheDepths, 4) |
		(LevelTheme::DancingStars, 3) |
		(LevelTheme::DancingStars, 4) |
		(LevelTheme::ForestOfMagic, 2) |
		(LevelTheme::Entrance, 2) |
		(LevelTheme::DreamFields, 3) |
		(LevelTheme::DreamFields, 4) => smallvec_inline![GMTileData {
			x: 0, y: -1,
			width: 1, height: 1,
			tx: 256, ty: 0
		}],
		(LevelTheme::Purple, 3) |
		(LevelTheme::BambooForest, 5) |
		(LevelTheme::BambooForest, 6) |
		(LevelTheme::DreamScraps, 4) |
		(LevelTheme::UltramarineRain, 5) |
		(LevelTheme::UltramarineRain, 6) |
		(LevelTheme::TheDepths, 5) |
		(LevelTheme::TheDepths, 6) |
		(LevelTheme::DancingStars, 5) |
		(LevelTheme::DancingStars, 6) |
		(LevelTheme::ForestOfMagic, 3) |
		(LevelTheme::Entrance, 3) |
		(LevelTheme::DreamFields, 5) |
		(LevelTheme::DreamFields, 6) => smallvec_inline![GMTileData {
			x: 0, y: -1,
			width: 1, height: 1,
			tx: 288, ty: 0
		}],
		(LevelTheme::Purple, 4) |
		(LevelTheme::DreamScraps, 5) |
		(LevelTheme::DreamScraps, 6) |
		(LevelTheme::DreamScraps, 7) |
		(LevelTheme::BambooForest, 7) |
		(LevelTheme::UltramarineRain, 7) |
		(LevelTheme::AzureWinter, 2) |
		(LevelTheme::Cirno, 2) |
		(LevelTheme::ForestOfMagic, 7) |
		(LevelTheme::DancingStars, 7) |
		(LevelTheme::TheDepths, 7) |
		(LevelTheme::Entrance, 7) |
		(LevelTheme::DreamFields, 7) => smallvec_inline![GMTileData {
			x: 0, y: -1,
			width: 1, height: 1,
			tx: 320, ty: 0
		}],
		(LevelTheme::Entrance, 8) |
		(LevelTheme::DreamScraps, 8) |
		(LevelTheme::TheDepths, 8) |
		(LevelTheme::DancingStars, 8) |
		(LevelTheme::Cirno, 8) |
		(LevelTheme::DreamFields, 8) => smallvec![GMTileData {
			x: -1, y: -5,
			width: 3, height: 3,
			tx: 288, ty: 128
		}, GMTileData {
			x: 0, y: -2,
			width: 1, height: 1,
			tx: 320, ty: 32
		}, GMTileData {
			x: 0, y: -1,
			width: 1, height: 1,
			tx: 352, ty: 96
		}],

		(LevelTheme::ForestOfMagic, 8) |
		(LevelTheme::UltramarineRain, 8) |
		(LevelTheme::BambooForest, 8) => smallvec![GMTileData {
			x: -1, y: -4,
			width: 3, height: 3,
			tx: 288, ty: 128
		}, GMTileData {
			x: 0, y: -1,
			width: 1, height: 1,
			tx: 352, ty: 96
		}],

		(LevelTheme::Cirno, 3) |
		(LevelTheme::AzureWinter, 3) => smallvec_inline![GMTileData {
			x: 0, y: -1,
			width: 1, height: 1,
			tx: 160, ty: 96
		}],
		(LevelTheme::Cirno, 4) |
		(LevelTheme::AzureWinter, 4) => smallvec_inline![GMTileData {
			x: 0, y: -2,
			width: 1, height: 2,
			tx: 192, ty: 64
		}],
		(LevelTheme::Cirno, 5) |
		(LevelTheme::AzureWinter, 5) => smallvec_inline![GMTileData {
			x: 0, y: -2,
			width: 1, height: 2,
			tx: 224, ty: 96
		}],
		(LevelTheme::Cirno, 6) |
		(LevelTheme::AzureWinter, 6) => smallvec![GMTileData {
			x: 0, y: -3,
			width: 1, height: 2,
			tx: 224, ty: 96
		}, GMTileData {
			x: 0, y: -1,
			width: 1, height: 1,
			tx: 224, ty: 128
		}],
		(LevelTheme::Cirno, 7) |
		(LevelTheme::AzureWinter, 7) => smallvec_inline![GMTileData {
			x: 0, y: -3,
			width: 1, height: 3,
			tx: 192, ty: 64
		}],

		(LevelTheme::ForestOfMagic, 4) => smallvec_inline![GMTileData {
			x: 0, y: -1,
			width: 1, height: 1,
			tx: 224, ty: 32
		}],
		(LevelTheme::ForestOfMagic, 5) => smallvec_inline![GMTileData {
			x: 0, y: -1,
			width: 1, height: 1,
			tx: 256, ty: 32
		}],
		(LevelTheme::ForestOfMagic, 6) => smallvec_inline![GMTileData {
			x: 0, y: -1,
			width: 1, height: 1,
			tx: 288, ty: 32
		}],

		_ => SmallVec::new_const()
	}
}

pub fn has(theme: LevelTheme) -> bool {
	match theme {
		LevelTheme::OutsideWorld |
		LevelTheme::ShiningNeedleCastle |
		LevelTheme::JerryAttack |
		LevelTheme::FarawayLabyrinth |
		LevelTheme::ReachOutToThatMoon |
		LevelTheme::MindBreak |
		LevelTheme::Fireflies |
		LevelTheme::Rumia |
		LevelTheme::Seija |
		LevelTheme::Rasobi |
		LevelTheme::Koumakan => false,
		_ => true
	}
}

pub fn down(theme: LevelTheme, variant: u32) -> SmallVec<[GMTileData; 1]> {
	match (theme, variant) {
		(LevelTheme::Cirno, 1) |
		(LevelTheme::AzureWinter, 1) |
		(LevelTheme::AzureWinter, 2) => smallvec_inline![GMTileData {
			x: 0, y: 1,
			width: 1, height: 1,
			tx: 224, ty: 128
		}],
		(LevelTheme::Cirno, 2) |
		(LevelTheme::AzureWinter, 3) => smallvec![GMTileData {
			x: 0, y: 2,
			width: 1, height: 1,
			tx: 224, ty: 128
		}, GMTileData {
			x: 0, y: 1,
			width: 1, height: 1,
			tx: 224, ty: 128
		}],

		(LevelTheme::Cirno, 3) => smallvec![GMTileData {
			x: 0, y: 3,
			width: 1, height: 1,
			tx: 224, ty: 128
		},GMTileData {
			x: 0, y: 2,
			width: 1, height: 1,
			tx: 224, ty: 128
		}, GMTileData {
			x: 0, y: 1,
			width: 1, height: 1,
			tx: 224, ty: 128
		}],

		_ => SmallVec::new_const()
	}
}

pub fn has_down(theme: LevelTheme) -> bool {
	theme == LevelTheme::AzureWinter || theme == LevelTheme::Cirno
}