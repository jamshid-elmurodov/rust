// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn new(x: T, y: T) -> Self {
//         Point { x, y }
//     }
// }

// struct Book {
//     title: String,
//     pages: u16,
// }

// trait Summary {
//     fn summarize(&self) -> String;
// }

// impl Summary for Book {
//     fn summarize(&self) -> String {
//         format!("{} has {} pages", self.title, self.pages)
//     }
// }

fn main() {
    // let pt: Point<u16> = Point::new(1, 2);

    // let book = Book {
    //     title: String::from("chess"),
    //     pages: 112,
    // };

    // println!("{}", book.summarize());
}

fn first<T>(arr: &[T]) -> &T {
    &arr[0]
}
