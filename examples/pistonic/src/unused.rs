
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

    //static mut data_opt:Option<AppData<'static>> = None;
    //let data = unsafe {
    //    if data_opt.is_none() {
    //        data_opt = Some(init_app_data());
    //    };
    //    data_opt.unwrap()
    //};
