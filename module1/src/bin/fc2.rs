// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
    let val = false;
    let msg = match val {
        true => "it's true",
        false => "it's false",
    };
    println!("{msg}");
}
