// based on https://github.com/stevelittlefish/c_vulkan_sdl3/blob/1d1f80693d44860b34e505c743610d4095d3e618/src/main.c

#include "os.h"

#include <stdio.h>
#include <stdint.h>
#include <SDL3/SDL.h>
#include <SDL3/SDL_clipboard.h>
#include <SDL3/SDL_events.h>
#include <SDL3/SDL_keycode.h>
#include <SDL3/SDL_stdinc.h>
#include <SDL3/SDL_video.h>
#include <SDL3/SDL_vulkan.h>

static struct {
  SDL_Window* window;
  fn_quit* onQuitRequest;
  fn_visible* onWindowVisible;
  fn_focus* onWindowFocus;
  fn_resize* onWindowResize;
  fn_key* onKeyboardEvent;
  fn_text* onTextEvent;
  fn_mouse_button* onMouseButton;
  fn_mouse_move* onMouseMove;
  fn_mousewheel_move* onMouseWheelMove;
  uint32_t width;
  uint32_t height;
} sdlState;

const char* os_get_clipboard_text(void) {
  return SDL_GetClipboardText();
}

void os_set_clipboard_text(const char* text) {
    SDL_SetClipboardText(text);
}

// -1 for unknown, otherwise returns an os_key
static int get_key(SDL_Keycode keycode) {
  switch (keycode) {
    case SDLK_A:            return OS_KEY_A;
    case SDLK_B:            return OS_KEY_B;
    case SDLK_C:            return OS_KEY_C;
    case SDLK_D:            return OS_KEY_D;
    case SDLK_E:            return OS_KEY_E;
    case SDLK_F:            return OS_KEY_F;
    case SDLK_G:            return OS_KEY_G;
    case SDLK_H:            return OS_KEY_H;
    case SDLK_I:            return OS_KEY_I;
    case SDLK_J:            return OS_KEY_J;
    case SDLK_K:            return OS_KEY_K;
    case SDLK_L:            return OS_KEY_L;
    case SDLK_M:            return OS_KEY_M;
    case SDLK_N:            return OS_KEY_N;
    case SDLK_O:            return OS_KEY_O;
    case SDLK_P:            return OS_KEY_P;
    case SDLK_Q:            return OS_KEY_Q;
    case SDLK_R:            return OS_KEY_R;
    case SDLK_S:            return OS_KEY_S;
    case SDLK_T:            return OS_KEY_T;
    case SDLK_U:            return OS_KEY_U;
    case SDLK_V:            return OS_KEY_V;
    case SDLK_W:            return OS_KEY_W;
    case SDLK_X:            return OS_KEY_X;
    case SDLK_Y:            return OS_KEY_Y;
    case SDLK_Z:            return OS_KEY_Z;
    case SDLK_0:            return OS_KEY_0;
    case SDLK_1:            return OS_KEY_1;
    case SDLK_2:            return OS_KEY_2;
    case SDLK_3:            return OS_KEY_3;
    case SDLK_4:            return OS_KEY_4;
    case SDLK_5:            return OS_KEY_5;
    case SDLK_6:            return OS_KEY_6;
    case SDLK_7:            return OS_KEY_7;
    case SDLK_8:            return OS_KEY_8;
    case SDLK_9:            return OS_KEY_9;
    case SDLK_SPACE:        return OS_KEY_SPACE;
    case SDLK_RETURN:       return OS_KEY_ENTER;
    case SDLK_TAB:          return OS_KEY_TAB;
    case SDLK_ESCAPE:       return OS_KEY_ESCAPE;
    case SDLK_BACKSPACE:    return OS_KEY_BACKSPACE;
    case SDLK_UP:           return OS_KEY_UP;
    case SDLK_DOWN:         return OS_KEY_DOWN;
    case SDLK_LEFT:         return OS_KEY_LEFT;
    case SDLK_RIGHT:        return OS_KEY_RIGHT;
    case SDLK_HOME:         return OS_KEY_HOME;
    case SDLK_END:          return OS_KEY_END;
    case SDLK_PAGEUP:       return OS_KEY_PAGE_UP;
    case SDLK_PAGEDOWN:     return OS_KEY_PAGE_DOWN;
    case SDLK_INSERT:       return OS_KEY_INSERT;
    case SDLK_DELETE:       return OS_KEY_DELETE;
    case SDLK_F1:           return OS_KEY_F1;
    case SDLK_F2:           return OS_KEY_F2;
    case SDLK_F3:           return OS_KEY_F3;
    case SDLK_F4:           return OS_KEY_F4;
    case SDLK_F5:           return OS_KEY_F5;
    case SDLK_F6:           return OS_KEY_F6;
    case SDLK_F7:           return OS_KEY_F7;
    case SDLK_F8:           return OS_KEY_F8;
    case SDLK_F9:           return OS_KEY_F9;
    case SDLK_F10:          return OS_KEY_F10;
    case SDLK_F11:          return OS_KEY_F11;
    case SDLK_F12:          return OS_KEY_F12;
    case SDLK_GRAVE:        return OS_KEY_BACKTICK;
    case SDLK_MINUS:        return OS_KEY_MINUS;
    case SDLK_EQUALS:       return OS_KEY_EQUALS;
    case SDLK_LEFTBRACKET:  return OS_KEY_LEFT_BRACKET;
    case SDLK_RIGHTBRACKET: return OS_KEY_RIGHT_BRACKET;
    case SDLK_BACKSLASH:    return OS_KEY_BACKSLASH;
    case SDLK_SEMICOLON:    return OS_KEY_SEMICOLON;
    case SDLK_APOSTROPHE:   return OS_KEY_APOSTROPHE;
    case SDLK_COMMA:        return OS_KEY_COMMA;
    case SDLK_PERIOD:       return OS_KEY_PERIOD;
    case SDLK_SLASH:        return OS_KEY_SLASH;
    case SDLK_KP_0:         return OS_KEY_KP_0;
    case SDLK_KP_1:         return OS_KEY_KP_1;
    case SDLK_KP_2:         return OS_KEY_KP_2;
    case SDLK_KP_3:         return OS_KEY_KP_3;
    case SDLK_KP_4:         return OS_KEY_KP_4;
    case SDLK_KP_5:         return OS_KEY_KP_5;
    case SDLK_KP_6:         return OS_KEY_KP_6;
    case SDLK_KP_7:         return OS_KEY_KP_7;
    case SDLK_KP_8:         return OS_KEY_KP_8;
    case SDLK_KP_9:         return OS_KEY_KP_9;
    case SDLK_KP_DECIMAL:   return OS_KEY_KP_DECIMAL;
    case SDLK_KP_DIVIDE:    return OS_KEY_KP_DIVIDE;
    case SDLK_KP_MULTIPLY:  return OS_KEY_KP_MULTIPLY;
    case SDLK_KP_MINUS:     return OS_KEY_KP_SUBTRACT;
    case SDLK_KP_PLUS:      return OS_KEY_KP_ADD;
    case SDLK_KP_ENTER:     return OS_KEY_KP_ENTER;
    case SDLK_KP_EQUALS:    return OS_KEY_KP_EQUALS;
    case SDLK_LCTRL:        return OS_KEY_LEFT_CONTROL;
    case SDLK_LSHIFT:       return OS_KEY_LEFT_SHIFT;
    case SDLK_LALT:         return OS_KEY_LEFT_ALT;
    case SDLK_LMETA:        return OS_KEY_LEFT_OS;
    case SDLK_RCTRL:        return OS_KEY_RIGHT_CONTROL;
    case SDLK_RSHIFT:       return OS_KEY_RIGHT_SHIFT;
    case SDLK_RALT:         return OS_KEY_RIGHT_ALT;
    case SDLK_RMETA:        return OS_KEY_RIGHT_OS;
    case SDLK_CAPSLOCK:     return OS_KEY_CAPS_LOCK;
    case SDLK_SCROLLLOCK:   return OS_KEY_SCROLL_LOCK;
    case SDLK_NUMLOCKCLEAR: return OS_KEY_NUM_LOCK;
    default: return -1;
  }
}

void os_poll_events(void) {
  SDL_Event event;
  while (SDL_PollEvent(&event)) {
    switch (event.type) {
      case SDL_EVENT_QUIT:
        if (sdlState.onQuitRequest) {
          sdlState.onQuitRequest();
        }
        break;
      case SDL_EVENT_WINDOW_SHOWN:
      case SDL_EVENT_WINDOW_HIDDEN:
        if (sdlState.onWindowVisible) {
          sdlState.onWindowVisible(event.type == SDL_EVENT_WINDOW_SHOWN);
        }
        break;
      case SDL_EVENT_WINDOW_FOCUS_GAINED:
      case SDL_EVENT_WINDOW_FOCUS_LOST:
        if (sdlState.onWindowVisible) {
          sdlState.onWindowFocus(event.type == SDL_EVENT_WINDOW_FOCUS_GAINED);
        }
        break;
      case SDL_EVENT_WINDOW_RESIZED:
        if (sdlState.onWindowResize) {
          sdlState.onWindowResize(event.window.data1, event.window.data2);
        }
        break;
      case SDL_EVENT_KEY_DOWN:
      case SDL_EVENT_KEY_UP:
        if (sdlState.onKeyboardEvent) {
          int key = get_key (event.key.key);
          if (key >= 0) {
            sdlState.onKeyboardEvent(
              event.key.down ? BUTTON_PRESSED : BUTTON_RELEASED,
              key,
              event.key.scancode,
              event.key.repeat);
          }
        }
        break;
      case SDL_EVENT_TEXT_INPUT:
        if (sdlState.onTextEvent) {
          const char *text = event.text.text;
          uint32_t codepoint = SDL_StepUTF8(&text, NULL);
          while (codepoint) {
            sdlState.onTextEvent(codepoint);
            codepoint = SDL_StepUTF8(&text, NULL);
          }
        }
        break;
      case SDL_EVENT_MOUSE_BUTTON_DOWN:
      case SDL_EVENT_MOUSE_BUTTON_UP:
        if (sdlState.onMouseButton) {
          int button = -1;
          switch (event.button.button) {
            case SDL_BUTTON_LEFT: button = MOUSE_LEFT; break;
            case SDL_BUTTON_RIGHT: button = MOUSE_RIGHT; break;
          }
          if (button >= 0) {
            sdlState.onMouseButton(button, event.button.down);
          }
        }
        break;
      case SDL_EVENT_MOUSE_MOTION:
        if (sdlState.onMouseMove) {
          sdlState.onMouseMove(event.motion.x, event.motion.y);
        }
        break;
      case SDL_EVENT_MOUSE_WHEEL:
        if (sdlState.onMouseWheelMove) {
          sdlState.onMouseWheelMove(event.wheel.x, event.wheel.y);
        }
        break;
    }
  }
}

bool os_window_open(const os_window_config* config) {
  if (sdlState.window) {
    return true;
  }

  if (!SDL_Init(SDL_INIT_VIDEO)) {
    SDL_Log("SDL initialization failed: %s\n", SDL_GetError());
    return false;
  }

  // window flags from config
  SDL_WindowFlags window_flags = SDL_WINDOW_VULKAN | SDL_WINDOW_HIDDEN;
  if (config->resizable) {
    window_flags |= SDL_WINDOW_RESIZABLE;
  }
  if (config->width == 0 && config->height == 0 || config->fullscreen) {
    window_flags |= SDL_WINDOW_FULLSCREEN;
  }
  SDL_DisplayID id = SDL_GetPrimaryDisplay();
  const SDL_DisplayMode* mode = SDL_GetDesktopDisplayMode(id);
  uint32_t width = config->width ? config->width : mode->w;
  uint32_t height = config->height ? config->height : mode->h;

  sdlState.window = SDL_CreateWindow(config->title, width, height, window_flags);
  if (!sdlState.window) {
    SDL_Log("SDL window creation failed: %s\n", SDL_GetError());
    SDL_Quit();
    return false;
  }

  SDL_SetWindowPosition(sdlState.window, SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED);
  // TODO: fullscreen not working?
  if (config->fullscreen) {
    SDL_SetWindowFullscreenMode(sdlState.window, mode);
  }
  if (config->icon.data) {
    SDL_Surface* icon_surface = SDL_CreateSurfaceFrom(
      config->icon.width,
      config->icon.height,
      SDL_PIXELFORMAT_RGBA32,
      config->icon.data,
      width * 4);
    SDL_SetWindowIcon(sdlState.window, icon_surface);
    SDL_DestroySurface(icon_surface);
  }
  SDL_ShowWindow(sdlState.window);

  sdlState.width = width;
  sdlState.height = height;

  return true;
}

bool os_window_is_open(void) {
  return sdlState.window;
}

bool os_window_is_visible(void) {
  if (!sdlState.window) {
    return false;
  }
  return !(SDL_GetWindowFlags(sdlState.window) & SDL_WINDOW_MINIMIZED);
}

bool os_window_is_focused(void) {
  if (!sdlState.window) {
    return false;
  }
  return SDL_GetWindowFlags(sdlState.window) & SDL_WINDOW_INPUT_FOCUS;
}

void os_window_get_size(uint32_t* width, uint32_t* height) {
  *width = sdlState.width;
  *height = sdlState.height;
}

float os_window_get_pixel_density(void) {
  if (!sdlState.window) {
    return 0.f;
  }
  return SDL_GetWindowPixelDensity(sdlState.window);
}

void os_on_quit(fn_quit* callback) {
  sdlState.onQuitRequest = callback;
}

void os_on_visible(fn_focus* callback) {
  sdlState.onWindowVisible = callback;
}

void os_on_focus(fn_focus* callback) {
  sdlState.onWindowFocus = callback;
}

void os_on_resize(fn_resize* callback) {
  sdlState.onWindowResize = callback;
}

void os_on_key(fn_key* callback) {
  sdlState.onKeyboardEvent = callback;
}

void os_on_text(fn_text* callback) {
  sdlState.onTextEvent = callback;
}

void os_on_mouse_button(fn_mouse_button* callback) {
  sdlState.onMouseButton = callback;
}

void os_on_mouse_move(fn_mouse_move* callback) {
  sdlState.onMouseMove = callback;
}

void os_on_mousewheel_move(fn_mousewheel_move* callback) {
  sdlState.onMouseWheelMove = callback;
}

void os_get_mouse_position(double* x, double* y) {
  if (!sdlState.window) {
    *x = *y = 0.;
  }
  float xf, yf;
  SDL_GetMouseState(&xf, &yf);
  *x = xf;
  *y = yf;
}

os_mouse_mode os_get_mouse_mode(void) {
  if (!sdlState.window) {
    return MOUSE_MODE_NORMAL;
  }
  return SDL_GetWindowMouseGrab(sdlState.window) ?
      MOUSE_MODE_GRABBED :
      MOUSE_MODE_NORMAL;
}

void os_set_mouse_mode(os_mouse_mode mode) {
  if (!sdlState.window) {
    return;
  }
  SDL_SetWindowMouseGrab(sdlState.window, mode == MOUSE_MODE_GRABBED);
}

uintptr_t os_get_win32_window(void) {
  return 0;
}

uintptr_t os_get_win32_instance(void) {
  return 0;
}

uintptr_t os_get_ca_metal_layer(void) {
  return 0;
}

uintptr_t os_get_xcb_connection(void) {
  return 0;
}

uintptr_t os_get_xcb_window(void) {
  return 0;
}

uintptr_t os_get_sdl_window(void) {
  return (uintptr_t) sdlState.window;
}
