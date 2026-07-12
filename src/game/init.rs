/* 
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
    */ 