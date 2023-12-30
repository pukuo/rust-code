enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    Change(i32,i32,i32),
    
}
impl Message{
    fn get_value(&self){
        match *self{
            Message::Quit=>println!("the progrmming exit..."),
            Message::Move{x,y}=>println!("the x move {},y move is {}",x,y),
            Message::Change(a,b,c)=>println!("Change a is {}.b is {}.c is {}",a,b,c),
            _=>println!("Write..."),
        }
    }
}
fn main() {
    //Message::get_value(Quit);
    let ms=Message::Quit;
    ms.get_value();

    let mv=Message::Move { x: 12, y: 13 };
    mv.get_value();

    let change=Message::Change(1,2,3);
    change.get_value();
}
