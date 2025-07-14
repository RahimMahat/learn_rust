/*
  In this section we'll be covering the project structure in Rust.
*/

/* Packages & Crates:
 Our previous Rust programs all lived in a single file. When we run the cargo new command, we
 create a Rust package.
 A 'package' is a folder with Cargo.toml file. The Cargo.toml file holds metadata about the
 package like it's name and version. we can say A package is a collection of one or more crates.
 A 'crate' is a collection of Rust code that produces an executable or a library
 A crate is the smallest amount of code that the Rust compiler considers at a time.
 A 'binary crate' is a carte that compiles to an executable. A binary crate has a main function
 that is the entrypoint for the executable.
 A 'library crate' exports functionality for other Rust programs to share and use. A library
 crate does not have a main function and does not compile to be an executable program.
 Module: A module is an organizational container that encapsulates related code.
*/
// For this section we will be creating a project called "warehouse" to explore the Rust project concepts.
// Our project:
//      The cargo new command will default to creating a package with binary crate.
//      The Cargo.toml file's name field set the name of the package, in our case 'warehouse'.
//      Cargo will look for a src/main.rs file. If it exists, Rust infers that we have a binary crate named warehouse.
//      Cargo will look for a src/lib.rs file. If it exists, Rust infers that we have a library crate name warehouse.

mod inventory {
    const FLOOR_SPACE: i32 = 10_000;
    pub const MANAGER: &str = "Ivan Inventory";

    #[derive(Debug)]
    enum ProductCategory {
        Ladder,
        Hammer,
    }

    #[derive(Debug)]
    struct Item {
        name: String,
        category: ProductCategory,
        quantity: u32,
    }

    fn talk_to_manager() {
        println!("Hey {MANAGER}, How's the code.");
    }
}

fn main() {
    // println!("The manager of our inventory is {}",MANAGER); // this will not work because the
    // constant Manager lives in a different module and inaccessible
    // println!("The manager of our inventory is {}",inventory::MANAGER); // this will also not work
    // because the Manager variable in inventory module is private.
    // We have to explicitly define every construct that we want to access outside of module as public using 'pub' keyword.
    println!("The manager of our inventory is {}", inventory::MANAGER);
}
