// struct Book {
//     title: String,
//     pages: u32,
// }

// impl Book {
//     fn print_info(&self) -> String {
//         format!("{} has {} pages", self.title, self.pages)
//     }
// }

// enum Status {
//     Active,
//     Banned,
//     Inactive,
// }

fn main() {
    // let book = Book {
    //     title: String::from("The Rust book"),
    //     pages: 333,
    // };

    // println!("{}", book.print_info())

    // let st = Status::Active;

    // match st {
    //     Status::Active => println!("Online"),
    //     Status::Banned => println!("Blocked"),
    //     Status::Inactive => println!("Offline"),
    // }

    if let Some(result) = safe_divide(0, 0) {
        println!("{}", result);
    }
}

fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }

    Some(a / b)
}
