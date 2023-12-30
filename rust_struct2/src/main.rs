fn main() {
    let dog = Dog {
        name: String::from("旺财"),
        weight: 192.0,
        height: 154.8,
    };
    println!("the value of dog is {:#?}", dog);
    println!("dog name is {}", dog.get_name());
    println!("dog weight is {}", dog.get_weight());
    println!("dog height is {}", dog.get_height());
}

#[derive(Debug)]
struct Dog {
    name: String,
    weight: f32,
    height: f32,
}

impl Dog {
    fn get_name(&self) -> &str {
        //self.name[..].to_string()
        &(self.name[..])
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }

    fn get_height(&self) -> f32 {
        self.height
    }
}
