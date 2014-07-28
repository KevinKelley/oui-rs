#![feature(globs)]
#![allow(non_snake_case_functions)]  // temporarily
#![allow(non_camel_case_types)]  // temporarily
#![allow(dead_code)]  // temporarily
#![allow(unused_variable)]

extern crate libc;

pub type Handle = u64;

mod ffi;
mod oui;

// maximum number of items that may be added
pub static MAX_ITEMS:u32 = ffi::UI_MAX_ITEMS;
// maximum size in bytes reserved for storage of application dependent data
// as passed to uiAllocData().
pub static MAX_BUFFERSIZE:u32 = ffi::UI_MAX_BUFFERSIZE;
// maximum size in bytes of a single data buffer passed to uiAllocData().
pub static MAX_DATASIZE:u32 = ffi::UI_MAX_DATASIZE;
// maximum depth of nested containers
pub static MAX_DEPTH:u32 = ffi::UI_MAX_DEPTH;




#[repr(u32)]
pub enum ItemState {
    // the item is inactive
    COLD   = ffi::UI_COLD,
    // the item is inactive, but the cursor is hovering over this item
    HOT    = ffi::UI_HOT,
    // the item is toggled or activated (depends on item kind)
    ACTIVE = ffi::UI_ACTIVE,
    // the item is unresponsive
    FROZEN = ffi::UI_FROZEN,
}

bitflags!(
    flags LayoutFlags: u32 {
        // anchor to left item or left side of parent
        static LEFT    = ffi::UI_LEFT,
        // anchor to top item or top side of parent
        static TOP     = ffi::UI_TOP,
        // anchor to right item or right side of parent
        static RIGHT   = ffi::UI_RIGHT,
        // anchor to bottom item or bottom side of parent
        static DOWN    = ffi::UI_DOWN,
        // anchor to both left and right item or parent borders
        static HFILL   = ffi::UI_HFILL,
        // anchor to both top and bottom item or parent borders
        static VFILL   = ffi::UI_VFILL,
        // center horizontally, with left margin as offset
        static HCENTER = ffi::UI_HCENTER,
        // center vertically, with top margin as offset
        static VCENTER = ffi::UI_VCENTER,
        // center in both directions, with left/top margin as offset
        static CENTER  = ffi::UI_CENTER,
        // anchor to all four directions
        static FILL    = ffi::UI_FILL
    }
)
impl LayoutFlags {
    //pub fn from_bits(bits: u32) -> LayoutFlags { LayoutFlags { bits: bits } }
}


bitflags!(
    flags EventFlags: u32 {
        // on button 0 down
        static BUTTON0_DOWN     = ffi::UI_BUTTON0_DOWN,
        // on button 0 up
        // when this event has a handler, uiGetState() will return UI_ACTIVE as
        // long as button 0 is down.
        static BUTTON0_UP       = ffi::UI_BUTTON0_UP,
        // on button 0 up while item is hovered
        // when this event has a handler, uiGetState() will return UI_ACTIVE
        // when the cursor is hovering the items rectangle; this is the
        // behavior expected for buttons.
        static BUTTON0_HOT_UP   = ffi::UI_BUTTON0_HOT_UP,
        // item is being captured (button 0 constantly pressed);
        // when this event has a handler, uiGetState() will return UI_ACTIVE as
        // long as button 0 is down.
        static BUTTON0_CAPTURE  = ffi::UI_BUTTON0_CAPTURE,
        // item has received a new child
        // this can be used to allow container items to configure child items
        // as they appear.
        static APPEND           =  ffi::UI_APPEND
    }
)
pub type Handler = Option<extern "C" fn(arg1: i32, arg2: EventFlags)>;

#[repr(C)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}
impl Vec2 {
    pub fn zero() -> Vec2 { Vec2 { x: 0, y: 0 } }
    pub fn as_mut_slice(&mut self) -> &mut [i32, ..2u] { unsafe { std::mem::transmute(self) } }
}
impl<'a> Index<uint, i32> for Vec2 {
    fn index<'a>(&'a self, index: &uint) -> &'a i32 {
        match *index {
            0u => { &self.x },
            1u => { &self.y },
            _  => { fail!("bad index: {}!", *index) }
        }
    }
}
impl<'a> IndexMut<uint, i32> for Vec2 {
    fn index_mut<'a>(&'a mut self, index: &uint) -> &'a mut i32 {
        match *index {
            0u => { &mut self.x },
            1u => { &mut self.y },
            _  => { fail!("bad index: {}!", *index) }
        }
    }
}

#[repr(C)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}
impl Rect {
    pub fn zero() -> Rect { Rect { x:0, y:0, w:0, h:0 } }
    pub fn as_mut_slice(&mut self) -> &mut [i32, ..4u] { unsafe { std::mem::transmute(self) } }
}
impl<'a> Index<uint, i32> for Rect {
    fn index<'a>(&'a self, index: &uint) -> &'a i32 {
        match *index {
            0u => { &self.x },
            1u => { &self.y },
            2u => { &self.w },
            3u => { &self.h },
            _  => { fail!("bad index: {}!", *index) }
        }
    }
}
impl<'a> IndexMut<uint, i32> for Rect {
    fn index_mut<'a>(&'a mut self, index: &uint) -> &'a mut i32 {
        match *index {
            0u => { &mut self.x },
            1u => { &mut self.y },
            2u => { &mut self.w },
            3u => { &mut self.h },
            _  => { fail!("bad index: {}!", *index) }
        }
    }
}

//trait Context {
//    //pub fn uiCreateContext<'a>() -> /* *mut*/ &'a mut Context;
//    fn uiMakeCurrent(ctx: &mut Context);
//    fn uiDestroyContext(ctx: /* *mut*/ &mut Context);
//
//    fn uiSetCursor(x: i32, y: i32);
//    fn uiGetCursor() -> Vec2;
//    fn uiGetCursorDelta() -> Vec2;
//    fn uiGetCursorStart() -> Vec2;
//    fn uiGetCursorStartDelta() -> Vec2;
//
//    fn uiSetButton(button: i32, enabled: i32);
//    fn uiGetButton(button: i32) -> i32;
//
//    fn uiClear();
//    fn uiLayout();
//    fn uiProcess();
//
//    fn uiItem() -> i32;
//
//    fn uiSetFrozen      (item: i32, enable: i32);
//    fn uiSetHandle      (item: i32, handle: Handle);
//    fn uiAllocData      (item: i32, size: i32) -> *mut u8;
//    fn uiSetHandler     (item: i32, handler: Handler, flags: i32);
//    fn uiAppend         (item: i32, child: i32) -> i32;
//    fn uiSetSize        (item: i32, w: i32, h: i32);
//    fn uiSetLayout      (item: i32, flags: i32);
//    fn uiSetMargins     (item: i32, l: i32, t: i32, r: i32, b: i32);
//    fn uiSetRelToLeft   (item: i32, other: i32);
//    fn uiSetRelToTop    (item: i32, other: i32);
//    fn uiSetRelToRight  (item: i32, other: i32);
//    fn uiSetRelToDown   (item: i32, other: i32);
//    fn uiFirstChild     (item: i32) -> i32;
//    fn uiLastChild      (item: i32) -> i32;
//    fn uiParent         (item: i32) -> i32;
//    fn uiNextSibling    (item: i32) -> i32;
//    fn uiPrevSibling    (item: i32) -> i32;
//    fn uiGetState       (item: i32) -> ItemState;
//    fn uiGetHandle      (item: i32) -> Handle;
//    fn uiGetData        (item: i32) -> *const u8;
//    fn uiGetHandler     (item: i32) -> Handler;
//    fn uiGetHandlerFlags(item: i32) -> i32;
//    fn uiGetChildCount  (item: i32) -> i32;
//    fn uiGetChildId     (item: i32) -> i32;
//    fn uiGetRect        (item: i32) -> Rect;
//    fn uiGetActiveRect  () -> Rect;
//    fn uiGetWidth       (item: i32) -> i32;
//    fn uiGetHeight      (item: i32) -> i32;
//    fn uiGetLayout      (item: i32) -> i32;
//    fn uiGetMarginLeft  (item: i32) -> i32;
//    fn uiGetMarginTop   (item: i32) -> i32;
//    fn uiGetMarginRight (item: i32) -> i32;
//    fn uiGetMarginDown  (item: i32) -> i32;
//    fn uiGetRelToLeft   (item: i32) -> i32;
//    fn uiGetRelToTop    (item: i32) -> i32;
//    fn uiGetRelToRight  (item: i32) -> i32;
//    fn uiGetRelToDown   (item: i32) -> i32;
//}
