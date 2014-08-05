
use std::gc::{Gc};
use std::cell::{Cell};  // shared refs to ui-updatable data

use blendish;
use blendish::*;
use blendish::ThemedContext;
use blendish::themed_draw::ThemedDraw;
use blendish::lowlevel_draw::LowLevelDraw;

use oui;
use oui::*;


// FIXME need Some<iconid> apparently (seems to use -1 for no-icon)
fn icon_id(x:u8, y:u8) -> i32 { ICONID(x, y) as i32 }
fn no_icon() -> i32 { -1 }
//struct IconID(i32);
// let iconid = IconID(icon_id(x, y)); let IconID(id) = iconid;
////////////////////////////////////////////////////////////////////////////////

pub enum Widget {
    Label { iconid:i32, text:String },
    Button { iconid:i32, text:String },
    Check { text:String, option: Gc<Cell<bool>> },
    Radio { iconid:i32, text:String, index: Gc<Cell<i32>> },
    Slider { text:String, progress: Gc<Cell<f32>> },
    Row { unused:i8 /*compiler doesn't support empty struct variants*/},
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
    let numsibs = ui.get_child_count(parent);
    if numsibs < 2 { return CORNER_NONE; }
    let kidid = ui.get_child_id(item);
    let widget = ui.get_widget(parent);
    match *widget {
        Column { unused:_ } => {
            // first child, sharp corners down
            if kidid == 0 { return CORNER_DOWN; }
            // last child, sharp corners up
            else if kidid == numsibs-1 { return CORNER_TOP; }
            // middle child, sharp everywhere
            else { return CORNER_ALL; }
        }
        Row { unused: _ } => {
            // first child, sharp right
            if kidid == 0 { return CORNER_RIGHT; }
            // last child, sharp left
            else if kidid == numsibs-1 { return CORNER_LEFT; }
            // middle child, sharp all
            else { return CORNER_ALL; }
        }
        _ => {}
    };
    return CORNER_NONE;
}

// TODO consolidate oui::ItemState and blendish::WidgetState
////oui::ItemState
//    COLD   = ffi::UI_COLD,    // default
//    HOT    = ffi::UI_HOT,     // hover
//    ACTIVE = ffi::UI_ACTIVE,  // active
//    FROZEN = ffi::UI_FROZEN,  // dimmed, disabled
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

    // OUI extends state, adding a "frozen" which gets dimmed
    let item_state = ui.get_state(item);
    let (widget_state, frozen) = match item_state {
        COLD => (DEFAULT, false),
        HOT => (HOVER, false),
        oui::ACTIVE => (blendish::ACTIVE, false),
        _ => (DEFAULT, true)
    };
    if frozen {
        vg.nvg().global_alpha(DISABLED_ALPHA);
    }

    let kidid = ui.get_child_id(item);

    let cornerflags = corner_flags(ui, item);

    match *ui.get_widget(item) {
        Panel { unused:_ } => {
            // TODO move draw_bevel from lowlevel_draw to themed,
            // using the theme bg
            vg.draw_bevel(x, y, w, h);
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
        Radio { iconid:iconid, text:ref label, index:index } => {
            let state =
                if (*index).get() == kidid { blendish::ACTIVE }
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
        _ => {}
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

///////////////////////////////////////////////////////////////////////
// widget constructors

fn label(ui:&mut Context<Widget>, parent: Item, iconid: i32, label: &str)
-> Item
{
    let lbl = Label { iconid:iconid, text:label.to_string() };
    let item = ui.item(lbl);
    ui.set_size(item, 0, WIDGET_HEIGHT);
    ui.append(parent, item);
    return item;
}

fn button(ui:&mut Context<Widget>, parent: Item, tag: Tag, iconid: i32, label: &str,
    handler: Handler<Widget>)
-> Item
{
    // create new ui item
    // (store some custom data with the button that we use for styling)
    let btn = Button { iconid:iconid, text:label.to_string() };
    let item = ui.item(btn);
    // set persistent tag for item that is used
    // to track activity over time
    ui.set_tag(item, tag);
    // set size of wiget; horizontal size is dynamic, vertical is fixed
    ui.set_size(item, 0, WIDGET_HEIGHT);
    // attach event handler e.g. demohandler above
    ui.set_handler(item, handler, BUTTON0_DOWN); // HOT_UP
    ui.append(parent, item);
    return item;
}

fn check(ui:&mut Context<Widget>, parent: Item, tag: Tag, label: &str,
    option: Gc<Cell<bool>>, handler: Handler<Widget>)
-> Item
{
    let chk = Check { text:label.to_string(), option:option };
    let item = ui.item(chk);
    ui.set_tag(item, tag);
    ui.set_size(item, 0, WIDGET_HEIGHT);
    ui.set_handler(item, handler, BUTTON0_DOWN);
    ui.append(parent, item);
    return item;
}

fn slider(ui:&mut Context<Widget>, parent: Item, tag: Tag, label: &str,
    progress: Gc<Cell<f32>>)
-> Item
{
    let sli = Slider { text:label.to_string(), progress:progress };
    let item = ui.item(sli);
    ui.set_tag(item, tag);
    ui.set_size(item, 0, WIDGET_HEIGHT);
    // attach our slider event handler and capture two classes of events
    ui.set_handler(item, Some(sliderhandler), BUTTON0_DOWN|BUTTON0_CAPTURE);
    ui.append(parent, item);
    return item;
}

fn radio(ui:&mut Context<Widget>, parent: Item, tag: Tag, iconid: i32, label: &str,
    index: Gc<Cell<i32>>)
-> Item
{
    let rad = Radio { iconid:iconid, text:label.to_string(), index:index };
    let item = ui.item(rad);
    ui.set_tag(item, tag);
    let w = if label.len() == 0 { TOOL_WIDTH } else { 0 };
    ui.set_size(item, w, WIDGET_HEIGHT);
    ui.set_handler(item, Some(radiohandler), BUTTON0_DOWN);
    ui.append(parent, item);
    return item;
}

fn panel(ui:&mut Context<Widget>) -> Item
{
    ui.item(Panel{unused:0})
}

fn column(ui:&mut Context<Widget>, parent: Item) -> Item
{
    let item = ui.item(Column{unused:0});
    ui.set_handler(item, Some(columnhandler), APPEND);
    ui.append(parent, item);
    return item;
}

fn row(ui: &mut Context<Widget>, parent: Item) -> Item
{
    let item = ui.item(Row{unused:0});
    ui.set_handler(item, Some(rowhandler), APPEND);
    ui.append(parent, item);
    return item;
}

fn vgroup(ui:&mut Context<Widget>, parent: Item) -> Item
{
    let item = ui.item(Column{unused:0});
    ui.set_handler(item, Some(vgrouphandler), APPEND);
    ui.append(parent, item);
    return item;
}

fn hgroup(ui:&mut Context<Widget>, parent: Item) -> Item
{
    let item = ui.item(Row{unused:0});
    ui.set_handler(item, Some(hgrouphandler), APPEND);
    ui.append(parent, item);
    return item;
}



///////////////////////////////////////////////////////////////////////
// handlers

fn demohandler(ui: &mut Context<Widget>, item: Item, _event: EventFlags) {
    let tag = ui.get_tag(item);
    let widget = ui.get_widget(item);
    match *widget {
        Button { text: ref mut label, iconid:_ } => {
            println!("clicked: #{} '{}'", tag, label);
        }
        _ => {}
    }
}
fn checkhandler(ui: &mut Context<Widget>, item: Item, _event: EventFlags) {
    let tag = ui.get_tag(item);
    let widget = ui.get_widget(item);
    match *widget {
        Check { text: ref mut label, option: option } => {
            println!("clicked: #{} '{}'", tag, label);
            let cell: Gc<Cell<bool>> = option;
            cell.set(!cell.get());
        }
        _ => {}
    }
}
fn freezehandler(ui: &mut Context<Widget>, item:Item, event:EventFlags) {
    // "super" call, handles default cell-update-on-click
    checkhandler(ui, item, event);
    // we didn't cache our target anywhere, so go find it.
    // (demo freezes the right column of the row that's above the
    // button that's above the "Freeze" checkbox)
    let tgt = {
        let it = ui.prev_sibling(item);
        let it = ui.prev_sibling(it);
        let tgt = ui.last_child(it);
        tgt
    };

    let tgt_id = ui.get_child_id(tgt);
    let friz = {
        let widget = ui.get_widget(item);
        match *widget {
            Check { text:_, option: option } => {
                option.get()
            }
            _ => { false }
        }
    };

    println!("freezing: #{} to '{}'", tgt_id, friz);
    ui.set_frozen(tgt, friz);
}
// simple logic for a radio button
fn radiohandler(ui: &mut Context<Widget>, item: Item, _event: EventFlags) {
    let tag = ui.get_tag(item);
    let kidid = ui.get_child_id(item);
    let widget = ui.get_widget(item);
    match *widget {
        Radio { iconid:_, text: ref mut label, index: index } => {
            println!("clicked: #{} '{}'", tag, label);
            let cell: Gc<Cell<i32>> = index;
            cell.set(kidid);
        }
        _ => {}
    }
}

// simple logic for a slider
// event handler for slider (same handler for all sliders)
fn sliderhandler(ui: &mut Context<Widget>, item: Item, event: EventFlags) {
    println!("tag slider #{} event {}", ui.get_tag(item), event);

    // starting offset of the currently active slider
    static mut sliderstart: f32 = 0.0;
    let pos = ui.get_cursor_start_delta();
    let rc = ui.get_rect(item);
    let widget = ui.get_widget(item);
    match event {
        BUTTON0_DOWN => {
            println!("button0 down");
            match *widget {
                Slider { text:_, progress: currval } => {
                    unsafe { sliderstart = currval.get() };
                }
                _ => {}
            }
        }
        BUTTON0_CAPTURE => {
            println!("button0 capture");
            let val = unsafe { sliderstart + (pos.x as f32 / rc.w as f32) };
            let val = clamp(val, 0.0, 1.0);
            match *widget {
                Slider { text:_, progress: currval } => {
                    currval.set(val);
                }
                _ => {}
            }
        }
        _ => { println!("missed a slider event: {}", event) }
    }
}

fn columnhandler(ui: &mut Context<Widget>, parent: Item, _event: EventFlags) {
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
fn rowhandler(ui: &mut Context<Widget>, parent: Item, _event: EventFlags) {
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
fn vgrouphandler(ui: &mut Context<Widget>, parent: Item, _event: EventFlags) {
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
fn hgrouphandler(ui: &mut Context<Widget>, parent: Item, _event: EventFlags) {
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
// end handlers
///////////////////////////////////////////////////////////////////////

pub fn create() -> Context<Widget> {
    Context::create_context()
}

// pub fn init(ui: &mut Context<Widget>, data: &'a AppData) {
pub fn init(app: &mut super::App) {

    let ui = &mut app.ui;

    // setup the UI

    ui.clear(); // removes any previous items, currently will break
                // if multiple-re-init.

    // build the ui hierarchy: start at root,
    // compose elements into nested groups that flow

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
        radio(ui, h, 3, icon_id(6,  3), "Item 3.0", app.data.enum1);
        radio(ui, h, 4, icon_id(0, 10), "", app.data.enum1);
        radio(ui, h, 5, icon_id(1, 10), "", app.data.enum1);
        radio(ui, h, 6, icon_id(6,  3), "Item 3.3", app.data.enum1);
    }

    {
        let row = row(ui, col);
        let left = vgroup(ui, row);
        label(ui, left, no_icon(), "Items 4.0:");
        let left_body = vgroup(ui, left);
        button(ui, left_body, 7, icon_id(6, 3), "Item 4.0.0", Some(demohandler));
        button(ui, left_body, 8, icon_id(6, 3), "Item 4.0.1", Some(demohandler));
        let right = vgroup(ui, row);
        ui.set_frozen(right, app.data.option1.get()); // a bullshit call, to make init match fake data
        label(ui, right, no_icon(), "Items 4.1:");
        let right_body = vgroup(ui, right);
        slider(ui, right_body,  9, "Item 4.1.0", app.data.progress1);
        slider(ui, right_body, 10, "Item 4.1.1", app.data.progress2);
    }

    button(ui, col, 11, icon_id(6, 3), "Item 5", None);

    check(ui, col, 12, "Freeze section 4.1", app.data.option1, Some(freezehandler));
    check(ui, col, 13, "Item 7", app.data.option2, Some(checkhandler));
    check(ui, col, 14, "Item 8", app.data.option3, Some(checkhandler));

    // structure is built, append-handlers have run (so edge-grabbers are set);
    // now complete the layout

    ui.layout();
}

pub fn update(ui: &mut Context<Widget>, (mx,my): (i32,i32), btn: bool, _t: f32) {
    // apply inputs: mouse and buttons, keys if needed

    ui.set_button(0/*left button*/, btn);
    ui.set_cursor(mx, my);

    // process input triggers to update item states
    ui.process();
}

pub fn draw(ui: &mut Context<Widget>, ctx: &mut ThemedContext, w:f32, h:f32)
{
    // draw the ui
    ctx.draw_background(0.0, 0.0, w, h);

    let root = ui.root();
    draw_ui(ui, ctx, root, 0, 0);
}
