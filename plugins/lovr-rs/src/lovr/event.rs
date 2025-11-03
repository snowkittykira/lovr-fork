use std::ffi::CStr;

use crate::{lovr_sys, lovr::*};

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

