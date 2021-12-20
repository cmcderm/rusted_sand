pub mod air;
pub mod sand;
pub mod water;

use std::fmt::Debug;
use macroquad::prelude::Color;

use crate::environment::world::World;

pub use air::*;
pub use sand::*;
pub use water::*;

#[derive(Debug, Clone)]
pub enum ElementKind {
	Air,
	Sand,
	Water
}

trait ElementData{
	fn new() -> Element;
	fn tick(elem: &Element, world: &mut World);
}

#[derive(Debug, Clone)]
pub struct Element {
	pub kind: ElementKind,
	pub color: Color,
}
impl Element {
	pub fn new_element_from_kind(kind: ElementKind) -> Element {
		match kind {
			ElementKind::Air => Air::new(),
			ElementKind::Sand => Sand::new(),
			ElementKind::Water => Water::new(),
		}
	}

	pub fn tick(&self, world: &mut World) {
		match self.kind {
			ElementKind::Air => Air::tick(&self, world),
			ElementKind::Sand => Sand::tick(&self, world),
			ElementKind::Water => Water::tick(&self, world),
		}
	}
}

fn new_element_from_kind(kind: ElementKind) -> Element {
	match kind {
		ElementKind::Air => Air::new(),
		ElementKind::Sand => Sand::new(),
		ElementKind::Water => Water::new()
	}
}