fn main() {
    println!("Hello, world!");
    
    // another_function(); // uncomment w/ basic ex
    // another_function(5); // uncomment w/ params ex 1
    
    print_labeled_measurement(5, 'h'); // uncomment w/ params ex 2
}
// initial basic example
// fn another_function() {
//     println!("Another function!");
// }

//parameters example
// fn another_function(x:i32) {
//     println!("The value of x is {x}");
// }

//params ex 2
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}

