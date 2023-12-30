pub struct User{
    name:String,
    age:i32,
}

pub struct Teacher{
    name:String,
    age:i32,
}

//定义一个trait
pub trait Get_information {
    fn get_name(&self)->&String;
    fn get_age(&self)->i32;
}

//trait默认实现
trait SchoolName{
    fn get_school_name(&self)->String{
        String::from("TongJIangxiang ShiyanXiaoxue")
    }
}


impl Get_information for User{
    fn get_name(&self)->&String {
        &self.name
    }
    fn get_age(&self)->i32 {
        self.age
    }
}

impl Get_information for Teacher{
    fn get_name(&self)->&String {
        &self.name
    }
    fn get_age(&self)->i32 {
        self.age
    }
    
}

impl SchoolName for User{}

impl SchoolName for Teacher{
    fn get_school_name(&self)->String {
        String::from("GuangmingSchool")
    }
}

//trait 作为参数
fn print_information(item : impl Get_information){
    println!("name={},age={}",item.get_name(),item.get_age());
}

fn main() {
    let stu=User{
        name:String::from("guolisen"),
        age:24,
    };
    let tea=Teacher{
        name:String::from("zc"),
        age:38,
    };

    //println!("stu name is {},stu age is {}",stu.get_name(),stu.get_age());
    //println!("tea name is {},tea age is {}",tea.get_name(),tea.get_age());
    //print_information(stu);
    //print_information(tea);

    let sname=stu.get_school_name();
    println!("sname is {}",sname);
    let tname=tea.get_school_name();
    println!("tname is {}",tname);

    print_information(stu);
    print_information(tea);
}
