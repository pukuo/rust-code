fn main() {
    let mut ve:Vec<i32> = Vec::new();

    let ve1 =vec![1,2,3];

    ve.push(1);
    ve.push(2);
    ve.push(3);

   // for i in &ve{
     //   println!("i:=",*i);
    //}

    let rs:&i32=&ve[0];
    println!("rs is {}",rs);

   // let res = ve.get(2);
   // println!("res is :{}",res);
    match ve.get(6){
        Some(value)=>{println!("value is {}",value)}
        None=>println!("none...."),

    }

    //不可变的遍历
    for i in &ve{
        println!("i:{}",i);
    }
    //可变的遍历
    for i in &mut ve{
        *i+=1;
        println!("i+ is :{}",i);
    }

    #[derive(Debug)]
    enum Value_v2{
        T1(String),
        T2(i32),
        T3(f32),
            
    }

    let vec=vec![
        Value_v2::T1(String::from("hello,world")),
        Value_v2::T2(127),
        Value_v2::T3(12.5),
    ];
   for i in &vec{
    println!("velce is :{:#?}",i);
}
   // let sqw:&Value_v2=&vec[0];
   // println!("{:?}",sqw);
}
