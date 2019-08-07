fn main() {
  // let _f0: f32 = 0; error: must be 0. or 0.0
  let _f1: f32 = 0.0;
  let _f2: f32 = 99.;
  let _f3: f64 = 17.2;
  let _f4 = -2009.9002; // default type is f64
  let _f5 = -2009.9002f32;
  let _f6 = 8_763.5f64;
  let _f7: f32 = std::f32::NAN;
  let _f8: f64 = std::f64::NAN;
  let _f9: f32 = std::f32::INFINITY;
  let _f10: f64 = std::f64::INFINITY;
  
  let f11 = 8722.5f64.ceil();
  println!("8722.5f64.ceil() == {}", f11);

  let f12 = 0.0f32.cos();
  println!("0.0f32.cos() == {}", f12);
  
  let f13 = -34.6f64.round();
  println!("-34.6f64.round() == {}", f13);
  
  let f14 = 144f32.sqrt();
  println!("144f32.sqrt() == {}", f14);
}
