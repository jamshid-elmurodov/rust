fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6];

    // let new = vec.iter().map(|a| a * 3).collect::<Vec<i32>>();
    // println!("{:?}", new);

    // let filtered = vec.into_iter().filter(|n| n > &0).collect::<Vec<i32>>();

    let new = vec.iter().fold(0, |sum, n| sum + n);

    println!("{}", new)
}
