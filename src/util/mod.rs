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
            let test_name = i.name.clone();
            match test_name {
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
    fn is_null(&self) -> bool {
        false
    }
    #[inline]
    fn is_keyboard(&self) -> bool {
        false
    }
    #[inline]
    fn is_mouse(&self) -> bool {
        false
    }
    #[inline]
    fn is_joypad(&self) -> bool {
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
    fn path_id(&self) -> u32 {
        (self.id >> 32) as u32
    }
    #[inline]
    fn device_id(&self) -> u32 {
        (self.id >> 0) as u32
    }
    #[inline]
    fn vendor_id(&self) -> u16 {
        (self.id >> 16) as u16
    }
    #[inline]
    fn product_id(&self) -> u16 {
        (self.id >> 0) as u16
    }

    fn append(&mut self, name: String) -> () {
        self.group.push(Group::new(Some(name)));
    }

    fn find(&self, name: String) -> Option<usize> {
        self.group.iter().position(move|g: &Group|{
            let test_name = g.name.clone();
            match test_name {
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