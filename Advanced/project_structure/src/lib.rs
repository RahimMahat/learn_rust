/// These are documentation strings with 3 forward slashes. you provide these doc comments right above the item you are documenting.
/// if you run 'cargo doc' this will generate a documentation webpage with all the documentation comments you have provided in your code. inside the target/doc/<package_name>/ path
/// Tools for inventory management
pub mod inventory;
/// Tools for orders management
pub mod orders;

pub use inventory::products::{Item, ProductCategory};
pub use inventory::{MANAGER as INVENTORY_MANAGER, talk_to_manager};
pub use orders::MANAGER as ORDERS_MANAGER;
