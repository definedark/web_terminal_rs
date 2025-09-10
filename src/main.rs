use macroquad::prelude::*;

struct Timer {
    current: f64,
}
impl Timer {
    fn now() -> Timer {
        Timer {
            current: get_time(),
        }
    }
    fn elapsed(&self) -> u64 {
        ((get_time() - self.current) * 1000.0) as u64
    }
}
#[macroquad::main("BasicShapes")]
async fn main() {
    let mut history: Vec<String> = Vec::new();
    let mut history_index = 0;
    let mut command = String::from("");
    let mut status = String::from("#");
    let mut time_cursor = Timer::now();
    let mut time_del = Timer::now();
    let mut toggle = false;
    const initial_y: f32 = 100.0;

    loop {
        clear_background(BLACK);

        match get_char_pressed() {
            Some(key)
                if (key.is_alphanumeric() || key.is_whitespace()) && key != '\r' && key != '\n' =>
            {
                command.push(key);
            }
            _ => (),
        }
        if is_key_pressed(KeyCode::Enter) {
            let line = status.clone() + &command;
            history.push(line);
            command.clear();
        }
        if is_key_down(KeyCode::Backspace) && time_del.elapsed() > 70 {
            time_del = Timer::now();
            command.pop();
        }
        let mut offset = 0.0;
        if history.len() - history_index > 20 {
            history_index += 1;
        }
        for text in &history[history_index..] {
            let rect = draw_text(text, 0.0, initial_y + offset, 30.0, GREEN);
            offset += rect.height + 2.0;
        }

        let status_rect = draw_text(&status, 0.0, initial_y + offset, 30.0, GREEN);
        let cmd_rect = draw_text(&command, status_rect.width, initial_y + offset, 30.0, GREEN);

        if time_cursor.elapsed() > 500 && !toggle {
            toggle = true;
            time_cursor = Timer::now();
        }
        if time_cursor.elapsed() > 500 && toggle {
            toggle = false;
            time_cursor = Timer::now();
        }

        if toggle {
            draw_text(
                "|",
                cmd_rect.width + status_rect.width,
                initial_y + offset,
                30.0,
                GREEN,
            );
        }
        next_frame().await
    }
}
