use macroquad::prelude::*;

mod block; 
mod ball; 
mod collision; 

use ball::Ball; 
use block::Block; 
use collision::{CollisionState, get_block_collision_time, get_wall_collision_time, Dir}; 

fn window_conf() -> Conf {
    Conf {
        window_title: "swiper".to_owned(),
        ..Default::default()
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
    let mut stick : Block = Block{x:screen_width() / 2. - 30.0, y: screen_height() * 0.9, w:200.0, h:80.0, tex: Some(load_texture("./perris.png").await.unwrap()), hp:10000, vx:screen_width() / 2.}; 

    let mut map_blocks : Vec<Block> = Vec::new(); 
    let tex_cat = load_texture("./cat.png").await.unwrap(); 
    init_vec(&tex_cat, &mut map_blocks); 
    let mut game_over : bool = false; 
    let mut score = 0; 
    loop {
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
            let mut time_limit = 0.007; 

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
                if idx == ll+1 && state.hit_dir == Dir::UD && kuromi_ball.vy > 0. {
                    game_over = true; break; 
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
                stick = Block{x:screen_width() / 2. - 30.0, y: screen_height() * 0.9, w:200.0, h:80.0, tex: Some(load_texture("./perris.png").await.unwrap()), hp:10000, vx:screen_width() / 2.}; 
                game_over = false; 
                init_vec(&tex_cat, &mut map_blocks); 
            }  

        }
        next_frame().await
    }
}



impl Block {
    pub fn move_block(&mut self) {
        if is_key_down(KeyCode::Left) {
            self.x -= self.vx * get_frame_time(); 
            if self.x < 0. {
                self.x = 0.; 
            }
        } 
        if is_key_down(KeyCode::Right) {
            self.x += self.vx * get_frame_time(); 
            if self.x + self.w > screen_width() {
                self.x = screen_width() - self.w; 
            }
        }   
    }
}

impl Ball {
    pub fn new() -> Ball {
        Ball::default() 
    }

    pub fn move_ball(&mut self, fps : f32) {
        self.x += self.vx * fps; 
        self.y += self.vy * fps; 
    }

    pub fn process_collision_state(&mut self, collision_state : &CollisionState) { 
        self.move_ball(collision_state.hit_time.unwrap()); 
        match collision_state.hit_dir {
            Dir::LR => {
                self.vx = - self.vx 
            },  
            Dir::UD => {
                self.vy = - self.vy; 
            }, 
        }
        self.move_ball(0.0000001);  // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    }
}


pub fn move_block(&mut self) {
    if is_key_down(KeyCode::Left) {
        self.x -= self.vx * get_frame_time(); 
        if self.x < 0. {
            self.x = 0.; 
        }
    } 
    if is_key_down(KeyCode::Right) {
        self.x += self.vx * get_frame_time(); 
        if self.x + self.w > screen_width() {
            self.x = screen_width() - self.w; 
        }
    }   
}

