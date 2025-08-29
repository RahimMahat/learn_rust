use std::cmp::Ordering;
use std::error::Error;
use std::fs;
use std::ops::{Deref, DerefMut};

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

fn linked_list() {
    #[derive(Debug)]
    enum LinkedList<T> {
        Empty,
        Node { value: T, next: Box<LinkedList<T>> },
    }
    let list = LinkedList::Node {
        value: 1,
        next: Box::new(LinkedList::Node {
            value: 2,
            next: Box::new(LinkedList::Node {
                value: 3,
                next: Box::new(LinkedList::Empty),
            }),
        }),
    };
    println!("{:?}", list);

    let im_with_you = LinkedList::Node {
        value: String::from("I'm With You"),
        next: Box::new(LinkedList::Empty),
    };
    let sk8r_boi = LinkedList::Node {
        value: String::from("Sk8r Boi"),
        next: Box::new(im_with_you),
    };
    let complicated = LinkedList::Node {
        value: String::from("Complicated"),
        next: Box::new(sk8r_boi),
    };

    println!("{:#?}", complicated);
}

fn truee() {
    #[derive(Debug)]
    enum FileSystemEntity {
        File {
            name: String,
        },
        Folder {
            name: String,
            content: Vec<FileSystemEntity>,
        },
    }

    let rust_file = FileSystemEntity::File {
        name: String::from("main.rs"),
    };
    let python_file = FileSystemEntity::File {
        name: String::from("main.py"),
    };
    let code_folder = FileSystemEntity::Folder {
        name: String::from("Code stuff"),
        content: vec![rust_file, python_file],
    };
    let screenplay = FileSystemEntity::File {
        name: String::from("My screenplay.txt"),
    };
    let all_documents = FileSystemEntity::Folder {
        name: String::from("Documents"),
        content: vec![screenplay, code_folder],
    };

    println!("{:#?}", all_documents);
}

fn binary_search_tree() {
    #[derive(Debug)]
    enum BinarySearchTree {
        Empty,
        Node {
            value: i32,
            left: Box<BinarySearchTree>,
            right: Box<BinarySearchTree>,
        },
    }

    impl BinarySearchTree {
        fn new() -> Self {
            BinarySearchTree::Empty
        }

        fn insert(&mut self, new_value: i32) {
            match self {
                BinarySearchTree::Empty => {
                    *self = BinarySearchTree::Node {
                        value: new_value,
                        left: Box::new(BinarySearchTree::Empty),
                        right: Box::new(BinarySearchTree::Empty),
                    }
                }
                BinarySearchTree::Node { value, left, right } => match new_value.cmp(value) {
                    Ordering::Equal => (),
                    Ordering::Less => left.insert(new_value),
                    Ordering::Greater => right.insert(new_value),
                },
            }
        }

        fn contains(&self, target: i32) -> bool {
            match self {
                BinarySearchTree::Empty => false,
                BinarySearchTree::Node { value, left, right } => match target.cmp(value) {
                    Ordering::Equal => true,
                    Ordering::Less => left.contains(target),
                    Ordering::Greater => right.contains(target),
                },
            }
        }
    }

    let mut binary_search_tree = BinarySearchTree::new();
    binary_search_tree.insert(5);
    binary_search_tree.insert(7);
    binary_search_tree.insert(1);
    binary_search_tree.insert(8);
    binary_search_tree.insert(1);
    binary_search_tree.insert(14);
    println!("Binary Search Tree:\n{:#?}", binary_search_tree);

    println!("tree contains 8 : {}", binary_search_tree.contains(8));
}

fn deref_and_deref_mut_traits() {
    struct CustomBox<T> {
        data: T,
    }

    impl<T> CustomBox<T> {
        fn new(data: T) -> Self {
            Self { data }
        }
    }

    impl<T> Deref for CustomBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    impl<T> DerefMut for CustomBox<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.data
        }
    }

    let mut custom_boxy = CustomBox::new(std::f64::consts::PI);
    println!("PI {}", *custom_boxy);
    *custom_boxy *= 2.0;
    println!("PI double {}", *custom_boxy);
}

fn trait_objects() {
    // A trait object is an instance of some type that implements a specific trait.
    trait Wearable {
        fn wear(&self) -> String;
    }

    #[derive(Debug)]
    struct Pants {
        fabric: String,
        waist_size: u32,
    }

    impl Wearable for Pants {
        fn wear(&self) -> String {
            format!("{} {} pants", self.fabric, self.waist_size)
        }
    }

    #[derive(Debug)]
    struct Tie {
        color: String,
    }

    impl Wearable for Tie {
        fn wear(&self) -> String {
            format!("{} tie", self.color)
        }
    }

    let pants = Pants {
        fabric: "Cotton".to_string(),
        waist_size: 34,
    };

    let tie = Tie {
        color: "Black".to_string(),
    };

    let outfit: Vec<Box<dyn Wearable>> = vec![Box::new(pants), Box::new(tie)];

    let items: Vec<String> = outfit.into_iter().map(|item| item.wear()).collect();
    println!("{:#?}", items);

    ////////////////////////////////////////////////////////////////////////////////

    fn read_number_from_file(path: &str) -> Result<i32, Box<dyn Error>> {
        let file_contents = fs::read_to_string(path)?;
        let parsed_number = file_contents.parse::<i32>()?;
        Ok(parsed_number)
    }

    let path = "notes.txt";
    match read_number_from_file(path) {
        Ok(value) => println!("The number is {value}"),
        Err(error) => eprintln!("The error is {error}"),
    }
}

fn main() {
    // raw_pointers();
    // box_smart_pointer();
    // linked_list();
    // truee();
    // binary_search_tree();
    // deref_and_deref_mut_traits();
    // trait_objects();
}
