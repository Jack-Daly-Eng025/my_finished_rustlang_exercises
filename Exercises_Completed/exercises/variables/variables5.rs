// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    let mut number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let binding = 3.to_string(); // don't rename this variable
    number = &binding;
    println!("Number plus two is : {}", number.to_owned() + &2.to_string());
}
