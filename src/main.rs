use macroquad::prelude::*;
use sapp_jsutils::JsObject;
const ABOUT: &str = include_str!("about.txt");
const LS: &str = "about.txt linkedin.sh";
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
// This function will be used for Web (WASM) builds
#[cfg(target_arch = "wasm32")]
fn open_url(url: &str) {
    unsafe {
        let obj = JsObject::string(url);
        native_open_url(obj);
    }
}
fn push_split_history(history: &mut Vec<String>, input: &str) {
    for line in input.lines() {
        history.push(line.to_string());
    }
}
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
unsafe extern "C" {
    fn native_open_url(js_object: JsObject);
}
#[macroquad::main("web_terminal")]
async fn main() {
    let mut history: Vec<String> = Vec::new();
    let mut history_index = 0;
    let mut command = String::from("");
    let mut status = String::from("#");
    let mut time_cursor = Timer::now();
    let mut time_del = Timer::now();
    let mut toggle = false;
    const INITIAL_Y: f32 = 100.0;

    loop {
        clear_background(BLACK);

        match get_char_pressed() {
            Some(key) if !key.is_ascii_control() => {
                command.push(key);
            }
            _ => (),
        }
        if is_key_pressed(KeyCode::Enter) {
            let line = status.clone() + &command;
            history.push(line);
            match command.as_str() {
                "clear" => {
                    history.clear();
                }
                "ls" => {
                    history.push(LS.to_string());
                }
                cmd => {
                    if cmd.starts_with("./") && cmd.len() > 1 {
                        match &cmd[2..] {
                            "linkedin.sh" => {
                                #[cfg(target_arch = "wasm32")]
                                open_url("https://www.linkedin.com/in/elias-rammos-b3548739");
                            }
                            _ => (),
                        }
                    }
                    if cmd.starts_with("cat ") && cmd.len() > 3 {
                        match &cmd[4..] {
                            "about.txt" => push_split_history(&mut history, ABOUT),
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
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
            // let rect = draw_text_ex(text, 0.0, INITIAL_Y + offset, textParams.clone());
            let rect = draw_text(text, 0.0, INITIAL_Y + offset, 30.0, GREEN);
            offset += rect.height + 5.0;
        }

        let status_rect = draw_text(&status, 0.0, INITIAL_Y + offset, 30.0, GREEN);
        let cmd_rect = draw_text(&command, status_rect.width, INITIAL_Y + offset, 30.0, GREEN);

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
                INITIAL_Y + offset,
                30.0,
                GREEN,
            );
        }
        next_frame().await
    }
}
