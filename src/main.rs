use macroquad::prelude::*;

#[macroquad::main("sprite stack")]
async fn main() {
    // a single radian.
    let rad: f32 = 1.0_f32.to_radians();
    // set of images that define the car. 
    // storing them in a vec is not good practice, usualy you create a single 
    //spritesheet with the images in it. i did it like this here to simplify the code
    let images = vec![
        load_texture("assets/img_0.png").await.unwrap(),
        load_texture("assets/img_1.png").await.unwrap(),
        load_texture("assets/img_2.png").await.unwrap(),
        load_texture("assets/img_3.png").await.unwrap(),
        load_texture("assets/img_4.png").await.unwrap(),
        load_texture("assets/img_5.png").await.unwrap(),
        load_texture("assets/img_6.png").await.unwrap(),
    ];
    // 
    let mut rotation = 0.0;
    let mut spread = 0;
    loop {
        clear_background(DARKGRAY);
        let input_tuple = (bool_int(is_key_pressed(KeyCode::A)),bool_int(is_key_pressed(KeyCode::D)));
        spread += input_tuple.0;
        spread -= input_tuple.1;
        rotation += rad;
        render_stack(&images,Vec2::new(250.0,250.0),rotation,spread);
        draw_text("press 'A' to increment the spread",0.0,30.0,48.0,RED);
        draw_text("press 'D' to decrement the spread",0.0,80.0,48.0,RED);
        draw_text(&format!("spread: {}",spread),0.0,140.0,48.0,RED);
        next_frame().await;
    }
}

// spread is how much distance is wanted in between the images
fn render_stack(images:&Vec<Texture2D>,pos:Vec2,rotation:f32,spread:i32) {
    for (i,image) in images.into_iter().enumerate() {
        let pos_x = pos.x - image.width()/2.0;
        let pos_y = pos.y - image.height()/2.0 - i as f32 * spread as f32;
        image.set_filter(FilterMode::Nearest);

        draw_texture_ex(
            &image,
            pos_x,
            pos_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(image.width()*5.0,image.height()*5.0)),
                rotation: rotation,
                ..Default::default()
            },            
        );
        
    }
}


fn bool_int(b:bool) -> i32 {
    if b {1} else {0}
}