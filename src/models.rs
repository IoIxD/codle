use raylib::prelude::*;

use crate::GRAY;

pub struct ModelStore {
    pub letters: Vec<(Model, Model)>,
    pub green_letters: Vec<(Model, Model)>,
    pub yellow_letters: Vec<(Model, Model)>,
    pub red_letters: Vec<(Model, Model)>,
    pub letters_tex: Vec<(RenderTexture2D, RenderTexture2D)>,
    pub green_letters_tex: Vec<(RenderTexture2D, RenderTexture2D)>,
    pub red_letters_tex: Vec<(RenderTexture2D, RenderTexture2D)>,
    pub yellow_letters_tex: Vec<(RenderTexture2D, RenderTexture2D)>,
}

impl ModelStore {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        font: &Font,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut letters_tex: Vec<(RenderTexture2D, RenderTexture2D)> = Vec::with_capacity(26);
        let mut green_letters_tex: Vec<(RenderTexture2D, RenderTexture2D)> = Vec::with_capacity(26);
        let mut yellow_letters_tex: Vec<(RenderTexture2D, RenderTexture2D)> =
            Vec::with_capacity(26);
        let mut red_letters_tex: Vec<(RenderTexture2D, RenderTexture2D)> = Vec::with_capacity(26);

        let mut letters: Vec<(Model, Model)> = Vec::with_capacity(26);
        let mut green_letters: Vec<(Model, Model)> = Vec::with_capacity(26);
        let mut yellow_letters: Vec<(Model, Model)> = Vec::with_capacity(26);

        let mut red_letters: Vec<(Model, Model)> = Vec::with_capacity(26);
        let avail_letters = vec![
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q",
            "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
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

        Ok(Self {
            letters,
            green_letters,
            yellow_letters,
            red_letters,
            letters_tex,
            green_letters_tex,
            red_letters_tex,
            yellow_letters_tex,
        })
    }
}
