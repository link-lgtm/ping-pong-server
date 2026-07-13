use serde::{Deserialize, Serialize};

use super::collision::{CollisionState,Dir}; 

#[derive(Debug,Default,Serialize,Deserialize)]
pub struct Ball {
    pub x: f32, 
    pub y: f32, 
    pub r: f32,
    pub vx: f32, 
    pub vy: f32, 
}

impl Ball {
    pub fn new() -> Ball {
        Ball::default() 
    }

    pub fn advance(&mut self, dt:f32) {
        self.x += self.vx * dt; 
        self.y += self.vy * dt; 
    }

    pub fn process_collision_state(&mut self, collision_state : &CollisionState) { 
        println!("hit before : {:?} {:?}", self, collision_state); 
        self.advance(collision_state.hit_time.unwrap()); 
        println!("first hit : {:?}", self); 
        match collision_state.hit_dir {
            Dir::LR => {
                self.vx = - self.vx 
            },  
            Dir::UD => {
                self.vy = - self.vy; 
            }, 
        }
        self.advance(0.00001);  // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        println!("second advance {:?}", self); 
    }

}

