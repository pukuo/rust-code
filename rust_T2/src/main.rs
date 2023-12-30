#[derive(Debug)]
struct Point<T,U>{
    x:T,
    y:U,
}
impl<T,U> Point<T,U> {
    fn create_point<W,V>(self,other_point:Point<W,V>)->Point<T,V>{
        Point{
            x:self.x,
            y:other_point.y,
        }
    }

}

fn main() {
    let point=Point{
        x:12,
        y:2.0,
    };
    let point2=Point{
        x:String::from("hello"),
        y:12.888888,
    };
    let point3=point.create_point(point2);
    println!("point3 is {:#?}",point3);
    
}
