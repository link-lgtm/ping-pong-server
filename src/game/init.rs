use super::ball::Ball; 
use super::config::*; 
use super::rec::{Obstacle,Player, Rec}; 
use crate::protocol::{GameState}; 

// 게임 레벨만 ai 도움을 받았음. 

pub fn game_create() -> GameState {
    let tick = 0; 
    let player1 = Player {
        rec : Rec {
            x : 64., 
            y : 296., 
            w : 32., 
            h : 128.,
        }, 
        vx : 1000., 
        vy : 900., 
        score : 0, 
    }; 
    let player2 = Player { 
        rec : Rec {
            x : SCREEN_WIDTH - 96., 
            y : 296., 
            w : 32., 
            h : 128.,
        }, 
        vx : 1000., 
        vy : 900., 
        score : 0, 
    }; 
    let ball = Ball {
        x : SCREEN_WIDTH / 2., 
        y : SCREEN_HEIGHT / 2.,
        r : 18., 
        vx : 400., 
        vy : 400. 
    }; 
    let mut obstacles : Vec::<Obstacle> = Vec::new(); 
    obstacles.push(Obstacle{
        rec: Rec {
            x: 380., 
            y: 190., 
            w: 80., 
            h: 50., 
        }, 
        vx : 0., 
        vy : 0., 
        hp : 2, 
    }); 
    obstacles.push(Obstacle{
        rec: Rec {
            x: 820., 
            y: 190., 
            w: 80., 
            h: 50., 
        }, 
        vx : 0., 
        vy : 0., 
        hp : 2, 
    }); 
    obstacles.push(Obstacle{
        rec: Rec {
            x: 380., 
            y: 480., 
            w: 80., 
            h: 50., 
        }, 
        vx : 0., 
        vy : 0., 
        hp : 2, 
    }); 
    obstacles.push(Obstacle{
        rec: Rec {
            x: 820., 
            y: 480., 
            w: 80., 
            h: 50., 
        }, 
        vx : 0., 
        vy : 0., 
        hp : 2, 
    }); 
    obstacles.push(Obstacle{
        rec: Rec {
            x: 604., 
            y: 80., 
            w: 72., 
            h: 100., 
        }, 
        vx : 0., 
        vy : 0., 
        hp : 2, 
    }); 
    obstacles.push(Obstacle{
        rec: Rec {
            x: 604., 
            y: 540., 
            w: 72., 
            h: 100., 
        }, 
        vx : 0., 
        vy : 0., 
        hp : 2, 
    }); 
    GameState{tick, player1, player2, ball, obstacles, is_game_over : false}
}
