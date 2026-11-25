use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Employee {
    name: String,
    experience: u8,
}

#[derive(Debug)]
struct Team {
    name: String,
    members: Vec<Rc<RefCell<Employee>>>,
}

fn main() {
    let alice = Rc::new(RefCell::new(Employee {
        name: String::from("Alice"),
        experience: 3,
    }));

    let engineering = Team {
        name: String::from("Engineering"),
        members: vec![alice.clone()],
    };

    let design = Team {
        name: String::from("Design"),
        members: vec![alice.clone()],
    };

    for member in &engineering.members {
        member.borrow_mut().experience += 1;
    }

    println!("{engineering:#?}");
    println!("{design:#?}");

    /*
        Team {
        name: "Engineering",
        members: [
            RefCell {
                value: Employee {
                    name: "Alice",
                    experience: 4,
                },
            },
        ],
    }
    Team {
        name: "Design",
        members: [
            RefCell {
                value: Employee {
                    name: "Alice",
                    experience: 4,
                },
            },
        ],
    }
    */
}
