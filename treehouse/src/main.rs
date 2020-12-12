#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8
}

impl Visitor {

    fn new(name:&str, action:VisitorAction, age:i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age
        }
    }

    fn greet(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the tree house, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the tree house, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            },
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name)
        }
    }

}

fn main() {

    let mut visitors = vec![
        Visitor::new("bert", VisitorAction::Accept, 45),
        Visitor::new("steve", VisitorAction::AcceptWithNote {note: String::from("Lactose-free milk is in the fridge")}, 15),
        Visitor::new("fred", VisitorAction::Refuse, 30),
    ];

    loop {

        println!("Hello, what's your name?");
        let name = what_is_your_name();

        let visitor = visitors
            .iter()
            .find(|v| v.name == name);

        match visitor {
            None => {
                if name.is_empty() {
                    break;
                }
                println!("{} is not on the visitor list.", name);
                visitors.push(Visitor::new(&name, VisitorAction::Probation, 0))
            }
            Some(v) => v.greet(),
        }

    }

    println!("The final list of visitors:");
    println!("{:#?}", visitors);

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