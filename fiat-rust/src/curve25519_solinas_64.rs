//! Autogenerated: 'src/ExtractionOCaml/solinas_reduction' --lang Rust --inline curve25519_solinas 64 '2^255 - 19' mul
//! curve description: curve25519_solinas
//! machine_wordsize = 64 (from "64")
//! requested operations: mul
//! s-c = 2^255 - [(1, 19)] (from "2^255 - 19")
//!
//! Computed values:
//!

#![allow(unused_parens)]
#![allow(non_camel_case_types)]

pub type fiat_curve25519_solinas_u1 = u8;
pub type fiat_curve25519_solinas_i1 = i8;
pub type fiat_curve25519_solinas_u2 = u8;
pub type fiat_curve25519_solinas_i2 = i8;


/// The function fiat_curve25519_solinas_addcarryx_u64 is an addition with carry.
///
/// Postconditions:
///   out1 = (arg1 + arg2 + arg3) mod 2^64
///   out2 = ⌊(arg1 + arg2 + arg3) / 2^64⌋
///
/// Input Bounds:
///   arg1: [0x0 ~> 0x1]
///   arg2: [0x0 ~> 0xffffffffffffffff]
///   arg3: [0x0 ~> 0xffffffffffffffff]
/// Output Bounds:
///   out1: [0x0 ~> 0xffffffffffffffff]
///   out2: [0x0 ~> 0x1]
#[inline]
pub fn fiat_curve25519_solinas_addcarryx_u64(out1: &mut u64, out2: &mut fiat_curve25519_solinas_u1, arg1: fiat_curve25519_solinas_u1, arg2: u64, arg3: u64) -> () {
  let x1: u128 = (((arg1 as u128) + (arg2 as u128)) + (arg3 as u128));
  let x2: u64 = ((x1 & (0xffffffffffffffff as u128)) as u64);
  let x3: fiat_curve25519_solinas_u1 = ((x1 >> 64) as fiat_curve25519_solinas_u1);
  *out1 = x2;
  *out2 = x3;
}

/// The function fiat_curve25519_solinas_subborrowx_u64 is a subtraction with borrow.
///
/// Postconditions:
///   out1 = (-arg1 + arg2 + -arg3) mod 2^64
///   out2 = -⌊(-arg1 + arg2 + -arg3) / 2^64⌋
///
/// Input Bounds:
///   arg1: [0x0 ~> 0x1]
///   arg2: [0x0 ~> 0xffffffffffffffff]
///   arg3: [0x0 ~> 0xffffffffffffffff]
/// Output Bounds:
///   out1: [0x0 ~> 0xffffffffffffffff]
///   out2: [0x0 ~> 0x1]
#[inline]
pub fn fiat_curve25519_solinas_subborrowx_u64(out1: &mut u64, out2: &mut fiat_curve25519_solinas_u1, arg1: fiat_curve25519_solinas_u1, arg2: u64, arg3: u64) -> () {
  let x1: i128 = (((arg2 as i128) - (arg1 as i128)) - (arg3 as i128));
  let x2: fiat_curve25519_solinas_i1 = ((x1 >> 64) as fiat_curve25519_solinas_i1);
  let x3: u64 = ((x1 & (0xffffffffffffffff as i128)) as u64);
  *out1 = x3;
  *out2 = (((0x0 as fiat_curve25519_solinas_i2) - (x2 as fiat_curve25519_solinas_i2)) as fiat_curve25519_solinas_u1);
}

/// The function fiat_curve25519_solinas_mulx_u64 is a multiplication, returning the full double-width result.
///
/// Postconditions:
///   out1 = (arg1 * arg2) mod 2^64
///   out2 = ⌊arg1 * arg2 / 2^64⌋
///
/// Input Bounds:
///   arg1: [0x0 ~> 0xffffffffffffffff]
///   arg2: [0x0 ~> 0xffffffffffffffff]
/// Output Bounds:
///   out1: [0x0 ~> 0xffffffffffffffff]
///   out2: [0x0 ~> 0xffffffffffffffff]
#[inline]
pub fn fiat_curve25519_solinas_mulx_u64(out1: &mut u64, out2: &mut u64, arg1: u64, arg2: u64) -> () {
  let x1: u128 = ((arg1 as u128) * (arg2 as u128));
  let x2: u64 = ((x1 & (0xffffffffffffffff as u128)) as u64);
  let x3: u64 = ((x1 >> 64) as u64);
  *out1 = x2;
  *out2 = x3;
}

/// The function fiat_curve25519_solinas_mul multiplies two field elements.
///
/// Postconditions:
///   eval out1 mod 57896044618658097711785492504343953926634992332820282019728792003956564819949 = (eval arg1 * eval arg2) mod 57896044618658097711785492504343953926634992332820282019728792003956564819949
///
/// Input Bounds:
///   arg1: [[0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff]]
///   arg2: [[0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff]]
/// Output Bounds:
///   out1: [[0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff]]
#[inline]
pub fn fiat_curve25519_solinas_mul(out1: &mut [u64; 4], arg1: &[u64; 4], arg2: &[u64; 4]) -> () {
  let mut x1: u64 = 0;
  let mut x2: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x1, &mut x2, (arg1[3]), (arg2[3]));
  let mut x3: u64 = 0;
  let mut x4: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x3, &mut x4, (arg1[3]), (arg2[2]));
  let mut x5: u64 = 0;
  let mut x6: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x5, &mut x6, (arg1[3]), (arg2[1]));
  let mut x7: u64 = 0;
  let mut x8: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x7, &mut x8, (arg1[3]), (arg2[0]));
  let mut x9: u64 = 0;
  let mut x10: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x9, &mut x10, (arg1[2]), (arg2[3]));
  let mut x11: u64 = 0;
  let mut x12: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x11, &mut x12, (arg1[2]), (arg2[2]));
  let mut x13: u64 = 0;
  let mut x14: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x13, &mut x14, (arg1[2]), (arg2[1]));
  let mut x15: u64 = 0;
  let mut x16: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x15, &mut x16, (arg1[2]), (arg2[0]));
  let mut x17: u64 = 0;
  let mut x18: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x17, &mut x18, (arg1[1]), (arg2[3]));
  let mut x19: u64 = 0;
  let mut x20: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x19, &mut x20, (arg1[1]), (arg2[2]));
  let mut x21: u64 = 0;
  let mut x22: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x21, &mut x22, (arg1[1]), (arg2[1]));
  let mut x23: u64 = 0;
  let mut x24: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x23, &mut x24, (arg1[1]), (arg2[0]));
  let mut x25: u64 = 0;
  let mut x26: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x25, &mut x26, (arg1[0]), (arg2[3]));
  let mut x27: u64 = 0;
  let mut x28: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x27, &mut x28, (arg1[0]), (arg2[2]));
  let mut x29: u64 = 0;
  let mut x30: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x29, &mut x30, (arg1[0]), (arg2[1]));
  let mut x31: u64 = 0;
  let mut x32: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x31, &mut x32, (arg1[0]), (arg2[0]));
  let mut x33: u64 = 0;
  let mut x34: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x33, &mut x34, 0x0, x28, x7);
  let mut x35: u64 = 0;
  let mut x36: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x35, &mut x36, x34, x26, x5);
  let x37: u64 = ((x36 as u64) + x18);
  let mut x38: u64 = 0;
  let mut x39: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x38, &mut x39, 0x0, x33, x13);
  let mut x40: u64 = 0;
  let mut x41: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x40, &mut x41, x39, x35, x8);
  let mut x42: u64 = 0;
  let mut x43: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x42, &mut x43, x41, x37, (0x0 as u64));
  let x44: u64 = ((x43 as u64) + x10);
  let mut x45: u64 = 0;
  let mut x46: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x45, &mut x46, 0x0, x30, x15);
  let mut x47: u64 = 0;
  let mut x48: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x47, &mut x48, x46, x38, x16);
  let mut x49: u64 = 0;
  let mut x50: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x49, &mut x50, x48, x40, x11);
  let mut x51: u64 = 0;
  let mut x52: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x51, &mut x52, x50, x42, x3);
  let mut x53: u64 = 0;
  let mut x54: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x53, &mut x54, x52, x44, (0x0 as u64));
  let x55: u64 = ((x54 as u64) + x2);
  let mut x56: u64 = 0;
  let mut x57: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x56, &mut x57, 0x0, x45, x21);
  let mut x58: u64 = 0;
  let mut x59: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x58, &mut x59, x57, x47, x19);
  let mut x60: u64 = 0;
  let mut x61: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x60, &mut x61, x59, x49, x14);
  let mut x62: u64 = 0;
  let mut x63: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x62, &mut x63, x61, x51, x6);
  let mut x64: u64 = 0;
  let mut x65: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x64, &mut x65, x63, x53, (0x0 as u64));
  let mut x66: u64 = 0;
  let mut x67: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x66, &mut x67, x65, x55, (0x0 as u64));
  let mut x68: u64 = 0;
  let mut x69: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x68, &mut x69, 0x0, x32, x23);
  let mut x70: u64 = 0;
  let mut x71: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x70, &mut x71, x69, x56, x24);
  let mut x72: u64 = 0;
  let mut x73: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x72, &mut x73, x71, x58, x22);
  let mut x74: u64 = 0;
  let mut x75: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x74, &mut x75, x73, x60, x17);
  let mut x76: u64 = 0;
  let mut x77: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x76, &mut x77, x75, x62, x9);
  let mut x78: u64 = 0;
  let mut x79: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x78, &mut x79, x77, x64, x1);
  let mut x80: u64 = 0;
  let mut x81: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x80, &mut x81, x79, x66, (0x0 as u64));
  let mut x82: u64 = 0;
  let mut x83: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x82, &mut x83, 0x0, x68, x29);
  let mut x84: u64 = 0;
  let mut x85: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x84, &mut x85, x83, x70, x27);
  let mut x86: u64 = 0;
  let mut x87: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x86, &mut x87, x85, x72, x25);
  let mut x88: u64 = 0;
  let mut x89: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x88, &mut x89, x87, x74, x20);
  let mut x90: u64 = 0;
  let mut x91: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x90, &mut x91, x89, x76, x12);
  let mut x92: u64 = 0;
  let mut x93: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x92, &mut x93, x91, x78, x4);
  let mut x94: u64 = 0;
  let mut x95: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x94, &mut x95, x93, x80, (0x0 as u64));
  let mut x96: u64 = 0;
  let mut x97: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x96, &mut x97, 0x26, x92);
  let mut x98: u64 = 0;
  let mut x99: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x98, &mut x99, 0x26, x90);
  let mut x100: u64 = 0;
  let mut x101: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x100, &mut x101, 0x26, x88);
  let mut x102: u64 = 0;
  let mut x103: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x102, &mut x103, 0x0, x82, x98);
  let mut x104: u64 = 0;
  let mut x105: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x104, &mut x105, x103, x84, x96);
  let mut x106: u64 = 0;
  let mut x107: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x106, &mut x107, 0x26, x94);
  let mut x108: u64 = 0;
  let mut x109: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x108, &mut x109, x105, x86, x106);
  let mut x110: u64 = 0;
  let mut x111: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x110, &mut x111, 0x26, x94);
  let x112: u64 = ((x109 as u64) + x111);
  let mut x113: u64 = 0;
  let mut x114: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x113, &mut x114, 0x0, x31, x100);
  let mut x115: u64 = 0;
  let mut x116: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x115, &mut x116, x114, x102, x101);
  let mut x117: u64 = 0;
  let mut x118: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x117, &mut x118, x116, x104, x99);
  let mut x119: u64 = 0;
  let mut x120: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x119, &mut x120, x118, x108, x97);
  let x121: u64 = ((x120 as u64) + x112);
  let mut x122: u64 = 0;
  let mut x123: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x122, &mut x123, 0x26, x121);
  let mut x124: u64 = 0;
  let mut x125: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x124, &mut x125, 0x0, x113, x122);
  let mut x126: u64 = 0;
  let mut x127: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x126, &mut x127, x125, x115, (0x0 as u64));
  let mut x128: u64 = 0;
  let mut x129: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x128, &mut x129, x127, x117, (0x0 as u64));
  let mut x130: u64 = 0;
  let mut x131: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x130, &mut x131, x129, x119, (0x0 as u64));
  let mut x132: u64 = 0;
  let mut x133: u64 = 0;
  fiat_curve25519_solinas_mulx_u64(&mut x132, &mut x133, 0x26, x131);
  let mut x134: u64 = 0;
  let mut x135: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x134, &mut x135, 0x0, x124, x132);
  let mut x136: u64 = 0;
  let mut x137: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x136, &mut x137, x135, x126, (0x0 as u64));
  let mut x138: u64 = 0;
  let mut x139: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x138, &mut x139, x137, x128, (0x0 as u64));
  let mut x140: u64 = 0;
  let mut x141: fiat_curve25519_solinas_u1 = 0;
  fiat_curve25519_solinas_addcarryx_u64(&mut x140, &mut x141, x139, x130, (0x0 as u64));
  out1[0] = x134;
  out1[1] = x136;
  out1[2] = x138;
  out1[3] = x140;
}
