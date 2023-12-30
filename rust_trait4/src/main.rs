/*
   对任何实现了特定trait的类型有条件的实现trait
 */

trait getName{
    fn get_name(&self)->&String;
}
trait printName{
    fn print_name(&self);
}
impl<T:getName> printName for T{
    fn print_name(&self) {
        println!("self name is {}",self.get_name());
    }
}
struct User{
    name:String,
}
impl getName for User{
    fn get_name(&self)->&String {
        &self.name
    }
}

fn main() {
    let u1=User{
        name:"hello,world".to_string(),
    };
    u1.print_name();
}
