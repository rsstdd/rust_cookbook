fn main() {
    println!("**\nStrings\n** \n");

    // Declaring a random string
    let rand_string = "I love Rust cookbook <3";

    // printing the length of the string
    println!("Printing the length of the string");
    println!("==>");
    println!("length of the string is {}", rand_string.len());
    println!("---- \n");

    println!("Splits the string");
    println!("==>");
    let(first, second) = rand_string.split_at(6);
    println!("First part of string: {0}. Second part of string: {1}", first, second);
    println!("---- \n");

    println!("Count using iterator count");
    println!("==>");
    // Count using iterator count
    let count = rand_string.chars().count();
    let chars_of_string = rand_string.chars();
    println!("{0} && {1:?}", count, chars_of_string);
    println!("---- \n");

    // Printing All Chars
    println!("Printing All Chars");
    println!("==>");
    let mut chars = rand_string.chars();
    let mut indiv_chars = chars.next();

    loop {
        // like C++ switch
        match indiv_chars {
            // Some == x is val that exists && has type T; thus, x can be used in print
            Some(x) => println!("{}", x),
            // None == val !exists && maybe the end of string; Handle by breaking
            None => break
        }
        indiv_chars = chars.next();
    }
    println!("---- \n");

    // Iterate over whitespace
    println!("Iterate over whitespace");
    println!("==>");
    let mut iter = rand_string.split_whitespace();
    let mut indiv_word = iter.next();

    loop {
        match indiv_word {
            Some(x) => println!("{}", x),
            None => break
        }
        indiv_word = iter.next();
    }
    println!("---- \n");

    // Iterate over next line (\n)
    println!("Iterate over next line");
    println!("==>");
    let rand_string2 = "I love \neverything about \nRust <3";
    let mut iter_line = rand_string2.lines();
    let mut indiv_sent = iter_line.next();

    loop {
        match indiv_sent {
            Some(x) => println!("{}", x),
            None => break
        }

        indiv_sent = iter_line.next();
    }
    println!("---- \n");
}
