use std::{collections::HashMap, str::Chars, time::SystemTime};

use raylib::prelude::*;
use serde_json::Value;

use crate::{models::ModelStore, Screen};
use rand::{rngs::StdRng, Rng, SeedableRng};

use std::{ffi::CString, ptr::null_mut, time::UNIX_EPOCH, vec};

lazy_static::lazy_static! {
    pub static ref DICTIONARY: Vec<Value> =
        serde_json::from_str(include_str!("./dictionary.json")).unwrap();
}

pub struct State<'a> {
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
    pub buffer: Vec<KeyboardKey>,
    pub guessed: Vec<String>,
    pub words: HashMap<i64, String>,
    pub models: ModelStore,
    pub screen: Screen,
    pub win_time: SystemTime,
    pub font: Font,
    pub camera: Camera3D,
    pub word: String,
    pub show_letters: bool,
    pub cube: Model,
    pub keys: Vec<Vec<&'a str>>,
}

impl<'a> State<'a> {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        raylib::set_trace_log(TraceLogLevel::LOG_ERROR);
        let mut buffer = Vec::new();
        let mut guessed: Vec<String> = Vec::new();
        let mut words = HashMap::new();
        let (mut rl, thread) = raylib::init().size(720, 1024).title("Infinle").build();

        let font = load_font(&thread, include_bytes!("./Ubuntu-Regular.ttf"));

        let camera = Camera3D::perspective(
            Vector3::new(-15.0, -40.0, -100.0),
            Vector3::new(-15.0, -40.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            60.0,
        );

        let keys: Vec<Vec<&str>> = vec![
            vec!["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"],
            vec!["A", "S", "D", "F", "G", "H", "J", "K", "L"],
            vec!["^", "Z", "X", "C", "V", "B", "N", "M", "<"],
        ];

        let models = ModelStore::new(&mut rl, &thread, &font)?;

        // regular cubes
        let mesh = unsafe { Mesh::gen_mesh_cube(&thread, 15.0, 15.0, 15.0).make_weak() };
        let cube = rl.load_model_from_mesh(&thread, mesh).unwrap();
        let mut show_letters = true;
        let word = get_word(1 as i64, &mut words);

        let mut screen = Screen::Title;
        let mut win_time = SystemTime::now();

        Ok(Self {
            buffer,
            guessed,
            words,
            models,
            screen,
            win_time,
            rl,
            thread,
            font,
            camera,
            word,
            show_letters,
            cube,
            keys,
        })
    }
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
