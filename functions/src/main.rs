fn main() {
    let x = plus_five(5);
    another_function(x, 'h');
}

fn plus_five(x: i32) -> i32 {
    x + 5
}

fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
