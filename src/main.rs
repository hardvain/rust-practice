fn main() {

  // 
  let _unused_integer = 0;
  let copyable_integer = 5;
  let copied_from_copyable_integer = copyable_integer;
  println!("copyable_integer can be used here: {}", copyable_integer);
  println!("copied_from_copyable_integer can also be used here: {}", copied_from_copyable_integer);
  let incremented_copyable_integer = copyable_integer + 1;
  println!("incremented_copyable_integer : {}", incremented_copyable_integer);
  println!("copyable_integer still has the same value: {}", copyable_integer);
  
  // copyable_integer can be re defined which will shadow the old definition
  let copyable_integer = 4;
  // copyable_integer cannot just be re assigned alone because bindings are mutable by default
  // copyable_integer = 1;

  // Effect of scopes on copyable values
  let root_scoped_value = 1;
  {
    println!("root scoped value inside beginning of first inner scope {}", root_scoped_value);
    let mut first_inner_scoped_value = root_scoped_value;
    println!("root scoped value after assignment to another binding {}", root_scoped_value);
    println!("first_inner_scoped_value {}", first_inner_scoped_value);
    first_inner_scoped_value = first_inner_scoped_value + 1;
    println!("root scoped value after mutation to assigned binding {}", root_scoped_value);
    println!("first_inner_scoped_value after mutation {}", first_inner_scoped_value);
  }

  println!("root scoped value at root scope {}", root_scoped_value);

  // Effect of immutable bindings on passing to functions

}

/*
  1. Bindings
    1. Immutable Bindings
    2. Mutable Bindings
  2. Scopes
    1. Current Scope
    2. Inner Scope
    3. Outer Scope
  3. Calling Member functions on variables
    1. Functions that accept by value
    2. Functions that accept by shared reference
    3. Functions that accept by mutable reference
  4. Calling Functions on
    1. Owned values
      1. That accept by value
      2. That accept by shared reference
      3. That accept by mutable reference
    2. Shared Reference values
      1. That accept by value
      2. That accept by shared reference
      3. That accept by mutable reference
    3. Mutable reference values
      1. That accept by value
      2. That accept by shared reference
      3. That accept by mutable reference
  5. Closures
  6. Pattern Matching
  7. Lifetimes
*/