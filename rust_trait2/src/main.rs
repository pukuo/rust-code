trait GetAge{
    fn get_age(&self)->i32;
}
trait GetName{
    fn get_name(&self)->&String;
}

#[derive(Debug)]
struct User{
    name:String,
    age:i32,
}

impl GetAge for User{
    fn get_age(&self)->i32 {
        self.age
    }
}

impl GetName for User{
    fn get_name(&self)->&String {
        &self.name
    }
}

fn get_information<T:GetAge+GetName>(item:T){
    println!("item age is : {}",item.get_age());
    println!("item name is : {}",item.get_name());
}


//trait 类型作为返回值
fn produce_item_with_age()->impl GetAge{
    User{
        name:String::from("trait im...."),
        age:24,
    }
}

fn get_infor<T>(item:T)where T:GetAge+GetName
{
    // println!("User is : {:#?}",item);
    println!("name is {}",item.get_name());
    println!("age is {}",item.get_age());
}
fn main() {
    let user=User{
        name:String::from("guolisen"),
        age:18,
    };

    //方法一
    //get_information(user);

    //方法二
    get_infor(user);

    let rs=produce_item_with_age();
    //println!("The Result is : {:#?}",rs);
}
