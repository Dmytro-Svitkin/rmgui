use std::u16;

use winit::monitor::MonitorHandle;
use crate::window::{Size};

struct Screen{
    pub(crate)priority_distinct:u16,
    pub(crate)is_primaly:bool,
    pub(crate)is_builtin:bool,

    pub(crate)size:Size<u32>,
    pub(crate)scale:f32,
    pub(crate)refresh_rate_hz:f32,
    pub(crate)rotation:f32
}

impl Screen{
    pub(crate)fn default()->Self{
        Self{priority_distinct:u16::MAX,
            is_primaly:false,
            is_builtin:false,
            size:Size{width:0,height:0},
            scale:1.0,
            refresh_rate_hz:0.0,
            rotation:0.0
        }
    }
}

pub(crate)fn get_all_screens()->Vec<Screen>{
    vec![Screen::default()]
}