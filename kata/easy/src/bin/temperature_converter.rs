fn convert(theta: f64, unit: &str) -> f64 {
    if unit == "C" {
        return 1.8 * theta + 32.0;
    } else {
        return (theta - 32.0) / 1.8;
    }
}

fn main() {
    // enter temp. value plus unit to convert to other unit.
    let theta = 32.0;
    let unit = "F";
    let result = convert(theta, unit);
    let alt_unit = if unit == "C" { "F" } else { "C" };

    println!("{theta:.1} °{unit} = {result:.1} °{alt_unit}");
}