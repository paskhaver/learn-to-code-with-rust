use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Employee {
    name: String,
    experience: u8,
    teams: Vec<Rc<Team>>,
}

impl Drop for Employee {
    fn drop(&mut self) {
        println!("Dropping employee {}", self.name);
    }
}

#[derive(Debug)]
struct Team {
    name: String,
    members: Vec<Rc<RefCell<Employee>>>,
}

impl Drop for Team {
    fn drop(&mut self) {
        println!("Dropping team {}", self.name);
    }
}

fn main() {
    let alice = Rc::new(RefCell::new(Employee {
        name: String::from("Alice"),
        experience: 3,
        teams: vec![],
    }));

    let engineering = Rc::new(Team {
        name: String::from("Engineering"),
        members: vec![Rc::clone(&alice)],
    });

    let design = Rc::new(Team {
        name: String::from("Design"),
        members: vec![Rc::clone(&alice)],
    });

    alice.borrow_mut().teams.push(Rc::clone(&engineering));
    alice.borrow_mut().teams.push(Rc::clone(&design));

    println!("Alice strong count: {}", Rc::strong_count(&alice));
    println!(
        "Engineering strong count: {}",
        Rc::strong_count(&engineering)
    );
    println!("Design strong count: {}", Rc::strong_count(&design));

    // Never finishes because of the cycle:
    // engineering -> alice -> engineering -> alice -> ...
    // Don't run this, it will not work
    // println!("{engineering:#?}");
}
