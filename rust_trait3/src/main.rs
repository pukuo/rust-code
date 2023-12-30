trait getName{
    fn get_name(&self)->&String;
}
trait getAge{
    fn get_age(&self)->i32;
}
struct getInformation<T,U>{
    student:T,
    teacher:U,
}
impl<T:getName+getAge,U:getName+getAge> getInformation<T,U>{
    fn get_information(&self){
        println!("student name is {}",self.student.get_name());
        println!("student age is {}",self.student.get_age());
        println!("teacher name is {}",self.teacher.get_name());
        println!("teacher age is {}",self.teacher.get_age());
    }
}
struct U1{
    name:String,
    age:i32,
}
impl getName for U1{
    fn get_name(&self)->&String {
        &self.name
    }
}
impl getAge for U1{
    fn get_age(&self)->i32 {
        self.age
    }
}
struct U2{
    name:String,
    age:i32,
}

impl getName for U2{
    fn get_name(&self)->&String {
        &self.name
    }
}
impl getAge for U2{
    fn get_age(&self)->i32 {
        self.age
    }
}

fn main() {
    let user1=U1{
        name:String::from("guolisen"),
        age:18,
    };
    let user2=U2{
        name:String::from("zc"),
        age:30,
        
    };
    let res=getInformation{
        student:user1,
        teacher:user2,
    };
    res.get_information();
}
