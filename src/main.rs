use macroquad::prelude::*;

#[macroquad::main("Type")]
async fn main() {
    request_new_screen_size(1920.0, 1080.0);
    let mut height;
    let mut width;
    request_new_screen_size(1920.0, 1080.0);

    // read text
    let text = include_str!("texts/first.txt");

    loop {
        height = screen_height();
        width = screen_width();
        clear_background(BLACK);
        // println!("{} {} ", width, height);

        // draw_line(40.0, 40.0, 100.0, 150.0, 60.3, BLUE);
        draw_text(&text, width / 2.0, height / 2.0, 48.0, WHITE);
        next_frame().await;
    }
}
