/*
  In this section we'll be covering the project structure in Rust.
*/

// For this section we will be creating a project called "warehouse" to explore the Rust project concepts.
// Our project:
//      The cargo new command will default to creating a package with binary crate.
//      The Cargo.toml file's name field set the name of the package, in our case 'warehouse'.
//      Cargo will look for a src/main.rs file. If it exists, Rust infers that we have a binary crate named warehouse.
//      Cargo will look for a src/lib.rs file. If it exists, Rust infers that we have a library crate name warehouse.

mod inventory;
mod orders;

fn main() {
    // println!("The manager of our inventory is {}",MANAGER); // this will not work because the
    // constant Manager lives in a different module and inaccessible
    // println!("The manager of our inventory is {}",inventory::MANAGER); // this will also not work
    // because the Manager variable in inventory module is private.
    // We have to explicitly define every construct that we want to access outside of module as public using 'pub' keyword.
    println!("The manager of our inventory is {}", inventory::MANAGER);
    // another advantage modules is that it allows for duplicate names within different namespace
    println!("The manager of our orders is {}", orders::MANAGER);
}
