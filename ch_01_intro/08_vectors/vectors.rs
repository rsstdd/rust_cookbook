fn main() {
    // Vectors are Dynamic arrays
    // Memory allocation can be increased or decreased @ runtime
    // Data allocated on the heap
    // Can hold any data type
    println!("***\nVectors\n*** \n");
    let mut vec1 = vec![1, 2, 3, 4, 5];

    // Printing element 3 in vector
    println!("Printing element 3 in vector");
    println!("Item 3 : {}", vec1[2]);
    println!("---- \n");

    // Iterate over vector
    // To iterate you have to take ownership with &
    println!("Iterate over vector");
    for i in &vec1 {
        println!("{}", i);
    }
    println!("---- \n");

    // Push an elem to vector
    println!("Push an elem to vector");
    vec1.push(6);
    println!("vec1.push(6); {:?}", vec1);
    println!("---- \n");

    vec1.pop();
    println!("vec1.pop(); {:?}", vec1);
    println!("---- \n");

    println!("---- \n");
}
