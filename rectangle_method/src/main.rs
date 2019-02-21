#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn main() {
    let rectangle1 = Rectangle{
        length: 20,
        width: 10
    };

    println!("Area of Rectangle is {}", rectangle1.area())

}
