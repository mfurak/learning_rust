use custom_macro::CustomMacro;
use custom_macro_derive::CustomMacro;

#[derive(CustomMacro)]
struct Customer1;

#[derive(CustomMacro)]
struct Customer2;

fn main() {
    Customer1::custom_macro();
    Customer2::custom_macro();
}
