use toolshed::Tool;

fn main() {
    toolshed::open_shop();
    let hammer = Tool::new("Hammer");
    println!("Ready to lend: {}", hammer.name);
}