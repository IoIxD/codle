use std::{collections::HashMap, ffi::CString, ptr::null_mut, time::UNIX_EPOCH, vec};

use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::Value;

const MAX_GUESSES: usize = 5;

use raylib::prelude::*;

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
lazy_static::lazy_static! {
    pub static ref DICTIONARY: Vec<Value> =
        serde_json::from_str(include_str!("./dictionary.json")).unwrap();
}

pub fn get_word(k: i64, words: &mut HashMap<i64, String>) -> String {
    if let Some(w) = words.get(&k) {
        w.clone()
    } else {
        let day = UNIX_EPOCH.elapsed().unwrap().as_millis() / 86400000;
        let mut rng = StdRng::seed_from_u64(k.abs() as u64 + day as u64);
        let num = rng.gen_range(0..DICTIONARY.len() - 1);
        let w = DICTIONARY
            .get(num)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
            .to_uppercase();

        words.insert(k, w.clone());
        w
    }
}

fn load_font(_thread: &RaylibThread, fontfile: &[u8]) -> Font {
    let fontfile_size = fontfile.len();
    let fontfile_type = CString::new(".ttf").unwrap();
    let chars = null_mut();
    unsafe {
        Font::from_raw(raylib::ffi::LoadFontFromMemory(
            fontfile_type.as_ptr(),
            fontfile.as_ptr(),
            fontfile_size.try_into().unwrap(),
            256,
            chars,
            100,
        ))
    }
}

#[cfg(feature = "wasm")]

extern "C" {
    fn GetWindowInnerWidth() -> i32;
    fn GetWindowInnerHeight() -> i32;
}

const GRAY: Color = Color::new(63, 63, 70, 255);
fn main() -> Result<(), Box<dyn std::error::Error>> {
    raylib::set_trace_log(TraceLogLevel::LOG_ERROR);
    let mut buffer = Vec::new();
    let mut guessed = Vec::new();
    let mut words = HashMap::new();
    let (mut rl, thread) = raylib::init().size(720, 1024).title("Infinle").build();

    let font = load_font(&thread, include_bytes!("./Ubuntu-Regular.ttf"));

    let camera = Camera3D::perspective(
        Vector3::new(-15.0, -40.0, -100.0),
        Vector3::new(-15.0, -40.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        60.0,
    );

    let mut letters_tex: Vec<(RenderTexture2D, RenderTexture2D)> = Vec::with_capacity(26);
    let mut green_letters_tex: Vec<(RenderTexture2D, RenderTexture2D)> = Vec::with_capacity(26);
    let mut yellow_letters_tex: Vec<(RenderTexture2D, RenderTexture2D)> = Vec::with_capacity(26);
    let mut red_letters_tex: Vec<(RenderTexture2D, RenderTexture2D)> = Vec::with_capacity(26);

    let mut letters: Vec<(Model, Model)> = Vec::with_capacity(26);
    let mut green_letters: Vec<(Model, Model)> = Vec::with_capacity(26);
    let mut yellow_letters: Vec<(Model, Model)> = Vec::with_capacity(26);

    let mut red_letters: Vec<(Model, Model)> = Vec::with_capacity(26);
    let avail_letters = vec![
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];
    let mut trio_tex = vec![
        &mut letters_tex,
        &mut green_letters_tex,
        &mut yellow_letters_tex,
        &mut red_letters_tex,
    ];
    let trio = vec![
        &mut letters,
        &mut green_letters,
        &mut yellow_letters,
        &mut red_letters,
    ];
    for v in &mut trio_tex {
        let buf = v.spare_capacity_mut();
        for idx in 0..26 {
            buf[idx].write((
                rl.load_render_texture(&thread, 64, 64)?,
                rl.load_render_texture(&thread, 64, 64)?,
            ));
        }
        unsafe { v.set_len(26) }
    }

    let keys = vec![
        vec!["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"],
        vec!["A", "S", "D", "F", "G", "H", "J", "K", "L"],
        vec!["^", "Z", "X", "C", "V", "B", "N", "M", "<"],
    ];
    {
        let mut d_ = rl.begin_drawing(&thread);
        let mut i = 0;
        let mut n = 0;
        for vec in &mut trio_tex {
            vec.iter_mut().for_each(|letter| {
                {
                    let mut _d = d_.begin_texture_mode(&thread, &mut letter.0);
                    match n {
                        0 => _d.clear_background(GRAY),
                        1 => _d.clear_background(Color::new(0, 204, 136, 255)),
                        2 => _d.clear_background(Color::new(255, 204, 0, 255)),
                        3 => _d.clear_background(Color::new(204, 0, 0, 255)),
                        _ => _d.clear_background(Color::BLACK),
                    }
                }
                {
                    let mut _d = d_.begin_texture_mode(&thread, &mut letter.1);
                    match n {
                        0 => _d.clear_background(GRAY),
                        1 => _d.clear_background(Color::new(0, 204, 136, 255)),
                        2 => _d.clear_background(Color::new(255, 204, 0, 255)),
                        3 => _d.clear_background(Color::new(204, 0, 0, 255)),
                        _ => _d.clear_background(Color::BLACK),
                    }
                    _d.draw_text_ex(
                        &font,
                        avail_letters.get(i).unwrap(),
                        Vector2::new(16.0, 0.0),
                        64.0,
                        1.0,
                        Color::WHITE,
                    );
                }

                i += 1;
            });
            n += 1;
            i = 0;
        }
    }
    let mut j = 0;
    for vec in trio {
        for i in 0..26 {
            let mesh1 = unsafe { Mesh::gen_mesh_cube(&thread, 15.0, 15.0, 15.0).make_weak() };
            let mesh2 = unsafe { Mesh::gen_mesh_cube(&thread, 15.0, 15.0, 15.0).make_weak() };
            let mut model1 = rl.load_model_from_mesh(&thread, mesh1).unwrap();
            model1.materials_mut()[0].set_material_texture(
                MaterialMapIndex::MATERIAL_MAP_ALBEDO,
                &trio_tex.get(j).unwrap().get(i).unwrap().0,
            );
            let mut model2 = rl.load_model_from_mesh(&thread, mesh2).unwrap();
            model2.materials_mut()[0].set_material_texture(
                MaterialMapIndex::MATERIAL_MAP_ALBEDO,
                &trio_tex.get(j).unwrap().get(i).unwrap().1,
            );
            vec.push((model1, model2));
        }
        j += 1;
    }

    #[cfg(not(feature = "wasm"))]
    let width = get_monitor_width(get_current_monitor_index()) / 2;
    #[cfg(not(feature = "wasm"))]
    let height = get_monitor_height(get_current_monitor_index()) - 256;

    // regular cubes
    let mesh = unsafe { Mesh::gen_mesh_cube(&thread, 15.0, 15.0, 15.0).make_weak() };
    let cube = rl.load_model_from_mesh(&thread, mesh).unwrap();
    let mut solved: HashMap<i64, usize> = HashMap::new();

    let word = get_word(1 as i64, &mut words);
    println!("{}", word);
    let word_chars = word.chars();

    while !rl.window_should_close() {
        #[cfg(feature = "wasm")]
        let width = unsafe { GetWindowInnerWidth() } / 2;
        #[cfg(feature = "wasm")]
        let height = unsafe { GetWindowInnerHeight() } - 16;
        rl.set_window_size(width, height);

        let valid = DICTIONARY.contains(&Value::String(
            buffer
                .iter()
                .map(|f| get_letter(f))
                .collect::<Vec<&str>>()
                .join("")
                .to_string()
                .to_lowercase(),
        ));

        if rl.is_key_released(KeyboardKey::KEY_ENTER) {
            if buffer.len() == 5 && valid {
                let mut g = String::new();
                for key in &buffer {
                    g += get_letter(key);
                }
                guessed.push(g.to_uppercase());
                buffer.truncate(0);
            }
        }
        if rl.is_key_released(KeyboardKey::KEY_BACKSPACE) {
            buffer.pop();
        }

        if let Some(k) = rl.get_key_pressed() {
            if buffer.len() < 5 {
                match k {
                    KeyboardKey::KEY_A
                    | KeyboardKey::KEY_B
                    | KeyboardKey::KEY_C
                    | KeyboardKey::KEY_D
                    | KeyboardKey::KEY_E
                    | KeyboardKey::KEY_F
                    | KeyboardKey::KEY_G
                    | KeyboardKey::KEY_H
                    | KeyboardKey::KEY_I
                    | KeyboardKey::KEY_J
                    | KeyboardKey::KEY_K
                    | KeyboardKey::KEY_L
                    | KeyboardKey::KEY_M
                    | KeyboardKey::KEY_N
                    | KeyboardKey::KEY_O
                    | KeyboardKey::KEY_P
                    | KeyboardKey::KEY_Q
                    | KeyboardKey::KEY_R
                    | KeyboardKey::KEY_S
                    | KeyboardKey::KEY_T
                    | KeyboardKey::KEY_U
                    | KeyboardKey::KEY_V
                    | KeyboardKey::KEY_W
                    | KeyboardKey::KEY_X
                    | KeyboardKey::KEY_Y
                    | KeyboardKey::KEY_Z => {
                        buffer.push(k);
                    }
                    _ => {}
                };
            }
        }
        let mut d_ = rl.begin_drawing(&thread);
        let mut offset = (camera.position.z as i64) / 10;
        d_.clear_background(Color::new(31, 41, 55, 255));

        {
            let mut d = d_.begin_mode3D(camera);

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

                    if let Some(g) = guessed.get(guess) {
                        let g = g.clone();
                        if let Some(ch) = g.chars().nth(letter) {
                            let arr = if g == word {
                                solved_num += 1;
                                &green_letters
                            } else {
                                if word.contains(ch) {
                                    if let Some(word_char) = word_chars.clone().nth(letter) {
                                        if word_char == ch {
                                            solved_num += 1;
                                            &green_letters
                                        } else {
                                            solved_num = 0;
                                            if !yellow_letters_marked.contains_key(&ch) {
                                                yellow_letters_marked.insert(ch, 1);
                                                &yellow_letters
                                            } else {
                                                let am = yellow_letters_marked.get(&ch).unwrap();

                                                let what =
                                                    word.matches(ch).collect::<Vec<&str>>().len();
                                                if am < &what {
                                                    &yellow_letters
                                                } else {
                                                    &letters
                                                }
                                            }
                                        }
                                    } else {
                                        solved_num = 0;
                                        &letters
                                    }
                                } else {
                                    solved_num = 0;
                                    &letters
                                }
                            };
                            if let Some(lette) = arr.get(ch as usize - 65) {
                                d.draw_model(&lette.1, pos, 1.0, Color::WHITE);
                            }
                        }
                    } else {
                        if guess == guessed.len() {
                            if let Some(t) = &buffer.get(letter) {
                                if let Some(ch) = get_letter(&t).chars().nth(0) {
                                    let letters = if valid || buffer.len() < 5 {
                                        &letters
                                    } else {
                                        &red_letters
                                    };
                                    if ch as usize >= 65 {
                                        if let Some(lette) = letters.get(ch as usize - 65) {
                                            d.draw_model(&lette.1, pos, 1.0, Color::WHITE);
                                            solved_num = 0;
                                        }
                                    }
                                }
                            } else {
                                d.draw_model(&cube, pos, 1.0, Color::new(24, 24, 27, 255))
                            }
                        } else {
                            d.draw_model(&cube, pos, 1.0, Color::new(24, 24, 27, 255))
                        }
                    }
                }
                if solved_num >= 5 {
                    if !solved.contains_key(&(1)) {
                        solved.insert(1, guess);
                    }
                } else {
                    solved_num = 0;
                }
            }
            if guessed.len() >= (MAX_GUESSES + 1) {
                let mut n = 0;
                let get_word = &get_word(1 as i64, &mut words);

                for letter in get_word.split("") {
                    if let Some(ch) = letter.chars().nth(0) {
                        if let Some(lette) = letters.get(ch as usize - 65) {
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

        let key_width = width / 10;
        let key_height = height / 12;
        let font_size = key_height as f32 * 0.75;
        let mut y = height - (key_height * 3);
        let vec = &guessed.clone();
        let l = vec
            .iter()
            .map(|f| f.split("").into_iter().collect::<Vec<&str>>())
            .flat_map(|f| f)
            .collect::<Vec<&str>>();

        for row in &keys {
            let boost = 10 - row.len();
            let mut x = 4 + ((key_width / 2) * (boost) as i32);
            let x_ = x;

            for key in row {
                let color = {
                    if l.contains(key) {
                        if word.contains(key) {
                            Color::new(128, 128, 128, 255)
                        } else {
                            Color::BLACK
                        }
                    } else {
                        Color::new(74, 74, 74, 255)
                    }
                };
                d_.draw_rectangle(x, y, key_width - 12, key_height - 12, color);

                let m = measure_text_ex(&font, key, font_size, 3.0);
                d_.draw_text_ex(
                    &font,
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
                            let k = get_key(&key);
                            if k != KeyboardKey::KEY_NULL {
                                buffer.push(k);
                            } else {
                                // BACKSPACE
                                if x == x_ as i32 {
                                    if buffer.len() == 5 && valid {
                                        let mut g = String::new();
                                        for key in &buffer {
                                            g += get_letter(key);
                                        }
                                        guessed.push(g.to_uppercase());
                                        buffer.truncate(0);
                                    }
                                }
                                if x == x_ + (key_width * 8) {
                                    buffer.pop();
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

    Ok(())
}

pub fn get_letter(key: &KeyboardKey) -> &str {
    match key {
        KeyboardKey::KEY_A => "A",
        KeyboardKey::KEY_B => "B",
        KeyboardKey::KEY_C => "C",
        KeyboardKey::KEY_D => "D",
        KeyboardKey::KEY_E => "E",
        KeyboardKey::KEY_F => "F",
        KeyboardKey::KEY_G => "G",
        KeyboardKey::KEY_H => "H",
        KeyboardKey::KEY_I => "I",
        KeyboardKey::KEY_J => "J",
        KeyboardKey::KEY_K => "K",
        KeyboardKey::KEY_L => "L",
        KeyboardKey::KEY_M => "M",
        KeyboardKey::KEY_N => "N",
        KeyboardKey::KEY_O => "O",
        KeyboardKey::KEY_P => "P",
        KeyboardKey::KEY_Q => "Q",
        KeyboardKey::KEY_R => "R",
        KeyboardKey::KEY_S => "S",
        KeyboardKey::KEY_T => "T",
        KeyboardKey::KEY_U => "U",
        KeyboardKey::KEY_V => "V",
        KeyboardKey::KEY_W => "W",
        KeyboardKey::KEY_X => "X",
        KeyboardKey::KEY_Y => "Y",
        KeyboardKey::KEY_Z => "Z",

        KeyboardKey::KEY_SPACE => " ",
        _ => "?",
    }
}

pub fn get_key(key: &str) -> KeyboardKey {
    match key {
        "A" => KeyboardKey::KEY_A,
        "B" => KeyboardKey::KEY_B,
        "C" => KeyboardKey::KEY_C,
        "D" => KeyboardKey::KEY_D,
        "E" => KeyboardKey::KEY_E,
        "F" => KeyboardKey::KEY_F,
        "G" => KeyboardKey::KEY_G,
        "H" => KeyboardKey::KEY_H,
        "I" => KeyboardKey::KEY_I,
        "J" => KeyboardKey::KEY_J,
        "K" => KeyboardKey::KEY_K,
        "L" => KeyboardKey::KEY_L,
        "M" => KeyboardKey::KEY_M,
        "N" => KeyboardKey::KEY_N,
        "O" => KeyboardKey::KEY_O,
        "P" => KeyboardKey::KEY_P,
        "Q" => KeyboardKey::KEY_Q,
        "R" => KeyboardKey::KEY_R,
        "S" => KeyboardKey::KEY_S,
        "T" => KeyboardKey::KEY_T,
        "U" => KeyboardKey::KEY_U,
        "V" => KeyboardKey::KEY_V,
        "W" => KeyboardKey::KEY_W,
        "X" => KeyboardKey::KEY_X,
        "Y" => KeyboardKey::KEY_Y,
        "Z" => KeyboardKey::KEY_Z,

        _ => KeyboardKey::KEY_NULL,
    }
}
