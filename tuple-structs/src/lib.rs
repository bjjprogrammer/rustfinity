pub struct Rectangle(pub f32, pub f32);

impl Rectangle {
    pub fn area(&self) -> f32 {
        self.0 * self.1
    }
}

pub fn area(rect: &Rectangle) -> f32 {
    rect.area()
}
