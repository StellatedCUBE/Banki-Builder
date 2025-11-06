use std::sync::Arc;

use lazy_static::lazy_static;

use crate::controller::{command_handler::CommandSender, level::{simple_object::{Colour, ObjectType, SwitchType}, tile_manager::Tile, Character, LevelTheme}, menu::editor::{t_cannon::ToolCannon, t_chandelier::ToolChandelier, t_cursor::ToolCursor, t_delete::ToolDelete, t_extra_head::ToolExtraHead, t_flipper::ToolFlipper, t_mochi::ToolMochi, t_move::ToolMove, t_onmyoudama_crawl::ToolOnmyoudamaCrawl, t_onmyoudama_shoot::ToolOnmyoudamaShoot, t_paired_object::{PairedObject, ToolPairedObject}, t_pan::ToolPan, t_select::ToolSelect, t_simple_object::ToolSimple, t_spike::ToolSpike, t_symbol::ToolSymbol, t_tile::ToolTile, t_tile_split::ToolTileSplit, t_undo::ToolUndo}, undo::UndoAction};

use super::{context_menu::ContextMenuItem, Editor};

pub type ArcTool = Arc<dyn Tool + Send + Sync>;
pub const ROW_COUNT: usize = 4;

pub struct Tools {
	pub cursor: ArcTool,
	pub pan: ArcTool,
	pub select: ArcTool,
	pub delete: ArcTool,
	pub undo: ArcTool,
	pub redo: ArcTool,
	pub move_: ArcTool,

	pub rows: [[ArcTool; 10]; ROW_COUNT]
}

impl Tools {
	pub fn from_id(&self, id: usize) -> Option<ArcTool> {
		Some(match id {
			0 => self.move_.clone(),
			1 => self.pan.clone(),
			2 => self.select.clone(),
			3 => self.delete.clone(),
			4 => self.undo.clone(),
			5 => self.redo.clone(),
			6..=9 => return None,
			id => if id >= ROW_COUNT * 10 + 10 {
				return None;
			} else {
				self.rows[id / 10 - 1][id % 10].clone()
			}
		})
	}
}

lazy_static!{
	pub static ref TOOLS: Tools = Tools {
		cursor: Arc::new(ToolCursor::default()),
		pan: Arc::new(ToolPan::default()),
		select: Arc::new(ToolSelect::default()),
		delete: Arc::new(ToolDelete::default()),
		undo: Arc::new(ToolUndo::undo()),
		redo: Arc::new(ToolUndo::redo()),
		move_: Arc::new(ToolMove::default()),

		rows: [[
			Arc::new(ToolTile::new(Tile::GroundBase)),
			Arc::new(ToolTile::new(Tile::BGCommon1)),
			Arc::new(ToolSimple::new(ObjectType::BrickBlock)),
			Arc::new(ToolSimple::new(ObjectType::Semisolid)),
			Arc::new(ToolSpike::default()),
			Arc::new(ToolSimple::new(ObjectType::SpringBasic)),
			Arc::new(ToolSimple::new(ObjectType::Ice(false))),
			Arc::new(ToolSimple::new(ObjectType::BounceBlock)),
			Arc::new(ToolTile::new(Tile::BlockRed)),
			Arc::new(ToolTile::new(Tile::AltGroundBase)),
		],
		[
			Arc::new(ToolSimple::new(ObjectType::Switch(SwitchType::Coloured(Colour::Red), false))),
			Arc::new(ToolSimple::new(ObjectType::SwitchBlock(Colour::Red, false))),
			Arc::new(ToolSimple::new(ObjectType::Key(Colour::Red))),
			Arc::new(ToolSimple::new(ObjectType::Lock(Colour::Red))),
			Arc::new(ToolSimple::new(ObjectType::Gem(Colour::Red))),
			Arc::new(ToolSimple::new(ObjectType::GemHole(Colour::Red))),
			Arc::new(ToolSimple::new(ObjectType::GemBlock(Colour::Red))),
			Arc::new(ToolSimple::new(ObjectType::Detector)),
			Arc::new(ToolSimple::new(ObjectType::RumiaSwitch)),
			Arc::new(ToolSimple::new(ObjectType::CirnoSwitch)),
		],
		[
			Arc::new(ToolSimple::new(ObjectType::Conveyor(true))),
			Arc::new(ToolSimple::new(ObjectType::Icicle)),
			Arc::new(ToolPairedObject { object: PairedObject::Warp }),
			Arc::new(ToolPairedObject { object: PairedObject::Teleporter }),
			Arc::new(ToolSimple::new(ObjectType::Bomb)),
			Arc::new(ToolCannon::default()),
			Arc::new(ToolSimple::new(ObjectType::BlinkyBlock(false))),
			Arc::new(ToolSimple::new(ObjectType::Bell)),
			Arc::new(ToolChandelier::default()),
			Arc::new(ToolMochi::default()),
		],
		[
			Arc::new(ToolSimple::new(ObjectType::PuzzlePiece)),
			Arc::new(ToolExtraHead::default()),
			Arc::new(ToolTile::new(Tile::Water)),
			Arc::new(ToolSimple::new(ObjectType::Fairy(false))),
			Arc::new(ToolOnmyoudamaShoot::default()),
			Arc::new(ToolOnmyoudamaCrawl::default()),
			Arc::new(ToolFlipper::default()),
			Arc::new(ToolPairedObject { object: PairedObject::MovingPlatform }),
			Arc::new(ToolSymbol::default()),
			Arc::new(ToolTileSplit::default()),
		]]
	};
}

pub trait Tool {
	fn use_start(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) -> Vec<UndoAction>;
	fn use_new_tile(
		&self,
		#[allow(unused_variables)] command_sender: &mut CommandSender,
		#[allow(unused_variables)] editor: &mut Editor,
		#[allow(unused_variables)] x: f32,
		#[allow(unused_variables)] y: f32
	) -> Vec<UndoAction> {vec![]}
	fn use_frame(
		&self,
		#[allow(unused_variables)] command_sender: &mut CommandSender,
		#[allow(unused_variables)] editor: &mut Editor,
		#[allow(unused_variables)] x: f32,
		#[allow(unused_variables)] y: f32
	) {}
	fn use_end(&self, #[allow(unused_variables)] command_sender: &mut CommandSender, #[allow(unused_variables)] editor: &mut Editor) {}
	fn sprite(&self, #[allow(unused_variables)] theme: LevelTheme, #[allow(unused_variables)] character: Character) -> i32 {-1}
	fn clear_selection(&self) -> bool {true}
	fn context_menu_items(&self, #[allow(unused_variables)] theme: LevelTheme) -> Vec<ContextMenuItem> {vec![]}
	fn handle_context_menu_action(&self, #[allow(unused_variables)] action: i32, #[allow(unused_variables)] theme: LevelTheme) {unreachable!()}
	fn block_context_menu(&self) -> bool {false}
	fn can_be_used_zoomed_out(&self) -> bool {false}
}