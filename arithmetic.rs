fn main() {
  assert_eq!((2 + 7 - 1) / 4, 2);

  assert_eq!(17 / 5, 3);
  assert_eq!(19 / 5, 3);
  assert_eq!(-17 / 5, -3);
  assert_eq!(-19 / 5, -3);
  
  assert_eq!(17 % 5, 2);
  assert_eq!(19 % 5, 4);
  assert_eq!(-17 % 5, -2);
  assert_eq!(-19 % 5, -4);
  
  assert_eq!(0b1011 << 2, 0b101100);
  assert_eq!(0x10 >> 2, 4);
  
  assert_eq!(!0b1u8, 0b11111110u8);
  assert_eq!(!254u8, 1); // not 0 as in C++
  
  assert_eq!(0b1010 | 0b0100, 0b1110);
  assert_eq!(0b1011 & 0b0101, 0b0001);
  assert_eq!(0b1011 ^ 0b0101, 0b1110);
  
  assert_eq!((-67i16).abs(), 67);
  assert_eq!((0x1Fi32).count_ones(), 5); // 0b11111
  
  let x: u16 = 767;
  let y1: u32 = x.into(); // mismatched types error without '.into()'
                          // .into() - expansion without loss of accuracy
  let y2: u8 = x as u8;
  assert_eq!(767, y1);
  assert_eq!(255, y2);
  
  let _v1: u8 = 0;
  // let _v2 = _v1 - 1; // in debug mode will cause error:
                        // thread 'main' panicked at 'attempt to subtract with overflow'
  let _v3 = i32::max_value(); // in debug mode will cause error:
  // let _v4 = _v3 + 1;       // thread 'main' panicked at 'attempt to add with overflow'
  
  let v4 = i32::max_value();
  let v5: u8 = 0;
  let v6 = v4.wrapping_add(1); // normal overflow behavior
  let v7 = v5.wrapping_sub(1); // normal overflow behavior
  assert_eq!(v6, i32::min_value());
  assert_eq!(v7, 255);
  
  let v8 = v4.saturating_add(1); // do not change value if overflow
  let v9 = v5.saturating_sub(1); // do not change value if overflow
  assert_eq!(v8, i32::max_value());
  assert_eq!(v9, 0);

  // normal overflow behavior with bool result: overflowed or not
  let (v10, overflowed) = v4.overflowing_add(1);
  assert!(overflowed);
  assert_eq!(v10, i32::min_value());
  let (v11, overflowed) = v5.overflowing_sub(1);
  assert!(overflowed);
  assert_eq!(v11, 255);
  
  match 255u8.checked_add(1) {
      Some(result) => println!("not overflowed and result is {}", result),
      None => println!("overflowed"),
  }
}
