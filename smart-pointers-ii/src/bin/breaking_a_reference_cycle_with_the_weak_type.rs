use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Employee {
    name: String,
    experience: u8,
    teams: Vec<Weak<Team>>,
}

impl Employee {
    fn show_teams(&self) {
        for weak_team in self.teams.iter() {
            if let Some(strong_team) = weak_team.upgrade() {
                println!("{} is on the {} team", self.name, strong_team.name);
            }
        }
    }
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

    alice.borrow_mut().teams.push(Rc::downgrade(&engineering));

    {
        let design = Rc::new(Team {
            name: String::from("Design"),
            members: vec![Rc::clone(&alice)],
        });

        alice.borrow_mut().teams.push(Rc::downgrade(&design));
    }

    alice.borrow().show_teams();
}
