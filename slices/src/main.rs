fn main() {
    let s = String::from("hello, world");
    let hello = &s[0..6];

    println!("Sliced String -> {}", hello);

    let world = &s[7..];

    println!("Next Sliced String -> {}", world);
}