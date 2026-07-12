use macroquad::prelude::*; 
use break_the_wall::game;
use break_the_wall::protocol::{GameState, GameInput, Button};
mod render;

fn window_conf() -> Conf {
    Conf {
        window_title: "sososososososos".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = game::init::game_create(); 
    let obstacle_tex = load_texture("./cat.png").await.unwrap(); 
    let ball_tex = load_texture("./kuromi.png").await.unwrap(); 
    let player_tex = load_texture("./perris.png").await.unwrap(); 
    let mut accumulate_time = 0.; 
    let mut step = 0; 
    loop {
        if !game_state.is_game_over {
            accumulate_time += get_frame_time(); 
            let p1gameinput = match (is_key_down(KeyCode::W),is_key_down(KeyCode::S) ){
                (false, true) => GameInput{id:1,sequence_number:0,button: Button::Down}, 
                (true,false) => GameInput{id:1,sequence_number:0,button: Button::Up}, 
                (_, _) => GameInput{id:1,sequence_number:0,button: Button::None},
            }; 

            let p2gameinput = match (is_key_down(KeyCode::Up),is_key_down(KeyCode::Down) ){
                (false, true) => GameInput{id:1,sequence_number:0,button: Button::Down}, 
                (true,false) => GameInput{id:1,sequence_number:0,button: Button::Up}, 
                (_, _) => GameInput{id:1,sequence_number:0,button: Button::None},
            }; 
            while step < (accumulate_time / game::config::TICK_TIME) as i32 {
                game::update(&mut game_state, &p1gameinput, &p2gameinput, game::config::TICK_TIME); 
                step += 1; 
            }
            render::draw_game(&game_state, Some(obstacle_tex.clone()), Some(ball_tex.clone()), Some(player_tex.clone())).await; 
        }
        next_frame().await
    }
}

