use sdl2::{gfx::primitives::DrawRenderer, pixels::Color};

use super::{entity::Entity, position};

#[derive(Debug)]
pub struct Player {
    // player's collision detection is a circle.
    // collision_size is the radius of the circle.
    collision_size: i32,

    display_size: i32,

    // position is the character's position.
    position: position::Position,
}

impl Player {
    pub fn new(collision_size: i32, position: position::Position) -> Player {
        Player {
            collision_size,
            display_size: collision_size * 2,
            position,
        }
    }

    pub fn is_hit<T: Entity>(&self, other: T) -> bool {
        let distance = self.position.distance(other.position());
        distance < self.collision_size + other.collision_size()
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let (x, y) = self.position.aquire();
        canvas
            .filled_circle(
                x.try_into().unwrap(),
                y.try_into().unwrap(),
                self.display_size as i16,
                Color::BLACK,
            )
            .unwrap();
        canvas.present();
    }
}
