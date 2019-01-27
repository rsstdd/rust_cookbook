fn main() {
    println!("**\nDeclaring arrays and using slices\n** \n");

    // Defining an array
    println!("Defining an array");
    println!("==>");
    let rand_array = [1, 2, 3, 4, 5];
    println!("random array {:?}", rand_array);
    println!("---- \n");

    // indexing starts with 0
    println!("indexing starts with 0");
    println!("random array [0] is {}", rand_array[0]);
    println!("---- \n");
    println!("random array length is {}", rand_array.len());
    println!("---- \n");
    // last two elements
    println!("Selecting a range of an array: slices");

    // Arrays are borrowed via slices where we mention the
    // Slices reference a contiguous sequence of the element of the arr
    // instead of the whole element
    println!("random array {:?}", &rand_array[1..3]);
    println!("---- \n");
}
