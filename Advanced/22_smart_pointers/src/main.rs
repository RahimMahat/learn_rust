fn raw_pointers() {
    let mut sushi = String::from("Yellowtail");
    let sushi_raw_pointer_1 = &raw const sushi;
    let sushi_raw_pointer_2: *const String = &sushi;
    let sushi_raw_mutable_pointer_1 = &raw mut sushi;
    let sushi_raw_mutable_pointer_2: *mut String = &mut sushi;

    unsafe {
        println!(
            "{} {} {} {} ",
            *sushi_raw_pointer_1,
            *sushi_raw_pointer_2,
            *sushi_raw_mutable_pointer_1,
            *sushi_raw_mutable_pointer_2
        );
    }
}

fn box_smart_pointer() {
    // The Box smart pointer stores a piece of data on the heap.
    // The Box is an owned type that is a container around the raw pointer that holds the momory
    // address of the allocated heap data.
    let my_box = Box::new(100);
    println!("{}", *my_box);
    // println!("{}", my_box); // this works fine
    // println!("{:?}", my_box); // this works fine
}

fn main() {
    // raw_pointers();
    box_smart_pointer();
}
