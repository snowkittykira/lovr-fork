use glam::{vec3, Mat4};
use crate::lovr::{self, event::Key, graphics::Pass};

pub struct Game;

impl lovr::Callbacks for Game {

    fn update(&mut self, _dt: f64) -> lovr::Result<()> {
        Ok(())
    }

    fn draw(&mut self, pass: &mut Pass) -> lovr::Result<bool> {
        pass.sphere(Mat4::from_translation(vec3(0., 0., -2.)), 16, 16)?;
        Ok(false)
    }

    fn key_pressed(&mut self, code: Key, _scancode: u32, _repeat: bool) -> lovr::Result<()> {
        if let Key::Escape = code {
            lovr::event::quit(0)
        }
        Ok(())
    }

}

