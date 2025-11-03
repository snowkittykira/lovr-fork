use glam::{Mat4};
use std::backtrace::Backtrace;
use std::ffi::CStr;

use crate::{lovr, lovr_sys};

pub mod event {
    use super::*;

    pub fn poll() -> Option<Event> {
        unsafe {
            let mut event: lovr_sys::Event = std::mem::zeroed();
            while lovr_sys::lovrEventPoll(&mut event) {
                match event.type_ {

                    lovr_sys::EventType_EVENT_QUIT => {
                        return Some(Event::Quit { exit_code: event.data.quit.exitCode, })
                    }

                    lovr_sys::EventType_EVENT_VISIBLE => {
                        return Some(Event::Visible {
                            visible: event.data.visible.visible,
                            display: match event.data.visible.display {
                                lovr_sys::DisplayType_DISPLAY_HEADSET => DisplayType::Headset,
                                lovr_sys::DisplayType_DISPLAY_WINDOW => DisplayType::Window,
                                _ => todo!() // TODO signal error somehow
                            }
                        })
                    }

                    lovr_sys::EventType_EVENT_FOCUS => {
                        return Some(Event::Focus {
                            focused: event.data.focus.focused,
                            display: match event.data.focus.display {
                                lovr_sys::DisplayType_DISPLAY_HEADSET => DisplayType::Headset,
                                lovr_sys::DisplayType_DISPLAY_WINDOW => DisplayType::Window,
                                _ => todo!() // TODO signal error somehow
                            }
                        })
                    }

                    lovr_sys::EventType_EVENT_MOUNT => {
                        return Some(Event::Mount {
                            mounted: event.data.mount.mounted
                        })
                    }

                    lovr_sys::EventType_EVENT_RECENTER => {
                        return Some(Event::Recenter)
                    }

                    lovr_sys::EventType_EVENT_MODELSCHANGED => {
                        return Some(Event::ModelsChanged)
                    }

                    lovr_sys::EventType_EVENT_RESIZE => {
                        return Some(Event::Resize {
                            width: event.data.resize.width,
                            height: event.data.resize.height,
                        })
                    }

                    lovr_sys::EventType_EVENT_KEYPRESSED => {
                        if let Some(code) = keycode_to_key(event.data.key.code) {
                            return Some(Event::KeyPressed {
                                code,
                                scancode: event.data.key.scancode,
                                repeat: event.data.key.repeat,
                            })
                        }
                    }

                    lovr_sys::EventType_EVENT_KEYRELEASED => {
                        if let Some(code) = keycode_to_key(event.data.key.code) {
                            return Some(Event::KeyReleased {
                                code,
                                scancode: event.data.key.scancode,
                            })
                        }
                    }

                    lovr_sys::EventType_EVENT_TEXTINPUT => {
                        let bytes: Vec<u8> = event.data.text.utf8
                            .iter()
                            .take_while(|&&b| b != 0)
                            .map(|&b| b as u8)
                            .collect();

                        return Some(Event::TextInput {
                            utf8: String::from_utf8(bytes).unwrap_or_default(),
                            codepoint: event.data.text.codepoint,
                        })
                    }

                    lovr_sys::EventType_EVENT_MOUSEPRESSED => {
                        return Some(Event::MousePressed {
                            x: event.data.mouse.x,
                            y: event.data.mouse.y,
                            button: event.data.mouse.button,
                        })
                    }

                    lovr_sys::EventType_EVENT_MOUSERELEASED => {
                        return Some(Event::MouseReleased {
                            x: event.data.mouse.x,
                            y: event.data.mouse.y,
                            button: event.data.mouse.button,
                        })
                    }

                    lovr_sys::EventType_EVENT_MOUSEMOVED => {
                        return Some(Event::MouseMoved {
                            x: event.data.mouse.x,
                            y: event.data.mouse.y,
                            dx: event.data.mouse.dx,
                            dy: event.data.mouse.dy,
                        })
                    }

                    lovr_sys::EventType_EVENT_MOUSEWHEELMOVED => {
                        return Some(Event::MouseWheelMoved {
                            x: event.data.mouse.x,
                            y: event.data.mouse.y,
                        })
                    }

                    lovr_sys::EventType_EVENT_THREAD_ERROR => {
                        let c_str = CStr::from_ptr(event.data.thread.error);
                        return Some(Event::ThreadError {
                            error: c_str.to_string_lossy().into_owned()
                        })
                    }

                    lovr_sys::EventType_EVENT_FILECHANGED => {
                        let path_c_str = CStr::from_ptr(event.data.file.path);
                        let old_path_c_str = CStr::from_ptr(event.data.file.oldpath);
                        return Some(Event::FileChanged {
                            path: path_c_str.to_string_lossy().into_owned(),
                            action: match event.data.file.action as u32 {
                                lovr_sys::FileAction_FILE_CREATE => FileAction::Create,
                                lovr_sys::FileAction_FILE_DELETE => FileAction::Delete,
                                lovr_sys::FileAction_FILE_MODIFY => FileAction::Modify,
                                lovr_sys::FileAction_FILE_RENAME => FileAction::Rename,
                                _ => todo!(), // TODO report an error
                            },
                            old_path: old_path_c_str.to_string_lossy().into_owned(),
                        })
                    }

                    lovr_sys::EventType_EVENT_PERMISSION => {
                        return Some(Event::Permission {
                            permission: match event.data.permission.permission {
                                lovr_sys::Permission_PERMISSION_AUDIO_CAPTURE => Permission::AudioCapture,
                                _ => todo!(), // TODO report an error
                            },
                            granted: event.data.permission.granted
                        })
                    }

                    lovr_sys::EventType_EVENT_CUSTOM => {
                        return Some(Event::Custom {})
                    }

                    _ => todo!() // TODO what to do here?
                }
            }
            None
        }
    }

    pub fn quit(exit_code: i32) {
        unsafe {
            lovr_sys::lovrEventPush(lovr_sys::Event {
                type_: lovr_sys::EventType_EVENT_QUIT,
                data: lovr_sys::EventData { quit: lovr_sys::QuitEvent { exitCode: exit_code } }
            });
        }
    }
}

pub mod graphics {
    use super::*;

    pub fn get_window_pass() -> LovrResult<Option<Pass>> {
        unsafe {
            let mut pass_ptr: *mut lovr_sys::Pass = std::ptr::null_mut();
            lovr_assert(lovr_sys::lovrGraphicsGetWindowPass(&mut pass_ptr))?;
            if pass_ptr.is_null() {
                Ok(None)
            } else {
                Ok(Some(Pass { ptr: pass_ptr }))
            }
        }
    }

    pub fn submit(passes: &mut [Pass]) -> LovrResult<()> {
        unsafe {
            let mut pass_ptrs: Vec<*mut lovr_sys::Pass> = passes.iter_mut().map(|p| p.ptr).collect();
            lovr_assert(lovr_sys::lovrGraphicsSubmit(pass_ptrs.as_mut_ptr(), passes.len().try_into().unwrap()))
        }
    }

    pub fn present() -> LovrResult<()> {
        unsafe {
            lovr_assert(lovr_sys::lovrGraphicsPresent())
        }
    }
}

pub mod headset {
    use super::*;

    pub fn is_active() -> bool {
        unsafe{
            lovr_sys::lovrHeadsetIsActive()
        }
    }

    pub fn update() -> LovrResult<f64> {
        unsafe{
            let mut dt = 0.;
            lovr_assert(lovr_sys::lovrHeadsetUpdate(&mut dt))?;
            Ok(dt)
        }
    }

    pub fn poll_events() -> LovrResult<()> {
        unsafe {
            lovr_assert(lovr_sys::lovrHeadsetPollEvents())
        }
    }
}

pub mod system {
    use super::*;

    pub fn poll_events() {
        unsafe {
            lovr_sys::lovrSystemPollEvents()
        }
    }
}

pub mod timer {
    use super::*;

    pub fn step() -> f64 {
        unsafe {
            lovr_sys::lovrTimerStep()
        }
    }
}

pub struct Pass {
    ptr: *mut lovr_sys::Pass
}

impl Pass {
    pub fn sphere(&mut self, transform: Mat4, longitudes: u32, latitudes:u32) -> LovrResult<()> {
        unsafe {
            lovr_assert(lovr_sys::lovrPassSphere(self.ptr, transform.to_cols_array().as_mut_ptr(), longitudes, latitudes))
        }
    }
}

// events

pub enum Event {
    Quit { exit_code: i32 },
    Visible { visible: bool, display: DisplayType },
    Focus { focused: bool, display: DisplayType },
    Mount { mounted: bool },
    Recenter,
    ModelsChanged,
    Resize { width: u32, height: u32 },
    KeyPressed { code: Key, scancode: u32, repeat: bool },
    KeyReleased { code: Key, scancode: u32 },
    TextInput { utf8: String, codepoint: u32 },
    MousePressed { x: f64, y: f64, button: i32 },
    MouseReleased { x: f64, y: f64, button: i32 },
    MouseMoved { x: f64, y: f64, dx: f64, dy: f64 },
    MouseWheelMoved { x: f64, y: f64 },
    ThreadError { error: String }, // TODO: thread parameter
    FileChanged { path: String, action: FileAction, old_path: String },
    Permission { permission: Permission, granted: bool },
    Custom {}, // TODO: variant parameter?
}

pub enum DisplayType {
    Headset,
    Window,
}

pub enum Key {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Number0, Number1, Number2, Number3, Number4, Number5, Number6, Number7, Number8, Number9,
    Space, Enter, Tab, Escape, Backspace, Up, Down, Left, Right, Home, End, PageUp, PageDown,
    Insert, Delete, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, Backtick, Minus, Equals,
    LeftBracket, RightBracket, Backslash, Semicolon, Apostrophe, Comma, Period, Slash,
    Kp0, Kp1, Kp2, Kp3, Kp4, Kp5, Kp6, Kp7, Kp8, Kp9, KpDecimal, KpDivide, KpMultiply, KpSubtract,
    KpAdd, KpEnter, KpEquals, LeftControl, LeftShift, LeftAlt, LeftOs, RightControl, RightShift,
    RightAlt, RightOs, CapsLock, ScrollLock, NumLock
}

pub enum FileAction {
    Create,
    Delete,
    Modify,
    Rename,
}

pub enum Permission {
    AudioCapture
}

fn keycode_to_key(code: u32) -> Option<Key> {
    match code {
        lovr_sys::os_key_OS_KEY_A               => Some(Key::A),
        lovr_sys::os_key_OS_KEY_B               => Some(Key::B),
        lovr_sys::os_key_OS_KEY_C               => Some(Key::C),
        lovr_sys::os_key_OS_KEY_D               => Some(Key::D),
        lovr_sys::os_key_OS_KEY_E               => Some(Key::E),
        lovr_sys::os_key_OS_KEY_F               => Some(Key::F),
        lovr_sys::os_key_OS_KEY_G               => Some(Key::G),
        lovr_sys::os_key_OS_KEY_H               => Some(Key::H),
        lovr_sys::os_key_OS_KEY_I               => Some(Key::I),
        lovr_sys::os_key_OS_KEY_J               => Some(Key::J),
        lovr_sys::os_key_OS_KEY_K               => Some(Key::K),
        lovr_sys::os_key_OS_KEY_L               => Some(Key::L),
        lovr_sys::os_key_OS_KEY_M               => Some(Key::M),
        lovr_sys::os_key_OS_KEY_N               => Some(Key::N),
        lovr_sys::os_key_OS_KEY_O               => Some(Key::O),
        lovr_sys::os_key_OS_KEY_P               => Some(Key::P),
        lovr_sys::os_key_OS_KEY_Q               => Some(Key::Q),
        lovr_sys::os_key_OS_KEY_R               => Some(Key::R),
        lovr_sys::os_key_OS_KEY_S               => Some(Key::S),
        lovr_sys::os_key_OS_KEY_T               => Some(Key::T),
        lovr_sys::os_key_OS_KEY_U               => Some(Key::U),
        lovr_sys::os_key_OS_KEY_V               => Some(Key::V),
        lovr_sys::os_key_OS_KEY_W               => Some(Key::W),
        lovr_sys::os_key_OS_KEY_X               => Some(Key::X),
        lovr_sys::os_key_OS_KEY_Y               => Some(Key::Y),
        lovr_sys::os_key_OS_KEY_Z               => Some(Key::Z),
        lovr_sys::os_key_OS_KEY_0               => Some(Key::Number0),
        lovr_sys::os_key_OS_KEY_1               => Some(Key::Number1),
        lovr_sys::os_key_OS_KEY_2               => Some(Key::Number2),
        lovr_sys::os_key_OS_KEY_3               => Some(Key::Number3),
        lovr_sys::os_key_OS_KEY_4               => Some(Key::Number4),
        lovr_sys::os_key_OS_KEY_5               => Some(Key::Number5),
        lovr_sys::os_key_OS_KEY_6               => Some(Key::Number6),
        lovr_sys::os_key_OS_KEY_7               => Some(Key::Number7),
        lovr_sys::os_key_OS_KEY_8               => Some(Key::Number8),
        lovr_sys::os_key_OS_KEY_9               => Some(Key::Number9),
        lovr_sys::os_key_OS_KEY_SPACE           => Some(Key::Space),
        lovr_sys::os_key_OS_KEY_ENTER           => Some(Key::Enter),
        lovr_sys::os_key_OS_KEY_TAB             => Some(Key::Tab),
        lovr_sys::os_key_OS_KEY_ESCAPE          => Some(Key::Escape),
        lovr_sys::os_key_OS_KEY_BACKSPACE       => Some(Key::Backspace),
        lovr_sys::os_key_OS_KEY_UP              => Some(Key::Up),
        lovr_sys::os_key_OS_KEY_DOWN            => Some(Key::Down),
        lovr_sys::os_key_OS_KEY_LEFT            => Some(Key::Left),
        lovr_sys::os_key_OS_KEY_RIGHT           => Some(Key::Right),
        lovr_sys::os_key_OS_KEY_HOME            => Some(Key::Home),
        lovr_sys::os_key_OS_KEY_END             => Some(Key::End),
        lovr_sys::os_key_OS_KEY_PAGE_UP         => Some(Key::PageUp),
        lovr_sys::os_key_OS_KEY_PAGE_DOWN       => Some(Key::PageDown),
        lovr_sys::os_key_OS_KEY_INSERT          => Some(Key::Insert),
        lovr_sys::os_key_OS_KEY_DELETE          => Some(Key::Delete),
        lovr_sys::os_key_OS_KEY_F1              => Some(Key::F1),
        lovr_sys::os_key_OS_KEY_F2              => Some(Key::F2),
        lovr_sys::os_key_OS_KEY_F3              => Some(Key::F3),
        lovr_sys::os_key_OS_KEY_F4              => Some(Key::F4),
        lovr_sys::os_key_OS_KEY_F5              => Some(Key::F5),
        lovr_sys::os_key_OS_KEY_F6              => Some(Key::F6),
        lovr_sys::os_key_OS_KEY_F7              => Some(Key::F7),
        lovr_sys::os_key_OS_KEY_F8              => Some(Key::F8),
        lovr_sys::os_key_OS_KEY_F9              => Some(Key::F9),
        lovr_sys::os_key_OS_KEY_F10             => Some(Key::F10),
        lovr_sys::os_key_OS_KEY_F11             => Some(Key::F11),
        lovr_sys::os_key_OS_KEY_F12             => Some(Key::F12),
        lovr_sys::os_key_OS_KEY_BACKTICK        => Some(Key::Backtick),
        lovr_sys::os_key_OS_KEY_MINUS           => Some(Key::Minus),
        lovr_sys::os_key_OS_KEY_EQUALS          => Some(Key::Equals),
        lovr_sys::os_key_OS_KEY_LEFT_BRACKET    => Some(Key::LeftBracket),
        lovr_sys::os_key_OS_KEY_RIGHT_BRACKET   => Some(Key::RightBracket),
        lovr_sys::os_key_OS_KEY_BACKSLASH       => Some(Key::Backslash),
        lovr_sys::os_key_OS_KEY_SEMICOLON       => Some(Key::Semicolon),
        lovr_sys::os_key_OS_KEY_APOSTROPHE      => Some(Key::Apostrophe),
        lovr_sys::os_key_OS_KEY_COMMA           => Some(Key::Comma),
        lovr_sys::os_key_OS_KEY_PERIOD          => Some(Key::Period),
        lovr_sys::os_key_OS_KEY_SLASH           => Some(Key::Slash),
        lovr_sys::os_key_OS_KEY_KP_0            => Some(Key::Kp0),
        lovr_sys::os_key_OS_KEY_KP_1            => Some(Key::Kp1),
        lovr_sys::os_key_OS_KEY_KP_2            => Some(Key::Kp2),
        lovr_sys::os_key_OS_KEY_KP_3            => Some(Key::Kp3),
        lovr_sys::os_key_OS_KEY_KP_4            => Some(Key::Kp4),
        lovr_sys::os_key_OS_KEY_KP_5            => Some(Key::Kp5),
        lovr_sys::os_key_OS_KEY_KP_6            => Some(Key::Kp6),
        lovr_sys::os_key_OS_KEY_KP_7            => Some(Key::Kp7),
        lovr_sys::os_key_OS_KEY_KP_8            => Some(Key::Kp8),
        lovr_sys::os_key_OS_KEY_KP_9            => Some(Key::Kp9),
        lovr_sys::os_key_OS_KEY_KP_DECIMAL      => Some(Key::KpDecimal),
        lovr_sys::os_key_OS_KEY_KP_DIVIDE       => Some(Key::KpDivide),
        lovr_sys::os_key_OS_KEY_KP_MULTIPLY     => Some(Key::KpMultiply),
        lovr_sys::os_key_OS_KEY_KP_SUBTRACT     => Some(Key::KpSubtract),
        lovr_sys::os_key_OS_KEY_KP_ADD          => Some(Key::KpAdd),
        lovr_sys::os_key_OS_KEY_KP_ENTER        => Some(Key::KpEnter),
        lovr_sys::os_key_OS_KEY_KP_EQUALS       => Some(Key::KpEquals),
        lovr_sys::os_key_OS_KEY_LEFT_CONTROL    => Some(Key::LeftControl),
        lovr_sys::os_key_OS_KEY_LEFT_SHIFT      => Some(Key::LeftShift),
        lovr_sys::os_key_OS_KEY_LEFT_ALT        => Some(Key::LeftAlt),
        lovr_sys::os_key_OS_KEY_LEFT_OS         => Some(Key::LeftOs),
        lovr_sys::os_key_OS_KEY_RIGHT_CONTROL   => Some(Key::RightControl),
        lovr_sys::os_key_OS_KEY_RIGHT_SHIFT     => Some(Key::RightShift),
        lovr_sys::os_key_OS_KEY_RIGHT_ALT       => Some(Key::RightAlt),
        lovr_sys::os_key_OS_KEY_RIGHT_OS        => Some(Key::RightOs),
        lovr_sys::os_key_OS_KEY_CAPS_LOCK       => Some(Key::CapsLock),
        lovr_sys::os_key_OS_KEY_SCROLL_LOCK     => Some(Key::ScrollLock),
        lovr_sys::os_key_OS_KEY_NUM_LOCK        => Some(Key::NumLock),
        _ => None
    }
}

// default callbacks

#[allow(unused_variables)] 
pub trait LovrCallbacks {
    fn load(&mut self, arg: &[String]) -> LovrResult<()> { Ok(()) }
    fn update(&mut self, dt: f64) -> LovrResult<()> { Ok(()) }
    fn draw(&mut self, pass: &mut Pass) -> LovrResult<bool> { Ok(false) }
    fn mirror(&mut self, pass: &mut Pass) -> LovrResult<bool> { self.draw(pass) }

    fn quit(&mut self) -> LovrResult<bool> { Ok(false) }
    fn visible(&mut self, visible: bool, display: DisplayType) -> LovrResult<()> { Ok(()) }
    fn focus(&mut self, focused: bool, display: DisplayType) -> LovrResult<()> { Ok(()) }
    fn mount(&mut self, mounted: bool) -> LovrResult<()> { Ok(()) }
    fn recenter(&mut self) -> LovrResult<()> { Ok(()) }
    fn models_changed(&mut self) -> LovrResult<()> { Ok(()) }
    fn resize(&mut self, width: u32, height: u32) -> LovrResult<()> { Ok(()) }
    fn key_pressed(&mut self, code: Key, scancode: u32, repeat: bool) -> LovrResult<()> { Ok(()) }
    fn key_released(&mut self, code: Key, scancode: u32) -> LovrResult<()> { Ok(()) }
    fn text_input(&mut self, utf8: String, codepoint: u32) -> LovrResult<()> { Ok(()) }
    fn mouse_pressed(&mut self, x: f64, y: f64, button: i32) -> LovrResult<()> { Ok(()) }
    fn mouse_released(&mut self, x: f64, y: f64, button: i32) -> LovrResult<()> { Ok(()) }
    fn mouse_moved(&mut self, x: f64, y: f64, dx: f64, dy: f64) -> LovrResult<()> { Ok(()) }
    fn mouse_wheel_moved(&mut self, x: f64, y: f64) -> LovrResult<()> { Ok(()) }
    fn thread_error(&mut self, error: String) -> LovrResult<()> { Ok(()) }
    fn file_changed(&mut self, path: String, action: FileAction, old_path: String) -> LovrResult<()> { Ok(()) }
    fn permission(&mut self, permission: Permission, granted: bool) -> LovrResult<()> { Ok(()) }
    fn custom(&mut self) -> LovrResult<()> { Ok(()) }

    fn run(mut self: Box<Self>) -> LovrResult<impl FnMut() -> LovrResult<RunCommand>> {
        lovr::timer::step();
        self.load(&[])?;
        Ok(move || -> LovrResult<RunCommand> {
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

fn lovr_assert(condition: bool) -> LovrResult<()> {
    if condition {
        return Ok(())
    }
    unsafe {
        let ptr = lovr_sys::lovrGetError();
        let msg = if ptr.is_null() {
            "Unknown LOVR error".to_string()
        } else {
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        };
        Err(LovrError(msg, Backtrace::force_capture()))
    }
}

#[derive(Debug)]
pub struct LovrError(String, Backtrace);

impl From<LovrError> for mlua::Error {
    fn from(err: LovrError) -> Self {
        let bt = err.1.to_string().lines()
            .take_while(|line| !line.contains(": lovr_rs::lovr_run::{{closure}}"))
            .collect::<Vec<_>>()
            .join("\n");
        mlua::Error::RuntimeError(err.0 + "\n\n" + &bt + "\n")
    }
}

pub type LovrResult<T> = Result<T, LovrError>;
