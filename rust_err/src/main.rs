use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    let r=read_username_from_file();
    match r{
        Ok(s)=> println!("s={}",s),
        Err(e)=>println!("err={:?}",e),
    }
}

fn read_username_from_file() -> Result<String,io::Error>{
    //let mut f=File::open("hello.txt")?; //方法2 修改为let mut f

    //方法1
    //let mut f = match f{
    //    Ok(file)=>file,
    //    Err(error)=>return Err(error),
    //};

    //方法2
    

    //let mut s =String::new();
    //match f.read_to_string(&mut s){
    //    Ok(_)=>Ok(s),
    //    Err(error)=>Err(error),
    //}
    //f.read_to_string(&mut s)?; //方法2
    //Ok(s)

    //方法3
    let mut s=String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)

    

   
}
