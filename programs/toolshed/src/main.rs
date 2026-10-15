use toolshed::Tool;
use front_desk::checkout_tool;

fn main() {
    toolshed::open_shop();
    let hammer = Tool::new("Hammer");
    println!("Ready to lend: {}", hammer.name);
}