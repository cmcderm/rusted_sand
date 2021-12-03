use crate::environment::element::Element;

#[derive(Debug)]
pub struct Map {
	
}

#[derive(Debug)]
pub struct World {
	pub map: Vec<Vec<Element>>
}
impl World {
	pub fn new(x: i32, y: i32) -> World {
		World {
			map: vec![vec![Element; y]; x]
		}
	}
}

pub struct Position {
	pub x: i32,
	pub y: i32,
}