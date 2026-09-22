use rmgui::window::Size;

use crate::window::get_screen_scale_or_0;

pub mod window;
mod window_display;
pub mod render;
pub mod internal;
pub mod web;
pub mod color;

fn main(){
    let a:Size<u32>=Size::new();
    println!("{a:?}")
}

