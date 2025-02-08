fn main() {
    let n = 5;
    let y = plus_one(n);
    println!("The value of y is: {y}");

    let a = Box::new([0 as usize; 1_000]);
    let b = a;

    println!("{}", b[10]);

    let name = String::from("Ferris");

    let mut name = add_suffix_borrow(name);
    add_suffix_ref(&mut name);

    let new_name = add_suffix_borrow(name.clone());

    println!("{name} and {new_name}");

    let lifetime = return_string_static();
    let lifetime_my = return_string_my();
    println!("{lifetime} {lifetime_my}");

    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    // We can't do v[1] = 5; since some element was borrowed by num
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", v);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn add_suffix_borrow(mut name: String) -> String {
    name.push_str("🚀");
    name
}

fn add_suffix_ref(name: &mut String) {
    add_sufix_ref_really(name); // References are not borrowed, so we'll still have access to them
    println!("Appended suffix to string \"{name}\"");
}

fn add_sufix_ref_really(name: &mut String) {
    name.push_str("🚀");
}

/// Wtf is lifetime
fn return_string_static() -> &'static str {
    let a: &str = "Hello";
    a
}

fn return_string_my<'a>() -> &'a str {
    let a: &str = "Hello";
    a
}
