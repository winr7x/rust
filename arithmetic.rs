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
}
