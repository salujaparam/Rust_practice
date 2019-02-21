fn main() {
    println!("Hello, world");

    main1();
    main2(20);
}

fn main1() {
    println!("i'm main1 function");
}

fn main2(a:i32) {
    println!("What is the value im getting - {}",a);
}