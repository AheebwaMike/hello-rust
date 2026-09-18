pub mod front_desk;

pub fn open_shop() {
    front_desk::checkout_tool("hammer");

    let hammer = inventory::Tool::new("Hammer");
}

pub mod inventory {
    pub struct Tool {
        pub name: String,
        condition: u8 // private
    }

    impl Tool {
        pub fn new(name: &str) -> Tool{
            Tool {
                name: name.to_string(),
                condition: 100
            }
        }
    }
}

use front_desk::checkout_tool;
pub use inventory::Tool;
