fn main() {
  let b1: bool = true;
  println!("b1 == {}", b1);
  
  // let b2: bool = 999_888_777i32; // error: expected bool, found i32
  
  let b3: bool = 999_888_777i64 != 0;
  println!("b3 = {}", b3);
  
  // let b4: i64 = b1; error: expected i64, found bool
  
  let b5: i64 = b1 as i64;
  println!("b5 = {}", b5);
  
  let b6: bool = always_true() || always_false(); // always_false() is not invoked
  println!("b6 == {}", b6);
}

fn always_false() -> bool {
    println!("always_false");
    return false;
}

fn always_true() -> bool {
    println!("always_true");
    return true;
}
