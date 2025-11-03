use crate::lovr;

use lovr::graphics::Pass;
use lovr::event::{DisplayType, Event, FileAction, Key, Permission};

#[allow(unused_variables)] 
pub trait Callbacks {
    fn load(&mut self, arg: &[String]) -> lovr::Result<()> { Ok(()) }
    fn update(&mut self, dt: f64) -> lovr::Result<()> { Ok(()) }
    fn draw(&mut self, pass: &mut Pass) -> lovr::Result<bool> { Ok(false) }
    fn mirror(&mut self, pass: &mut Pass) -> lovr::Result<bool> { self.draw(pass) }

    fn quit(&mut self) -> lovr::Result<bool> { Ok(false) }
    fn visible(&mut self, visible: bool, display: DisplayType) -> lovr::Result<()> { Ok(()) }
    fn focus(&mut self, focused: bool, display: DisplayType) -> lovr::Result<()> { Ok(()) }
    fn mount(&mut self, mounted: bool) -> lovr::Result<()> { Ok(()) }
    fn recenter(&mut self) -> lovr::Result<()> { Ok(()) }
    fn models_changed(&mut self) -> lovr::Result<()> { Ok(()) }
    fn resize(&mut self, width: u32, height: u32) -> lovr::Result<()> { Ok(()) }
    fn key_pressed(&mut self, code: Key, scancode: u32, repeat: bool) -> lovr::Result<()> { Ok(()) }
    fn key_released(&mut self, code: Key, scancode: u32) -> lovr::Result<()> { Ok(()) }
    fn text_input(&mut self, utf8: String, codepoint: u32) -> lovr::Result<()> { Ok(()) }
    fn mouse_pressed(&mut self, x: f64, y: f64, button: i32) -> lovr::Result<()> { Ok(()) }
    fn mouse_released(&mut self, x: f64, y: f64, button: i32) -> lovr::Result<()> { Ok(()) }
    fn mouse_moved(&mut self, x: f64, y: f64, dx: f64, dy: f64) -> lovr::Result<()> { Ok(()) }
    fn mouse_wheel_moved(&mut self, x: f64, y: f64) -> lovr::Result<()> { Ok(()) }
    fn thread_error(&mut self, error: String) -> lovr::Result<()> { Ok(()) }
    fn file_changed(&mut self, path: String, action: FileAction, old_path: String) -> lovr::Result<()> { Ok(()) }
    fn permission(&mut self, permission: Permission, granted: bool) -> lovr::Result<()> { Ok(()) }
    fn custom(&mut self) -> lovr::Result<()> { Ok(()) }

    fn run(mut self: Box<Self>) -> lovr::Result<impl FnMut() -> lovr::Result<RunCommand>> {
        lovr::timer::step();
        self.load(&[])?;
        Ok(move || -> lovr::Result<RunCommand> {
            // TODO headset stuff
            lovr::system::poll_events();

            while let Some(event) = lovr::event::poll() {
                match event {
                    Event::Quit { exit_code } => {
                        if !self.quit()? {
                            return Ok(RunCommand::Quit(exit_code))
                        }
                    },
                    Event::Visible { visible, display } => self.visible(visible, display)?,
                    Event::Focus { focused, display } => self.focus(focused, display)?,
                    Event::Mount { mounted } => self.mount(mounted)?,
                    Event::Recenter => self.recenter()?,
                    Event::ModelsChanged => self.models_changed()?,
                    Event::Resize { width, height } => self.resize(width, height)?,
                    Event::KeyPressed { code, scancode, repeat } => self.key_pressed(code, scancode, repeat)?,
                    Event::KeyReleased { code, scancode } => self.key_released(code, scancode)?,
                    Event::TextInput { utf8, codepoint } => self.text_input(utf8, codepoint)?,
                    Event::MousePressed { x, y, button } => self.mouse_pressed(x, y, button)?,
                    Event::MouseReleased { x, y, button } => self.mouse_released(x, y, button)?,
                    Event::MouseMoved { x, y, dx, dy } => self.mouse_moved(x, y, dx, dy)?,
                    Event::MouseWheelMoved { x, y } => self.mouse_wheel_moved(x, y)?,
                    Event::ThreadError { error } => self.thread_error(error)?,
                    Event::FileChanged { path, action, old_path } => self.file_changed(path, action, old_path)?,
                    Event::Permission { permission, granted } => self.permission(permission, granted)?,
                    Event::Custom {} => self.custom()?,
                }
            }
            let dt = lovr::timer::step();
            self.update(dt)?;
            if let Some(mut pass) = lovr::graphics::get_window_pass()? && !self.mirror(&mut pass)? {
                lovr::graphics::submit(&mut [pass])?
            }
            lovr::graphics::present()?;

            Ok(RunCommand::Continue)
        })
    }
}

pub enum RunCommand {
    Continue,
    Quit(i32),
}

