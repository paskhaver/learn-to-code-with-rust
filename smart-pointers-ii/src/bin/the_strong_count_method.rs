use std::rc::Rc;

#[derive(Debug)]
struct Employee {
    name: String,
}

#[derive(Debug)]
struct Team {
    name: String,
    members: Vec<Rc<Employee>>,
}

fn main() {
    let alice = Rc::new(Employee {
        name: String::from("Alice"),
    });
    println!("Strong count: {}", Rc::strong_count(&alice));

    let engineering = Team {
        name: String::from("Engineering"),
        members: vec![Rc::clone(&alice)],
    };
    println!("Strong count: {}", Rc::strong_count(&alice));

    {
        let design = Team {
            name: String::from("Design"),
            members: vec![Rc::clone(&alice)],
        };
        println!("Strong count: {}", Rc::strong_count(&alice));
    }

    println!("Strong count: {}", Rc::strong_count(&alice));
}
