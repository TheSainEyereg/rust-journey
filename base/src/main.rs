fn tuple_test(to: Option<u8>) -> (u8, u8, String) {
    (to.unwrap_or(10), 2, "Hi funny crab".to_string())
}

/// Returns a random number
fn get_random() -> usize {
    257 // random enough
}

fn main() {
    let mut lamo: u8 = get_random() as u8;
    {
        let lmao = &mut lamo;
        *lmao += 2;
    }
    println!("{lamo}");

    println!("{}", 5. / 2.);

    println!("{}", (0b00110010_u8));

    let array = [0 as u8; 8];

    let x = {
        let y = 2;
        y + 1
    };

    println!("x is {x} and array.len is {}", array.len());

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                println!("Sorry, we're breaking up");
                break 'counting_up;
                // println!(":("); // This would not be executed
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let (to, from, text) = tuple_test(None);

    for _ in from..to {
        println!("{text}");
    }
}
