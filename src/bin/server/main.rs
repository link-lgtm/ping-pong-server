use break_the_wall::protocol::{GameInput, GameState};
use break_the_wall::game;
use std::io::prelude::*; 
use std::thread;
use std::sync::mpsc;
use std::net::TcpStream;
use std::net::TcpListener; 
use serde_json;

fn push(mut tcpstream : TcpStream, rx : mpsc::Receiver<GameState>) {
    loop { 
        while let Ok(x) = rx.recv() {
            let v = serde_json::to_vec(&x).unwrap(); 
            tcpstream.write_all(& u32::to_le_bytes(v.len() as u32)).unwrap(); 
            tcpstream.write_all(& v).unwrap(); 
        }
    }
}

fn pull(mut tcpstream : TcpStream, tx : mpsc::Sender<GameInput>) {
    loop {
        let mut buf = [0 as u8; 4]; 
        while tcpstream.read(&mut buf).is_err() {} 
        let sz = u32::from_le_bytes(buf); 
        let mut buf = vec![0 as u8; sz as usize]; 
        while tcpstream.read_exact(&mut buf).is_err() {}; 
        let game_input : GameInput = serde_json::from_slice(&mut buf).unwrap(); 
        tx.send(game_input).unwrap(); 
    }

}

fn main() {
    let tcplistner = TcpListener::bind("127.0.0.1:1557").unwrap(); 

    let client1_stream = match tcplistner.accept() {
        Ok((_socket, addr)) => {
            println!("first client: {addr:?}"); 
            _socket 
        }, 
        Err(e) => {
            panic!("couldn't get client: {e:?}"); 
        }, 
    }; 

    let client2_stream = match tcplistner.accept() {
        Ok((_socket, addr)) => {
            println!("first client: {addr:?}"); 
            _socket 
        }, 
        Err(e) => {
            panic!("couldn't get client: {e:?}"); 
        }, 
    }; 
    
    // 입력 출력 

    let (tx_keyboard_input_client1, rx_keyboard_input_client1) = mpsc::channel(); // keyboard input -> network out으로 채널 이동 -> tcp send 
    let (tx_keyboard_input_client2, rx_keyboard_input_client2) = mpsc::channel();


    let client1_stream_ = client1_stream.try_clone().unwrap(); 
    let client2_stream_ = client2_stream.try_clone().unwrap(); 


    let _keyboard_channel_1 = thread::spawn(move || {
        pull(client1_stream, tx_keyboard_input_client1); 
    }); 

    let _keyboard_channel_2 = thread::spawn(move || {
        pull(client2_stream, tx_keyboard_input_client2); 
    }); 


    let (tx_gamestate_client1, rx_gamestate_client1) = mpsc::channel(); 
    let (tx_gamestate_client2, rx_gamestate_client2) = mpsc::channel(); 

    let _gamestate_channel_1 = thread::spawn(move || {
        push(client1_stream_, rx_gamestate_client1); 
    }); 

    let _gamestate_channel_2 = thread::spawn(move || {
        push(client2_stream_, rx_gamestate_client2); 
    }); 


    
    let mut last_time_tick = -1; 
    let mut last_game_state = GameState::default(); 
    loop {
        while step < (accumulate_time / game::config::TICK_TIME) as i32 {
            game::update(&mut game_state, &p1gameinput, &p2gameinput, game::config::TICK_TIME); 
            step += 1; 
        }
    }
}

