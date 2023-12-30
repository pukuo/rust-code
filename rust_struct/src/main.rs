#[derive(Debug)]
struct User{
    name:String,
    age:i32,
    email:String,
}
fn main() {
    let user0=User{
        name:String::from("guolisen"),
        age:18,
        email:String::from("guolisen@gmail.com"),
    };
    
    let user1={
        ..user0
    };

    println!("the value of user1 is {:#?}",user1);
}
