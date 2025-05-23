fn main() {
    // variables
    // let mut x = 3;
    // x += 2;
    // let x = x * 10;
    // let x = "result";
    // println!("{x}");
    // println!("{}", add(12223, 12223));
    // println!("{}", square(2))

    // if else statement
    // let num = 7;
    // if num == 7 {
    //     println!("CR7");
    // } else if num == 10 {
    //     println!("Pessi");
    // } else {
    //     println!("Idk who is he");
    // }

    // loop, while, for
    // let mut i = 0;
    // loop {
    //     println!("{}", i);
    //     if i == 7 {
    //         break;
    //     }

    //     i += 1;
    // }
    //
    // let mut i = 0;
    // while i < 5 {
    //     println!("{}", i);
    //     i += 1;
    // }
    //
    // for i in 1..=10 {
    //     println!("{}", i);
    // }

    // Ownership & Borrowing
    // let name = String::from("Jamshid");
    // let name2 = name.clone();

    // println!("{name}, {name2}")
    //
    // let mut s = String::from("World");
    // update(&mut s);
    // println!("{s}")
    //
    let name = String::from("Jamshid");
    println!("{}", len_of(&name));
}

fn len_of(s: &String) -> usize {
    s.len()
}

fn update(s: &mut String) {
    s.push_str(" hello");
}

// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn square(num: i32) -> i32 {
//     num * num
// }
