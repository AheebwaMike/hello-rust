pub fn checkout_tool(tool_name: &str) {
    println!("Checked out: {tool_name}");
}

fn log_transaction(tool_name: &str) {
    println!("[log] {tool_name} left the building");
}

pub mod checkout;
