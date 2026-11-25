use std::cell::RefCell;

#[derive(Debug)]
struct ConcertTicket {
    section: String,
    seat: String,
    scanned: bool,
}

impl ConcertTicket {
    fn new(section: String, seat: String) -> Self {
        Self {
            section,
            seat,
            scanned: false,
        }
    }
}

fn main() {
    let ticket = RefCell::new(ConcertTicket::new(String::from("AA"), String::from("13")));

    ticket.borrow_mut().seat = String::from("D");
    println!("{}", ticket.borrow().seat); // D

    ticket.borrow_mut().seat = String::from("K");
    println!("{}", ticket.borrow().seat); // K

    println!("{ticket:#?}");
    /*
        RefCell {
        value: ConcertTicket {
            section: "AA",
            seat: "K",
            scanned: false,
        },
    }
    */
}
