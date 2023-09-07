struct Rectangle {
    length: u32,
    width: u32
}

fn main() {
    let rect1 = Rectangle {length: 30, width: 50};
    println!(
        "The area of the rectangle is {}",
        area(&rect1)
    );
}

fn area(rectangel: &Rectangle) -> u32 {
    rectangel.length * rectangel.width
}
