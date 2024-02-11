use std::{collections::HashMap, ffi::CString, ptr::null_mut, time::UNIX_EPOCH, vec};

use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::Value;

const MAX_GUESSES: usize = 5;

use raylib::prelude::*;

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

const GRAY: Color = Color::new(63, 63, 70, 255);
fn main() -> Result<(), Box<dyn std::error::Error>> {
    raylib::set_trace_log(TraceLogLevel::LOG_ERROR);
    let mut buffer = Vec::new();
    let mut guessed = Vec::new();
    let mut words = HashMap::new();
    let (mut rl, thread) = raylib::init().size(640, 480).title("Infinle").build();

    let font = load_font(&thread, include_bytes!("./Ubuntu-Regular.ttf"));

    let camera = Camera3D::perspective(
        Vector3::new(-15.0, -30.0, -100.0),
        Vector3::new(-15.0, -30.0, 0.0),
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

    // regular cubes
    let mesh = unsafe { Mesh::gen_mesh_cube(&thread, 15.0, 15.0, 15.0).make_weak() };
    let cube = rl.load_model_from_mesh(&thread, mesh).unwrap();
    let mut solved: HashMap<i64, usize> = HashMap::new();
    while !rl.window_should_close() {
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
                            let w = get_word(1 as i64, &mut words);

                            let arr = if g == w {
                                solved_num += 1;
                                &green_letters
                            } else {
                                if w.contains(ch) {
                                    if let Some(word_char) = w.chars().nth(letter) {
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
                                                    w.matches(ch).collect::<Vec<&str>>().len();
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
                                    if let Some(lette) = letters.get(ch as usize - 65) {
                                        d.draw_model(&lette.1, pos, 1.0, Color::WHITE);
                                        solved_num = 0;
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
                println!("{}", get_word);

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
