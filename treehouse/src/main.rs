#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {

    fn new(name:&str, greeting:&str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string()
        }
    }

    fn greet(&self) {
        println!("{}", self.greeting);
    }

}

fn main() {
    println!("Hello, what's your name?");
    let your_name = what_is_your_name();
    let mut visitor:Option<&Visitor> = None;
    let visitors = [
        Visitor::new("bert", "Hello Bert, enjoy your treehouse"),
        Visitor::new("steve", "Hi Steve. Your milk is in the fridge."),
        Visitor::new("fred", "Wow, who invited Fred?"),
    ];
    for v in visitors.iter() {
        if v.name == your_name {
            visitor = Option::from(v);
        }
    }
    match visitor {
        None => println!("Sorry, you aren't on the list"),
        Some(v) => v.greet(),
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name
        .trim()
        .to_lowercase()
}