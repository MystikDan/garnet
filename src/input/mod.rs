struct Input;

pub struct Interface {
    p: Input
}

impl Interface {
    // this was a nall::function in ruby
    fn on_change(&self, device: &::util::Device, group: usize, input: usize, old_vlue: i16, new_value: i16) -> () {}

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

    fn acquire(&self) -> bool {unimplemented!()}
    fn unacquire(&self) -> bool {unimplemented!()}
    fn acquired(&self) -> bool {unimplemented!()}

    fn poll(&self) -> Vec<&::util::Device> {unimplemented!()}
    fn rumble(&self, id: u64, enable: bool) -> bool {unimplemented!()}

    fn new() {}
}

impl Drop for Interface {
    fn drop(&mut self) {}
}