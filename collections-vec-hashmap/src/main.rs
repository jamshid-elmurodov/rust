use std::collections::HashMap;

fn main() {
    // let vec = vec![3, 7, 2, 8];

    // for n in vec.iter() {
    //     println!("{}", n * n)
    // }

    // if let Some(max) = find_max(&vec![1, -4, 5, 2, 8, 6]) {
    //     println!("Max: {}", max);
    // } else {
    //     println!("Vec is empty")
    // }

    let mut map: HashMap<String, u8> = HashMap::new();

    map.insert(String::from("jamshid"), 16);
    map.insert(String::from("Toshmat"), 19);

    println!("{}", map.contains_key("Jamshid"));

    map.entry(String::from("ali")).or_insert(11);
}

// fn find_max(vec: &Vec<i32>) -> Option<i32> {
//     if vec.is_empty() {
//         return None;
//     }

//     Some(*vec.iter().max().unwrap())
// }
