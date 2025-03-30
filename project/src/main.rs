mod rect;

use rect::Rectangle;

fn main() {
    let rect1 = Rectangle::default();

    println!(
        "Rect1 is {rect1:?} and it's area is {}",
        Rectangle::area(&rect1)
    );

    let rect2 = Rectangle {
        width: dbg!(rect1.width * 2),
        ..rect1
    };

    println!("Rect2 is {rect2:#?} and it's area is {}", rect2.area());
}
