use crate::colors::YELLOW;

use crate::environment::element::{Element, ElementData, ElementKind};

pub struct Sand {}
impl ElementData for Sand {
	fn new() -> Element {
		Element {
			kind: ElementKind::Sand,
			color: YELLOW
		}
	}

	fn tick(elem: &Element, world: &mut crate::environment::world::World) {
		println!("{:?} {:?} ticked", elem.color, elem.kind);
	}
}