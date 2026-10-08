use display_info::DisplayInfo;
use crate::window::{Size};

fn get_primary_screen()->Option<DisplayInfo>{
    let Ok(displays)=DisplayInfo::all()else{return None};
    for display in&displays{if display.is_primary{return Some(display.clone())}}
    None
}

fn get_screen()->Option<DisplayInfo>{
    let Ok(displays)=DisplayInfo::all()else{return None};
    if!displays.is_empty(){
        if!&displays.len()==1{
            for display in&displays{if display.is_primary{return Some(display.clone())}}
            for display in&displays{if display.is_builtin{return Some(display.clone())}}// Fallback in case of primary screen not showing as primary but being built-in; might remove it after some testing.
        }
        return Some(displays[0].clone())
    }
    None
}

fn get_all_screens()->Vec<DisplayInfo>{
    let Ok(display)=DisplayInfo::all()else{return Vec::new()};
    display
}

pub fn get_screen_quantity()->usize{
    get_all_screens().len()
}
//
pub fn get_primary_screen_size()->Size<u32>{
    let Some(display)=get_primary_screen()else{panic!("[!] PRIMARY SCREEN NOT FOUND")};    
    Size{width:display.width,height:display.height}
}

pub fn get_screen_size()->Size<u32>{
    let Some(display)=get_screen()else{panic!("[!] SCREEN NOT FOUND")};    
    Size{width:display.width,height:display.height}
}

pub fn get_primary_screen_size_or(fallback_size:Size<u32>)->Size<u32>{
    let Some(display)=get_primary_screen()else{return fallback_size};
    Size{width:display.width,height:display.height}
}

pub fn get_screen_size_or(fallback_size:Size<u32>)->Size<u32>{
    let Some(display)=get_screen()else{return fallback_size};    
    Size{width:display.width,height:display.height}
}

pub fn get_primary_screen_size_or_0()->Size<u32>{
    get_primary_screen_size_or(Size{width:0,height:0})
}

pub fn get_screen_size_or_0()->Size<u32>{
    get_screen_size_or(Size{width:0,height:0})
}

pub fn get_primary_screen_size_or_1080()->Size<u32>{
    get_primary_screen_size_or(Size{width:1920,height:1080})
}

pub fn get_screen_size_or_1080()->Size<u32>{
    get_screen_size_or(Size{width:1920,height:1080})
}

pub fn get_all_screens_sizes()->Vec<Size<u32>>{
    let displays:Vec<DisplayInfo>=get_all_screens();
    let mut result:Vec<Size<u32>>=Vec::new();
    for display in&displays{
        result.push(Size{width:display.width,height:display.height})
    }
    result
}
// Size above, scale below.
pub fn get_primary_screen_scale()->f32{// Might change f32 return to a struct LogicalPixel(relation:f32) or alike.
    let Some(display)=get_primary_screen()else{panic!("[!] PRIMARY SCREEN NOT FOUND")};    
    display.scale_factor
}

pub fn get_screen_scale()->f32{
    let Some(display)=get_screen()else{panic!("[!] SCREEN NOT FOUND")};    
    display.scale_factor
}

pub fn get_primary_screen_scale_or(fallback_scale:f32)->f32{
    let Some(display)=get_primary_screen()else{return fallback_scale};
    display.scale_factor
}

pub fn get_screen_scale_or(fallback_scale:f32)->f32{
    let Some(display)=get_screen()else{return fallback_scale};    
    display.scale_factor
}

pub fn get_primary_screen_scale_or_0()->f32{
    get_primary_screen_scale_or(0.0)
}


pub fn get_screen_scale_or_0()->f32{
    get_screen_scale_or(1.0)
}

pub fn get_primary_screen_scale_or_1()->f32{
    get_primary_screen_scale_or(1.0)
}

pub fn get_screen_scale_or_1()->f32{
    get_screen_scale_or(0.0)
}

pub fn get_all_screens_scales()->Vec<f32>{
    let displays:Vec<DisplayInfo>=get_all_screens();
    let mut result:Vec<f32>=Vec::new();
    for display in&displays{
        result.push(display.scale_factor);
    };
    result
}
// Scale above, frequency below.
pub fn get_primary_screen_frequency()->f32{
    let Some(display)=get_primary_screen()else{panic!("[!] PRIMARY SCREEN NOT FOUND")};    
    display.frequency
}

pub fn get_screen_frequency()->f32{
    let Some(display)=get_screen()else{panic!("[!] SCREEN NOT FOUND")};    
    display.frequency
}

pub fn get_primary_screen_frequency_or(fallback_scale:f32)->f32{
    let Some(display)=get_primary_screen()else{return fallback_scale};
    display.frequency
}

pub fn get_screen_frequency_or(fallback_scale:f32)->f32{
    let Some(display)=get_screen()else{return fallback_scale};    
    display.frequency
}

pub fn get_primary_screen_frequency_or_0()->f32{
    get_primary_screen_frequency_or(0.0)
}

pub fn get_screen_frequency_or_0()->f32{
    get_screen_frequency_or(0.0)
}

pub fn get_primary_screen_frequency_or_60()->f32{
    get_primary_screen_frequency_or(60.0)
}

pub fn get_screen_frequency_or_60()->f32{
    get_screen_frequency_or(60.0)
}

pub fn get_all_screens_frequencies()->Vec<f32>{
    let displays:Vec<DisplayInfo>=get_all_screens();
    let mut result:Vec<f32>=Vec::new();
    for display in&displays{
        result.push(display.frequency)
    }
    result
}

pub fn get_min_screens_frequency()->Option<f32>{
    get_all_screens_frequencies().iter().copied().reduce(f32::min)
}

pub fn get_max_screens_frequency()->Option<f32>{
    get_all_screens_frequencies().iter().copied().reduce(f32::max)
}
// Frequency above, rotation below.
pub fn get_primary_screen_rotation()->f32{
    let Some(display)=get_primary_screen()else{panic!("[!] PRIMARY SCREEN NOT FOUND")};    
    display.rotation
}

pub fn get_screen_rotation()->f32{
    let Some(display)=get_screen()else{panic!("[!] SCREEN NOT FOUND")};    
    display.rotation
}

pub fn get_primary_screen_rotation_or(fallback_scale:f32)->f32{
    let Some(display)=get_primary_screen()else{return fallback_scale};
    display.rotation
}

pub fn get_screen_rotation_or(fallback_scale:f32)->f32{
    let Some(display)=get_screen()else{return fallback_scale};    
    display.rotation
}

pub fn get_primary_screen_rotation_or_0()->f32{
    get_primary_screen_rotation_or(0.0)
}

pub fn get_screen_rotation_or_0()->f32{
    get_screen_rotation_or(0.0)
}

pub fn get_all_screens_rotations()->Vec<f32>{
    let displays:Vec<DisplayInfo>=get_all_screens();
    let mut result:Vec<f32>=Vec::new();
    for display in&displays{
        result.push(display.rotation)
    }
    result
}