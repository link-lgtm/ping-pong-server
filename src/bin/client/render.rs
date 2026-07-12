use break_the_wall::protocol::GameState;
use macroquad::prelude::*; 
use break_the_wall::game::ball::Ball; 
use break_the_wall::game::rec::Rec; 
use break_the_wall::game::config::*; 

// Game 객체 -> render 를 역할함. 

fn transpose(x:f32,y:f32) -> [f32;2] {
    let xx = x * screen_width() / SCREEN_WIDTH; 
    let yy = y * screen_height() / SCREEN_HEIGHT; 
    [xx,yy] 
}

pub fn draw_ball(ball: &Ball, ball_color:Color, tex:Option<Texture2D>) {
    let sides = 40; 
    let mut vertices = Vec::<Vertex>::with_capacity(sides as usize + 2);
    let mut indices = Vec::<u16>::with_capacity(sides as usize * 3);
    let rot = 0.0; 
    let [x,y] = transpose(ball.x, ball.y); 
    let [srx,sry] = transpose(ball.r, ball.r);
    vertices.push(Vertex::new(x,y,0.,0.5,0.5,ball_color)); 
    for i in 0..=sides {

        let rx = (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).cos();
        let ry = (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).sin();

        let vertex = Vertex::new(x + srx * rx, y + sry * ry, 0., 0.5 + 0.5 * rx , 0.5 + 0.5 * ry , ball_color);

        vertices.push(vertex);
        if i != sides {
            indices.extend_from_slice(&[0, i as u16 + 1, i as u16 + 2]);
        }
    }
    let wtf : Mesh = Mesh { vertices: vertices, indices : indices, texture : tex};
    draw_mesh(&wtf); 
}

pub fn draw_rec(rec:&Rec, rec_color:Color, tex:Option<Texture2D>) {
    let [x,y] = transpose(rec.x, rec.y); 
    let [w,h] = transpose(rec.w, rec.h); 
    let mut vertices = Vec::<Vertex>::new(); 
    let mut indices = Vec::<u16>::new(); 
    vertices.push(Vertex::new(x,y,0.,0.,0.,rec_color)); 
    vertices.push(Vertex::new(x, y + h, 0., 0., 1.,rec_color)); 
    vertices.push(Vertex::new(x+w, y + h,0.,1.,1.,rec_color)); 
    vertices.push(Vertex::new(x+w,y,0.,1.,0.,rec_color));
    indices.extend_from_slice(&[0,1,2]); 
    indices.extend_from_slice(&[0,2,3]); 
    let wtf : Mesh = Mesh { vertices: vertices, indices : indices, texture : tex}; 
    draw_mesh(&wtf); 
}

pub async fn draw_game(game_state:&GameState, obstacle_tex:Option<Texture2D>, ball_tex:Option<Texture2D>, player_tex:Option<Texture2D>) {
    if game_state.is_game_over {
        clear_background(GRAY); 
        let text = format!("Game is over. Press [Enter] to play again."); 
        let font_size = 30.; 
        let text_size = measure_text(&text, None, font_size as _, 1.0);

        draw_text(
            &text,
            screen_width() / 2. - text_size.width / 2.,
            screen_height() / 2. + text_size.height / 2.,
            font_size,
            DARKGRAY,
        );
    }
    else {
        clear_background(GRAY); 
        for x in &game_state.obstacles {
            draw_rec(&x.rec, WHITE, obstacle_tex.clone()); 
        }
        draw_ball(&game_state.ball, WHITE, ball_tex); 
        draw_rec(&game_state.player1.rec, WHITE, player_tex.clone()); 
        draw_rec(&game_state.player2.rec, WHITE, player_tex.clone()); 
    }
}