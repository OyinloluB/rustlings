// variables2.rs
// Make me compile! Execute the command `rustlings hint variables2` if you want a hint :)

fn main() {
    // What happens if you annotate line 7 with a type annotation?
    // let x : u32;

    // What if you give x a value?
    // let x = 0;

    // What if you do both?
    let x: u32 = 0;

    // What type should x be, anyway?
    // An integer because different types cannot be compared

    if x == 10 {
        println!("Ten!");
    } else {
        println!("Not ten!");
    }
}
