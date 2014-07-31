
use std::mem::size_of;
use std::rc::Rc;
use std::cell::{RefCell, Cell};  // shared refs to ui-updatable data
use nanovg::{Ctx};
use blendish;
use blendish::*;
use blendish::{ CORNER_NONE, CORNER_ALL, CORNER_DOWN, CORNER_TOP, CORNER_RIGHT, CORNER_LEFT };
use blendish::{ DISABLED_ALPHA, ACTIVE, WIDGET_HEIGHT, };
use blendish::constants::*;
use blendish::themed_draw::ThemedDraw;
use blendish::lowlevel_draw::LowLevelDraw;
use oui;
use oui::*;
use oui::{Item, Context, LEFT};


// FIXME need Some<iconid> apparently (seems to use -1 for no-icon)
fn icon_id(x:u8, y:u8) -> i32 { ICONID(x, y) as i32 }
fn no_icon() -> i32 { -1 }
//struct IconID(i32);
// let iconid = IconID(icon_id(x, y)); let IconID(id) = iconid;
////////////////////////////////////////////////////////////////////////////////

pub enum Widget<'a> {
    Label { iconid:i32, text:String },
    Button { iconid:i32, text:String },
    Check { text:String, option: &'a Cell<bool> },
    Radio { iconid:i32, text:String, value: &'a Cell<i32> },
    Slider { text:String, progress: &'a Cell<f32> },
    Row { unused:i8 },
    Column { unused:i8 },
    Panel { unused:i8 }
}
////////////////////////////////////////////////////////////////////////////////

// calculate which corners are sharp for an item, depending on whether
// the container the item is in has negative spacing, and the item
// is first or last element in a sequence of 2 or more elements.
fn corner_flags(ui: &mut Context<Widget>, item: Item) -> CornerFlags {
    let parent = ui.parent(item);
    if parent.invalid() { return CORNER_NONE };
    let numkids = ui.get_child_count(parent);
    if numkids < 2 { return CORNER_NONE; }
    let kidid = ui.get_child_id(item);
    let widget = ui.get_widget(parent);
    match *widget {
        Column { unused:_ } => {
            // first child, sharp corners down
            if kidid == 0 { return CORNER_DOWN; }
            // last child, sharp corners up
            else if kidid == numkids-1 { return CORNER_TOP; }
            // middle child, sharp everywhere
            else { return CORNER_ALL; }
        }
        Row { unused: _ } => {
            // first child, sharp right
            if kidid == 0 { return CORNER_RIGHT; }
            // last child, sharp left
            else if kidid == numkids-1 { return CORNER_LEFT; }
            // middle child, sharp all
            else { return CORNER_ALL; }
        }
        _ => {}
    };
    return CORNER_NONE;
}

////oui::ItemState
//    COLD   = ffi::UI_COLD,
//    HOT    = ffi::UI_HOT,
//    ACTIVE = ffi::UI_ACTIVE,
//    FROZEN = ffi::UI_FROZEN,
////blendish::WidgetState
//    DEFAULT  = ffi::BND_DEFAULT,
//    HOVER    = ffi::BND_HOVER,
//    ACTIVE   = ffi::BND_ACTIVE,


// draw item and recurse for its children
fn draw_ui(ui: &mut Context<Widget>, vg: &mut ThemedContext, item: Item, x: i32, y: i32) {
    let (x,y,w,h) = {
        let rect = ui.get_rect(item);
        ((rect.x + x) as f32, (rect.y + y) as f32, rect.w as f32, rect.h as f32)
    };
    let item_state = ui.get_state(item);

    // OUI extends state, adding a "frozen" which gets dimmed
    let widget_state = match item_state {
        COLD => DEFAULT,
        HOT => HOVER,
        oui::ACTIVE => blendish::ACTIVE,
        _ => DEFAULT
    };

    let frozen = item_state == FROZEN;
    if frozen {
        vg.nvg().global_alpha(DISABLED_ALPHA);
    }

    let kidid = ui.get_child_id(item);

    let cornerflags = corner_flags(ui, item);

    match *ui.get_widget(item) {
        Panel { unused:_ } => {
            // TODO move draw_bevel from lowlevel_draw to themed,
            // using the theme bg
            let bg = vg.theme().backgroundColor;
            vg.nvg().draw_bevel(x, y, w, h, bg);
        }
        Label { iconid:iconid, text:ref label } => {
            vg.draw_label(x, y, w, h, iconid as u32, label.as_slice());
        }
        Button { iconid:iconid, text:ref label } => {
            vg.draw_tool_button(x, y, w, h,
                cornerflags, widget_state,
                iconid as u32, label.as_slice());
        }
        Check { text:ref label, option:option } => {
            let state =
                if option.get() { blendish::ACTIVE }
                else { widget_state };
            vg.draw_option_button(x, y, w, h, state, label.as_slice());
        }
        Radio { iconid:iconid, text:ref label, value:value } => {
            let state =
                if value.get() == kidid { blendish::ACTIVE }
                else { widget_state };
            vg.draw_radio_button(x, y, w, h,
                cornerflags, state,
                iconid as u32, label.as_slice());
        }
        Slider { text:ref label, progress:progress } => {
            let val = progress.get();
            let val_str = format!("{}", val*100.0);
            vg.draw_slider(x, y, w, h,
                cornerflags, widget_state,
                val, label.as_slice(), val_str.as_slice());
        }
        _ => {
            //testrect(vg, rect);
        }
    }

    let mut kid = ui.first_child(item);
    while kid.valid() { // was, > 0 meaning valid and not root ?
        draw_ui(ui, vg, kid, x as i32, y as i32);
        kid = ui.next_sibling(kid);
    }

    if frozen {
        vg.nvg().global_alpha(1.0);  // was frozen, restore full alpha
    }
}

fn label(ui:&mut Context<Widget>, parent: Item, iconid: i32, label: &str) -> Item {
    let lbl = Label { iconid:iconid, text:label.to_string() };
    let item = ui.item(lbl);
    ui.set_size(item, 0, WIDGET_HEIGHT);
    ui.append(parent, item);
    return item;
}

fn button(ui:&mut Context<Widget>, parent: Item, handle: Handle, iconid: i32, label: &str, handler: Handler<Widget>) -> Item {
    // create new ui item
    // (store some custom data with the button that we use for styling)
    let btn = Button { iconid:iconid, text:label.to_string() };
    let item = ui.item(btn);
    // set persistent handle for item that is used
    // to track activity over time
    ui.set_handle(item, handle);
    // set size of wiget; horizontal size is dynamic, vertical is fixed
    ui.set_size(item, 0, WIDGET_HEIGHT);
    // attach event handler e.g. demohandler above
    //ui.set_handler(item, handler, BUTTON0_HOT_UP);

    ui.append(parent, item);
    return item;
}

fn check<'a>(ui:&mut Context<Widget<'a>>, parent: Item, handle: Handle, label: &str, option: &'a Cell<bool>) -> Item {
    // create new ui item
    let chk = Check { text:label.to_string(), option:option };
    let item = ui.item(chk);
    // set persistent handle for item that is used
    // to track activity over time
    ui.set_handle(item, handle);
    // set size of wiget; horizontal size is dynamic, vertical is fixed
    ui.set_size(item, 0, WIDGET_HEIGHT);
    // attach event handler e.g. demohandler above
    //ui.set_handler(item, Some(checkhandler), BUTTON0_DOWN);
    ui.append(parent, item);
    return item;
}

fn slider<'a>(ui:&mut Context<Widget<'a>>, parent: Item, handle: Handle, label: &str, progress: &'a Cell<f32>) -> Item {
    // create new ui item
    let sli = Slider { text:label.to_string(), progress:progress };
    let item = ui.item(sli);
    // set persistent handle for item that is used
    // to track activity over time
    ui.set_handle(item, handle);
    // set size of wiget; horizontal size is dynamic, vertical is fixed
    ui.set_size(item, 0, WIDGET_HEIGHT);
    // attach our slider event handler and capture two classes of events
    //ui.set_handler(item, Some(sliderhandler), BUTTON0_DOWN | BUTTON0_CAPTURE);
    ui.append(parent, item);
    return item;
}

fn radio<'a>(ui:&mut Context<Widget<'a>>, parent: Item, handle: Handle, iconid: i32, label: &str, value: &'a Cell<i32>) -> Item {
    let rad = Radio { iconid:iconid, text:label.to_string(), value:value };
    let item = ui.item(rad);
    ui.set_handle(item, handle);
    let w = if label.len() == 0 { TOOL_WIDTH } else { 0 };
    ui.set_size(item, w, WIDGET_HEIGHT);
    //ui.set_handler(item, Some(radiohandler), BUTTON0_DOWN);
    ui.append(parent, item);
    return item;
}

fn panel(ui:&mut Context<Widget>) -> Item {
    ui.item(Panel{unused:0})
}

fn column(ui:&mut Context<Widget>, parent: Item) -> Item {
    let item = ui.item(Column{unused:0});
    ui.set_handler(item, Some(columnhandler), APPEND);
    ui.append(parent, item);
    return item;
}

fn row(ui: &mut Context<Widget>, parent: Item) -> Item {
    let item = ui.item(Row{unused:0});
    ui.set_handler(item, Some(rowhandler), APPEND);
    ui.append(parent, item);
    return item;
}

fn vgroup(ui:&mut Context<Widget>, parent: Item) -> Item {
    let item = ui.item(Column{unused:0});
    ui.set_handler(item, Some(vgrouphandler), APPEND);
    ui.append(parent, item);
    return item;
}

fn hgroup(ui:&mut Context<Widget>, parent: Item) -> Item {
    let item = ui.item(Row{unused:0});
    ui.set_handler(item, Some(hgrouphandler), APPEND);
    ui.append(parent, item);
    return item;
}


///////////////////////////////////////////////////////////////////////
// handlers

fn demohandler(ui: &mut Context<Widget>, item: Item, event: EventFlags) {
    let handle = ui.get_handle(item);
    let widget = ui.get_widget(item);
    match *widget {
        Button { text: ref mut label, iconid:_ } => {
            println!("clicked: {} {}", handle, label);
        }
        _ => {}
    }
}
fn checkhandler(ui: &mut Context<Widget>, item: Item, event: EventFlags) {
    let handle = ui.get_handle(item);
    let widget = ui.get_widget(item);
    match *widget {
        Check { text: ref mut label, option: option } => {
            println!("clicked: {} {}", handle, label);
            option.set(!option.get());
        }
        _ => {}
    }
}
// simple logic for a radio button
fn radiohandler(ui: &mut Context<Widget>, item: Item, event: EventFlags) {
    let handle = ui.get_handle(item);
    let kidid = ui.get_child_id(item);
    let widget = ui.get_widget(item);
    match *widget {
        Radio { iconid:_, text: ref mut label, value: value } => {
            println!("clicked: {} {}", handle, label);
            value.set(kidid);
        }
        _ => {}
    }
}

// simple logic for a slider
// starting offset of the currently active slider
//static sliderstart: f32 = 0.0;

// event handler for slider (same handler for all sliders)
fn sliderhandler(ui: &mut Context<Widget>, item: Item, event: EventFlags) {
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

fn columnhandler(ui: &mut Context<Widget>, parent: Item, event: EventFlags) {
    let item = ui.last_child(parent);
    let last = ui.prev_sibling(item);
    // mark the new item as positioned under the previous item
    ui.set_rel_to_top(item, last);
    // fill parent horizontally, anchor to previous item vertically
    ui.set_layout(item, HFILL|TOP);
    // if not the first item, add a margin of 1
    let gap = if last.invalid() { 0 } else { 1 };
    ui.set_margins(item, 0,gap,0,0);
}
fn rowhandler(ui: &mut Context<Widget>, parent: Item, event: EventFlags) {
    let item = ui.last_child(parent);
    let last = ui.prev_sibling(item);
    ui.set_rel_to_left(item, last);
    if last.valid() {
        ui.set_rel_to_right(last, item);
    }
    ui.set_layout(item, LEFT|RIGHT);
    let gap = if last.invalid() { 0 } else { 8 };
    ui.set_margins(item, gap,0,0,0);
}
fn vgrouphandler(ui: &mut Context<Widget>, parent: Item, event: EventFlags) {
    let item = ui.last_child(parent);
    let last = ui.prev_sibling(item);
    // mark the new item as positioned under the previous item
    ui.set_rel_to_top(item, last);
    // fill parent horizontally, anchor to previous item vertically
    ui.set_layout(item, HFILL|TOP);
    // if not the first item, add a margin
    let gap = if last.invalid() { 0 } else { -2 };
    ui.set_margins(item, 0,gap,0,0);
}
fn hgrouphandler(ui: &mut Context<Widget>, parent: Item, event: EventFlags) {
    let item = ui.last_child(parent);
    let last = ui.prev_sibling(item);
    ui.set_rel_to_left(item, last);
    if last.valid() {
        ui.set_rel_to_right(last, item);
    }
    ui.set_layout(item, LEFT|RIGHT);
    let gap = if last.invalid() { 0 } else { -1 };
    ui.set_margins(item, gap,0,0,0);
}
// handlers
///////////////////////////////////////////////////////////////////////

pub fn draw(ctx: &mut ThemedContext, _w:f32, _h:f32, _t: f32)
{
    let enum1     = Cell::new(1i32);
    let progress1 = Cell::new(0.25f32);
    let progress2 = Cell::new(0.75f32);
    let option1   = Cell::new(true);
    let option2   = Cell::new(false);
    let option3   = Cell::new(false);

    let mut oui = Context::create_context();
    let ui = &mut oui;

    ui.clear();

    let root = panel(ui);
    // position root element
    ui.set_layout(root, LEFT|TOP);
    ui.set_margins(root, 60, 10, 0, 0);
    ui.set_size(root, 450, 400);

    let col = column(ui, root);
    ui.set_margins(col, 10, 10, 10, 10);
    ui.set_layout(col, TOP|HFILL);

    button(ui, col, 1, icon_id(6, 3), "Item 1", Some(demohandler));
    button(ui, col, 2, icon_id(6, 3), "Item 2", Some(demohandler));

    {
        let h = hgroup(ui, col);
        radio(ui, h, 3, icon_id(6,  3), "Item 3.0", &enum1);
        radio(ui, h, 4, icon_id(0, 10), "", &enum1);
        radio(ui, h, 5, icon_id(1, 10), "", &enum1);
        radio(ui, h, 6, icon_id(6,  3), "Item 3.3", &enum1);
    }

    {
        let rows = row(ui, col);
        let coll = vgroup(ui, rows);
        label(ui, coll, no_icon(), "Items 4.0:");
        let coll = vgroup(ui, coll);
        button(ui, coll, 7, icon_id(6, 3), "Item 4.0.0", Some(demohandler));
        button(ui, coll, 8, icon_id(6, 3), "Item 4.0.1", Some(demohandler));
        let colr = vgroup(ui, rows);
        ui.set_frozen(colr, option1.get());
        label(ui, colr, no_icon(), "Items 4.1:");
        let colr = vgroup(ui, colr);
        slider(ui, colr,  9, "Item 4.1.0", &progress1);
        slider(ui, colr, 10, "Item 4.1.1", &progress2);
    }

    button(ui, col, 11, icon_id(6, 3), "Item 5", None);

    check(ui, col, 12, "Frozen", &option1);
    check(ui, col, 13, "Item 7", &option2);
    check(ui, col, 14, "Item 8", &option3);

    ui.layout();
    draw_ui(ui, ctx, root, 0, 0);
    ui.process();
}
