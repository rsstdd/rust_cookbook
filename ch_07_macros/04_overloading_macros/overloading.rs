//! Implementing Overloading Macros

macro_rules! test {
    ($left:expr; and $right:expr) => (
        println!("{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    );

    ($left:expr; or $right:expr) => (
        println!("{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    );
}

fn main() {
    test!(1u32 + 1 == 2u32; and 2i32 * 2 == 4i32);
    test!(true; or false)
}
