use crate::protocol::{Button}; 

#[derive(Debug, Default)]
pub struct Rec {
    pub x: f32, 
    pub y: f32,
    pub w: f32, 
    pub h: f32, 
}

pub struct Obstacle {
    pub rec: Rec, 
    pub vx : f32, 
    pub vy : f32,
    pub hp : i32, 
}
pub struct Player {
    pub rec: Rec, 
    pub score: i32, 
    pub vx : f32, 
    pub vy : f32, 
}


impl Obstacle {
    pub fn advance(&mut self, dt: f32) {
        self.rec.x += self.vx * dt; 
        self.rec.y += self.vy * dt; 
    }
}

impl Player {
    pub fn advance(&mut self, dt: f32, button: &Button) {
        match button {
            Button::Down => {
                self.rec.y += self.vy * dt; 
            }, 
            Button::Up => {
                self.rec.y -= self.vy * dt; 
            }, 
            _ => {
                
            }, 
        }
    }
}
