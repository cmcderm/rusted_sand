use crate::element::{Element, ElementData, ElementKind};

pub struct Air {}
impl ElementData for Air {
	fn new() -> Element {
		Element {
			kind: ElementKind::Air,
			color: BLANK
		}
	}
}