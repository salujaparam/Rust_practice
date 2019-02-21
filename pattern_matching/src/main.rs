fn main() {
    let v_name = String::from("param saluja");

    println!("Char at position 9 is {}",
    match v_name.chars().nth(9) {
        Some(v) => v.to_string(),
        None => "No char at 9th position".to_string(),
    });
}
