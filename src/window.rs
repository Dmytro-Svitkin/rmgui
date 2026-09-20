//static WINREG:Vec<Option<Box<WindowData>>>=Vec::from(None);
use crate::window;

const HALF:u32=i32::MAX as u32;

#[derive(Debug,Default,Clone,Copy,PartialEq,Eq)]
pub struct Window{id:usize}

pub struct WindowData{
    running:bool,

    winit_window:Option<u32>,
    winit_id:u32,

    wgpu_renderer:Option<u32>,
    internal_elements:Vec<u32>,

    title:String,
    icon:Icon,

    size:WindowSize,
    position:WindowPosition,

    decoration:WindowDecoration,
    fullscreen:WindowDisplayMode,
    visibility:WindowVisibility,

    //call:WindowCall
}

struct Icon{
    path:String,
    bytes:Vec<u8>
}

struct WindowSize{
    size:Size<u32>,
    min:Size<u32>,
    max:Size<u32>,
    unity:SizeUnity,
    resizable:[bool;2]
}

struct Size<T>{
    width:T,height:T,
}

enum SizeUnity{
    Pixel,LogicalPixel,
    PixelGrid(Size<u8>),LogicalPixelGrid(Size<u8>),
    ScreenWidth,ScreenHeight,
}

struct WindowPosition{
    x:i32,y:i32,
}
/*
struct Position<T>{
    x:T,y:T
}
*/
struct WindowDecoration{
    taskbar_style:WindowTaskbarStyle,taskbar_theme:WindowTaskbarTheme,
    minimize_button:bool,maximize_button:bool,close_button:bool
}

enum WindowTaskbarStyle{
    None,

    Native,PseudoNative,

    Lite,ASCII,Scratch,

    Windows1,Windows3,Windows95,WindowsXP,Windows7,Windows8,Windows10,Windows11,
    
    Mac1,Mac8,MacCheetah,MacPanther,MacLeopard,MacYosemite,MacBigSur,

    CDE,
    
    Xerox
}

enum WindowTaskbarTheme{
    System,Light,Dark
}

enum WindowDisplayMode{
    Fullscreen(FullscreenMode),
    Windowed,
    Maximized
}

enum FullscreenMode{
    Normal,Video
}

struct WindowVisibility{
    visible:bool,
    visible_to_taskbar:bool,
    visible_to_taskbar_paernt:bool,
    visible_to_screenshare:bool,
    opaque:bool
}

struct WindowCall{
    on_close:String,on_minimize:String,on_maximize:String,
}

impl WindowData{
    fn default()->Self{
        Self{
            running:false,

            winit_window:None,winit_id:1,
            
            wgpu_renderer:None,
            internal_elements:Vec::new(),
            
            title:String::from("Application"),
            icon:Icon{path:String::new(),bytes:Vec::new()},
            
            size:WindowSize{
                size:Size{width:800,height:600},
                min:Size{width:0,height:0},
                max:Size{width:HALF,height:HALF},
                unity:SizeUnity::LogicalPixel,
                resizable:[true,true]},
            
            position:WindowPosition{x:0,y:0},
            
            decoration:WindowDecoration{
                taskbar_style:WindowTaskbarStyle::Native,taskbar_theme:WindowTaskbarTheme::System,
                minimize_button:true,maximize_button:true,close_button:true},
            
            fullscreen:WindowDisplayMode::Windowed,
            visibility:WindowVisibility{visible:true,visible_to_taskbar:true,visible_to_taskbar_paernt:true,visible_to_screenshare:true,opaque:true}
        }
    }
}