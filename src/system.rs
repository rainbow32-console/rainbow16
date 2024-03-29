// #[cfg(any(target_family = "unix", target_family = "windows"))]
use crate::systems::sdl2::{
    copy_to_clipboard as sdl2_copy_to_clipboard, get_size as sdl2_get_size, init as sdl2_init,
    read_clipboard as sdl2_read_clipboard, show_cursor as sdl2_show_cursor,
};

// #[cfg(target_family = "wasm")]
// use crate::systems::wasm::{
//     copy_to_clipboard as wasm_copy_to_clipboard, get_size as wasm_get_size, init as wasm_init,
//     read_clipboard as wasm_read_clipboard, show_cursor as wasm_show_cursor, update as wasm_update,
// };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keycode {
    Unknown,
    Backspace,
    Tab,
    Return,
    Escape,
    Space,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Delete,
    CapsLock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Right,
    Left,
    Down,
    Up,
    Ctrl,
    Shift,
    Alt,
    AltGr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Drop { file_data: Vec<u8>, name: String },
    Keydown { keycode: Keycode },
    Keyup { keycode: Keycode },
    Text { char: char },
    MouseDown { button: MouseButton, x: i32, y: i32 },
    MouseUp { button: MouseButton, x: i32, y: i32 },
    Scroll { x: i32, y: i32 },
    MouseMove { x: i32, y: i32 },
}

pub fn get_size() -> (u32, u32) {
    // #[cfg(any(target_family = "unix", target_family = "windows"))]
    return sdl2_get_size();
    // #[cfg(target_family = "wasm")]
    // return wasm_get_size();
}

pub fn show_cursor(value: bool) {
    // #[cfg(any(target_family = "unix", target_family = "windows"))]
    sdl2_show_cursor(value);
    // #[cfg(target_family = "wasm")]
    // wasm_show_cursor(value);
}

pub fn init() {
    // #[cfg(any(target_family = "unix", target_family = "windows"))]
    sdl2_init();
    // #[cfg(target_family = "wasm")]
    // wasm_init();
}

pub fn read_clipboard() -> String {
    // #[cfg(any(target_family = "unix", target_family = "windows"))]
    return sdl2_read_clipboard();
    // #[cfg(target_family = "wasm")]
    // return wasm_read_clipboard();
}

pub fn copy_to_clipboard(str: &str) {
    // #[cfg(target_family = "wasm")]
    // wasm_copy_to_clipboard(str);
    // #[cfg(any(target_family = "unix", target_family = "windows"))]
    sdl2_copy_to_clipboard(str);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirEntryType {
    Folder,
    File,
}

pub struct DirEntry {
    name: String,
    typ: DirEntryType
}

impl DirEntry {
    pub fn new(name: String, typ: DirEntryType) -> Self {
        Self {name, typ}
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn is_dir(&self) -> bool {
        self.typ == DirEntryType::Folder
    }

    pub fn is_file(&self) -> bool {
        self.typ == DirEntryType::File
    }
}