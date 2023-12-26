use std::{f32::consts::PI, ops::Deref};

pub mod modules;

pub trait ToArea {
    fn area(&self) -> f32;
}

pub struct Circle {
    pub radius: f32,
}

impl PartialEq<Square> for Circle {
    fn eq(&self, other: &Square) -> bool {
        self.area() == other.area()
    }
}
impl ToArea for Circle {
    fn area(&self) -> f32 {
        PI * self.radius * self.radius
    }
}

pub struct Square {
    pub width: f32,
    pub height: f32,
}
impl ToArea for Square {
    fn area(&self) -> f32 {
        self.height * self.width
    }
}

impl Deref for Square {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        println!("yo");
        &1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp() {
        let circle = Circle { radius: 10.0 };
        assert_eq!(circle.area(), PI * 100.0);
        let square = &Square {
            height: 5.0,
            width: 4.0,
        };
        assert_eq!(square.area(), 20.0);

        assert!(circle == *square); // circle.eq(&square));
    }
}
