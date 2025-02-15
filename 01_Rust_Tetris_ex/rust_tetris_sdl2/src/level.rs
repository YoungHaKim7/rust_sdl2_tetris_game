use sdl2::{rect::Rect, render::TextureQuery};

use crate::bdimentions;

pub struct Level {}

impl Level {
    pub fn new() -> Level {
        Level {}
    }

    pub fn draw_level(&self, canvas: &mut sdl2::render::WindowCanvas, level_string: String) {
        // Draw some retangles here!
        let white: sdl2::pixels::Color = sdl2::pixels::Color::RGB(191, 191, 191);

        let dimentions = bdimentions::Bdimentions::new();

        let yellow: sdl2::pixels::Color = sdl2::pixels::Color::RGB(226, 244, 66);
        let texture_creator = canvas.texture_creator();

        // Load a font
        let ttf_context = sdl2::ttf::init().unwrap();
        let font = ttf_context
            .load_font("../assets/Roboto-Regular.ttf", 128)
            .unwrap();

        let surface = font.render(&level_string).blended(white).unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new(
            (dimentions.left - dimentions.unit_size * 6) as i32,
            dimentions.top - 10,
            width / 3 as u32,
            height / 3 as u32,
        );
        canvas.set_draw_color(yellow);
        canvas.copy(&texture, None, Some(target)).unwrap();
    }

    pub fn draw_score(&self, canvas: &mut sdl2::render::WindowCanvas, score: i32) {
        let dimentions = bdimentions::Bdimentions::new();

        let white: sdl2::pixels::Color = sdl2::pixels::Color::RGB(191, 191, 191);
        let yellow: sdl2::pixels::Color = sdl2::pixels::Color::RGB(226, 244, 66);
        let texture_creator = canvas.texture_creator();

        // Load a font
        let ttf_context = sdl2::ttf::init().unwrap();
        let font = ttf_context
            .load_font("../assets/Roboto-Regular.ttf", 128)
            .unwrap();
        let score_string = "Score".to_string();

        let surface = font.render(&score_string).blended(white).unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new(
            (dimentions.left - dimentions.unit_size * 6) as i32,
            dimentions.top + dimentions.unit_size * 2,
            width / 3 as u32,
            height / 3 as u32,
        );
        canvas.set_draw_color(yellow);
        canvas.copy(&texture, None, Some(target)).unwrap();
        // End top half

        let surface = font.render(&score.to_string()).blended(white).unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        let TextureQuery { width, height, .. } = texture.query();

        // let mut number_target = Rect::new((dimentions.left - dimentions.unit_size *6) as i32,dimentions.top+dimentions.unit_size * 5,width/3 as u32,height/2 as u32);
        let number_target = Rect::new(
            (dimentions.left - dimentions.unit_size * 6) as i32,
            dimentions.top + dimentions.unit_size * 4,
            width / 3 as u32,
            height / 3 as u32,
        );
        canvas.set_draw_color(yellow);
        canvas.copy(&texture, None, Some(number_target)).unwrap();
    }
}
