fn main() {
    let a = 9;

    if a > 20 {
        println!("Statement is true");
    }
    else {
        println!("statement is false");
    }

    if a % 2 == 0 {
        println!("a can be divided by 2");
    }
    else if a % 3 == 0 {
        println!("a can be divided by 3");
    }
    else {
        println!("cannot be divided by 2 or 5");
    }
}
