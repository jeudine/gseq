use crate::action::Action;
use crate::instance::Instance;

pub struct Item {
	pub file_name: String,
	pub params: Vec<(Instance, Action)>,
}
