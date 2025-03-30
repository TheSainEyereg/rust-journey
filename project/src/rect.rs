#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self::new(4, 3)
    }
}
