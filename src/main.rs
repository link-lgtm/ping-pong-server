use macroquad::prelude::*;

mod block; 
mod ball; 
mod collision; 

use ball::Ball; 
use block::Block; 
use collision::{CollisionState, get_block_collision_time, get_wall_collision_time}; 

fn window_conf() -> Conf {
    Conf {
        window_title: "swiper".to_owned(),
        ..Default::default()
    }
}

fn init_vec(cat: &Texture2D, map_blocks: &mut Vec<Block>) {
    map_blocks.clear(); 
    let row_blocks = 5; 
    let col_blocks = 9; 
    for i in 0..row_blocks {
        for j in 0..col_blocks { 
            let x = screen_width() * (2.*i as f32 + 1.) / 11.; 
            let w = screen_width() / 11.; 
            let y = screen_height() * (2.*j as f32 + 1.) / 32.; 
            let h = screen_height() / 32.; 
            let tex = Some(cat.clone()); 
            let bb : Block = Block{x,w,y,h,tex,hp:1,vx:0.}; 
            map_blocks.push(bb); 
        }
    }
}

fn swappy(idx: &mut i32, i: usize, col_state : CollisionState, state : &mut CollisionState) {
    if *state > col_state {
        *state = col_state; *idx = i as i32; 
    }
}



#[macroquad::main(window_conf)]
async fn main() {
    let rustacean_tex = load_texture("./kuromi.png").await.unwrap(); 
    let mut kuromi_ball : Ball = Ball::new(Some(rustacean_tex.clone())); 
    let mut stick : Block = Block{x:screen_width() / 2. - 30.0, y: screen_height() * 0.9, w:200.0, h:80.0, tex: Some(load_texture("./rustacean_happy.png").await.unwrap()), hp:10000, vx:screen_width() / 2.}; 

    let mut map_blocks : Vec<Block> = Vec::new(); 
    let tex_cat = load_texture("./cat.png").await.unwrap(); 
    init_vec(&tex_cat, &mut map_blocks); 
    let mut game_over : bool = false; 
    let mut score = 0; 
    loop {
        //println!("fps : {}",get_fps());
        if !game_over {
            if kuromi_ball.vy > 0. && kuromi_ball.y + kuromi_ball.r + kuromi_ball.vy * get_frame_time() > screen_height() {
                println!("{:?}",kuromi_ball); 
                game_over = true; 
            }
        }
        if !game_over {     
            clear_background(GRAY);
            let score_text = format!("score : {}",score); 
            let font_size = 20.; 
            let text_size = measure_text(&score_text, None, font_size as _, 1.0);

            draw_text(
                &score_text,
                10., 40., 
                font_size,
                DARKGRAY,
            );
            /* 
            각 블록, 스틱, 벽에 대해서 체크 후 시간 제일 짧은거 -> 실행 반복 
            */
            let mut time_limit = get_frame_time(); 

            loop {
                let mut idx = -1 as i32; // 0~size-1 -> map_blocks size -> stick size+1 -> wall 
                let mut state = CollisionState::trash(); 
                for i in 0.. map_blocks.len() {
                    swappy(&mut idx, i, get_block_collision_time(& kuromi_ball, & map_blocks[i], time_limit), &mut state); 
                }
                swappy(&mut idx, map_blocks.len(), get_block_collision_time(& kuromi_ball, & stick, time_limit), &mut state); 
                swappy(&mut idx, map_blocks.len() + 1 as usize, get_wall_collision_time(& kuromi_ball, time_limit), &mut state); 
                
                if idx == -1 {
                    break; 
                }
                let ll = map_blocks.len() as i32 ; 
                if idx < ll {
                    map_blocks.remove(idx as usize); 
                    score += 1; 
                }
                kuromi_ball.process_collision_state(&state); 
                time_limit -= state.hit_time.unwrap(); 
            }
            kuromi_ball.move_ball(time_limit);
            stick.move_block(); 
            // draw 
            map_blocks.iter().for_each(|x| x.draw()); 
            kuromi_ball.draw(); 
            stick.draw(); 
        } 
        if game_over {
            clear_background(GRAY); 
            let text = format!("Game is over. Final score : {}. Press [Enter] to play again.",score); 
            let font_size = 30.; 
            let text_size = measure_text(&text, None, font_size as _, 1.0);

            draw_text(
                &text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. + text_size.height / 2.,
                font_size,
                DARKGRAY,
            );

            if is_key_down(KeyCode::Enter) {
                score = 0; 
                kuromi_ball = Ball::new(Some(rustacean_tex.clone())); 
                stick = Block{x:screen_width() / 2. - 30.0, y: screen_height() * 0.9, w:200.0, h:80.0, tex: Some(load_texture("./rustacean_happy.png").await.unwrap()), hp:10000, vx:screen_width() / 2.}; 
                game_over = false; 
                init_vec(&tex_cat, &mut map_blocks); 
            }  

        }
        next_frame().await
    }
}