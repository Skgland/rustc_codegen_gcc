#![feature(const_option, core_intrinsics)]

fn main() {
    /*test_float!(f64, f64, f64::INFINITY, f64::NEG_INFINITY, f64::NAN);
    ($modname: ident, $fty: ty, $inf: expr, $neginf: expr, $nan: expr) => {*/

    /*assert_eq!((0.0 as f64).min(0.0), 0.0);
    assert!((0.0 as f64).min(0.0).is_sign_positive());
    assert_eq!((-0.0 as f64).min(-0.0), -0.0);

    fn is_sign_neg(num: f64) -> bool {
        num.to_bits() & 0x8000_0000_0000_0000 != 0
    }
    //assert!(is_sign_neg(-0.0_f64));
    println!("{:b}", (-0.0_f64).to_bits());
    let var = 1.2_f64;
    println!("{}", 1.2_f64 == var);*/

    /*fn my_float() -> f64 {
        -0.0
    }

    let float = my_float();*/
    //println!("{}", (-0.0_f64).to_bits());

    // FIXME: seems like the asm is calling comisd which seems to convert -0.0 to 0.
    /*assert!((-0.0_f64).to_bits() & 0x8000_0000_0000_0000 != 0);
    println!("1");*/
    //println!("{}", float);
    //std::process::exit(float as i32)

    /*assert!((-0.0 as f64).min(-0.0).is_sign_negative());
    assert_eq!((9.0 as f64).min(9.0), 9.0);
    assert_eq!((-9.0 as f64).min(0.0), -9.0);
    assert_eq!((0.0 as f64).min(9.0), 0.0);
    assert!((0.0 as f64).min(9.0).is_sign_positive());
    assert_eq!((-0.0 as f64).min(9.0), -0.0);
    assert!((-0.0 as f64).min(9.0).is_sign_negative());*/

    //println!("{}", 9);
    //println!("{}", 9.0_f32);
    //assert_eq!(-9.0_f32, -9 as f32);

    //assert_eq!("1", format!("{:.0}", 1.0f64));
    //assert_eq!("9", format!("{:.0}", 9.4f64));
    //assert_eq!("10", format!("{:.0}", 9.9f64));
    //assert_eq!("9.8", format!("{:.1}", 9.849f64));
    //assert_eq!("9.9", format!("{:.1}", 9.851f64));
    //assert_eq!("1", format!("{:.0}", 0.5f64));

    //assert_eq!(2.3f32.copysign(-1.0), -2.3f32);
    /*let f = 9.4f32;
    println!("{}", f);*/
    //println!("{}", 9.4f32); // FIXME: this is using bytes_in_context(), but gives a wrong value.

    /*extern {
        //pub fn printf(format: *const i8, ...) -> i32;
        pub fn printf(format: *const i8, arg: f64) -> i32;
    }

    unsafe {
        printf(b"Num: %f\n\0" as *const _ as *const _, 9.4f64);
    }
    println!("{}", 9.4f64);*/

    // FIXME: the code seems to be the same when using an integer, but somehow, it doesn't work for
    // a float. Could it be related to the fact that floating-points use different registers?

    /*let f = 1234567.89f64;
    assert_eq!("1.23456789e6", format!("{:e}", f));
    println!("{:e}", f);
    println!("{:e}", 1234567.89f64);*/
    //assert_eq!("1.23456789e6", format!("{:e}", 1234567.89f64));
    /*assert_eq!("1.23456789e3", format!("{:e}", 1234.56789f64));
    assert_eq!("1.23456789E6", format!("{:E}", 1234567.89f64));
    assert_eq!("1.23456789E3", format!("{:E}", 1234.56789f64));
    assert_eq!("0.0", format!("{:?}", 0.0f64));
    assert_eq!("1.01", format!("{:?}", 1.01f64));*/

    /*assert_eq!((-0.0 as f64).min(-9.0), -9.0);
    assert_eq!((f64::INFINITY as f64).min(9.0), 9.0);
    assert_eq!((9.0 as f64).min(f64::INFINITY), 9.0);
    assert_eq!((f64::INFINITY as f64).min(-9.0), -9.0);
    assert_eq!((-9.0 as f64).min(f64::INFINITY), -9.0);
    assert_eq!((f64::NEG_INFINITY as f64).min(9.0), f64::NEG_INFINITY);
    assert_eq!((9.0 as f64).min(f64::NEG_INFINITY), f64::NEG_INFINITY);
    assert_eq!((f64::NEG_INFINITY as f64).min(-9.0), f64::NEG_INFINITY);
    assert_eq!((-9.0 as f64).min(f64::NEG_INFINITY), f64::NEG_INFINITY);*/
    // Cranelift fmin has NaN propagation
    //assert_eq!((f64::NAN as f64).min(9.0), 9.0);
    //assert_eq!((f64::NAN as f64).min(-9.0), -9.0);
    //assert_eq!((9.0 as f64).min(f64::NAN), 9.0);
    //assert_eq!((-9.0 as f64).min(f64::NAN), -9.0);
    //assert!((f64::NAN as f64).min(f64::NAN).is_nan());

    /*let max: f64 = f32::MAX.into();
    assert_eq!(max as f32, f32::MAX);
    assert!(max.is_normal());

    let min: f64 = f32::MIN.into();
    assert_eq!(min as f32, f32::MIN);
    assert!(min.is_normal());

    let min_positive: f64 = f32::MIN_POSITIVE.into();
    assert_eq!(min_positive as f32, f32::MIN_POSITIVE);
    assert!(min_positive.is_normal());

    let epsilon: f64 = f32::EPSILON.into();
    assert_eq!(epsilon as f32, f32::EPSILON);
    assert!(epsilon.is_normal());

    let zero: f64 = (0.0f32).into();
    assert_eq!(zero as f32, 0.0f32);
    assert!(zero.is_sign_positive());

    let neg_zero: f64 = (-0.0f32).into();
    assert_eq!(neg_zero as f32, -0.0f32);
    assert!(neg_zero.is_sign_negative());

    let infinity: f64 = f32::INFINITY.into();
    assert_eq!(infinity as f32, f32::INFINITY);
    assert!(infinity.is_infinite());
    assert!(infinity.is_sign_positive());

    let neg_infinity: f64 = f32::NEG_INFINITY.into();
    assert_eq!(neg_infinity as f32, f32::NEG_INFINITY);
    assert!(neg_infinity.is_infinite());
    assert!(neg_infinity.is_sign_negative());

    let nan: f64 = f32::NAN.into();
    assert!(nan.is_nan());*/

    /*use std::convert::TryFrom;

    /*let max = <i128>::MAX;
    let min = <i128>::MIN;*/
    let zero: i128 = 0;
    /*let t_max = <u64>::MAX;
    let t_min = <u64>::MIN;
    assert!(<u64 as TryFrom<i128>>::try_from(max).is_err());
    assert!(<u64 as TryFrom<i128>>::try_from(min).is_err());*/
    println!("{:?}", i128_to_u64(zero));
    assert_eq!(<u64 as TryFrom<i128>>::try_from(zero).unwrap(), zero as u64);
    /*assert_eq!(
        <u64 as TryFrom<i128>>::try_from(t_max as i128).unwrap(),
        t_max as u64
    );
    assert_eq!(
        <u64 as TryFrom<i128>>::try_from(t_min as i128).unwrap(),
        t_min as u64
    );*/
    */

        /*
    //let mut num = u128::MAX >> 20;
    //let mut num = u128::MAX;
    #[inline(never)]
    fn two() -> u128 {
        2
    }

    //let mut num = 340282366920938463463374607431768211455_u128 >> two();
    let mut num = 340282366920938463463374607431768211455_u128 >> 2;
    //let mut num = 340282366920938463463374607431768211455_u128;
    //let mut num = 10_u128 >> 2;
    const MASK: u128 = 0x80000000000000000000000000000000;
    for _ in 0..128 {
        if num & MASK == MASK {
            print!("1");
        }
        else {
            print!("0");
        }
        num <<= 1;
    }
    println!();
*/

    /*let mut r = 2 as i128;
    assert_eq!(r.pow(2), 4 as i128);
    assert_eq!(r.pow(0), 1 as i128);
    assert_eq!(r.wrapping_pow(2), 4 as i128);
    assert_eq!(r.wrapping_pow(0), 1 as i128);
    assert_eq!(r.checked_pow(2), Some(4 as i128));
    assert_eq!(r.checked_pow(0), Some(1 as i128));
    assert_eq!(r.overflowing_pow(2), (4 as i128, false));
    assert_eq!(r.overflowing_pow(0), (1 as i128, false));
    assert_eq!(r.saturating_pow(2), 4 as i128);
    assert_eq!(r.saturating_pow(0), 1 as i128);

    r = i128::MAX;
    // use `^` to represent .pow() with no overflow.
    // if itest::MAX == 2^j-1, then itest is a `j` bit int,
    // so that `itest::MAX*itest::MAX == 2^(2*j)-2^(j+1)+1`,
    // thussaturating_pow the overflowing result is exactly 1.
    assert_eq!(r.wrapping_pow(2), 1 as i128);
    assert_eq!(r.checked_pow(2), None);
    assert_eq!(r.overflowing_pow(2), (1 as i128, true));
    assert_eq!(r.saturating_pow(2), i128::MAX);
    //test for negative exponent.
    r = -2 as i128;
    assert_eq!(r.pow(2), 4 as i128);
    assert_eq!(r.pow(3), -8 as i128);
    assert_eq!(r.pow(0), 1 as i128);
    assert_eq!(r.wrapping_pow(2), 4 as i128);
    assert_eq!(r.wrapping_pow(3), -8 as i128);
    assert_eq!(r.wrapping_pow(0), 1 as i128);
    assert_eq!(r.checked_pow(2), Some(4 as i128));
    assert_eq!(r.checked_pow(3), Some(-8 as i128));
    assert_eq!(r.checked_pow(0), Some(1 as i128));
    assert_eq!(r.overflowing_pow(2), (4 as i128, false));
    assert_eq!(r.overflowing_pow(3), (-8 as i128, false));
    assert_eq!(r.overflowing_pow(0), (1 as i128, false));
    assert_eq!(r.saturating_pow(2), 4 as i128);
    assert_eq!(r.saturating_pow(3), -8 as i128);
    assert_eq!(r.saturating_pow(0), 1 as i128);*/

    fn max64() -> i128 {
        18446744073709551615_i128
        //18446744073709551615_u64 as i128

        /*println!("{}", u64::MAX);
        u64::MAX as i128*/
    }

    println!("1. {}", max64());

    /*use std::convert::TryFrom;

    fn try_from(u: i128) -> Option<u64> {
        let min = u64::MIN as i128;
        let max = u64::MAX as i128;
        /*println!("{} < {} == {}", u, min, u < min);
        println!("{} > {} == {}", u, max, u > max);*/
        if u < min || u > max {
            None
        } else {
            Some(u as u64)
        }
    }

    let max = <i128>::MAX;
    let min = <i128>::MIN;
    let zero: i128 = 0;
    let t_max = <u64>::MAX;
    let t_min = <u64>::MIN;
    assert!(<u64 as TryFrom<i128>>::try_from(max).is_err());
    assert!(<u64 as TryFrom<i128>>::try_from(min).is_err());
    println!("{:?}", try_from(zero));
    println!("{:?}", <u64 as TryFrom<i128>>::try_from(zero));
    assert_eq!(<u64 as TryFrom<i128>>::try_from(zero).unwrap(), zero as u64);
    assert_eq!(
        <u64 as TryFrom<i128>>::try_from(t_max as i128).unwrap(),
        t_max as u64
    );
    assert_eq!(
        <u64 as TryFrom<i128>>::try_from(t_min as i128).unwrap(),
        t_min as u64
    );*/
}
