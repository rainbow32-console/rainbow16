use std::path::PathBuf;

use crate::{
    c_singleton,
    file_parser::load_r16_png,
    fs::{read, read_dir},
    gamestate::{load_game, run_game},
    get_s_val,
    image::Image,
    keyboard::{button_is_pressed, is_key_down},
    set_s_val,
    system::Keycode,
    Singleton, CARTSPATH,
};

use super::{
    background::{render_bg, update_bg},
    canvas_functions::set_pixel,
    menu::{self, get_selected},
    message::set_message,
    overlay::hide_overlay,
};

c_singleton!(PATH, PathBuf, || get_s_val!(CARTSPATH).to_path_buf());
c_singleton!(ITEMS, Vec<String>, || {
    let mut new = if get_s_val!(PATH) == get_s_val!(CARTSPATH) {
        vec![]
    } else {
        vec!["..".to_string()]
    };

    println!("Reading {}", get_s_val!(PATH).display());
    match read_dir(get_s_val!(PATH)) {
        Some(entries) => {
            for e in entries {
                if e.is_dir() {
                    new.push(e.get_name() + "/");
                } else if e.is_file() {
                    new.push(e.get_name());
                }
            }
        }
        None => println!("Failed to read the directory"),
    }

    new
});

c_singleton!(VALUE, (u32, Option<Image>, bool), || (0, None, false));

pub fn update() {
    update_bg();
    if let Some(idx) = menu::update(get_s_val!(ITEMS)) {
        if !button_is_pressed(crate::keyboard::Button::A) {
            return;
        }
        let item = get_s_val!(ITEMS)[idx as usize].clone();
        if item == ".." {
            get_s_val!(PATH).pop();
            unsafe {
                ITEMS.reset();
                VALUE.reset();
            }
        } else if item.ends_with("/") {
            unsafe {
                get_s_val!(PATH).push(&item[0..item.len() - 1]);
                ITEMS.reset();
                VALUE.reset();
            }
        } else {
            let path = get_s_val!(PATH).join(item);
            unsafe {
                PATH.reset();
                VALUE.reset();
                ITEMS.reset();
            }
            let v = read(&path);
            if let Some(v) = v {
                if load_game(v, path.to_str().map(|s| s.to_string())) {
                    if let Some(e) = run_game() {
                        set_message(&format!("{}", e));
                    }
                } else {
                    set_message("failed to read the file");
                }
            } else {
                set_message("failed to read the file");
            }
        }
    }
    let value = get_s_val!(VALUE);
    if value.0 != get_selected() {
        unsafe { VALUE.reset() };
        set_s_val!(VALUE, (get_selected(), None, false));
    }
    if get_s_val!(ITEMS).len() > get_selected() as usize
        && get_s_val!(VALUE).1.is_none()
        && get_s_val!(ITEMS)[get_selected() as usize].ends_with(".r16.png")
        && !get_s_val!(VALUE).2
    {
        let mut path = get_s_val!(PATH).clone();
        path.push(&get_s_val!(ITEMS)[get_selected() as usize]);
        let mut changed = false;
        if let Some(data) = read(&path) {
            if let Some(data) = load_r16_png(data, None) {
                if let Some(img) = data.preview_image {
                    set_s_val!(VALUE, (get_selected(), Some(img), true));
                    changed = true;
                }
            }
        }
        if !changed {
            set_s_val!(VALUE, (get_selected(), None, true));
        }
    }
    if is_key_down(Keycode::Escape) {
        hide_overlay();
    }
}

pub fn render() {
    if let Some(img) = &get_s_val!(VALUE).1 {
        img.put_on_canvas(set_pixel, 0, 0);
    } else {
        render_bg();
    }
    menu::render(get_s_val!(ITEMS));
}

pub fn init() {
    unsafe {
        PATH.reset();
        ITEMS.reset();
    }
}
