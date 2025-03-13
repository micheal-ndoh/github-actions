// Inner attribute applied to an entire crate
#![allow(unused_variables)]
// Outer attribute applied to a struct
#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

fn main() {
    let user = User {
        name: String::from("Alice"),
        age: 30,
    };

             let mich = String::from("Hello");
        println!("{:?}", user);



    let num = 1..20;

    for number in num {
        println!("Hello number {}", number);
    }
}

fn find_value(vec: &[i32], target: i32) -> Option<i32> {
        for &value in vec {
        if value == target {
            return Some(value);
        }
    }
    None
}
