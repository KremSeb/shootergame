use macroquad::prelude::*;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect::new(
            self.x - self.size / 2.0,
            self.y - self.size / 2.0,
            self.size,
            self.size,
        )
    }
}

#[macroquad::main("Space Shooter")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;

    rand::srand(miniquad::date::now() as u64);

    let mut squares = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };
    let mut gameover = false;

    loop {
        clear_background(DARKPURPLE);
        if !gameover {
            let delta_time = get_frame_time();
            if is_key_down(KeyCode::Left) {
                circle.x -= circle.speed * delta_time;
            }
            if is_key_down(KeyCode::Right) {
                circle.x += circle.speed * delta_time;
            }
            if is_key_down(KeyCode::Up) {
                circle.y -= circle.speed * delta_time;
            }
            if is_key_down(KeyCode::Down) {
                circle.y += circle.speed * delta_time;
            }

            // clamp x and y to be within the screen
            circle.x = clamp(circle.x, 0.0, screen_width());
            circle.y = clamp(circle.y, 0.0, screen_height());

            // generate a new square
            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                });
            }

            // move square
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }

            // remove square below bottom of screen
            squares.retain(|s| s.y < screen_height() + s.size);
        }

        // check for collisions
        if squares.iter().any(|s| circle.collides_with(s)) {
            gameover = true;
        }

        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            gameover = false;
        }

        // draw everything
        draw_circle(circle.x, circle.y, circle.size / 2.0, YELLOW);
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                GREEN,
            );
        }

        if gameover {
            let text = "Game Over! Press Space to Restart";
            let text_dimensions = measure_text(text, None, 30, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0 + text_dimensions.height / 2.0,
                30.0,
                RED,
            );
        }
        next_frame().await;
    }
}
