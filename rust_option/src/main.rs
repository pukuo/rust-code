fn main() {
    let x:Option<i32> = Some(5);
    let y:Option<i32> = None;

    let ms=value_test(x);
    match ms{
        Some(i)=>println!("the value is:{}",i),
        None=>{println!("do nothing")}
    }
}

fn value_test(a:Option<i32>)->Option<i32>{
    match a{
        Some(a)=>Some(a+1),
        None=>None,
    }
    
}
