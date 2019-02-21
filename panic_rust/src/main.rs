fn main() {
    // panic!("crash and quit!!");

    let v = vec![1,2,3];

    let x =10;
    
    if x>2 {
        panic!("vector index out of bounds!!");
    }
    else {
        println!("{}",v[x]);
    }
}
