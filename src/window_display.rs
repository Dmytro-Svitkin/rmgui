use crate::window::{Size};

fn get_primary_screen()->Option<display_info::DisplayInfo>{
    let Ok(displays)=display_info::DisplayInfo::all()else{return None};
    for display in&displays{if display.is_primary{return Some(display.clone())}}
    None
}

fn get_screen()->Option<display_info::DisplayInfo>{
    let Ok(displays)=display_info::DisplayInfo::all()else{return None};
    if!displays.is_empty(){
        if!&displays.len()==1{
            for display in&displays{if display.is_primary{return Some(display.clone())}}
            for display in&displays{if display.is_builtin{return Some(display.clone())}}// Fallback in case of primary screen not showing as primary but being built-in; might remove it after some testing.
        }
        return Some(displays[0].clone())
    }
    None
}

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
    get_screen_scale_or(0.0)
}
// Scale above, frequency below.
pub fn get_primary_screen_frequency()->f32{// Might change f32 return to a struct LogicalPixel(relation:f32) or alike.
    let Some(display)=get_primary_screen()else{panic!("[!] PRIMARY SCREEN NOT FOUND")};    
    display.frequency
}

pub fn get_screen_frequency()->f32{
    let Some(display)=get_screen()else{panic!("[!] SCREEN NOT FOUND")};    
    display.frequency
}

pub fn get_primary_screen_frequency_or(fallback_scale:f32)->f32{
    let Some(display)=get_primary_screen()else{return fallback_scale};
    display.scale_factor
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

pub fn get_all_screens_frequencies()->Vec<f32>{
    let Ok(displays)=display_info::DisplayInfo::all()else{return Vec::new()};
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