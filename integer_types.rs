fn main() {
  let _v1: i8 = -128; // _ - unused variable
  let _v2: i16 = -32768;
  let _v3: i32 = -2_147_483_648;
  // 9 quintillions 223  quadrillions ...
  let _v4: i64 = -9_223_372_036_854_775_808;
  // 127  undecillions 141 decillions 183 nonillions 460 octillions
  // 469 septillions 231 sextillions 731 quintillions 687 quadrillions ...
  let _v5: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;
  // i32 or i64 (-9_223_372_036_854_775_808)
  let _v6: isize = -2_147_483_648;
  
  let _v7: u8 = 255;
  let _v8: u16 = 65535;
  let _v9: u32 = 4_294_967_295;
  let _v10: u64 = 18_446_744_073_709_551_615;
  let _v11: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
  // u32 or u64 (18_446_744_073_709_551_615)
  let _v12: usize = 4_294_967_295;
  
  let _byte_i: u8 = b'i';
  let _byte_x = b'x';
  assert_eq!(_byte_i, 105); // ASCII code 105
  assert_eq!(_byte_x, 120); // ASCII code 120
}
