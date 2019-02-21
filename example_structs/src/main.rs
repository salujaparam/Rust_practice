#[derive(Debug)]
struct Square {
    side: u32,
}

fn main() {
    let Square1 = Square{
        side: 10,
    };

    let area = area(&Square1);

    println!("area = {}", area);
}


fn area(square: &Square) -> u32 {
    square.side * square.side
}