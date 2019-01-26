fn main() {
  println!("Undersanding Assignment");

  // Compiler figues out Type if not Specified
  let num = 10;
  println!("Num is {}", num);

  // 32 bit integer
  let age: i32 = 40;
  println!("Age is {}", age);

  // Max and min of a 32 bit integer
  // (Will not compile example) //
  // println!("Max i32 {}", i32::MAX);
  // println!("Min i32 {}", i32::MIN);

  // Multiple variable assignment
  // Printing: Specify order of printed variables
  let(f_name, l_name) = ("viki", "dangers");
  println!("First name {0} and last name {1}", f_name, l_name);
}
