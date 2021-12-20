use crate::colors::BLUE;

use crate::environment::element::{Element, ElementData, ElementKind};

pub struct Water{}
impl ElementData for Water {
	fn new() -> Element {
		Element {
			kind: ElementKind::Water,
			color: BLUE,
		}
	}

	fn tick(elem: &Element, world: &mut crate::environment::world::World) {
		println!("{:?} {:?} ticked", elem.color, elem.kind);
	}
}