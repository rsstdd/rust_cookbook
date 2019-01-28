#[derive(Debug)]
enum Hero {
    Fast,
    Info { name: String, secret: String },
    Strong(i32),
}

fn main() {
    println!("***\nEnums\n*** \n");
    println!("Enums");

    let hulk = Hero::Strong(100);
    let fasty = Hero::Fast;

    // converting from
    let spiderman = Hero::Info {
        name: "Spiderman".to_owned(), //     ensure the string is owned when borrowed
        secret: "Peter Parker".to_owneds) // as &str is an immmutable ref to a string.
    };

    get_info(spiderman);
    get_info(hulk);
    get_info(fasty);

    println!("---- \n");
}

fn get_info(h: Hero) {
    match h {
        Hero::Fast => println!("Fast"),
        Hero::Strong(num) => println!("Lifts tons {}", num),
        Hero::Info { name, secret } => {
            println!("Name is {0}; Secret is {1}", name, secret);
        },
    }
}
