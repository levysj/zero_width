extern crate zero_width;

use zero_width::zero_width_fun;

fn main() {
    let temp = String::from("ABC");
    let test = zero_width_fun::to_zero_width(&temp);
    println!("ABCD{}EFG", test);

    let ret = match zero_width_fun::from_zero_width(&String::from("a")) {
        Ok(r) => r,
        Err(e) => String::from(e.description()),
    };

    println!("{}", ret);
    println!("{} {}", zero_width_fun::detect_zero_width(&test), zero_width_fun::detect_zero_width(&temp));
    println!("{}", zero_width::embbed_zero_width(&temp, &temp, &temp));
}

