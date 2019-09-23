use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[macro_export]
macro_rules! bad_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    let v: Vec<u32> = bad_vec![1,2,3];

    println!("yes: {:?}", v);

    Pancakes::hello_macro();
}
