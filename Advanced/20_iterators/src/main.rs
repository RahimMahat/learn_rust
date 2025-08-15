fn main() {
    let numbers = vec![4, 8, 15, 16, 23, 42];
    let iterator = numbers.into_iter();

    for number in iterator {
        println!("{number}");
    }
}
