use std::{collections::HashMap, time::SystemTime};

use serde_json::Value;

const MAX_GUESSES: usize = 5;

use raylib::prelude::*;
use state::{get_word, State, DICTIONARY};

mod models;
mod state;
mod utils;
/* used:
    - Javascript reserved keywords
    - HTML tags
    - CSS properties
    - Python keywords
    - SQL keywords
    - Bash keywords
    - Java keywords
    - C# keywords
    - C/C++ keywords
    - PHP keywords
    - Powershell keywords
    - Golang keywords
    - Rust keywords
    - Kotlin keywords
    - Ruby keywords
    - Lua keywords
    - Dart keywords
    - Swift keywords
    - the words "Swift", "Scala", "Julia", "OCaml", "Apex".

    hard mode mixes in:
    - x86 mnemonics
    - ARM64 mnemonics
    - PPC mnemonics
*/

#[cfg(feature = "wasm")]

extern "C" {
    fn GetWindowInnerWidth() -> i32;
    fn GetWindowInnerHeight() -> i32;
}

#[derive(PartialEq)]
enum Screen {
    Title,
    Game,
    Won,
}

const GRAY: Color = Color::new(63, 63, 70, 255);
const BLUE: Color = Color::new(31, 41, 55, 255);
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = State::new()?;
    let word_chars = state.word.chars().clone();

    #[cfg(not(feature = "wasm"))]
    let width = get_monitor_width(get_current_monitor_index()) / 2;
    #[cfg(not(feature = "wasm"))]
    let height = get_monitor_height(get_current_monitor_index()) - 256;

    while !state.rl.window_should_close() {
        #[cfg(feature = "wasm")]
        let width = unsafe { GetWindowInnerWidth() };
        #[cfg(feature = "wasm")]
        let height = unsafe { GetWindowInnerHeight() };
        state.rl.set_window_size(width, height);

        let screen_width = state.rl.get_screen_width();
        let screen_height = state.rl.get_screen_height();

        let valid = DICTIONARY.contains(&Value::String(
            state
                .buffer
                .iter()
                .map(|f| utils::get_letter(f))
                .collect::<Vec<&str>>()
                .join("")
                .to_string()
                .to_lowercase(),
        ));
        let f_width = state.rl.get_screen_width() as f32 / 10.0;
        match state.screen {
            Screen::Title => {
                let mut d = state.rl.begin_drawing(&state.thread);
                d.clear_background(BLUE);

                utils::draw_text_centered(
                    &mut d,
                    &state.font,
                    f_width,
                    "CODLE",
                    screen_width,
                    32.0,
                    1.0,
                );

                let f_width = f_width * 0.5;
                utils::draw_text_centered(
                    &mut d,
                    &state.font,
                    f_width,
                    "Get 6 chances to guess a 5 letter",
                    screen_width,
                    (screen_height / 4) as f32,
                    1.0,
                );
                utils::draw_text_centered(
                    &mut d,
                    &state.font,
                    f_width,
                    "word that's related to programming",
                    screen_width,
                    (screen_height / 4) as f32 + (f_width + 4.0),
                    1.0,
                );
                utils::draw_text_centered(
                    &mut d,
                    &state.font,
                    f_width,
                    "Click anywhere to begin.",
                    screen_width,
                    (screen_height - (screen_height / 4)) as f32,
                    1.0,
                );

                if d.is_gesture_detected(Gesture::GESTURE_TAP) {
                    state.screen = Screen::Game;
                }
            }
            Screen::Won | Screen::Game => {
                if let Some(k) = state.rl.get_key_pressed() {
                    if state.screen == Screen::Game {
                        utils::push_valid_word(&mut state.buffer, k);
                    }
                }
                let mut d_ = state.rl.begin_drawing(&state.thread);
                let mut offset = (state.camera.position.z as i64) / 10;
                d_.clear_background(BLUE);

                {
                    let mut d = d_.begin_mode3D(state.camera);

                    if offset >= 100 {
                        offset = 100;
                    }
                    let mut solved_num = 0;

                    for guess in 0..=MAX_GUESSES {
                        let mut yellow_letters_marked: HashMap<char, usize> = HashMap::new();
                        for letter in 0..=4 {
                            let pos = Vector3::new(
                                16.0 - (letter as f32 * 16.0),
                                16.0 - (guess as f32 * 16.0),
                                (7.0) - (offset as f32),
                            );

                            if let Some(g) = state.guessed.get(guess) {
                                let g = g.clone();
                                if let Some(ch) = g.chars().nth(letter) {
                                    let arr = if g == state.word {
                                        solved_num += 1;
                                        &state.models.green_letters
                                    } else {
                                        if state.word.contains(ch) {
                                            if let Some(word_char) = word_chars.clone().nth(letter)
                                            {
                                                if word_char == ch {
                                                    solved_num += 1;
                                                    &state.models.green_letters
                                                } else {
                                                    solved_num = 0;
                                                    if !yellow_letters_marked.contains_key(&ch) {
                                                        yellow_letters_marked.insert(ch, 1);
                                                        &state.models.yellow_letters
                                                    } else {
                                                        let am =
                                                            yellow_letters_marked.get(&ch).unwrap();

                                                        let what = state
                                                            .word
                                                            .matches(ch)
                                                            .collect::<Vec<&str>>()
                                                            .len();
                                                        if am < &what {
                                                            &state.models.yellow_letters
                                                        } else {
                                                            &state.models.letters
                                                        }
                                                    }
                                                }
                                            } else {
                                                solved_num = 0;
                                                &state.models.letters
                                            }
                                        } else {
                                            solved_num = 0;
                                            &state.models.letters
                                        }
                                    };

                                    if let Some(lette) = arr.get(ch as usize - 65) {
                                        let l = match state.show_letters {
                                            true => &lette.1,
                                            false => &lette.0,
                                        };
                                        d.draw_model(l, pos, 1.0, Color::WHITE);
                                    }
                                }
                            } else {
                                if guess == state.guessed.len() {
                                    if let Some(t) = &state.buffer.get(letter) {
                                        if let Some(ch) = utils::get_letter(&t).chars().nth(0) {
                                            let letters = if valid || state.buffer.len() < 5 {
                                                &state.models.letters
                                            } else {
                                                &state.models.red_letters
                                            };
                                            if ch as usize >= 65 {
                                                if let Some(lette) = letters.get(ch as usize - 65) {
                                                    let l = match state.show_letters {
                                                        true => &lette.1,
                                                        false => &lette.0,
                                                    };
                                                    d.draw_model(l, pos, 1.0, Color::WHITE);
                                                    solved_num = 0;
                                                }
                                            }
                                        }
                                    } else {
                                        d.draw_model(
                                            &state.cube,
                                            pos,
                                            1.0,
                                            Color::new(24, 24, 27, 255),
                                        )
                                    }
                                } else {
                                    d.draw_model(&state.cube, pos, 1.0, Color::new(24, 24, 27, 255))
                                }
                            }
                        }
                        if solved_num >= 5 {
                            if state.screen != Screen::Won {
                                state.screen = Screen::Won;
                                state.win_time = SystemTime::now();
                            }
                        } else {
                            solved_num = 0;
                        }
                    }
                    if state.guessed.len() >= (MAX_GUESSES + 1) {
                        let mut n = 0;
                        let get_word = &get_word(1 as i64, &mut state.words);

                        for letter in get_word.split("") {
                            if let Some(ch) = letter.chars().nth(0) {
                                if let Some(lette) = state.models.letters.get(ch as usize - 65) {
                                    d.draw_model(
                                        &lette.1,
                                        Vector3::new(
                                            16.0 - (n as f32 * 16.0) + 16.0,
                                            16.0 - (6.0 * 16.0),
                                            (7.0) - (offset as f32),
                                        ),
                                        1.0,
                                        Color::WHITE,
                                    );
                                }
                            }
                            n += 1;
                        }
                    }
                }

                match state.screen {
                    Screen::Game => {
                        if d_.is_key_released(KeyboardKey::KEY_ENTER) {
                            if state.buffer.len() == 5 && valid {
                                let mut g = String::new();
                                for key in &state.buffer {
                                    g += utils::get_letter(key);
                                }
                                state.guessed.push(g.to_uppercase());
                                state.buffer.truncate(0);
                            }
                        }
                        if d_.is_key_released(KeyboardKey::KEY_BACKSPACE) {
                            state.buffer.pop();
                        }

                        let key_width = width / 10;
                        let key_height = height / 12;
                        let font_size = key_height as f32 * 0.75;
                        let mut y = height - (key_height * 3);
                        let vec = &state.guessed.clone();
                        let l = vec
                            .iter()
                            .map(|f| f.split("").into_iter().collect::<Vec<&str>>())
                            .flat_map(|f| f)
                            .collect::<Vec<&str>>();

                        for row in &state.keys {
                            let boost = 10 - row.len();
                            let mut x = 4 + ((key_width / 2) * (boost) as i32);
                            let x_ = x;

                            for key in row {
                                let color = {
                                    if l.contains(key) {
                                        if state.word.contains(key) {
                                            Color::new(128, 128, 128, 255)
                                        } else {
                                            Color::BLACK
                                        }
                                    } else {
                                        Color::new(74, 74, 74, 255)
                                    }
                                };
                                d_.draw_rectangle(x, y, key_width - 12, key_height - 12, color);

                                let m = measure_text_ex(&state.font, key, font_size, 3.0);
                                d_.draw_text_ex(
                                    &state.font,
                                    key,
                                    Vector2::new(
                                        x as f32 + (key_width as f32 / 2.0) - (m.x / 2.0) - 6.0,
                                        y as f32,
                                    ),
                                    font_size,
                                    3.0,
                                    Color::WHITE,
                                );

                                let mx = d_.get_touch_x();
                                let my = d_.get_touch_y();

                                if d_.is_gesture_detected(Gesture::GESTURE_TAP) {
                                    if mx >= x && mx <= x + key_width {
                                        if my >= y && my <= y + key_height {
                                            let k = utils::get_key(&key);
                                            if k != KeyboardKey::KEY_NULL {
                                                state.buffer.push(k);
                                            } else {
                                                // BACKSPACE
                                                if x == x_ as i32 {
                                                    if state.buffer.len() == 5 && valid {
                                                        let mut g = String::new();
                                                        for key in &state.buffer {
                                                            g += utils::get_letter(key);
                                                        }
                                                        state.guessed.push(g.to_uppercase());
                                                        state.buffer.truncate(0);
                                                    }
                                                }
                                                if x == x_ + (key_width * 8) {
                                                    state.buffer.pop();
                                                }
                                            }
                                        }
                                    }
                                }

                                x += key_width;
                            }
                            y += key_height;
                        }
                    }
                    _ => {
                        state.show_letters = false;
                        if state.win_time.elapsed()?.as_secs() >= 1 {
                            let alpha = {
                                if state.win_time.elapsed()?.as_secs() <= 1 {
                                    state.win_time.elapsed()?.as_secs_f32() - 1.0
                                } else {
                                    1.0
                                }
                            };
                            d_.draw_rectangle(
                                screen_width / 4,
                                0,
                                screen_width / 2,
                                screen_height,
                                Color::BLACK.fade(alpha * 0.50),
                            );
                            let f_width = f_width * 0.50;
                            utils::draw_text_centered(
                                &mut d_,
                                &state.font,
                                f_width,
                                format!("Codle {}/6", state.guessed.len()).as_str(),
                                screen_width,
                                (screen_height / 4) as f32,
                                alpha,
                            );

                            utils::draw_text_centered(
                                &mut d_,
                                &state.font,
                                f_width,
                                "ioi-xd.net/codle",
                                screen_width,
                                (screen_height / 4) as f32 + (f_width * 2.0),
                                alpha,
                            )
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
