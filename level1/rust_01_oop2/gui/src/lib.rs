pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub trait Draw {
    fn draw(&self);
}
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for v in self.components.iter() {
            v.draw();
        }
    }
}

pub struct Button {
    pub height: i32,
    pub width: i32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "the component height is {},
            width is{},label is {}",
            self.height, self.width, self.label
        );
    }
}

pub struct SelectButton {
    pub height: i32,
    pub width: i32,
    pub option: Vec<String>,
}

impl Draw for SelectButton {
    fn draw(&self) {
        println!(
            "the SelectButton component height is {},
            width is{}, option is {:#?}",
            self.height, self.width, self.option
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
