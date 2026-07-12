use super::ball::Ball; 
use super::config::*; 
use super::rec::{Obstacle,Player, Rec}; 
use crate::protocol::{GameState}; 

pub fn game_create() -> GameState {
    let tick = 0; 
    let player1 = Player {
        rec : Rec {
            x : 100., 
            y : 285., 
            w : 50., 
            h : 150.,
        }, 
        vx : 1000., 
        vy : 900., 
        score : 0, 
    }; 
    let player2 = Player { 
        rec : Rec {
            x : 1130., 
            y : 285., 
            w : 50., 
            h : 150.,
        }, 
        vx : 1000., 
        vy : 900., 
        score : 0, 
    }; 
    let ball = Ball {
        x : 640., 
        y : 360.,
        r : 40., 
        vx : 400., 
        vy : 400. 
    }; 
    let mut obstacles : Vec::<Obstacle> = Vec::new(); 
    obstacles.push(Obstacle{
        rec: Rec {
            x: 500., 
            y: 120., 
            w: 60., 
            h: 120., 
        }, 
        vx : 0., 
        vy : 0., 
        hp : 2, 
    }); 
    GameState{tick, player1, player2, ball, obstacles, is_game_over : false}
}