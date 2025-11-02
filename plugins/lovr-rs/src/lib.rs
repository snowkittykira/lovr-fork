// game

struct Game;

impl LovrCallbacks for Game {

    fn draw(&mut self, pass: &mut Pass) -> LovrResult<bool> {
        pass.sphere(Mat4::from_translation(vec3(0., 0., -2.)), 16, 16)?;
        Ok(false)
    }

    fn key_pressed(&mut self, code: Key, _scancode: u32, _repeat: bool) -> LovrResult<()> {
        if let Key::Escape = code {
            lovr::event::quit(0)
        }
        Ok(())
    }

}

// bindings

mod lovr_sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use glam::{vec3, Mat4};
use mlua::prelude::*;
use std::backtrace::Backtrace;
use std::ffi::CStr;

#[mlua::lua_module]
fn lovr_rs(lua: &Lua) -> LuaResult<LuaValue> {
    lua.globals()
        .get::<LuaTable>("lovr")?
        .set("run", lua.create_function(lovr_run)?)?;

    Ok(LuaNil)
}

fn lovr_run(lua: &Lua, _: ()) -> LuaResult<LuaFunction> {
    let game = Box::new(Game);
    let mut game_loop = game.run();
    lua.create_function_mut(move |_lua, _: ()| -> LuaResult<LuaValue> {
        match game_loop()? {
            RunCommand::Quit(code) => Ok(LuaValue::Number(code.into())),
            RunCommand::Continue => Ok(LuaNil),
        }
    })
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
struct LovrError(String, Backtrace);

impl From<LovrError> for mlua::Error {
    fn from(err: LovrError) -> Self {
        let bt = err.1.to_string().lines()
            .take_while(|line| !line.contains("lovr_rs::lovr_run::{{closure}}"))
            .collect::<Vec<_>>()
            .join("\n");
        mlua::Error::RuntimeError(err.0 + "\n\n" + &bt + "\n")
    }
}

type LovrResult<T> = Result<T, LovrError>;

mod lovr {
    use super::*;

    pub mod event {
        use super::*;

        pub fn poll() -> Option<Event> {
            unsafe {
                let mut event: lovr_sys::Event = std::mem::zeroed();
                if lovr_sys::lovrEventPoll(&mut event) {
                    match event.type_ {
                        lovr_sys::EventType_EVENT_QUIT => Some(Event::Quit {
                            exit_code: event.data.quit.exitCode,
                        }),
                        lovr_sys::EventType_EVENT_KEYPRESSED => 
                            if let Some(code) = keycode_to_key(event.data.key.code) {
                                Some(Event::KeyPressed {
                                    code,
                                    scancode: event.data.key.scancode,
                                    repeat: event.data.key.repeat,
                                })
                            } else {
                                Some(Event::Other) // TODO: what to do here?
                            }
                        _ => Some(Event::Other) // TODO: handle other events
                    }
                } else {
                    None
                }
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

    pub mod system {
        use super::*;

        pub fn poll_events() {
            unsafe {
                lovr_sys::lovrSystemPollEvents()
            }
        }
    }
}

struct Pass {
    ptr: *mut lovr_sys::Pass
}

impl Pass {
    fn sphere(&mut self, transform: Mat4, longitudes: u32, latitudes:u32) -> LovrResult<()> {
        unsafe {
            lovr_assert(lovr_sys::lovrPassSphere(self.ptr, transform.to_cols_array().as_mut_ptr(), longitudes, latitudes))
        }
    }
}

pub enum Event {
    Quit { exit_code: i32 },
    KeyPressed { code: Key, scancode: u32, repeat: bool },
    Other // TODO remove
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

#[allow(unused_variables)] 
trait LovrCallbacks {
    fn draw(&mut self, pass: &mut Pass) -> LovrResult<bool> {
        Ok(false)
    }

    fn key_pressed(&mut self, code: Key, scancode: u32, repeat: bool) -> LovrResult<()> {
        Ok(())
    }

    fn quit(&mut self) -> LovrResult<bool> {
        Ok(false)
    }

    fn run(mut self: Box<Self>) -> impl FnMut() -> LovrResult<RunCommand> {
        move || -> LovrResult<RunCommand> {
            lovr::system::poll_events();

            while let Some(event) = lovr::event::poll() {
                match event {
                    Event::Quit { exit_code } => {
                        if !self.quit()? {
                            return Ok(RunCommand::Quit(exit_code))
                        }
                    },
                    Event::KeyPressed { code, scancode, repeat } => {
                        self.key_pressed(code, scancode, repeat)?;
                    }
                    _ => () // TODO: remove
                }
            }

            if let Some(mut pass) = lovr::graphics::get_window_pass()? && !self.draw(&mut pass)? {
                lovr::graphics::submit(&mut [pass])?
            }

            lovr::graphics::present()?;

            Ok(RunCommand::Continue)
        }
    }
}

enum RunCommand {
    Continue,
    Quit(i32),
}
