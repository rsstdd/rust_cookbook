//! Structs within Modules: creating a nested_mod module

mod sample_struct {
    println!("A pub struct with a public field of generic Type `T`");
    // A pub struct with a public field of generic Type `T`
    pub struct WhiteBox<T> {
        pub information: T,
    }

    // A public struct w/ a private feild of generic Type `T`
    #[allow(dead_code)]
    pub struct BlackBox<T> {
        information: T,
    }

    impl WhiteBox {
        pub fn get_info(&self) -> String {
            self.information
        }
    }

    impl<T> BlackBox<T> {
        // A public constructor method
        pub fn const_new(information: T) -> BlackBox<T> {
            BlackBox {
                information: information,
            }
        }
    }
}

fn main() {
    println!("***\nStructs within Modules\n*** \n");

    // Pub struct w/pub feild
    // let white_box = sample_struct::WhiteBox { information: "Pub info \n" };
    let white_box = sample_struct::WhiteBox { information: "Pub info \n" };

    // And their fields can be normally accessed
    // println!("the white box contains => {}", white_box.get_info());

    // Public structs with private feilds cannon be constructed using feild names
    // Error! `BlackBox` has private fields
    // let black_box = sample_struct::BlackBox::const_new("classified info \n");

    // and the private feilds of a public struct cannot bne accessed
    // Error! The `information` field is private
    // println!("The black_box contains {:?}", black_box.information);

    println!("---- \n");
}
