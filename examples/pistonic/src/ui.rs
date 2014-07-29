
use std::mem::transmute;
use std::mem::size_of;
use nanovg::{Ctx};
use blendish::*;
use blendish::{ CORNER_NONE,CORNER_ALL,CORNER_DOWN,CORNER_TOP,CORNER_RIGHT,CORNER_LEFT };
use blendish::{ DISABLED_ALPHA, ACTIVE, WIDGET_HEIGHT, };
use blendish::constants::*;
use blendish::themed_draw::ThemedDraw;
use blendish::lowlevel_draw::LowLevelDraw;
use oui::*;
use oui::{LEFT};


// FIXME need Some<iconid> apparently (seems to use -1 for no-icon)
fn icon_id(x:u8, y:u8) -> i32 { ICONID(x,y) as i32 }
fn no_icon() -> i32 { -1 }

////////////////////////////////////////////////////////////////////////////////

enum SubType {
    ST_LABEL = 0,
    ST_BUTTON = 1,
    ST_RADIO = 2,
    ST_SLIDER = 3,
    ST_COLUMN = 4,
    ST_ROW = 5,
    ST_CHECK = 6,
    ST_PANEL = 7,
}

struct UIData { subtype: SubType }
struct UIButtonData<'a> { head: UIData, iconid: i32, label: &'a str, }
struct UICheckData<'a>  { head: UIData, label: &'a str, option: &'a mut bool, }
struct UIRadioData<'a>  { head: UIData, iconid: i32, label: &'a str, value: &'a mut i32, }
struct UISliderData<'a> { head: UIData, label: &'a str, progress: &'a mut f32, }

////////////////////////////////////////////////////////////////////////////////

//void init(vg: nanovg::Ctx) {
//    bndSetFont(nvgCreateFont(vg, "system", "../DejaVuSans.ttf"));
//    bndSetIconImage(nvgCreateImage(vg, "../blender_icons16.png"));
//}

// calculate which corners are sharp for an item, depending on whether
// the container the item is in has negative spacing, and the item
// is first or last element in a sequence of 2 or more elements.
fn corner_flags(ui: &mut Context, item: Item) -> CornerFlags {
    let parent = ui.parent(item);
    let numkids = ui.get_child_count(parent);
    if numkids < 2 {return CORNER_NONE;}
    let head: UIData = unsafe { transmute(*(ui.get_data(parent).as_mut_ptr())) };
//    if (head) {
        let numid = ui.get_child_id(item);
        match head.subtype {
            ST_COLUMN => {
                // first child, sharp corners down
                if numid == 0 { return CORNER_DOWN; }
                // last child, sharp corners up
                else if numid == numkids-1 { return CORNER_TOP; }
                // middle child, sharp everywhere
                else { return CORNER_ALL; }
            },
            ST_ROW => {
                // first child, sharp right
                if numid == 0 { return CORNER_RIGHT; }
                // last child, sharp left
                else if numid == numkids-1 { return CORNER_LEFT; }
                // middle child, sharp all
                else { return CORNER_ALL; }
            },
            _ => {}
        }
//    }
    return CORNER_NONE;
}

fn draw_ui(vg: &mut Ctx, item: Item, x: i32, y: i32) {
//    let head = (const UIData *)ui.get_data(item);
//    let rect = ui.get_rect(item);
//    rect.x += x;
//    rect.y += y;
//    if (ui.get_state(item) == FROZEN) {
//        nvgGlobalAlpha(vg, DISABLED_ALPHA);
//    }
//    if (head) {
//        switch(head.subtype) {
//            default: {
//                testrect(vg,rect);
//            } break;
//            case ST_PANEL: {
//                bndBevel(vg,rect.x,rect.y,rect.w,rect.h);
//            } break;
//            case ST_LABEL: {
//                assert(head);
//                const UIButtonData *data = (UIButtonData*)head;
//                bndLabel(vg,rect.x,rect.y,rect.w,rect.h,
//                    data.iconid,data.label);
//            } break;
//            case ST_BUTTON: {
//                const UIButtonData *data = (UIButtonData*)head;
//                bndToolButton(vg,rect.x,rect.y,rect.w,rect.h,
//                    corner_flags(item),(BNDwidgetState)ui.get_state(item),
//                    data.iconid,data.label);
//            } break;
//            case ST_CHECK: {
//                const UICheckData *data = (UICheckData*)head;
//                BNDwidgetState state = (BNDwidgetState)ui.get_state(item);
//                if (*data.option)
//                    state = ACTIVE;
//                bndOptionButton(vg,rect.x,rect.y,rect.w,rect.h, state,
//                    data.label);
//            } break;
//            case ST_RADIO:{
//                const UIRadioData *data = (UIRadioData*)head;
//                BNDwidgetState state = (BNDwidgetState)ui.get_state(item);
//                if (*data.value == ui.get_child_id(item))
//                    state = ACTIVE;
//                bndRadioButton(vg,rect.x,rect.y,rect.w,rect.h,
//                    corner_flags(item),state,
//                    data.iconid,data.label);
//            } break;
//            case ST_SLIDER:{
//                const UISliderData *data = (UISliderData*)head;
//                BNDwidgetState state = (BNDwidgetState)ui.get_state(item);
//                static char value[32];
//                spri32f(value,"%.0f%%",(*data.progress)*100.0);
//                bndSlider(vg,rect.x,rect.y,rect.w,rect.h,
//                    corner_flags(item),state,
//                    *data.progress,data.label,value);
//            } break;
//        }
//    } else {
//        testrect(vg,rect);
//    }
//
//    i32 kid = ui.first_child(item);
//    while (kid > 0) {
//        draw_ui(vg, kid, rect.x, rect.y);
//        kid = ui.next_sibling(kid);
//    }
//    if (ui.get_state(item) == FROZEN) {
//        nvgGlobalAlpha(vg, 1.0);
//    }
}

fn label(ui:&mut Context, parent: Item, iconid: i32, label: &str) -> Item {
    let item = ui.item();
    ui.set_size(item, 0, WIDGET_HEIGHT);
//    {
//        //UIButtonData *data = (UIButtonData *)ui.alloc_data(item, sizeof(UIButtonData));
//        let data = ui.alloc_data(item, size_of::<UIButtonData>());
//        let mut data: UIButtonData = unsafe { transmute(data) };
//        data.head.subtype = ST_LABEL;
//        data.iconid = iconid;
//        data.label = label;
//    }
    ui.append(parent, item);
    return item;
}

fn demohandler(item: Item, event: EventFlags) {
//    let data = (const UIButtonData *)ui.get_data(item);
//    pri32f("clicked: %lld %s\n", ui.get_handle(item), data.label);
}

fn button(ui:&mut Context, parent: Item, handle: Handle, iconid: i32, label: &str, handler: Handler) -> Item {
    // create new ui item
    let item = ui.item();
    // set persistent handle for item that is used
    // to track activity over time
    ui.set_handle(item, handle);
    // set size of wiget; horizontal size is dynamic, vertical is fixed
    ui.set_size(item, 0, WIDGET_HEIGHT);
    // attach event handler e.g. demohandler above
    ui.set_handler(item, handler, BUTTON0_HOT_UP);
    // store some custom data with the button that we use for styling
//    {
//        //UIButtonData *data = (UIButtonData *)ui.alloc_data(item, sizeof(UIButtonData));
//        let data = ui.alloc_data(item, size_of::<UIButtonData>());
//        let mut data: UIButtonData = unsafe { transmute(data) };
//        data.head.subtype = ST_BUTTON;
//        data.iconid = iconid;
//        data.label = label;
//    }
    ui.append(parent, item);
    return item;
}

fn checkhandler(item: Item, event: EventFlags) {
//    let data = (const UICheckData *)ui.get_data(item);
//    *data.option = !(*data.option);
}

fn check(ui:&mut Context, parent: Item, handle: Handle, label: &str, option: &mut bool) -> Item {
    // create new ui item
    let item = ui.item();
    // set persistent handle for item that is used
    // to track activity over time
    ui.set_handle(item, handle);
    // set size of wiget; horizontal size is dynamic, vertical is fixed
    ui.set_size(item, 0, WIDGET_HEIGHT);
    // attach event handler e.g. demohandler above
    ui.set_handler(item, Some(checkhandler), BUTTON0_DOWN);
    // store some custom data with the button that we use for styling
//    {
//    //UICheckData *data = (UICheckData *)ui.alloc_data(item, sizeof(UICheckData));
//        let data = ui.alloc_data(item, size_of::<UICheckData>());
//        let mut data: UICheckData = unsafe { transmute(data) };
//        data.head.subtype = ST_CHECK;
//        data.label = label;
//        data.option = option;
//    }
    ui.append(parent, item);
    return item;
}

// simple logic for a slider

// starting offset of the currently active slider
//static sliderstart: f32 = 0.0;

// event handler for slider (same handler for all sliders)
fn sliderhandler(item: Item, event: EventFlags) {
//    // retrieve the custom data we saved with the slider
//    let data = (UISliderData *)ui.get_data(item);
//    switch(event) {
//        default: break;
//        case BUTTON0_DOWN: {
//            // button was pressed for the first time; capture initial
//            // slider value.
//            sliderstart = *data.progress;
//        } break;
//        case BUTTON0_CAPTURE: {
//            // called for every frame that the button is pressed.
//            // get the delta between the click point and the current
//            // mouse position
//            UIvec2 pos = ui.get_cursor_start_delta();
//            // get the items layouted rectangle
//            UIrect rc = ui.get_rect(item);
//            // calculate our new offset and clamp
//            let value = sliderstart + ((float)pos.x / (float)rc.w);
//            value = (value<0)?0:(value>1)?1:value;
//            // assign the new value
//            *data.progress = value;
//        } break;
//    }
}

fn slider(ui:&mut Context, parent: Item, handle: Handle, label: &str, progress: &mut f32) -> Item {
    // create new ui item
    let item = ui.item();
    // set persistent handle for item that is used
    // to track activity over time
    ui.set_handle(item, handle);
    // set size of wiget; horizontal size is dynamic, vertical is fixed
    ui.set_size(item, 0, WIDGET_HEIGHT);
    // attach our slider event handler and capture two classes of events
    ui.set_handler(item, Some(sliderhandler),
        BUTTON0_DOWN | BUTTON0_CAPTURE);
    // store some custom data with the button that we use for styling
    // and logic, e.g. the pointer to the data we want to alter.
//    {
//        //UISliderData *data = (UISliderData *)ui.alloc_data(item, sizeof(UISliderData));
//        let data = ui.alloc_data(item, size_of::<UISliderData>());
//        let mut data: UISliderData = unsafe { transmute(data) };
//        data.head.subtype = ST_SLIDER;
//        data.label = label;
//        data.progress = progress;
//    }
    ui.append(parent, item);
    return item;
}

// simple logic for a radio button
fn radiohandler(item: Item, event: EventFlags) {
//    UIRadioData *data = (UIRadioData *)ui.get_data(item);
//    *data.value = ui.get_child_id(item);
}

fn radio(ui:&mut Context, parent: Item, handle: Handle, iconid: i32, label: &str, value: &mut i32) -> Item {
    let item = ui.item();
    ui.set_handle(item, handle);
    let w = if label.len() > 0 { TOOL_WIDTH } else { 0 };
    ui.set_size(item, w, WIDGET_HEIGHT);
//    {
//        //UIRadioData *data = (UIRadioData *)ui.alloc_data(item, sizeof(UIRadioData));
//        let data = ui.alloc_data(item, size_of::<UIRadioData>());
//        let mut data: UIRadioData = unsafe { transmute(data) };
//        data.head.subtype = ST_RADIO;
//        data.iconid = iconid;
//        data.label = label;
//        data.value = value;
//    }
    ui.set_handler(item, Some(radiohandler), BUTTON0_DOWN);
    ui.append(parent, item);
    return item;
}


fn panel(ui:&mut Context) -> Item {
    let item = ui.item();
    let data = ui.alloc_data(item, size_of::<UIData>());
        let mut data: UIData = unsafe { transmute(*data.as_mut_ptr()) };
    data.subtype = ST_PANEL;
    return item;
}

fn columnhandler(parent: Item, event: EventFlags) {
//    let item = ui.last_child(parent);
//    let last = ui.prev_sibling(item);
//    // mark the new item as positioned under the previous item
//    ui.set_rel_to_top(item, last);
//    // fill parent horizontally, anchor to previous item vertically
//    ui.set_layout(item, HFILL|TOP);
//    // if not the first item, add a margin of 1
//    ui.set_margins(item, 0, (last < 0)?0:1, 0, 0);
}

fn column(ui:&mut Context, parent: Item) -> Item {
    let item = ui.item();
    ui.set_handler(item, Some(columnhandler), APPEND);
    ui.append(parent, item);
    return item;
}

fn rowhandler(parent: Item, event: EventFlags) {
//    let item = ui.last_child(parent);
//    let last = ui.prev_sibling(item);
//    ui.set_rel_to_left(item, last);
//    if (last > 0)
//        ui.set_rel_to_right(last, item);
//    ui.set_layout(item, LEFT|RIGHT);
//    ui.set_margins(item, (last < 0)?0:8, 0, 0, 0);
}

fn row(ui: &mut Context, parent: Item) -> Item {
    let item = ui.item();
    ui.set_handler(item, Some(rowhandler), APPEND);
    ui.append(parent, item);
    return item;
}

fn vgrouphandler(parent: Item, event: EventFlags) {
//    let item = ui.last_child(parent);
//    let last = ui.prev_sibling(item);
//    // mark the new item as positioned under the previous item
//    ui.set_rel_to_top(item, last);
//    // fill parent horizontally, anchor to previous item vertically
//    ui.set_layout(item, HFILL|TOP);
//    // if not the first item, add a margin
//    ui.set_margins(item, 0, (last < 0)?0:-2, 0, 0);
}

fn vgroup(ui:&mut Context, parent: Item) -> Item {
    let item = ui.item();
    {
        let data = ui.alloc_data(item, size_of::<UIData>());
        let mut data: UIData = unsafe { transmute(*data.as_mut_ptr()) };
        data.subtype = ST_COLUMN;
    }
    ui.set_handler(item, Some(vgrouphandler), APPEND);
    ui.append(parent, item);
    return item;
}

fn hgrouphandler(parent: Item, event: EventFlags) {
//    let item = ui.last_child(parent);
//    let last = ui.prev_sibling(item);
//    ui.set_rel_to_left(item, last);
//    if (last > 0)
//        ui.set_rel_to_right(last, item);
//    ui.set_layout(item, LEFT|RIGHT);
//    ui.set_margins(item, (last < 0)?0:-1, 0, 0, 0);
}

fn hgroup(ui:&mut Context, parent: Item) -> Item {
    let item = ui.item();
    {
        let mut data = ui.alloc_data(item, size_of::<UIData>());
        let mut data: UIData = unsafe { transmute(*data.as_mut_ptr()) };
        data.subtype = ST_ROW;
    }
    ui.set_handler(item, Some(hgrouphandler), APPEND);
    ui.append(parent, item);
    return item;
}


pub fn draw(ui: &mut Context, ctx: &mut ThemedContext, w:f32, h:f32, t: f32)
{
    // some OUI stuff

    // some persistent variables for demonstration
    static mut enum1: i32 = 0;
    static mut progress1:f32 = 0.25;
    static mut progress2:f32 = 0.75;
    static mut option1: bool = true;
    static mut option2: bool = false;
    static mut option3: bool = false;

    ui.clear();

    let root = panel(ui);
    // position root element
    ui.set_layout(root, LEFT|TOP);
    ui.set_margins(root, 600, 10, 0, 0);
    ui.set_size(root, 250, 400);

    let col = column(ui, root);
    ui.set_margins(col, 10, 10, 10, 10);
    ui.set_layout(col, TOP|HFILL);

    button(ui, col, 1, icon_id(6, 3), "Item 1", Some(demohandler));
    button(ui, col, 2, icon_id(6, 3), "Item 2", Some(demohandler));

    {
        let h = hgroup(ui, col);
        unsafe { // mutating static vars
            radio(ui, h, 3, icon_id(6, 3), "Item 3.0", &mut enum1);
            radio(ui, h, 4, icon_id(0, 10), "", &mut enum1);
            radio(ui, h, 5, icon_id(1, 10), "", &mut enum1);
            radio(ui, h, 6, icon_id(6, 3), "Item 3.3", &mut enum1);
        }
    }

    {
        let rows = row(ui, col);
        let coll = vgroup(ui, rows);
        label(ui, coll, no_icon(), "Items 4.0:");
        let coll = vgroup(ui, coll);
        button(ui, coll, 7, icon_id(6, 3), "Item 4.0.0", Some(demohandler));
        button(ui, coll, 8, icon_id(6, 3), "Item 4.0.1", Some(demohandler));
        let colr = vgroup(ui, rows);
        unsafe { // mutating static vars
            ui.set_frozen(colr, option1);
        }
        label(ui, colr, no_icon(), "Items 4.1:");
        let colr = vgroup(ui, colr);
        unsafe { // mutating static vars
            slider(ui, colr, 9, "Item 4.1.0", &mut progress1);
            slider(ui, colr, 10, "Item 4.1.1", &mut progress2);
        }
    }

    button(ui, col, 11, icon_id(6, 3), "Item 5", None);

    unsafe { // mutating static vars
        check(ui, col, 12, "Frozen", &mut option1);
        check(ui, col, 13, "Item 7", &mut option2);
        check(ui, col, 14, "Item 8", &mut option3);
    }
    ui.layout();
    draw_ui(ctx.nvg(), root, 0, 0);
    ui.process();
}
