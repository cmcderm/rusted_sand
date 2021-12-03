pub mod air;

use macroquad::prelude::{Color, BLANK, YELLOW};
use crate::environment::World;
use air::*;

enum ElementKind {
	Air,
	Sand,
	Water
}

trait ElementData{
	fn new() -> Element;
	fn tick(world: &mut World);
}

struct Element {
	pub kind: ElementKind,
	pub color: Color,
}
impl Element {
	fn new(kind: ElementKind) -> Element {
		
	}

	fn get_color(&self) -> &'static Color {
		panic!("NOT IMPLEMENTED");
	}

	fn tick(&self) {
		panic!("NOT IMPLEMENTED");
	}

	fn new_element_from_kind(kind: ElementKind) -> Element {
		match kind {
			ElementKind::Air => Air::new(),
			ElementKind::Sand => Sand::new(),
			ElementKind::Water => Water::new();
		}
	}
}

fn new_element_from_kind(kind: ElementKind) -> Element {
	match kind {
		ElementKind::Air => 
	}
}

pub struct Air {}
impl ElementData for Air {
	fn new() -> Element {
		Element {
			kind: ElementKind::Air,
			color: BLANK
		}
	}
}

struct Sand {}
impl ElementData for Sand {
	fn new() -> Element {
		Element {
			kind: water
		}
	}
}