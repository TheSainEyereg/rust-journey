#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
}

fn main() {
	let rect1 = Rectangle {
		width: 30,
		height: 50
	};

	println!("Rect1 is {rect1:?} and it's area is {}", Rectangle::area(&rect1));

	let rect2 = Rectangle {
		width: dbg!(rect1.width * 2),
		..rect1
	};

	println!("Rect2 is {rect2:#?} and it's area is {}", rect2.area());
}
