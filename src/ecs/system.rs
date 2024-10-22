pub trait System {
    fn apply(&self);
}

pub struct Render {
}

impl System for Render {
    fn apply(&self) {
        println!("Apply for Render");
    }
}

pub struct Transform {
}

impl System for Transform {
    fn apply(&self) {
        println!("Apply for Transform");
    }
}
