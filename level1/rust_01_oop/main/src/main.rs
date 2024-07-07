use getaver;

fn main() {
    let mut oop = getaver::AveCollect::new();
    oop.add(1);

    println!("oop average value is :{:#?}", oop.update_average());
    oop.add(2);

    println!("oop average value is :{:#?}", oop.update_average());
    oop.add(3);

    println!("oop average value is :{:#?}", oop.update_average());
    oop.delete();

    println!("oop average value is :{:#?}", oop.update_average());
}
