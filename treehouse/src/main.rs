use std::io::stdin;
use crate::VisitorAction::{Accept, AcceptWithNote, Refuse, Probation};

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote {
        note: String
    },
    Probation,
    Refuse
}

impl Visitor{

    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Visitor {
            name: name.to_lowercase(),
            action,
            age
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            Accept => println!("Welcome to the treehouse {}", self.name),
            AcceptWithNote{note} => {
                println!("Welcome to the treehouse {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            },
            Probation => println!("{} is now a probationary member", self.name),
            Refuse => println!("Do not allow {} in!", self.name)
        }
    }

}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("bert", Accept, 45),
        Visitor::new("steve", AcceptWithNote {
            note: String::from("Lactose-free milk is in the fridge")
        }, 15),
        Visitor::new("fred", Refuse, 40)
    ];

    println!("Hello, what's your name? (Leave empty and press ENTER to quit");

    loop {
        let name = what_is_your_name();
        if name.trim().is_empty() {
            break;
        }
        let known_visitor = visitor_list.iter()
            .find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                println!("You are not on the visitor list.");
                visitor_list.push(Visitor::new(&name, Probation, 0))
            }
        }
    }

    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);

}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line.");
    your_name
        .trim()
        .to_lowercase()
}