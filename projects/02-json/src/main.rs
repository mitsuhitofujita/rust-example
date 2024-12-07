mod untyped_example;
mod typed_example;

fn main() {
    untyped_example::untyped_example().expect("Error");
    typed_example::typed_example().expect("Error")
}
