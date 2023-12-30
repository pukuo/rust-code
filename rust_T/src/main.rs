#[derive(Debug)]
struct User<T>{
    x:T,
    y:T,
    
    
}   

fn main() {
    let number=User{
        x:1,
        y:2,
    };
    println!("x is {:#?}",number);    
}
