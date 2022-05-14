#[allow(unused_variables)] // So that there are no unused var warnings
#[allow(unused_assignments)] // No unused assignment warnings, either

fn main(){
  // basic variable declaration
  let some_data: bool = true;

  // basic mutable variable declaration and assignment
  let mut some_other_data: bool = true;
  // reassignment
  some_other_data = false;

  // **INTEGERS**
  // `i` below stands for integer
  // `8` stands for "8 bits of memory"
  // `i8` has enough memory to store 256 possible
  // integers, including zero (2 to the 8th power) from
  // -128 to +127
  let some_integer: i8 = 16;

  // To see these values...
  println!("Max is: {}", std::i8::MAX);
  println!("Min is: {}", std::i8::MIN)
}