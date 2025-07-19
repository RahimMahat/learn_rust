// declaring a submodule, again you can create these submodule with the 3 options that we created
// modules with i.e inline, file and folder (sub)modules
// Inline
// inventory/products.rs - file submodule
// inventory/products/mod.rs - folder submodule
// for this example we've create file submodule and calling it in the inventory module.
// to access it outside of this module the products submodule has to be made public.
pub mod products;

// you can use the 'pub use' keyword to export items from submodule eg. pub use products::{Items,
// ProductCategory}; and then in create root we don't have to reach into products submodule to get these
// items as they will be available in the inventory module itself.

pub const FLOOR_SPACE: i32 = 10_000;
pub const MANAGER: &str = "Ivan Inventory";

pub fn talk_to_manager() {
    println!("Hey {MANAGER}, we have {FLOOR_SPACE} sqkm of floor space.");
}

// There are 2 ways you can specify the path to a module
// An absolute path is the full, complete path to a name starting from the crate root. eg. crate::inventory::MANAGER
// A relatvie path is the path to a name starting from the current location/module eg.MANAGER.
