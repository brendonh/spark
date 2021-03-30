#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shell {
    pub length: f32,
    pub width: f32,
}

impl Shell {
    pub fn new(length: f32, width: f32) -> Shell {
        Shell {
            length,
            width,
        }
    }
}
