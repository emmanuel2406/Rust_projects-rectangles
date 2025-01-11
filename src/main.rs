use rectangles::Rectangle;

fn main() {
    let scale = 2;

    let rect1 = Rectangle {
        width: dbg!(scale * 30),
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        // rather give reference to struct, since we don't want to take ownership
        rect1.area()
    );
    // :? for print, and :#? for pretty print (with newlines added)
    println!("rect1 is {rect1:#?}"); // prints to stdout
    // dbg!(&rect1); // prints to stderr but uses pretty print by default
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
