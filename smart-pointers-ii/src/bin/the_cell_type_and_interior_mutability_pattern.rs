use std::cell::Cell;

#[derive(Debug)]
struct ConcertTicket {
    section: String,
    seat: String,
    scanned: Cell<bool>,
}

impl ConcertTicket {
    fn new(section: String, seat: String) -> Self {
        Self {
            section,
            seat,
            scanned: Cell::new(false),
        }
    }

    fn admit_attendee(&self) {
        self.scanned.set(true);
    }
}

fn main() {
    let ticket = ConcertTicket::new(String::from("A"), String::from("3"));
    println!("{}", ticket.scanned.get());

    ticket.admit_attendee();
    println!("{}", ticket.scanned.get());
}
