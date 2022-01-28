use crate::environment::element::Element;
use crate::environment::ElementKind;

#[derive(Debug)]
pub struct Map {
	elements: Vec<<Vec<Element>>,
	height: i32,
}

#[derive(Debug)]
pub struct World {
	pub map: Map,
}
impl World {
	pub fn new(height: i32, width: i32) -> World {
		World {
			map: Map {
				elements: vec![vec![Element::new_element_from_kind(ElementKind::Air); height]; width],
				height,
				width
			}
		}
	}

	pub fn tick(&self) {
		for row in self.map {
			for elem in row {
				elem.tick(&mut self);
			}
		}
	}
}

pub struct Position {
	pub x: i32,
	pub y: i32,
}