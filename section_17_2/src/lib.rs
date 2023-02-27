pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

pub trait Draw {
    fn draw(&self);
}

impl<T> Screen<T> where T: Draw{
    pub fn run(&self) {
	for component in self.components.iter() {
	    component.draw();
	}
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
    }
}
