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
    fn print_object(&self) {
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


#[macroquad::main(window_conf)]
async fn main() {
    let mut dx : f32 = rand::gen_range(-6., 6.); 
    let mut dy : f32 = rand::gen_range(-6., 6.); 
    let rustacean_tex = load_texture("./image.png").await.unwrap(); 
    let mut kuromi_ : Kuromi = Kuromi{x:60.,y:60.,r:60.,tex:rustacean_tex}; 

    // draw texture 
    loop {
        clear_background(RED);

        if kuromi_.x - kuromi_.r < 0. {
            dx = rand::gen_range(2.0, 7.0); 
        }

        if kuromi_.x + kuromi_.r > screen_width() {
            dx = rand::gen_range(-7.0, -2.0);  
        }

        if kuromi_.y - kuromi_.r < 0. {
            dy = rand::gen_range(2.0, 7.0); 
        }

        if kuromi_.y + kuromi_.r > screen_height() {
            dy = rand::gen_range(-7.0, -2.0); 
        }

        kuromi_.x = kuromi_.x + dx; 
        kuromi_.y = kuromi_.y + dy; 
        kuromi_.print_object();
        next_frame().await
    }
}