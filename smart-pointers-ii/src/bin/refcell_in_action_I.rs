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
