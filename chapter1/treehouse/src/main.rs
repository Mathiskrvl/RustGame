#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

fn main() {
    let mut visitor_list = vec![
        Visitor::new("Bert", VisitorAction::Accept, 45),
        Visitor::new(
            "Steve",
            VisitorAction::AcceptWithNote {
                note: String::from("Lactose-free milk is in the fridge"),
            },
            15,
        ),
        Visitor::new("Fred", VisitorAction::Refuse, 30),
    ];
    visitor_list.push(Visitor::new("Mathis", VisitorAction::Accept, 21));
    loop {
        let name: String = what_is_ur_name();
        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
        if let Some(visitor) = known_visitor {
            visitor.greet_visitor();
        } else {
            if name.is_empty() {
                break;
            }
            visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
            println!("Vous avez était noté");
        }
    }
    println!("The final list of visitors: ");
    println!("{visitor_list:#?}");
}
#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }
    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the tree house {}", self.name),
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the tree house {}", self.name);
                println!("{note}");
                if self.age < 18 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Refuse => println!("Do not allow {} in", self.name),
        }
    }
}

fn what_is_ur_name() -> String {
    println!("Hello what your name ?");
    let mut ur_name: String = String::new();
    stdin()
        .read_line(&mut ur_name)
        .expect("Failed to read line");
    ur_name.trim().to_lowercase()
}
