use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "swiper".to_owned(),
        ..Default::default()
    }
}

struct Kuromi {
    x: f32, 
    y: f32, 
    r: f32,
    tex: Texture2D,
}

impl Kuromi {
    fn draw(&self) {
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
        let wtf : Mesh = Mesh { vertices: vertices, indices : indices, texture : Some(self.tex.clone())};
        draw_mesh(&wtf); 
    }
}

struct block {
    x: f32, 
    y: f32,
    w: f32, 
    h: f32, 
    tex: Option<Texture2D>, 
}

impl block {
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
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut dx : f32 = rand::gen_range(-6., 6.); 
    let mut dy : f32 = rand::gen_range(-6., 6.); 
    let rustacean_tex = load_texture("./kuromi.png").await.unwrap(); 
    let mut kuromi_ : Kuromi = Kuromi{x:60.,y:60.,r:60.,tex:rustacean_tex.clone()}; 
    let mut stick : block = block{x:screen_width() / 2. - 30.0, y: screen_height() * 0.9, w:200.0, h:80.0, tex: Some(load_texture("./rustacean_happy.png").await.unwrap())}; 

    // draw texture 
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


            if kuromi_.x - kuromi_.r < 0. {
                dx = rand::gen_range(2.0, 4.0); 
            }

            if kuromi_.x + kuromi_.r > screen_width() {
                dx = rand::gen_range(-4.0, -2.0);  
            }

            if kuromi_.y - kuromi_.r < 0. {
                dy = rand::gen_range(2.0, 4.0); 
            }

            if kuromi_.y + kuromi_.r > screen_height() {
                game_over = true; 
            }

            if is_key_down(KeyCode::Left) && stick.x > 0. {
                stick.x -= 4.0; 
            } 

            if is_key_down(KeyCode::Right) && stick.x + stick.w < screen_width() {
                stick.x += 4.0; 
            }   
            //println!("{}",(kuromi_.y + kuromi_.r - stick.y).abs());

            if (kuromi_.y + kuromi_.r - stick.y).abs() < 2.0 && kuromi_.x >= stick.x && kuromi_.x <= stick.x + stick.w { //&& kuromi_.x >= stick.x && kuromi_.x <= stick.x + stick.w 
                dy = -dy; 
                println!("kuromi hit!!!!!!"); 
                score += 1; 
            }

            kuromi_.x = kuromi_.x + dx; 
            kuromi_.y = kuromi_.y + dy; 
            kuromi_.draw(); 
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
                kuromi_ = Kuromi{x:60.,y:60.,r:60.,tex:rustacean_tex.clone()}; 
                stick = block{x:screen_width() / 2. - 30.0, y: screen_height() * 0.9, w:200.0, h:80.0, tex: Some(load_texture("./rustacean_happy.png").await.unwrap())}; 
                game_over = false; 
            }  
        }

        next_frame().await
    }
}