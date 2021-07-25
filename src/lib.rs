#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod area {
    use std::f32::consts::PI;

    pub enum TwoDObject {
        Circle { radius: f32 },
        Rectangle { width: f32, height: f32 },
    }

    pub fn figure_out_area(object: TwoDObject) -> f32 {
        match object {
            TwoDObject::Circle { radius } => PI * radius * radius,
            TwoDObject::Rectangle { width, height } => width * height,
        }
    }
}

mod volume {
    use std::f32::consts::PI;

    pub enum ThreeDObject {
        Sphere { radius: f32 },
        Parallelepiped { width: f32, height: f32, length: f32 },
    }

    pub fn figure_out_volume(object: ThreeDObject) -> f32 {
        match object {
            ThreeDObject::Sphere { radius } => (4.0 / 3.0) * PI * radius * radius * radius,
            ThreeDObject::Parallelepiped { width, height, length } => width * height * length,
        }
    }
}