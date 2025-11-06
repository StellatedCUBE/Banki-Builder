use super::{command_handler::CommandSender, event::Event, global_state::ControllerGlobalState};

pub mod not_in_editor;
pub mod main_menu;
mod setup_menu;
mod pre_publish;
mod publish;
mod shape;
pub mod editor;
pub mod play;
pub mod error;

pub trait Menu {
	fn name(&self) -> &'static str;
	fn on_enter(&mut self, command_sender: &mut CommandSender);
	fn on_leave(&mut self, command_sender: &mut CommandSender);
	fn on_event(&mut self, command_sender: &mut CommandSender, event: Event, global_state: &mut ControllerGlobalState);

	fn create_chain_object_inserter(
		&mut self, 
		#[allow(unused_variables)] command_sender: &mut CommandSender,
		#[allow(unused_variables)] object: usize,
		#[allow(unused_variables)] index: usize,
		#[allow(unused_variables)] sprite: i32
	) {}
}