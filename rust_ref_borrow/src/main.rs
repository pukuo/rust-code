//引用，不改变值
fn keep_old(val:&String)->&String{
    &val
}

//借用，改变值
fn modefiy_value(val2:&mut String){
    val2.push_str(",is hero!");
}

fn main() {
    let s1=String::from("hello");

    /*
        引用：&
        用法：让我们创建一个指向值的引用，但是并不拥有它，因而不拥有这个值，所以，当引用离开其值指向的作用域后也不会被丢弃
     */
    keep_old(&s1);
    println!("the value of s1 is:{}",s1);

    let mut s2=String::from("guolisen");

    //在任意给定时间内，有了可变引用之后，就不能再有不可变引用
    //let s3=&s2;

    /*
        借用：&mut
     */
    modefiy_value(&mut s2);
    println!("the value of s2 is:{}",s2);
    //println!("the value of s3 is:{}",s3);
}
