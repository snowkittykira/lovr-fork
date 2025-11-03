use glam::{vec3, Mat4};
use crate::lovr::LovrResult;
use crate::lovr;

pub struct Game;

impl lovr::LovrCallbacks for Game {

    fn draw(&mut self, pass: &mut lovr::Pass) -> LovrResult<bool> {
        pass.sphere(Mat4::from_translation(vec3(0., 0., -2.)), 16, 16)?;
        Ok(false)
    }

    fn key_pressed(&mut self, code: lovr::Key, _scancode: u32, _repeat: bool) -> LovrResult<()> {
        if let lovr::Key::Escape = code {
            lovr::event::quit(0)
        }
        Ok(())
    }

}

