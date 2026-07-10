use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "swiper".to_owned(),
        ..Default::default()
    }
}

struct Ball {
    x: f32, 
    y: f32, 
    r: f32,
    tex: Option<Texture2D>,
    vx: f32, 
    vy: f32, 
}


impl Ball {
    fn new(tex : Option<Texture2D>) -> Ball {
        Ball {
            x:60., 
            y:60.,
            r:40.,
            tex,
            vx:screen_width() * rand::gen_range(0.3,0.5), 
            vy:screen_height() * rand::gen_range(0.3,0.5), 
        }
    }

    fn draw(&self) {
        //println!("wtf?? {} {} {} {}",self.x, self.y, screen_height(), screen_width()); 
        let sides = 40; 
        let mut vertices = Vec::<Vertex>::with_capacity(sides as usize + 2);
        let mut indices = Vec::<u16>::with_capacity(sides as usize * 3);
        let rot = 0.0; 
        vertices.push(Vertex::new(self.x,self.y,0.,0.5,0.5,WHITE)); 
        for i in 0..=sides {
            let rx = (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).cos();
            let ry = (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).sin();

            let vertex = Vertex::new(self.x + self.r * rx, self.y + self.r * ry, 0., 0.5 + 0.5 * rx , 0.5 + 0.5 * ry , WHITE);

            vertices.push(vertex);

            if i != sides {
                indices.extend_from_slice(&[0, i as u16 + 1, i as u16 + 2]);
            }
        }
        let wtf : Mesh = Mesh { vertices: vertices, indices : indices, texture : Some(self.tex.as_ref().unwrap().clone())};
        draw_mesh(&wtf); 
    }

    fn check_hit_the_wall(&mut self) -> bool{
        let mut game_over:bool = false; 
        if self.x - self.r < 0. {
            self.x = self.r + 1.; 
            self.vx = -self.vx;
        }

        if self.x + self.r > screen_width() {
            self.x = screen_width() - self.r - 1.; 
            self.vx = -self.vx;
        }

        if self.y - self.r < 0. {
            self.y = self.r + 1.; 
            self.vy = -self.vy;
        }

        if self.y + self.r > screen_height() {
            game_over = true; 
        }
        game_over 
    }

    fn move_ball(&mut self) {
        let fps : f32 = get_frame_time(); 
        self.x += self.vx * fps; 
        self.y += self.vy * fps; 
    }
}

struct Block {
    x: f32, 
    y: f32,
    w: f32, 
    h: f32, 
    tex: Option<Texture2D>, 
    hp: i32, 
    vx: f32,
}

impl Block {
    fn draw(&self) {
        let mut vertices = Vec::<Vertex>::new(); 
        let mut indices = Vec::<u16>::new(); 
        vertices.push(Vertex::new(self.x,self.y,0.,0.,0.,WHITE)); 
        vertices.push(Vertex::new(self.x, self.y + self.h,0., 0., 1.,WHITE)); 
        vertices.push(Vertex::new(self.x+self.w, self.y + self.h,0.,1.,1.,WHITE)); 
        vertices.push(Vertex::new(self.x+self.w,self.y,0.,1.,0.,WHITE));
        indices.extend_from_slice(&[0,1,2]); 
        indices.extend_from_slice(&[0,2,3]); 
        let wtf : Mesh = Mesh { vertices: vertices, indices : indices, texture : self.tex.clone()}; 
        draw_mesh(&wtf); 
    }

    fn move_block(&mut self) {
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

enum dir {
    LR, 
    UD, 
    None, 
}

struct collision_state{
    hit_dir:dir, 
    hit_time:f32, 
}

impl collision_state { 
    
}

fn check_block_collision_time(& ball:Ball, & block:Block, time_limit:f32) -> collision_state{
    // ball.x -> ball.x + ball.vx * fps (get_frame_time()) 
    // 만날때까지 시간 게산하기 
    // [block.x-ball.r, block.x+block.w+ball.r] x [block.y-ball.r, block.y+block.h+ball.r] 에 도달? -> 아 time interval idea!!!! 
    let mut tx = (block.x - ball.r - ball.x) / ball.vx; 
    let mut txx = (block.x+block.w+ball.r - ball.x) / ball.vx; 
    let mut ty = (block.y - ball.r - ball.x) / ball.vy; 
    let mut tyy = (block.y+block.h+ball.r - ball.y) / ball.vy; 
    if tx>txx {
        let mut tmp = tx; 
        tx = txx; 
        txx = tmp; 
    } 

    if ty>tyy {
        let mut tmp = ty; 
        ty = tyy; 
        tyy = tmp; 
    }
    // tx = tx.max(0.); txx = txx.max(0.); ty = ty.max(0.); tyy = tyy.max(0.); 
    if txx < 0.0 || tyy < 0.0 || tx > tyy || ty > txx{
        return collision_state::None; 
    }
    let col_t = tx.max(ty); 
    if col_t > time_limit {
        return collision_state::None; 
    }
    let dir_ : dir = dir::None; 
    if tx > ty {
        dir_ = dif::LR; 
    } else {
        dir_ = dif::UD; 
    }
    return collision_state{hit_dir:dir_, hit_time:col_t}; 
}

fn check_wall_collision_time(& ball: Ball, time_limit:f32) -> collision_state { 
    let mut tx = 0.; 
    let mut ty = 0.; 
    if ball.vx > 0. {
        tx = (screen_width() - ball.x - ball.r) / ball.vx; 
    } else {
        tx = (ball.x - ball.r) / ball.vx.abs(); 
    } 

    if ball.vy > 0. {
        ty = (screen_height() - ball.y - ball.r) / ball.vy; 
    } else {
        ty = (ball.y - ball.r) / ball.vy.abs(); 
    }

    if tx.min(ty) < time_limit {
        return collision_state::None; 
    } 
    let dif_ : dif = dif::None; 
    if tx > ty {
        dir_ = dif::UD; 
    } else {
        dir_ = dif::LR; 
    }
    return collision_state{hit_dir:dir_, hit_time:tx.min(ty)}; 
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
            let mut bb : Block = Block{x,w,y,h,tex,hp:1,vx:0.}; 
            map_blocks.push(bb); 
        }
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
    // draw texture 
    let mut game_over : bool = false; 
    let mut score = 0; 
    loop {
        println!("fps : {}",get_fps());
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
                let mut idx = -1; // 0~size-1 -> map_blocks size -> stick size+1 -> wall 
                let mut state = 1271894789142.; 
                for i in 0.. map_blocks.size() {
                    let cur_state = check_block_collision_time(& kuromi_ball, & map_blocks[i]); 
                    if let Hit(t) = cur_state {
                        if t < state {
                            idx = i; 
                            state = t; 
                        }
                    }
                }
                {
                    let cur_state = check_block_collision_time(& kuromi_ball, & stick); 
                    if let Hit(t) = cur_state {
                        if t < state {
                            idx = map_blocks.size() as i32; 
                            state = t; 
                        }
                    }
                } 
                {
                    let cur_state = check_wall_collision_time(& kuromi_ball); 
                    if let Hit(t) = cur_state {
                        if t < state {
                            idx = map_blocks.size() as i32; 
                            state = t; 
                        }
                    }
                }
                if idx == -1 {
                    break; 
                }

            }


            let mut tmp = 1; 
            kuromi_ball.move_ball(); 
            stick.move_block(); 
            collision(&mut kuromi_ball,&mut stick,&mut tmp); 
            if let Some(index) = map_blocks.iter_mut().position(|bb| collision(&mut kuromi_ball, bb, &mut score)) {
                map_blocks.remove(index); 
            }
            map_blocks.iter().for_each(|x| x.draw()); 
            game_over = kuromi_ball.check_hit_the_wall(); 
            
            // first interact 


            // draw 
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