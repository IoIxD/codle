use raylib::prelude::*;

pub fn draw_text_centered<A>(
    rl: &mut A,
    font: &Font,
    font_size: f32,
    text: &str,
    width: i32,
    y: f32,
    alpha: f32,
) where
    A: RaylibDraw,
{
    let w = measure_text_ex(&font, &text, font_size, 3.0);

    rl.draw_text_ex(
        &font,
        text,
        Vector2::new((width / 2) as f32 - (w.x / 2.0), y),
        font_size,
        3.0,
        Color::WHITE.fade(alpha),
    );
}

pub fn push_valid_word(buffer: &mut Vec<KeyboardKey>, k: KeyboardKey) {
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
