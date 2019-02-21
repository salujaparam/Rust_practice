fn main() {
    println!("Hello, world!");
    let b = sum(2,4);

    println!("The sum is {}",b);
}

fn sum (a:i32, b:i32) -> i32{
    let z = a+b;

    return z * 10; // z*10
}