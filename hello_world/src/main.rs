fn main(){
    //let mut s1=String::from("hello,world");
    //s1.push_str("guolisen");
    //println!("s1 is {}",s1);
    let s1=String::from("hello");
    let s2=s1;
    println!("s2 is {}",s2);
    //println!("s1 is {}",s1); //the value of s1 is moved to s2,s1 invalid

    //clone 方法(深拷贝，把分配在堆上的值给一起拷贝过来了)=====>堆上的拷贝
    let s3=s2.clone();
    println!("s3 is {}",s3);
    println!("s2 is {}",s2);

    //copy 方法 栈上的拷贝
    
}
