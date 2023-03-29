fn main() {
    println!("Hello, world!");

    // statements vs expressions ------------------------------
    // //this is the statement
    // let y = {
    //     //these 2 lines are the expression
    //     let x = 3;
    //     x + 1 // note expression doesn't need a semi colon
    // };

    // println!("The value of y is {y}!");
    //----------------------------------------------------------
    
    
    //--------------------------------------------------
    // another_function(); // uncomment w/ basic ex
    // another_function(5); // uncomment w/ params ex 1
    //--------------------------------------------------


    //-----------------------------------------------------------------
    // print_labeled_measurement(5, 'h'); // uncomment w/ params ex 2
    //-----------------------------------------------------------------
    
    //-------------------------------------
    // function w/ returns ex 1
    // let x = five();
    // println!("The value of x is {x}!");
    //-------------------------------------

    //-------------------------------------
    // function w/ returns ex 2
    // let x = plus_one(5);
    // println!("The value of x is {x}!");
    //-------------------------------------
    
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
// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is {value}{unit_label}");
// }

//function w/ returns example
// fn five() -> i32 {
//     5 // note: no let, fn calls, macros or return...just 5
// }
//function w/ returns ex 2
// fn plus_one(x: i32) -> i32 {
//     x + 1 
//     // x + 1; // trying w/ ; gives an error and helpful compiler message
// }


