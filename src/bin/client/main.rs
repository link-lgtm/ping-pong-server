use macroquad::prelude::*; 
use break_the_wall::protocol::{GameInput, Button, GameState};
use std::io::prelude::*; 
use std::thread;
use std::sync::mpsc;
use std::net::TcpStream;
use serde_json;
mod render;
mod client; 


fn window_conf() -> Conf {
    Conf {
        window_title: "sososososososos".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

fn push(mut tcpstream : TcpStream, rx : mpsc::Receiver<GameInput>) {
    loop {
        while let Ok(x) = rx.recv() {
            let v = serde_json::to_vec(&x).unwrap();
            tcpstream.write_all(& u32::to_le_bytes(v.len() as u32)).unwrap(); 
            tcpstream.write_all(& v).unwrap(); 
        }
    }
}

fn pull(mut tcpstream : TcpStream, tx : mpsc::Sender<GameState>) {
    loop {
        let mut buf = [0 as u8; 4]; 
        while tcpstream.read(&mut buf).is_err() {} 
        let sz = u32::from_le_bytes(buf); 
        let mut buf = vec![0 as u8; sz as usize]; 
        while tcpstream.read_exact(&mut buf).is_err() {}; 
        let game_state : GameState = serde_json::from_slice(&mut buf).unwrap(); 
        tx.send(game_state).unwrap(); 
    }

}

#[macroquad::main(window_conf)]
async fn main() {
    let obstacle_tex = load_texture("./cat.png").await.unwrap(); 
    let ball_tex = load_texture("./kuromi.png").await.unwrap(); 
    let player_tex = load_texture("./perris.png").await.unwrap(); 

    let tcpstream = TcpStream::connect("127.0.0.1:1557").unwrap(); 
    let tcpstream1 = tcpstream.try_clone().unwrap(); 
    
    let (tx_keyboard_input, rx_keyboard_input) = mpsc::channel(); // keyboard input -> network out으로 채널 이동 -> tcp send 

    let _keyboard_channel = thread::spawn(move || {
        push(tcpstream, rx_keyboard_input); 
    }); 

    let (tx_gamestate, rx_gamestate) = mpsc::channel(); 

    let _gamestate_channel = thread::spawn(move || {
        pull(tcpstream1, tx_gamestate); 
    }); 

    let mut last_time_tick = -1; 
    let mut last_game_state = GameState::default(); 
    loop {
        let gameinput = match (is_key_down(KeyCode::Up),is_key_down(KeyCode::Down) ){
            (false, true) => GameInput{id:1,sequence_number:0,button: Button::Down}, 
            (true,false) => GameInput{id:1,sequence_number:0,button: Button::Up}, 
            (_, _) => GameInput{id:1,sequence_number:0,button: Button::None},
        }; 

        tx_keyboard_input.send(gameinput).unwrap(); 
        loop {
            if let Ok(game_state) = rx_gamestate.try_recv() {
                if game_state.tick > last_time_tick {
                    last_time_tick = game_state.tick; 
                    last_game_state = game_state; 
                }
            } else {
                break; 
            }
        }
        render::draw_game(&last_game_state, Some(obstacle_tex.clone()), Some(ball_tex.clone()), Some(player_tex.clone())); 
        next_frame().await
    }
}
