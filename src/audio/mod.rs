struct Audio;

pub struct Interface {
    p: Audio
}

impl Interface {
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

    fn sample(&self, left: u16, right: u16) -> () {}
    fn clear(&self) -> () {}

    fn new() {}
}

impl Drop for Interface {
    fn drop(&mut self) {}
}