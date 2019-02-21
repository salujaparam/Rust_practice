fn main() {

    let s = String::from("param");
    main1(&s);

    println!("{}",&s);
}

fn main1(x:&String) {
    println!("{}", x);
}