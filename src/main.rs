use rand::{thread_rng, Rng};
use raylib::prelude::*;

static mut POINTS: u32 = 0;

#[derive(Clone, Copy)]
struct Ballon {
    pub position: Vector2,
    pub color: Color,
    pub speed: f32,
}

impl Ballon {
    pub fn new(speed: f32, position: Vector2, c: Color) -> Self {
        Self {
            color: c,
            position: position,
            speed: speed,
        }
    }
    pub fn draw(&self, drawer: &mut RaylibDrawHandle) {
        drawer.draw_rectangle_v(self.position, Vector2 { x: 35.0, y: 35.0 }, self.color);
    }
    pub fn update(&mut self, height: i32, width: i32) {
        if self.position.y <= height as f32 && self.position.y > 0.0 {
            self.position.y -= self.speed
        } else {
            self.update_random(height, width);
        };
    }
    pub fn update_random(&mut self, height: i32, width: i32) {
        let y = thread_rng().gen_range(height - 50..height - 5);
        let x = thread_rng().gen_range(50..width - 50);
        self.position.y = y as f32;
        self.position.x = x as f32;
    }
    pub fn reset_pos(&mut self, height: i32) {
        self.position.y = height as f32;
    }
}
fn main() {
    let (mut rl, thread) = raylib::init().size(800, 600).title("Hello, World").build();

    let mut ballons = vec![
        Ballon::new(2.5, Vector2 { x: 750.0, y: 550.0 }, Color::GREEN),
        Ballon::new(2.5, Vector2 { x: 550.0, y: 350.0 }, Color::RED),
        Ballon::new(2.5, Vector2 { x: 350.0, y: 250.0 }, Color::BLUE),
        Ballon::new(2.5, Vector2 { x: 150.0, y: 450.0 }, Color::YELLOW),
        Ballon::new(2.5, Vector2 { x: 600.0, y: 550.0 }, Color::PURPLE),
    ];
    rl.set_target_fps(250);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let mouse_off_y = d.get_mouse_y();
        let mouse_off_x = d.get_mouse_x();

        for ballon in &mut ballons {
            if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                if mouse_off_y >= ballon.position.y as i32
                    && mouse_off_y <= (ballon.position.y + 35.0) as i32
                    && mouse_off_x >= ballon.position.x as i32
                    && mouse_off_x <= (ballon.position.x + 35.0) as i32
                {
                    ballon.reset_pos(d.get_screen_height());
                    unsafe { POINTS += 1 };
                }
            }

            ballon.draw(&mut d);
            ballon.update(d.get_screen_height(), d.get_screen_width());
        }
        d.draw_text(
            format!("POINTS: {}", unsafe { POINTS }).as_str(),
            12,
            12,
            20,
            Color::BLUE,
        );
        d.clear_background(Color::WHITE);
    }
}
