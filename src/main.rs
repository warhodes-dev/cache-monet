use macroz_derive::{TypeName, prepend_task};
use macroz::TypeName;

#[derive(TypeName)]
struct Pancakes;

#[derive(TypeName)]
struct Waffles;

#[prepend_task]
fn invoke() {
    println!("This prints after")
}

fn main() {
    Pancakes::print_typename();
    Waffles::print_typename();
    invoke();
    println!("Done.")
}
