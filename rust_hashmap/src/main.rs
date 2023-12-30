use std::collections::HashMap;
fn main() {
    //1.HashMap(k,v)
    let mut scores = HashMap::new();
    scores.insert(String::from("hello"),1);

    let keys=vec![
        String::from("Blue"),
        String::from("Yellow"),
    ];
    let values=vec![10,20];

    let scores:HashMap<_,_> =keys.iter().zip(values.iter()).collect();

    let key = String::from("Yellow");
    if let Some(v)=scores.get(&key){ //get 返回的是一个Option类型
        println!("v = {}",v);
    }

    let v1 = scores.get(&key);
    match v1{
        Some(val)=>println!("val is :{}",val),
        None=>println!("do nothing..."),
    }
    //2.创建HashMap
    //3.读取
    //4.遍历
    for (key,value) in &scores{
        println!("key is {}:value is {}",key,value);
    }
    //5.更新

    //直接插入
    let mut ss = HashMap::new();
    ss.insert(String::from("one"),1);
    ss.insert(String::from("two"),2);
    ss.insert(String::from("three"),3);

    println!("value is {:?}",ss);
    // 键不存在的时候再插入
    ss.entry(String::from("Four")).or_insert(6666666);

    println!("value is {:?}",ss);

    //根据旧值来更新一个值
    let text="hello world hello guolisen world";
    let mut map=HashMap::new();
    for res in text.split_whitespace(){
       let count= map.entry(res).or_insert(0);
        *count+=1;
    }
    println!("map is {:#?}",map);
    
}
