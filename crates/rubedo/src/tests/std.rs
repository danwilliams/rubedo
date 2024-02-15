#![allow(non_snake_case)]

//		Packages

use super::*;
use crate::sugar::s;
use rust_decimal::prelude::*;



//		Tests

//§		AsStr																	
#[cfg(test)]
mod as_str {
	use super::*;
	
	//		as_str																
	#[test]
	fn as_str__string() {
		let foo = s!("Test");
		assert_eq!(foo.as_str(), "Test");
	}
	#[test]
	fn as_str__str() {
		let foo = "Test";
		assert_eq!(foo.as_str(), "Test");
	}
}

//§		FromIntWithScale														
#[cfg(test)]
mod from_int_with_scale {
	use super::*;
	
	//		from_int_with_scale													
	#[test]
	fn from_int_with_scale__f32_success_scale_0() {
		assert_eq!(f32::from_int_with_scale(  123_i8,   0), Some(  123_f32));
		assert_eq!(f32::from_int_with_scale(1_234_i16,  0), Some(1_234_f32));
		assert_eq!(f32::from_int_with_scale(1_234_i32,  0), Some(1_234_f32));
		assert_eq!(f32::from_int_with_scale(1_234_i64,  0), Some(1_234_f32));
		assert_eq!(f32::from_int_with_scale(1_234_i128, 0), Some(1_234_f32));
		assert_eq!(f32::from_int_with_scale(  123_u8,   0), Some(  123_f32));
		assert_eq!(f32::from_int_with_scale(1_234_u16,  0), Some(1_234_f32));
		assert_eq!(f32::from_int_with_scale(1_234_u32,  0), Some(1_234_f32));
		assert_eq!(f32::from_int_with_scale(1_234_u64,  0), Some(1_234_f32));
		assert_eq!(f32::from_int_with_scale(1_234_u128, 0), Some(1_234_f32));
	}
	#[test]
	fn from_int_with_scale__f64_success_scale_0() {
		assert_eq!(f64::from_int_with_scale(  123_i8,   0), Some(  123_f64));
		assert_eq!(f64::from_int_with_scale(1_234_i16,  0), Some(1_234_f64));
		assert_eq!(f64::from_int_with_scale(1_234_i32,  0), Some(1_234_f64));
		assert_eq!(f64::from_int_with_scale(1_234_i64,  0), Some(1_234_f64));
		assert_eq!(f64::from_int_with_scale(1_234_i128, 0), Some(1_234_f64));
		assert_eq!(f64::from_int_with_scale(  123_u8,   0), Some(  123_f64));
		assert_eq!(f64::from_int_with_scale(1_234_u16,  0), Some(1_234_f64));
		assert_eq!(f64::from_int_with_scale(1_234_u32,  0), Some(1_234_f64));
		assert_eq!(f64::from_int_with_scale(1_234_u64,  0), Some(1_234_f64));
		assert_eq!(f64::from_int_with_scale(1_234_u128, 0), Some(1_234_f64));
	}
	#[test]
	fn from_int_with_scale__decimal_success_scale_0() {
		assert_eq!(Decimal::from_int_with_scale(  123_i8,   0), Some(Decimal::from(  123)));
		assert_eq!(Decimal::from_int_with_scale(1_234_i16,  0), Some(Decimal::from(1_234)));
		assert_eq!(Decimal::from_int_with_scale(1_234_i32,  0), Some(Decimal::from(1_234)));
		assert_eq!(Decimal::from_int_with_scale(1_234_i64,  0), Some(Decimal::from(1_234)));
		assert_eq!(Decimal::from_int_with_scale(1_234_i128, 0), Some(Decimal::from(1_234)));
		assert_eq!(Decimal::from_int_with_scale(  123_u8,   0), Some(Decimal::from(  123)));
		assert_eq!(Decimal::from_int_with_scale(1_234_u16,  0), Some(Decimal::from(1_234)));
		assert_eq!(Decimal::from_int_with_scale(1_234_u32,  0), Some(Decimal::from(1_234)));
		assert_eq!(Decimal::from_int_with_scale(1_234_u64,  0), Some(Decimal::from(1_234)));
		assert_eq!(Decimal::from_int_with_scale(1_234_u128, 0), Some(Decimal::from(1_234)));
	}
	#[test]
	fn from_int_with_scale__f32_success_scale_2() {
		assert_eq!(f32::from_int_with_scale(  123_i8,   2), Some( 1.23_f32));
		assert_eq!(f32::from_int_with_scale(1_234_i16,  2), Some(12.34_f32));
		assert_eq!(f32::from_int_with_scale(1_234_i32,  2), Some(12.34_f32));
		assert_eq!(f32::from_int_with_scale(1_234_i64,  2), Some(12.34_f32));
		assert_eq!(f32::from_int_with_scale(1_234_i128, 2), Some(12.34_f32));
		assert_eq!(f32::from_int_with_scale(  123_u8,   2), Some( 1.23_f32));
		assert_eq!(f32::from_int_with_scale(1_234_u16,  2), Some(12.34_f32));
		assert_eq!(f32::from_int_with_scale(1_234_u32,  2), Some(12.34_f32));
		assert_eq!(f32::from_int_with_scale(1_234_u64,  2), Some(12.34_f32));
		assert_eq!(f32::from_int_with_scale(1_234_u128, 2), Some(12.34_f32));
	}
	#[test]
	fn from_int_with_scale__f64_success_scale_2() {
		assert_eq!(f64::from_int_with_scale(  123_i8,   2), Some( 1.23_f64));
		assert_eq!(f64::from_int_with_scale(1_234_i16,  2), Some(12.34_f64));
		assert_eq!(f64::from_int_with_scale(1_234_i32,  2), Some(12.34_f64));
		assert_eq!(f64::from_int_with_scale(1_234_i64,  2), Some(12.34_f64));
		assert_eq!(f64::from_int_with_scale(1_234_i128, 2), Some(12.34_f64));
		assert_eq!(f64::from_int_with_scale(  123_u8,   2), Some( 1.23_f64));
		assert_eq!(f64::from_int_with_scale(1_234_u16,  2), Some(12.34_f64));
		assert_eq!(f64::from_int_with_scale(1_234_u32,  2), Some(12.34_f64));
		assert_eq!(f64::from_int_with_scale(1_234_u64,  2), Some(12.34_f64));
		assert_eq!(f64::from_int_with_scale(1_234_u128, 2), Some(12.34_f64));
	}
	#[test]
	fn from_int_with_scale__decimal_success_scale_2() {
		assert_eq!(Decimal::from_int_with_scale(  123_i8,   2), Some(Decimal::from_str( "1.23").unwrap()));
		assert_eq!(Decimal::from_int_with_scale(1_234_i16,  2), Some(Decimal::from_str("12.34").unwrap()));
		assert_eq!(Decimal::from_int_with_scale(1_234_i32,  2), Some(Decimal::from_str("12.34").unwrap()));
		assert_eq!(Decimal::from_int_with_scale(1_234_i64,  2), Some(Decimal::from_str("12.34").unwrap()));
		assert_eq!(Decimal::from_int_with_scale(1_234_i128, 2), Some(Decimal::from_str("12.34").unwrap()));
		assert_eq!(Decimal::from_int_with_scale(  123_u8,   2), Some(Decimal::from_str( "1.23").unwrap()));
		assert_eq!(Decimal::from_int_with_scale(1_234_u16,  2), Some(Decimal::from_str("12.34").unwrap()));
		assert_eq!(Decimal::from_int_with_scale(1_234_u32,  2), Some(Decimal::from_str("12.34").unwrap()));
		assert_eq!(Decimal::from_int_with_scale(1_234_u64,  2), Some(Decimal::from_str("12.34").unwrap()));
		assert_eq!(Decimal::from_int_with_scale(1_234_u128, 2), Some(Decimal::from_str("12.34").unwrap()));
	}
	#[test]
	fn from_int_with_scale__f32_max_scale_0() {
		assert_eq!(f32::from_int_with_scale(  i8::MAX, 0), Some(  0x7F_i8  as f32));
		assert_eq!(f32::from_int_with_scale( i16::MAX, 0), Some(0x7FFF_i16 as f32));
		assert_eq!(f32::from_int_with_scale( i32::MAX, 0), None);
		assert_eq!(f32::from_int_with_scale( i64::MAX, 0), None);
		assert_eq!(f32::from_int_with_scale(i128::MAX, 0), None);
		assert_eq!(f32::from_int_with_scale(  u8::MAX, 0), Some(  0xFF_u8  as f32));
		assert_eq!(f32::from_int_with_scale( u16::MAX, 0), Some(0xFFFF_u16 as f32));
		assert_eq!(f32::from_int_with_scale( u32::MAX, 0), None);
		assert_eq!(f32::from_int_with_scale( u64::MAX, 0), None);
		assert_eq!(f32::from_int_with_scale(u128::MAX, 0), None);
	}
	#[test]
	fn from_int_with_scale__f64_max_scale_0() {
		assert_eq!(f64::from_int_with_scale(  i8::MAX, 0), Some(       0x7F_i8  as f64));
		assert_eq!(f64::from_int_with_scale( i16::MAX, 0), Some(     0x7FFF_i16 as f64));
		assert_eq!(f64::from_int_with_scale( i32::MAX, 0), Some(0x7FFF_FFFF_i32 as f64));
		assert_eq!(f64::from_int_with_scale( i64::MAX, 0), None);
		assert_eq!(f64::from_int_with_scale(i128::MAX, 0), None);
		assert_eq!(f64::from_int_with_scale(  u8::MAX, 0), Some(       0xFF_u8  as f64));
		assert_eq!(f64::from_int_with_scale( u16::MAX, 0), Some(     0xFFFF_u16 as f64));
		assert_eq!(f64::from_int_with_scale( u32::MAX, 0), Some(0xFFFF_FFFF_u32 as f64));
		assert_eq!(f64::from_int_with_scale( u64::MAX, 0), None);
		assert_eq!(f64::from_int_with_scale(u128::MAX, 0), None);
	}
	#[test]
	fn from_int_with_scale__decimal_max_scale_0() {
		assert_eq!(Decimal::from_int_with_scale(  i8::MAX, 0), Some(Decimal::from(                  0x7F_i8)));
		assert_eq!(Decimal::from_int_with_scale( i16::MAX, 0), Some(Decimal::from(                0x7FFF_i16)));
		assert_eq!(Decimal::from_int_with_scale( i32::MAX, 0), Some(Decimal::from(           0x7FFF_FFFF_i32)));
		assert_eq!(Decimal::from_int_with_scale( i64::MAX, 0), Some(Decimal::from( 0x7FFF_FFFF_FFFF_FFFF_i64)));
		assert_eq!(Decimal::from_int_with_scale(i128::MAX, 0), None);
		assert_eq!(Decimal::from_int_with_scale(  u8::MAX, 0), Some(Decimal::from(                 0xFF_u8)));
		assert_eq!(Decimal::from_int_with_scale( u16::MAX, 0), Some(Decimal::from(               0xFFFF_u16)));
		assert_eq!(Decimal::from_int_with_scale( u32::MAX, 0), Some(Decimal::from(          0xFFFF_FFFF_u32)));
		assert_eq!(Decimal::from_int_with_scale( u64::MAX, 0), Some(Decimal::from(0xFFFF_FFFF_FFFF_FFFF_u64)));
		assert_eq!(Decimal::from_int_with_scale(u128::MAX, 0), None);
	}
	#[test]
	fn from_int_with_scale__f32_scale_overflow() {
		assert_eq!(f32::from_int_with_scale(  123_i8,   10), None);
		assert_eq!(f32::from_int_with_scale(1_234_i16,  10), None);
		assert_eq!(f32::from_int_with_scale(1_234_i32,  10), None);
		assert_eq!(f32::from_int_with_scale(1_234_i64,  10), None);
		assert_eq!(f32::from_int_with_scale(1_234_i128, 10), None);
		assert_eq!(f32::from_int_with_scale(  123_u8,   10), None);
		assert_eq!(f32::from_int_with_scale(1_234_u16,  10), None);
		assert_eq!(f32::from_int_with_scale(1_234_u32,  10), None);
		assert_eq!(f32::from_int_with_scale(1_234_u64,  10), None);
		assert_eq!(f32::from_int_with_scale(1_234_u128, 10), None);
	}
	#[test]
	fn from_int_with_scale__f64_scale_overflow() {
		assert_eq!(f64::from_int_with_scale(  123_i8,   20), None);
		assert_eq!(f64::from_int_with_scale(1_234_i16,  20), None);
		assert_eq!(f64::from_int_with_scale(1_234_i32,  20), None);
		assert_eq!(f64::from_int_with_scale(1_234_i64,  20), None);
		assert_eq!(f64::from_int_with_scale(1_234_i128, 20), None);
		assert_eq!(f64::from_int_with_scale(  123_u8,   20), None);
		assert_eq!(f64::from_int_with_scale(1_234_u16,  20), None);
		assert_eq!(f64::from_int_with_scale(1_234_u32,  20), None);
		assert_eq!(f64::from_int_with_scale(1_234_u64,  20), None);
		assert_eq!(f64::from_int_with_scale(1_234_u128, 20), None);
	}
	#[test]
	fn from_int_with_scale__decimal_scale_overflow() {
		assert_eq!(Decimal::from_int_with_scale(  123_i8,   29), None);
		assert_eq!(Decimal::from_int_with_scale(1_234_i16,  29), None);
		assert_eq!(Decimal::from_int_with_scale(1_234_i32,  29), None);
		assert_eq!(Decimal::from_int_with_scale(1_234_i64,  29), None);
		assert_eq!(Decimal::from_int_with_scale(1_234_i128, 29), None);
		assert_eq!(Decimal::from_int_with_scale(  123_u8,   29), None);
		assert_eq!(Decimal::from_int_with_scale(1_234_u16,  29), None);
		assert_eq!(Decimal::from_int_with_scale(1_234_u32,  29), None);
		assert_eq!(Decimal::from_int_with_scale(1_234_u64,  29), None);
		assert_eq!(Decimal::from_int_with_scale(1_234_u128, 29), None);
	}
}

//§		ToIntWithScale															
#[cfg(test)]
mod to_int_with_scale {
	use super::*;
	
	//		to_int_with_scale													
	#[test]
	fn to_int_with_scale__f32_success_scale_0() {
		assert_eq!(  123_f32.to_int_with_scale(0), Some(  123_i8));
		assert_eq!(1_234_f32.to_int_with_scale(0), Some(1_234_i16));
		assert_eq!(1_234_f32.to_int_with_scale(0), Some(1_234_i32));
		assert_eq!(1_234_f32.to_int_with_scale(0), Some(1_234_i64));
		assert_eq!(1_234_f32.to_int_with_scale(0), Some(1_234_i128));
		assert_eq!(  123_f32.to_int_with_scale(0), Some(  123_u8));
		assert_eq!(1_234_f32.to_int_with_scale(0), Some(1_234_u16));
		assert_eq!(1_234_f32.to_int_with_scale(0), Some(1_234_u32));
		assert_eq!(1_234_f32.to_int_with_scale(0), Some(1_234_u64));
		assert_eq!(1_234_f32.to_int_with_scale(0), Some(1_234_u128));
	}
	#[test]
	fn to_int_with_scale__f64_success_scale_0() {
		assert_eq!(  123_f64.to_int_with_scale(0), Some(  123_i8));
		assert_eq!(1_234_f64.to_int_with_scale(0), Some(1_234_i16));
		assert_eq!(1_234_f64.to_int_with_scale(0), Some(1_234_i32));
		assert_eq!(1_234_f64.to_int_with_scale(0), Some(1_234_i64));
		assert_eq!(1_234_f64.to_int_with_scale(0), Some(1_234_i128));
		assert_eq!(  123_f64.to_int_with_scale(0), Some(  123_u8));
		assert_eq!(1_234_f64.to_int_with_scale(0), Some(1_234_u16));
		assert_eq!(1_234_f64.to_int_with_scale(0), Some(1_234_u32));
		assert_eq!(1_234_f64.to_int_with_scale(0), Some(1_234_u64));
		assert_eq!(1_234_f64.to_int_with_scale(0), Some(1_234_u128));
	}
	#[test]
	fn to_int_with_scale__decimal_success_scale_0() {
		assert_eq!(Decimal::from(  123).to_int_with_scale(0), Some(  123_i8));
		assert_eq!(Decimal::from(1_234).to_int_with_scale(0), Some(1_234_i16));
		assert_eq!(Decimal::from(1_234).to_int_with_scale(0), Some(1_234_i32));
		assert_eq!(Decimal::from(1_234).to_int_with_scale(0), Some(1_234_i64));
		assert_eq!(Decimal::from(1_234).to_int_with_scale(0), Some(1_234_i128));
		assert_eq!(Decimal::from(  123).to_int_with_scale(0), Some(  123_u8));
		assert_eq!(Decimal::from(1_234).to_int_with_scale(0), Some(1_234_u16));
		assert_eq!(Decimal::from(1_234).to_int_with_scale(0), Some(1_234_u32));
		assert_eq!(Decimal::from(1_234).to_int_with_scale(0), Some(1_234_u64));
		assert_eq!(Decimal::from(1_234).to_int_with_scale(0), Some(1_234_u128));
	}
	#[test]
	fn to_int_with_scale__f32_success_scale_2() {
		assert_eq!( 1.23_f32.to_int_with_scale(2), Some(  123_i8));
		assert_eq!(12.34_f32.to_int_with_scale(2), Some(1_234_i16));
		assert_eq!(12.34_f32.to_int_with_scale(2), Some(1_234_i32));
		assert_eq!(12.34_f32.to_int_with_scale(2), Some(1_234_i64));
		assert_eq!(12.34_f32.to_int_with_scale(2), Some(1_234_i128));
		assert_eq!( 1.23_f32.to_int_with_scale(2), Some(  123_u8));
		assert_eq!(12.34_f32.to_int_with_scale(2), Some(1_234_u16));
		assert_eq!(12.34_f32.to_int_with_scale(2), Some(1_234_u32));
		assert_eq!(12.34_f32.to_int_with_scale(2), Some(1_234_u64));
		assert_eq!(12.34_f32.to_int_with_scale(2), Some(1_234_u128));
	}
	#[test]
	fn to_int_with_scale__f64_success_scale_2() {
		assert_eq!( 1.23_f64.to_int_with_scale(2), Some(  123_i8));
		assert_eq!(12.34_f64.to_int_with_scale(2), Some(1_234_i16));
		assert_eq!(12.34_f64.to_int_with_scale(2), Some(1_234_i32));
		assert_eq!(12.34_f64.to_int_with_scale(2), Some(1_234_i64));
		assert_eq!(12.34_f64.to_int_with_scale(2), Some(1_234_i128));
		assert_eq!( 1.23_f64.to_int_with_scale(2), Some(  123_u8));
		assert_eq!(12.34_f64.to_int_with_scale(2), Some(1_234_u16));
		assert_eq!(12.34_f64.to_int_with_scale(2), Some(1_234_u32));
		assert_eq!(12.34_f64.to_int_with_scale(2), Some(1_234_u64));
		assert_eq!(12.34_f64.to_int_with_scale(2), Some(1_234_u128));
	}
	#[test]
	fn to_int_with_scale__decimal_success_scale_2() {
		assert_eq!(Decimal::from_str( "1.23").unwrap().to_int_with_scale(2), Some(  123_i8));
		assert_eq!(Decimal::from_str("12.34").unwrap().to_int_with_scale(2), Some(1_234_i16));
		assert_eq!(Decimal::from_str("12.34").unwrap().to_int_with_scale(2), Some(1_234_i32));
		assert_eq!(Decimal::from_str("12.34").unwrap().to_int_with_scale(2), Some(1_234_i64));
		assert_eq!(Decimal::from_str("12.34").unwrap().to_int_with_scale(2), Some(1_234_i128));
		assert_eq!(Decimal::from_str( "1.23").unwrap().to_int_with_scale(2), Some(  123_u8));
		assert_eq!(Decimal::from_str("12.34").unwrap().to_int_with_scale(2), Some(1_234_u16));
		assert_eq!(Decimal::from_str("12.34").unwrap().to_int_with_scale(2), Some(1_234_u32));
		assert_eq!(Decimal::from_str("12.34").unwrap().to_int_with_scale(2), Some(1_234_u64));
		assert_eq!(Decimal::from_str("12.34").unwrap().to_int_with_scale(2), Some(1_234_u128));
	}
	#[test]
	fn to_int_with_scale__f32_max_scale_0() {
		assert_eq!({ let i: Option<i8>   = (2u32.pow(24) as f32).to_int_with_scale(0); i}, None);
		assert_eq!({ let i: Option<i16>  = (2u32.pow(24) as f32).to_int_with_scale(0); i}, None);
		assert_eq!({ let i: Option<i32>  = (2u32.pow(24) as f32).to_int_with_scale(0); i}, Some(0x0100_0000_i32));
		assert_eq!({ let i: Option<i64>  = (2u32.pow(24) as f32).to_int_with_scale(0); i}, Some(0x0100_0000_i64));
		assert_eq!({ let i: Option<i128> = (2u32.pow(24) as f32).to_int_with_scale(0); i}, Some(0x0100_0000_i128));
		assert_eq!({ let i: Option<u8>   = (2u32.pow(24) as f32).to_int_with_scale(0); i}, None);
		assert_eq!({ let i: Option<u16>  = (2u32.pow(24) as f32).to_int_with_scale(0); i}, None);
		assert_eq!({ let i: Option<u32>  = (2u32.pow(24) as f32).to_int_with_scale(0); i}, Some(0x0100_0000_u32));
		assert_eq!({ let i: Option<u64>  = (2u32.pow(24) as f32).to_int_with_scale(0); i}, Some(0x0100_0000_u64));
		assert_eq!({ let i: Option<u128> = (2u32.pow(24) as f32).to_int_with_scale(0); i}, Some(0x0100_0000_u128));
	}
	#[test]
	fn to_int_with_scale__f64_max_scale_0() {
		assert_eq!({ let i: Option<i8>   = (2u64.pow(53) as f64).to_int_with_scale(0); i }, None);
		assert_eq!({ let i: Option<i16>  = (2u64.pow(53) as f64).to_int_with_scale(0); i }, None);
		assert_eq!({ let i: Option<i32>  = (2u64.pow(53) as f64).to_int_with_scale(0); i }, None);
		assert_eq!({ let i: Option<i64>  = (2u64.pow(53) as f64).to_int_with_scale(0); i }, Some(0x0020_0000_0000_0000_i64));
		assert_eq!({ let i: Option<i128> = (2u64.pow(53) as f64).to_int_with_scale(0); i }, Some(0x0020_0000_0000_0000_i128));
		assert_eq!({ let i: Option<u8>   = (2u64.pow(53) as f64).to_int_with_scale(0); i }, None);
		assert_eq!({ let i: Option<u16>  = (2u64.pow(53) as f64).to_int_with_scale(0); i }, None);
		assert_eq!({ let i: Option<u32>  = (2u64.pow(53) as f64).to_int_with_scale(0); i }, None);
		assert_eq!({ let i: Option<u64>  = (2u64.pow(53) as f64).to_int_with_scale(0); i }, Some(0x0020_0000_0000_0000_u64));
		assert_eq!({ let i: Option<u128> = (2u64.pow(53) as f64).to_int_with_scale(0); i }, Some(0x0020_0000_0000_0000_u128));
	}
	#[test]
	fn to_int_with_scale__decimal_max_scale_0() {
		assert_eq!({ let i: Option<i8>   = Decimal::MAX.to_int_with_scale(0); i }, None);
		assert_eq!({ let i: Option<i16>  = Decimal::MAX.to_int_with_scale(0); i }, None);
		assert_eq!({ let i: Option<i32>  = Decimal::MAX.to_int_with_scale(0); i }, None);
		assert_eq!({ let i: Option<i64>  = Decimal::MAX.to_int_with_scale(0); i }, None);
		assert_eq!({ let i: Option<i128> = Decimal::MAX.to_int_with_scale(0); i }, Some(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_i128));
		assert_eq!({ let i: Option<u8>   = Decimal::MAX.to_int_with_scale(0); i }, None);
		assert_eq!({ let i: Option<u16>  = Decimal::MAX.to_int_with_scale(0); i }, None);
		assert_eq!({ let i: Option<u32>  = Decimal::MAX.to_int_with_scale(0); i }, None);
		assert_eq!({ let i: Option<u64>  = Decimal::MAX.to_int_with_scale(0); i }, None);
		assert_eq!({ let i: Option<u128> = Decimal::MAX.to_int_with_scale(0); i }, Some(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_u128));
	}
	#[test]
	fn to_int_with_scale__f32_scale_overflow() {
		assert_eq!({ let i: Option<i8>   = 12.34_f32.to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<i16>  = 12.34_f32.to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<i32>  = 12.34_f32.to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<i64>  = 12.34_f32.to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<i128> = 12.34_f32.to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<u8>   = 12.34_f32.to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<u16>  = 12.34_f32.to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<u32>  = 12.34_f32.to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<u64>  = 12.34_f32.to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<u128> = 12.34_f32.to_int_with_scale(20); i }, None);
	}
	#[test]
	fn to_int_with_scale__f64_scale_overflow() {
		assert_eq!({ let i: Option<i8>   = 12.34_f64.to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<i16>  = 12.34_f64.to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<i32>  = 12.34_f64.to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<i64>  = 12.34_f64.to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<i128> = 12.34_f64.to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<u8>   = 12.34_f64.to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<u16>  = 12.34_f64.to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<u32>  = 12.34_f64.to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<u64>  = 12.34_f64.to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<u128> = 12.34_f64.to_int_with_scale(20); i }, None);
	}
	#[test]
	fn to_int_with_scale__decimal_scale_overflow() {
		assert_eq!({ let i: Option<i8>   = Decimal::from_str("12.34").unwrap().to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<i16>  = Decimal::from_str("12.34").unwrap().to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<i32>  = Decimal::from_str("12.34").unwrap().to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<i64>  = Decimal::from_str("12.34").unwrap().to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<i128> = Decimal::from_str("12.34").unwrap().to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<u8>   = Decimal::from_str("12.34").unwrap().to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<u16>  = Decimal::from_str("12.34").unwrap().to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<u32>  = Decimal::from_str("12.34").unwrap().to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<u64>  = Decimal::from_str("12.34").unwrap().to_int_with_scale(20); i }, None);
		assert_eq!({ let i: Option<u128> = Decimal::from_str("12.34").unwrap().to_int_with_scale(20); i }, None);
	}
	#[test]
	fn to_int_with_scale__f32_overflow() {
		assert_eq!({ let i: Option<i8>   = (  i8::MAX as f32).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<i16>  = ( i16::MAX as f32).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<i32>  = ( i32::MAX as f32).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<i64>  = ( i64::MAX as f32).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<i128> = (i128::MAX as f32).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<u8>   = (  u8::MAX as f32).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<u16>  = ( u16::MAX as f32).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<u32>  = ( u32::MAX as f32).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<u64>  = ( u64::MAX as f32).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<u128> = (u128::MAX as f32).to_int_with_scale(1); i }, None);
	}
	#[test]
	fn to_int_with_scale__f64_overflow() {
		assert_eq!({ let i: Option<i8>   = (  i8::MAX as f64).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<i16>  = ( i16::MAX as f64).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<i32>  = ( i32::MAX as f64).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<i64>  = ( i64::MAX as f64).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<i128> = (i128::MAX as f64).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<u8>   = (  u8::MAX as f64).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<u16>  = ( u16::MAX as f64).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<u32>  = ( u32::MAX as f64).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<u64>  = ( u64::MAX as f64).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<u128> = (u128::MAX as f64).to_int_with_scale(1); i }, None);
	}
	#[test]
	fn to_int_with_scale__decimal_overflow() {
		assert_eq!({ let i: Option<i8>   = Decimal::from(  i8::MAX).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<i16>  = Decimal::from( i16::MAX).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<i32>  = Decimal::from( i32::MAX).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<i64>  = Decimal::from( i64::MAX).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<i128> = Decimal::MAX            .to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<u8>   = Decimal::from(  u8::MAX).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<u16>  = Decimal::from( u16::MAX).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<u32>  = Decimal::from( u32::MAX).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<u64>  = Decimal::from( u64::MAX).to_int_with_scale(1); i }, None);
		assert_eq!({ let i: Option<u128> = Decimal::MAX            .to_int_with_scale(1); i }, None);
	}
}

//§		IteratorExt																
#[cfg(test)]
mod iterator_ext {
	use super::*;
	
	//		limit																
	#[test]
	fn limit__empty() {
		let vec:    Vec<usize> = Vec::new();
		let result: Vec<_>     = vec.iter().limit(Some(10)).collect();
		assert_eq!(result.len(), 0);
	}
	#[test]
	fn limit__no_limit() {
		let vec            = vec![1, 2, 3, 4, 5];
		let result: Vec<_> = vec.iter().limit(None).cloned().collect();
		assert_eq!(result.len(), vec.len());
		assert_eq!(result,       vec);
	}
	#[test]
	fn limit__within_limit() {
		let vec            = vec![1, 2, 3, 4, 5];
		let result: Vec<_> = vec.iter().limit(Some(10)).cloned().collect();
		assert_eq!(result.len(), vec.len());
		assert_eq!(result,       vec);
	}
	#[test]
	fn limit__exceeds_limit() {
		let vec            = vec![1, 2, 3, 4, 5];
		let result: Vec<_> = vec.iter().limit(Some(3)).cloned().collect();
		assert_eq!(result.len(), 3);
		assert_eq!(result,       vec![1, 2, 3]);
	}
}
	
//§		PathExt																	
#[cfg(test)]
mod path_ext {
	use super::*;
	
	//		append																
	#[test]
	fn append() {
		let mut path: PathBuf;
		
		path = PathBuf::from("");
		assert_eq!(path.append(""), PathBuf::from(""));
		
		path = PathBuf::from("tests/std.rs");
		assert_eq!(path.append(".bak"),                 PathBuf::from("tests/std.rs.bak"));
		assert_eq!(path.append(".bak".to_owned()),      PathBuf::from("tests/std.rs.bak"));
		assert_eq!(path.append(OsString::from(".bak")), PathBuf::from("tests/std.rs.bak"));
		assert_eq!(path.append(PathBuf::from(".bak")),  PathBuf::from("tests/std.rs.bak"));
		assert_eq!(path.append(Path::new(".bak")),      PathBuf::from("tests/std.rs.bak"));
		
		let path: &Path;
		path = Path::new("tests/std.rs");
		assert_eq!(path.append(".bak"), PathBuf::from("tests/std.rs.bak"));
	}
	
	//		is_subjective														
	#[test]
	fn is_subjective() {
		assert_eq!(PathBuf::from(".").is_subjective(),      true);
		assert_eq!(PathBuf::from("./").is_subjective(),     true);
		assert_eq!(PathBuf::from("./foo").is_subjective(),  true);
		assert_eq!(PathBuf::from("..").is_subjective(),     true);
		assert_eq!(PathBuf::from("../").is_subjective(),    true);
		assert_eq!(PathBuf::from("../foo").is_subjective(), true);
		assert_eq!(PathBuf::from("foo").is_subjective(),    false);
		assert_eq!(PathBuf::from(".bak").is_subjective(),   false);
		assert_eq!(PathBuf::from("..bak").is_subjective(),  false);
		assert_eq!(PathBuf::from("/").is_subjective(),      false);
		assert_eq!(PathBuf::from("/.").is_subjective(),     false);
		assert_eq!(PathBuf::from("/..").is_subjective(),    false);
		assert_eq!(PathBuf::from("/foo").is_subjective(),   false);
		
		assert_eq!(Path::new(".").is_subjective(),          true);
	}
	
	//		normalize															
	#[test]
	fn normalize() {
		let cwd = env::current_dir().unwrap();
		let mut path: PathBuf;
		
		path = PathBuf::from("");
		assert_eq!(path.normalize(), cwd);
		
		path = PathBuf::from(".");
		assert_eq!(path.normalize(), cwd);
		
		path = PathBuf::from("..");
		assert_eq!(path.normalize(), cwd.parent().unwrap());
		
		path = PathBuf::from("./");
		assert_eq!(path.normalize(), cwd);
		
		path = PathBuf::from("./.");
		assert_eq!(path.normalize(), cwd);
		
		path = PathBuf::from("./..");
		assert_eq!(path.normalize(), cwd.parent().unwrap());
		
		path = PathBuf::from("././/.");
		assert_eq!(path.normalize(), cwd);
		
		path = PathBuf::from("/");
		assert_eq!(path.normalize(), PathBuf::from("/"));
		
		path = PathBuf::from("//");
		assert_eq!(path.normalize(), PathBuf::from("/"));
		
		path = PathBuf::from("/.");
		assert_eq!(path.normalize(), PathBuf::from("/"));
		
		path = PathBuf::from("/./");
		assert_eq!(path.normalize(), PathBuf::from("/"));
		
		path = PathBuf::from("/tests/std.rs");
		assert_eq!(path.normalize(), PathBuf::from("/tests/std.rs"));
		
		path = PathBuf::from("/tests//std.rs");
		assert_eq!(path.normalize(), PathBuf::from("/tests/std.rs"));
		
		path = PathBuf::from("/tests/./std.rs");
		assert_eq!(path.normalize(), PathBuf::from("/tests/std.rs"));
		
		path = PathBuf::from("/tests/../std.rs");
		assert_eq!(path.normalize(), PathBuf::from("/std.rs"));
		
		path = PathBuf::from("/tests/../../std.rs");
		assert_eq!(path.normalize(), PathBuf::from("/std.rs"));
		
		path = PathBuf::from("tests/std.rs");
		assert_eq!(path.normalize(), cwd.join("tests/std.rs"));
		
		path = PathBuf::from("tests//std.rs");
		assert_eq!(path.normalize(), cwd.join("tests/std.rs"));
		
		path = PathBuf::from("tests/./std.rs");
		assert_eq!(path.normalize(), cwd.join("tests/std.rs"));
		
		path = PathBuf::from("tests/one/two/three/std.rs");
		assert_eq!(path.normalize(), cwd.join("tests/one/two/three/std.rs"));
		
		path = PathBuf::from("tests/one/two/three/../../../std.rs");
		assert_eq!(path.normalize(), cwd.join("tests/std.rs"));
		
		path = PathBuf::from("tests//one/./two/../three/.//std.rs");
		assert_eq!(path.normalize(), cwd.join("tests/one/three/std.rs"));
		
		path = PathBuf::from("tests/.rs");
		assert_eq!(path.normalize(), cwd.join("tests/.rs"));
		
		path = PathBuf::from("tests/🥳.rs");
		assert_eq!(path.normalize(), cwd.join("tests/🥳.rs"));
		
		let path: &Path;
		path = Path::new("/tests/std.rs");
		assert_eq!(path.normalize(), Path::new("/tests/std.rs"));
		assert_eq!(path.normalize(), PathBuf::from("/tests/std.rs"));
	}

	//		restrict															
	#[test]
	fn restrict() {
		let cwd = env::current_dir().unwrap();
		let mut path: PathBuf;
		
		path = PathBuf::from("");
		assert_eq!(path.restrict(""),  cwd);
		
		path = PathBuf::from("");
		assert_eq!(path.restrict("."), cwd);
		
		path = PathBuf::from(".");
		assert_eq!(path.restrict(""),  cwd);
		
		path = PathBuf::from(".");
		assert_eq!(path.restrict("."), cwd);
		
		path = PathBuf::from("..");
		assert_eq!(path.restrict("."), cwd);
		
		path = PathBuf::from("/");
		assert_eq!(path.restrict("."), cwd);
		
		path = PathBuf::from("/tests/std.rs");
		assert_eq!(path.restrict("."), cwd);
		
		path = PathBuf::from("tests/std.rs");
		assert_eq!(path.restrict("."), cwd.join("tests/std.rs"));
		
		path = PathBuf::from("tests/../std.rs");
		assert_eq!(path.restrict("."), cwd.join("std.rs"));
		
		path = PathBuf::from("tests/../../std.rs");
		assert_eq!(path.restrict("."), cwd);
		
		path = PathBuf::from("tests/../../one/two/three/std.rs");
		assert_eq!(path.restrict("."), cwd);
		
		path = PathBuf::from("../tests/std.rs");
		assert_eq!(path.restrict("."), cwd);
		
		path = PathBuf::from("");
		assert_eq!(path.restrict(Path::new("/foo/bar")), PathBuf::from("/foo/bar"));

		path = PathBuf::from(".");
		assert_eq!(path.restrict(Path::new(".")), cwd);

		path = PathBuf::from("/tests/std.rs");
		assert_eq!(path.restrict(Path::new(".")), cwd);
		
		path = PathBuf::from("tests/std.rs");
		assert_eq!(path.restrict(Path::new(".")), cwd.join("tests/std.rs"));
		
		path = PathBuf::from("/foo/tests/std.rs");
		assert_eq!(path.restrict(Path::new("/foo/bar")), PathBuf::from("/foo/bar"));
		
		path = PathBuf::from("/foo/bar/tests/std.rs");
		assert_eq!(path.restrict(Path::new("/foo/bar")), PathBuf::from("/foo/bar/tests/std.rs"));
		
		let path: &Path;
		path = Path::new("/foo/bar/tests/std.rs");
		assert_eq!(path.restrict("/foo/bar"),                Path::new("/foo/bar/tests/std.rs"));
		assert_eq!(path.restrict("/foo/bar".to_owned()),     Path::new("/foo/bar/tests/std.rs"));
		assert_eq!(path.restrict(&Path::new("/foo/bar")),    Path::new("/foo/bar/tests/std.rs"));
		assert_eq!(path.restrict(&Path::new("/foo/bar")),    PathBuf::from("/foo/bar/tests/std.rs"));
		assert_eq!(path.restrict(PathBuf::from("/foo/bar")), Path::new("/foo/bar/tests/std.rs"));
		assert_eq!(path.restrict(PathBuf::from("/foo/bar")), PathBuf::from("/foo/bar/tests/std.rs"));
	}
	
	//		strip_parentdirs													
	#[test]
	fn strip_parentdirs() {
		let mut path: PathBuf;
		
		path = PathBuf::from("");
		assert_eq!(path.strip_parentdirs(true), path);
		
		path = PathBuf::from(".");
		assert_eq!(path.strip_parentdirs(true), path);
		
		path = PathBuf::from("..");
		assert_eq!(path.strip_parentdirs(true), PathBuf::from(""));
		
		path = PathBuf::from("/");
		assert_eq!(path.strip_parentdirs(true), path);
		
		path = PathBuf::from("/tests/std.rs");
		assert_eq!(path.strip_parentdirs(true), PathBuf::from("/tests/std.rs"));
		
		path = PathBuf::from("tests/std.rs");
		assert_eq!(path.strip_parentdirs(true), PathBuf::from("tests/std.rs"));
		
		path = PathBuf::from("../tests/std.rs");
		assert_eq!(path.strip_parentdirs(true), PathBuf::from("tests/std.rs"));
		
		path = PathBuf::from("/../tests/std.rs");
		assert_eq!(path.strip_parentdirs(true), PathBuf::from("/tests/std.rs"));
		
		path = PathBuf::from("../../../tests/std.rs");
		assert_eq!(path.strip_parentdirs(true), PathBuf::from("tests/std.rs"));
		
		path = PathBuf::from("/tests/../std.rs");
		assert_eq!(path.strip_parentdirs(true), PathBuf::from("/tests/std.rs"));
		
		path = PathBuf::from("tests/../std.rs");
		assert_eq!(path.strip_parentdirs(true), PathBuf::from("tests/std.rs"));
		
		path = PathBuf::from("../tests/../../std.rs");
		assert_eq!(path.strip_parentdirs(true), PathBuf::from("tests/std.rs"));
		
		path = PathBuf::from("");
		assert_eq!(path.strip_parentdirs(false), path);
		
		path = PathBuf::from(".");
		assert_eq!(path.strip_parentdirs(false), path);
		
		path = PathBuf::from("..");
		assert_eq!(path.strip_parentdirs(false), PathBuf::from(""));
		
		path = PathBuf::from("/");
		assert_eq!(path.strip_parentdirs(false), path);
		
		path = PathBuf::from("/tests/std.rs");
		assert_eq!(path.strip_parentdirs(false), PathBuf::from("/tests/std.rs"));
		
		path = PathBuf::from("tests/std.rs");
		assert_eq!(path.strip_parentdirs(false), PathBuf::from("tests/std.rs"));
		
		path = PathBuf::from("../tests/std.rs");
		assert_eq!(path.strip_parentdirs(false), PathBuf::from("tests/std.rs"));
		
		path = PathBuf::from("/../tests/std.rs");
		assert_eq!(path.strip_parentdirs(false), PathBuf::from("/../tests/std.rs"));
		
		path = PathBuf::from("../../../tests/std.rs");
		assert_eq!(path.strip_parentdirs(false), PathBuf::from("tests/std.rs"));
		
		path = PathBuf::from("/tests/../std.rs");
		assert_eq!(path.strip_parentdirs(false), PathBuf::from("/tests/../std.rs"));
		
		path = PathBuf::from("tests/../std.rs");
		assert_eq!(path.strip_parentdirs(false), PathBuf::from("tests/../std.rs"));
		
		path = PathBuf::from("../tests/../../std.rs");
		assert_eq!(path.strip_parentdirs(false), PathBuf::from("tests/../../std.rs"));
		
		let path: &Path;
		path = Path::new("tests/std.rs");
		assert_eq!(path.strip_parentdirs(false), Path::new("tests/std.rs"));
		assert_eq!(path.strip_parentdirs(false), PathBuf::from("tests/std.rs"));
	}
	
	//		strip_root															
	#[test]
	fn strip_root() {
		let mut path: PathBuf;
		
		path = PathBuf::from("");
		assert_eq!(path.strip_root(), path);
		
		path = PathBuf::from(".");
		assert_eq!(path.strip_root(), path);
		
		path = PathBuf::from("..");
		assert_eq!(path.strip_root(), path);
		
		path = PathBuf::from("/");
		assert_eq!(path.strip_root(), PathBuf::from(""));
		
		path = PathBuf::from("/tests/std.rs");
		assert_eq!(path.strip_root(), PathBuf::from("tests/std.rs"));
		
		path = PathBuf::from("//tests/std.rs");
		assert_eq!(path.strip_root(), PathBuf::from("tests/std.rs"));
		
		if cfg!(windows) {
			path = PathBuf::from(r"C:\tests\std.rs");
			assert_eq!(path.strip_root(), PathBuf::from(r"tests\std.rs"));
			
			path = PathBuf::from(r"C:tests\std.rs");
			assert_eq!(path.strip_root(), PathBuf::from(r"tests\std.rs"));
			
			path = PathBuf::from(r"\tests\std.rs");
			assert_eq!(path.strip_root(), PathBuf::from(r"tests\std.rs"));
			
			path = PathBuf::from(r"\\tests\std.rs");
			assert_eq!(path.strip_root(), PathBuf::from(r"tests\std.rs"));
		}
		
		let path: &Path;
		path = Path::new("tests/std.rs");
		assert_eq!(path.strip_root(), Path::new("tests/std.rs"));
		assert_eq!(path.strip_root(), PathBuf::from("tests/std.rs"));
	}
}


