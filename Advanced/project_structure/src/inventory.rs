pub const FLOOR_SPACE: i32 = 10_000;
pub const MANAGER: &str = "Ivan Inventory";

pub fn talk_to_manager() {
    println!("Hey {MANAGER}, How's the code.");
}

// declaring a submodule, again you can create these submodule with the 3 options that we created
// modules with i.e inline, file and folder (sub)modules
// Inline
// inventory/products.rs - file submodule
// inventory/products/mod.rs - folder submodule
// for this example we've create file submodule and calling it in the inventory module.
// to access it outside of this module the products submodule has to be made public.
pub mod products;
