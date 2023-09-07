use macroquad::prelude::*;

#[macroquad::main("sprite stack")]
async fn main() {
    let rad: f32 = 1.0_f32.to_radians();
    let images = vec![
        load_texture("assets/img_0.png").await.unwrap(),
        load_texture("assets/img_1.png").await.unwrap(),
        load_texture("assets/img_2.png").await.unwrap(),
        load_texture("assets/img_3.png").await.unwrap(),
        load_texture("assets/img_4.png").await.unwrap(),
        load_texture("assets/img_5.png").await.unwrap(),
        load_texture("assets/img_6.png").await.unwrap(),
    ];
    let mut frame = 0.0;
    loop {
        clear_background(DARKGRAY);
        frame += rad;
        render_stack(&images,Vec2::new(250.0,250.0),frame,2);
        next_frame().await;
    }
}


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