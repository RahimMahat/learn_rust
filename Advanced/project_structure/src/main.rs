/*
  In this section we'll be covering the project structure in Rust.
*/

// For this section we will be creating a project called "warehouse" to explore the Rust project concepts.
// Our project:
//      The cargo new command will default to creating a package with binary crate.
//      The Cargo.toml file's name field set the name of the package, in our case 'warehouse'.
//      Cargo will look for a src/main.rs file. If it exists, Rust infers that we have a binary crate named warehouse.
//      Cargo will look for a src/lib.rs file. If it exists, Rust infers that we have a library crate name warehouse.

// Importing convention is:
// 1. Imports from Rust's standard library.
// 2. External crate Imports
// 3. local module Imports.

mod inventory;
mod orders;

// The 'use' keyword brings a name into the current scope. It creates a shortcut to a name in a nested module.
// to use multiple names within a module you use {} with comma separated names
use inventory::products::{Item, ProductCategory};
// importing the names with aliases.
use inventory::{MANAGER as INVENTORY_MANAGER, talk_to_manager};
use orders::MANAGER as ORDERS_MANAGER;

fn main() {
    // println!("The manager of our inventory is {}",MANAGER); // this will not work because the
    // constant Manager lives in a different module and inaccessible
    // println!("The manager of our inventory is {}",inventory::MANAGER); // this will also not work
    // because the Manager variable in inventory module is private.
    // We have to explicitly define every construct that we want to access outside of module as public using 'pub' keyword.
    println!("The manager of our inventory is {}", INVENTORY_MANAGER);
    // another advantage modules is that it allows for duplicate names within different namespace
    println!("The manager of our orders is {}", ORDERS_MANAGER);

    // Accesing other public items.
    talk_to_manager();
    // For an enum when you define it as public all of it's variant become public and can be
    // accessible, however that's not the case with struct
    let favourite_cateogory = ProductCategory::Hammer;
    println!("My favourite category of item is {favourite_cateogory:?}");
    // Here while accessing the struct we'll get field is private error even though the struct is
    // public the fields within the struct in a module are private. to resolve it we have to mark
    // all the fields that we want to be public with pub keyword back in inventory module.
    let tall_ladder = Item {
        name: String::from("Ladder Omega"),
        category: ProductCategory::Ladder,
        quantity: 100,
    };
    println!("tall ladder is {:#?}", tall_ladder);
    // using an associated function to create a struct instance.
    let short_ladder = Item::new("Ladder Beta".to_string(), ProductCategory::Ladder, 50);
    println!("short ladder is {:#?}", short_ladder);
}
