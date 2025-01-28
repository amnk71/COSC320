fn main(){
    let mut x = 3;//variables are immutable by default, so we make it mutable to be able to reassign the same variable
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");}