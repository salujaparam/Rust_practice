fn main() {
    let tup:(i32, i32, i32) =(1,2,3);

    let (x, y, z) = tup;

    let a = tup.0;

    println!("A - {}",a);

    print!("x - {} y - {} z - {}",x,y,z);
}