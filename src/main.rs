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


    // testing out sort_by_key
    // sort_by_key needs a closure with an FnMut trait since it calls it multiple times, once for every elt in the list
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 2, height: 3 },
        Rectangle { width: 8, height: 7 },
    ];
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1; // We must make sure to not use closures with the FnOnce trait that captures values
        r.width
    });
    println!("{list:#?} sorted in {num_sort_operations} operations");
}
