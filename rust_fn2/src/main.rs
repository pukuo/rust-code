fn takes_ownership(str1: String) -> String{
    println!("the value of str1 is:{}", str1);
    str1
}

fn makes_copy(i1: i32) {
    println!("the value of i1 is:{}", i1);
}

fn main() {
    let s1 = String::from("hello,world");
    let s2=takes_ownership(s1);
    println!("the value of s2 is:{}", s2);
    let num = 12;
    makes_copy(num);
    println!("the value of num is:{}", num);
}
