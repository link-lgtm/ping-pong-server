/*
client 와 server 가 통신할 프로토콜을 정의한다. 

1. client -> server (Input) 
2. server -> client (GamState)
*/

use super::game::ball::Ball; 
use super::game::rec::*; 
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct GameState {
    pub tick: i32, 
    pub player1: Player, 
    pub player2: Player, 
    pub ball: Ball, 
    pub obstacles: Vec<Obstacle>,
    pub is_game_over: bool,
}

#[derive(Serialize, Deserialize, Default)]
pub enum Button {
    Up, 
    Down,
    #[default] None, 
}

#[derive(Serialize, Deserialize, Default)]
pub struct GameInput {
    pub id: i32, 
    pub sequence_number: i32, 
    pub button: Button, 
}