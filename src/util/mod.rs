struct Input {
    name: Option<String>,
    value: i16
}

impl Input {
    fn new(name: Option<String>) -> Input {
        Input {
            name: name,
            value: 0
        }
    }
}

struct Group {
    name: Option<String>,
    input: Vec<Input>
}

impl Group {
    fn append(&mut self, name: String) -> () {
        self.input.push(Input::new(Some(name)));
    }

    fn find(&self, name: String) -> Option<usize> {
        self.input.iter().position(move|i: &Input|{
            let testName = i.name.clone();
            match testName {
                Some(n) => n == name,
                None => false
            }
        })
    }

    fn new(name: Option<String>) -> Group {
        Group {
            name: name,
            input: vec![]
        }
    }
}

trait DeviceTrait {
    #[inline]
    fn isNull(&self) -> bool {
        false
    }
    #[inline]
    fn isKeyboard(&self) -> bool {
        false
    }
    #[inline]
    fn isMouse(&self) -> bool {
        false
    }
    #[inline]
    fn isJoypad(&self) -> bool {
        false
    }
}

pub struct Device {
    id: u64,
    name: Option<String>,
    group: Vec<Group>
}

impl Device {
    #[inline]
    fn pathID(&self) -> u32 {
        (self.id >> 32) as u32
    }
    #[inline]
    fn deviceID(&self) -> u32 {
        (self.id >> 0) as u32
    }
    #[inline]
    fn vendorID(&self) -> u16 {
        (self.id >> 16) as u16
    }
    #[inline]
    fn productID(&self) -> u16 {
        (self.id >> 0) as u16
    }

    fn append(&mut self, name: String) -> () {
        self.group.push(Group::new(Some(name)));
    }

    fn find(&self, name: String) -> Option<usize> {
        self.group.iter().position(move|g: &Group|{
            let testName = g.name.clone();
            match testName {
                Some(n) => n == name,
                None => false
            }
        })
    }

    fn new(name: Option<String>) -> Device {
        Device {
            id: 0,
            name: name,
            group: vec![]
        }
    }
}

impl DeviceTrait for Device {}