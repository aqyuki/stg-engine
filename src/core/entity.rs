use super::position;

pub trait Entity {
    fn position(&self) -> position::Position;
    fn collision_size(&self) -> i32;
}

#[derive(Debug)]
pub enum Type {
    Enemy,
}
