pub trait Video {
    fn cap(&self, name: &str) -> bool { false }
    // fn getHandle(&self) -> &::std::any::Any { false }
    fn get_synchronize(&self) -> bool { false }
    fn get_depth(&self) -> usize { 0 }
    fn get_filter(&self) -> usize { 0 }
    fn get_shader(&self) -> &str { "" }
    // fn setHandle(&self, value: &::std::any::Any) -> bool { false }
    fn set_synchronize(&self, value: bool) -> bool { false }
    fn set_depth(&self, value: usize) -> bool { false }
    fn set_filter(&self, value: usize) -> bool { false }
    fn set_shader(&self, value: &str) -> bool { false }

    fn lock(&self, data: &u32, pitch: &usize, width: usize, height: usize) -> bool { false }
    fn unlock(&self) -> () {}

    fn clear(&self) -> () {}
    fn refresh(&self) -> () {}
    fn init(&self) -> bool { true }
    fn term(&self) -> () {}

    fn new() -> Self ;
}

pub mod consts {
    const FILTER_NEAREST: usize = 0;
    const FILTER_LINEAR: usize = 1;
}

enum Driver {
    OpenGL,
    Direct3D,
    DirectDraw,
    GDI,
    QtOpenGL,
    QtRaster,
    SDL,
    XShm,
    XVideo
}

pub struct Interface<V:Video> {
    p: V
}

impl <V:Video> Interface <V> {


    fn driver(&self, driver: Option<&str>) -> () {
        let driver = match driver {
            Some(s) => s,
            None => ""
        };
    }

    fn optimal_driver(&self) -> &str {unimplemented!()}
    fn safest_driver(&self) -> &str {unimplemented!()}
    fn available_drivers(&self) -> &str {unimplemented!()}
    fn init(&self) -> bool {unimplemented!()}
    fn term(&self) -> () {}

    fn cap(&self, name: &String) -> bool {unimplemented!()}
    fn get(&self, name: &String) -> &::std::any::Any {unimplemented!()}
    fn set(&self, name: &String, value: &::std::any::Any) -> bool {unimplemented!()}

    fn lock(&self, data: &u32, pitch: &usize, width: usize, height: usize) -> bool {unimplemented!()}
    fn unlock(&self) -> () {}
    fn clear(&self) -> () {}
    fn refresh(&self) -> () {}

    fn new() {}
}

impl <V:Video> Drop for Interface <V> {
    fn drop(&mut self) {}
}