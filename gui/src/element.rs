use bytemuck::{Pod, Zeroable};

use crate::Vec2;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Element {
    pos: Vec2,
    size: Vec2,
}

impl Element {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        Element { pos, size }
    }
}
