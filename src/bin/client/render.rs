use macroquad::prelude::*; 
use break_the_wall::game::ball::Ball; 
use break_the_wall::game::rec::Rec; 
use break_teh_wall::game::config::*; 

// Game 객체 -> render 를 역할함. 

fn transpose(x:f32,y:f32) -> [f32;2] {
    let xx = x * screen_width() / SCREEN_WIDTH; 
    let yy = y * screen_height() / SCREEN_HEIGHT; 
    [xx,yy] 
}

pub fn draw_ball(ball: &Ball) {
    let sides = 40; 
    let mut vertices = Vec::<Vertex>::with_capacity(sides as usize + 2);
    let mut indices = Vec::<u16>::with_capacity(sides as usize * 3);
    let rot = 0.0; 
    let [x,y] = transpose(ball.x, ball.y); 
    let [srx,sry] = transpose(ball.r, ball.r);
    vertices.push(Vertex::new(x,y,0.,0.5,0.5,WHITE)); 
    for i in 0..=sides {

        let rx = (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).cos();
        let ry = (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).sin();

        let vertex = Vertex::new(x + srx * rx, y + sry * ry, 0., 0.5 + 0.5 * rx , 0.5 + 0.5 * ry , WHITE);

        vertices.push(vertex);
        if i != sides {
            indices.extend_from_slice(&[0, i as u16 + 1, i as u16 + 2]);
        }
    }
    let wtf : Mesh = Mesh { vertices: vertices, indices : indices, texture : None};
    draw_mesh(&wtf); 
}


pub fn draw_rec(rec:&Rec) {
    let [x,y] = transpose(rec.x, rec.y); 
    let [w,h] = transpose(rec.w, rec.h); 
    let mut vertices = Vec::<Vertex>::new(); 
    let mut indices = Vec::<u16>::new(); 
    vertices.push(Vertex::new(x,y,0.,0.,0.,WHITE)); 
    vertices.push(Vertex::new(x, y + h, 0., 0., 1.,WHITE)); 
    vertices.push(Vertex::new(x+w, y + h,0.,1.,1.,WHITE)); 
    vertices.push(Vertex::new(x+w,y,0.,1.,0.,WHITE));
    indices.extend_from_slice(&[0,1,2]); 
    indices.extend_from_slice(&[0,2,3]); 
    let wtf : Mesh = Mesh { vertices: vertices, indices : indices, texture : None}; 
    draw_mesh(&wtf); 
}
