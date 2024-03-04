use bytemuck::{Pod, Zeroable};

use crate::{math::Colour, Vec2};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Element {
    pos: Vec2,
    size: Vec2,
    colour: Colour
}

impl Element {
    pub fn new(pos: Vec2, size: Vec2, colour: Colour) -> Self {
        Element { pos, size, colour }
    }
}
