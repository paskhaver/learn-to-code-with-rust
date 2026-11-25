use std::rc::Rc;

#[derive(Debug)]
struct Employee {
    name: String,
}

struct Team {
    name: String,
    members: Vec<Rc<Employee>>,
}

fn main() {
    let alice = Rc::new(Employee {
        name: String::from("Alice"),
    });

    let engineering = Team {
        name: String::from("Engineering"),
        members: vec![Rc::clone(&alice)],
    };

    let design = Team {
        name: String::from("Design"),
        members: vec![Rc::clone(&alice)],
    };

    println!("{:#?}", engineering.members);
    println!("{:#?}", design.members);
    /*
    [
        Employee {
            name: "Alice",
        },
    ]
    [
        Employee {
            name: "Alice",
        },
    ]
    */
}
