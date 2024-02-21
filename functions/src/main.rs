fn main() {
    let y = plus_two(7);

    println!("The value of y is: {}", y);
}

fn plus_two(x: i32) -> i32 {
    println!("This function adds two!");
    x + 2
}