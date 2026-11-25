use std::fs;

trait Writer {
    fn write_to_file(&self, msg: &str);
}

struct CSVWriter {
    path: String,
}

impl Writer for CSVWriter {
    fn write_to_file(&self, msg: &str) {
        let contents = format!("ColumnHeader\n{msg}\n");
        fs::write(&self.path, contents).expect("Could not write the CSV file");
    }
}

struct Content<T: Writer> {
    writer: T,
    data: String,
}

impl<T: Writer> Content<T> {
    fn write(&self) {
        self.writer.write_to_file(&self.data);
    }
}

fn main() {
    let writer = CSVWriter {
        path: String::from("output.csv"),
    };

    let content = Content {
        writer,
        data: String::from("5"),
    };
    content.write();
    /*
        Creates output.csv containing:
        ColumnHeader
        5
    */

    println!("Done! Check output.csv for the results.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockWriter {
        sent_messages: RefCell<Vec<String>>,
    }

    impl Writer for MockWriter {
        fn write_to_file(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(msg.to_string());
        }
    }

    #[test]
    fn it_writes_content_to_writer() {
        let writer = MockWriter {
            sent_messages: RefCell::new(vec![]),
        };
        let content = Content {
            writer,
            data: String::from("Hi there"),
        };
        content.write();
        assert_eq!(
            content.writer.sent_messages.borrow().as_slice(),
            vec![String::from("Hi there")]
        )
    }
}
