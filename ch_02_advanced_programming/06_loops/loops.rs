
fn main() {
    println!("***\nLoop\n*** \n");
    // Loop
    // mutable var
    println!("Loop even nums");
    let mut x = 1;
    loop {
        if x % 2 == 0 {
            println!("{:?}", x);
            x += 1;
            continue;
        }

        if x > 10 {
            break;
        }

        x += 1;
    }
    println!("---- \n");

    // While loop
    println!("While");
    println!("while 1..9");
    let mut y = 1;
    while y < 10 {
        println!("{:?}", y);
        y += 1;
    }
    println!("---- \n");

    // For loop
    println!("For");
    println!("For 1..9");
    let z = 1;
    for z in 1..10 {
        println!("{}", z);
    }
    println!("---- \n");
}
