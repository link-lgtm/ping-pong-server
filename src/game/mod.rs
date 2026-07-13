pub mod collision;
pub mod rec; 
pub mod ball; 
pub mod init; 
pub mod config; 

use super::protocol::{GameInput,GameState}; 
use collision::{CollisionState, Dir, get_rec_collision_time, get_wall_collision_time}; 

fn swappy(idx: &mut i32, i: usize, col_state : CollisionState, state : &mut CollisionState) {
    if *state > col_state {
        *state = col_state; *idx = i as i32; 
    }
}

pub fn update(game_state : &mut GameState, p1_game_input: &GameInput, p2_game_input: &GameInput, dt: f32) {
    // assert!(!game_state.is_game_over); 
    let mut time_limit = dt; 
    let ball = &mut game_state.ball; 
    let map_blocks = &mut game_state.obstacles; 
    let player1 = &mut game_state.player1; 
    let player2 = &mut game_state.player2; 
    game_state.tick += 1; 
    loop {
        // println!("wtf??? kuromi?????"); 
        let mut idx = -1 as i32; // 0~size-1 -> map_blocks size -> stick size+1 -> wall 
        let mut state = CollisionState::trash(); 
        for i in 0.. map_blocks.len() {
            swappy(&mut idx, i, get_rec_collision_time(ball, &map_blocks[i].rec, time_limit), &mut state); 
        }
        swappy(&mut idx, map_blocks.len(), get_rec_collision_time(ball, &player1.rec, time_limit), &mut state); 
        swappy(&mut idx, map_blocks.len() + 1 as usize, get_rec_collision_time(ball, &player2.rec, time_limit), &mut state);
        swappy(&mut idx, map_blocks.len() + 2 as usize, get_wall_collision_time(& ball, time_limit), &mut state); 

        if idx == -1 {
            break; 
        }

        let ll = map_blocks.len() as i32 ; 
        if idx < ll {
            map_blocks[idx as usize].hp -= 1; 
            if map_blocks[idx as usize].hp == 0 {
                map_blocks.remove(idx as usize); 
            }
        }
        if idx == ll+2 && state.hit_dir == Dir::LR {
            game_state.is_game_over = true; break; 
        }
        // println!("{:?} {:?} {:?}", state, ball, idx); 
        ball.process_collision_state(&state); 
        time_limit -= state.hit_time.unwrap() ; 
    }
    // println!("wtf?? time {:?}",time_limit); 
    ball.advance(time_limit);
    
    player1.advance(dt, &p1_game_input.button);
    player2.advance(dt, &p2_game_input.button);
}


