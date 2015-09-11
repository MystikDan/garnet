struct Input;

pub struct Interface {
    p: Input
}

impl Interface {
    // this was a nall::function in ruby
    fn onChange(&self, device: &::util::Device, group: usize, input: usize, oldValue: i16, newValue: i16) -> () {}

    fn driver(&self, driver: Option<&str>) -> () {
        let driver = match driver {
            Some(s) => s,
            None => ""
        };
    }

    fn optimalDriver(&self) -> &str {unimplemented!()}
    fn safestDriver(&self) -> &str {unimplemented!()}
    fn availableDrivers(&self) -> &str {unimplemented!()}
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