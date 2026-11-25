#[derive(Debug)]
struct Employee {
    name: String,
}

#[derive(Debug)]
struct Team {
    name: String,
    members: Vec<Employee>,
}

fn main() {
    let alice = Employee {
        name: String::from("Alice"),
    };

    let engineering = Team {
        name: String::from("Engineering"),
        members: vec![alice],
    };

    // Uncommenting this code causes a compile error:
    // error[E0382]: use of moved value: `alice`
    // let design = Team {
    //     name: String::from("Design"),
    //     members: vec![alice],
    // };
}
