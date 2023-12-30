fn main() {
    let mut str=String::from("你好世界");
    let mut str1=String::new();
    str.push_str(",world");
    str1.push_str("hello");
    str1.push_str(",world");
    str1.push('!');

    for i in str.chars(){
        println!("str : {}",i);
    }

    println!("==============================");
    
    for i in str1.bytes(){
        println!("str1 is :{}",i);
    }

    let a="hello,world!!!!".to_string();
    let b=String::from("guolisen");

    let c = a+&b;
    println!("c is :{}",c);


    let x="hello".to_string();
    let y="world".to_string();
    
    let d =format!("{}-{}",x,y);
    println!("d is {}",d);
    
}
