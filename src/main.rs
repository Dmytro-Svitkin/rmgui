use rmgui::window::Size;

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

