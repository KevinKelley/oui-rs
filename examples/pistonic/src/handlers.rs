
pub struct AppData<'a> {
    // some persistent variables for demonstration
    pub enum1:     Cell<i32>,
    pub progress1: Cell<f32>,
    pub progress2: Cell<f32>,
    pub option1:  Cell<bool>,
    pub option2:  Cell<bool>,
    pub option3:  Cell<bool>,
}
pub fn init_app_data<'a>() -> AppData<'a> {
    AppData {
        enum1:     Cell::new(0),
        progress1: Cell::new(0.25),
        progress2: Cell::new(0.75),
        option1:   Cell::new(true),
        option2:   Cell::new(false),
        option3:   Cell::new(false),
    }
}


fn demohandler(item: Item, event: EventFlags) {
//    let data = (const UIButtonData *)ui.get_data(item);
//    printf("clicked: %lld %s\n", ui.get_handle(item), data.label);
}


fn checkhandler(item: Item, event: EventFlags) {
//    let data = (const UICheckData *)ui.get_data(item);
//    *data.option = !(*data.option);
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

// simple logic for a radio button
fn radiohandler(item: Item, event: EventFlags) {
//    UIRadioData *data = (UIRadioData *)ui.get_data(item);
//    *data.value = ui.get_child_id(item);
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


fn rowhandler(parent: Item, event: EventFlags) {
//    let item = ui.last_child(parent);
//    let last = ui.prev_sibling(item);
//    ui.set_rel_to_left(item, last);
//    if (last > 0)
//        ui.set_rel_to_right(last, item);
//    ui.set_layout(item, LEFT|RIGHT);
//    ui.set_margins(item, (last < 0)?0:8, 0, 0, 0);
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


fn hgrouphandler(parent: Item, event: EventFlags) {
//    let item = ui.last_child(parent);
//    let last = ui.prev_sibling(item);
//    ui.set_rel_to_left(item, last);
//    if (last > 0)
//        ui.set_rel_to_right(last, item);
//    ui.set_layout(item, LEFT|RIGHT);
//    ui.set_margins(item, (last < 0)?0:-1, 0, 0, 0);
}



    //static mut data_opt:Option<AppData<'static>> = None;
    //let data = unsafe {
    //    if data_opt.is_none() {
    //        data_opt = Some(init_app_data());
    //    };
    //    data_opt.unwrap()
    //};
