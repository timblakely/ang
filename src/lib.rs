#![cfg_attr(not(feature = "std"), no_std)]

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use core::cmp::Ordering;
use core::f64::consts::PI;
use core::fmt::{Display, Error, Formatter};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use num_traits::{
    cast::{cast, NumCast},
    Num, Signed, Zero,
};

#[cfg(feature = "std")]
use num_traits::Float;

/// An angle.
///
/// Might be a value in degrees or in radians.
#[derive(Copy, Clone, Debug, Hash)]
pub enum Angle<T = f64> {
    /// The angle value in radians.
    Radians(T),
    /// The angle value in degrees.
    Degrees(T),
}

impl<T: Copy + NumCast> Angle<T> {
    /// Yield the value encoded in radians.
    #[inline]
    pub fn in_radians(self) -> T {
        match self {
            Radians(v) => v,
            Degrees(v) => cast(cast::<T, f64>(v).unwrap() / 180.0 * PI).unwrap(),
        }
    }

    /// Yield the value encoded in degrees.
    #[inline]
    pub fn in_degrees(self) -> T {
        match self {
            Radians(v) => cast(cast::<T, f64>(v).unwrap() / PI * 180.0).unwrap(),
            Degrees(v) => v,
        }
    }

    /// An angle of 45°.
    #[inline]
    pub fn eighth() -> Angle<T> {
        Degrees(cast(45).unwrap())
    }

    /// An angle of 90° (right angle).
    #[inline]
    pub fn quarter() -> Angle<T> {
        Degrees(cast(90).unwrap())
    }

    /// An angle of 180° (straight).
    #[inline]
    pub fn half() -> Angle<T> {
        Degrees(cast(180).unwrap())
    }

    /// An angle of 360° (perigon).
    #[inline]
    pub fn full() -> Angle<T> {
        Degrees(cast(360).unwrap())
    }
}

impl<T: Copy + Num + NumCast + PartialOrd> Angle<T> {
    /// Create a new angle by normalizing the value into the range of
    /// [0, 2π) rad.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ang::*;
    /// # use std::f64::consts::PI;
    /// let alpha = Degrees(-90.0f64).normalized();
    /// assert!((alpha.in_degrees() - 270.0).abs() < 1.0e-10);
    ///
    /// let beta = Radians(2.0 * PI).normalized();
    /// assert!((beta.in_radians() - 0.0).abs() < 1.0e-10);
    /// ```
    #[inline]
    pub fn normalized(self) -> Self {
        let (v, upper) = match self {
            Radians(v) => (v, cast(2.0 * PI).unwrap()),
            Degrees(v) => (v, cast(360.0).unwrap()),
        };

        let normalized = if v < upper && v >= Zero::zero() {
            v
        } else {
            let v = v % upper;

            if v >= Zero::zero() {
                v
            } else {
                v + upper
            }
        };

        match self {
            Radians(_) => Radians(normalized),
            Degrees(_) => Degrees(normalized),
        }
    }
}

#[cfg(feature = "std")]
impl<T: Float> Angle<T> {
    /// Computes the minimal unsigned distance between two normalized angles. Returns an
    /// angle in the range of [0, π] rad.
    ///
    /// ```rust
    /// # use ang::*;
    /// let distance = Degrees(345.0).min_dist(Degrees(15.0));
    /// assert!((distance.in_degrees() - 30.0) < 1.0e-10);
    /// ```
    #[inline]
    pub fn min_dist(self, other: Angle<T>) -> Angle<T> {
        let pi = cast(PI).unwrap();
        let two_pi = cast(2.0 * PI).unwrap();

        let a = self.in_radians();
        let b = other.in_radians();

        let d = (a - b).abs();

        // short-circuit if both angles are normalized
        Radians(
            if a >= T::zero() && a < two_pi && b >= T::zero() && b < two_pi {
                d.min(two_pi - d)
            } else {
                pi - ((d % two_pi) - pi).abs()
            },
        )
    }
}

impl<T: Signed> Angle<T> {
    /// Compute the absolute angle.
    #[inline]
    pub fn abs(&self) -> Self {
        match *self {
            Radians(ref v) => Radians(v.abs()),
            Degrees(ref v) => Degrees(v.abs()),
        }
    }

    /// Returns a number that represents the sign of self.
    ///
    /// * `1.0` if the number is positive, `+0.0` or `Float::infinity()`
    /// * `-1.0` if the number is negative, `-0.0` or `Float::neg_infinity()`
    /// * `Float::nan()` if the number is `Float::nan()`
    #[inline]
    pub fn signum(&self) -> Self {
        match *self {
            Radians(ref v) => Radians(v.signum()),
            Degrees(ref v) => Degrees(v.signum()),
        }
    }

    /// Returns `true` if the number is positive and `false` if the number is zero or negative
    #[inline]
    pub fn is_positive(&self) -> bool {
        match *self {
            Radians(ref v) => v.is_positive(),
            Degrees(ref v) => v.is_positive(),
        }
    }

    /// Returns `true` if the number is negative and `false` if the number is zero or positive.
    #[inline]
    pub fn is_negative(&self) -> bool {
        match *self {
            Radians(ref v) => v.is_negative(),
            Degrees(ref v) => v.is_negative(),
        }
    }
}

#[cfg(feature = "std")]
impl<T: Float + NumCast> Angle<T> {
    /// Compute the sine of the angle.
    #[inline]
    pub fn sin(self) -> T {
        self.in_radians().sin()
    }

    /// Compute the cosine of the angle.
    #[inline]
    pub fn cos(self) -> T {
        self.in_radians().cos()
    }

    /// Compute the tangent of the angle.
    #[inline]
    pub fn tan(self) -> T {
        self.in_radians().tan()
    }

    /// Simultaneously compute the sine and cosine of the number, `x`.
    ///
    /// Return `(sin(x), cos(x))`.
    #[inline]
    pub fn sin_cos(self) -> (T, T) {
        self.in_radians().sin_cos()
    }
}

impl<T: Zero + Copy + NumCast> Zero for Angle<T> {
    #[inline]
    fn zero() -> Self {
        Radians(T::zero())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        match self {
            &Radians(ref v) => v.is_zero(),
            &Degrees(ref v) => v.is_zero(),
        }
    }
}

impl<T: Copy + NumCast + PartialEq> PartialEq for Angle<T> {
    #[inline]
    fn eq(&self, other: &Angle<T>) -> bool {
        if let (Degrees(ref a), Degrees(ref b)) = (self, other) {
            a.eq(b)
        } else {
            self.in_radians().eq(&other.in_radians())
        }
    }
}

impl<T: Copy + Eq + NumCast> Eq for Angle<T> {}

impl<T: AbsDiffEq + Copy + NumCast> AbsDiffEq for Angle<T> {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        match (*self, *other) {
            (Radians(ref v0), Radians(ref v1)) => v0.abs_diff_eq(&v1, epsilon),
            (_, _) => self.in_degrees().abs_diff_eq(&other.in_degrees(), epsilon),
        }
    }
}

impl<T: RelativeEq + Copy + NumCast> RelativeEq for Angle<T> {
    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        match (*self, *other) {
            (Radians(ref v0), Radians(ref v1)) => v0.relative_eq(&v1, epsilon, max_relative),
            (_, _) => self
                .in_degrees()
                .relative_eq(&other.in_degrees(), epsilon, max_relative),
        }
    }
}

impl<T: UlpsEq + Copy + NumCast> UlpsEq for Angle<T> {
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        match (*self, *other) {
            (Radians(ref v0), Radians(ref v1)) => v0.ulps_eq(&v1, epsilon, max_ulps),
            (_, _) => self
                .in_degrees()
                .ulps_eq(&other.in_degrees(), epsilon, max_ulps),
        }
    }
}

macro_rules! math_additive(
    ($bound:ident, $func:ident, $assign_bound:ident, $assign_func:ident) => (
        impl<T: $bound + Copy + NumCast> $bound for Angle<T> {
            type Output = Angle<T::Output>;
            #[inline]
            fn $func(self, rhs: Angle<T>) -> Self::Output {
                if let (Degrees(a), Degrees(b)) = (self, rhs) {
                    Degrees(a.$func(b))
                } else {
                    Radians(self.in_radians().$func(rhs.in_radians()))
                }
            }
        }

        impl<T: $assign_bound + Copy + NumCast  > $assign_bound for Angle<T> {
            #[inline]
            fn $assign_func(&mut self, rhs: Angle<T>) {
                if let (Degrees(ref mut a), Degrees(b)) = (*self, rhs)  {
                    a.$assign_func(b);
                    *self = Degrees(*a);
                } else {
                    let mut val = self.in_radians();
                    val.$assign_func(rhs.in_radians());
                    *self = Radians(val);
                }
            }
        }
    );
);

math_additive!(Add, add, AddAssign, add_assign);
math_additive!(Sub, sub, SubAssign, sub_assign);

macro_rules! math_multiplicative(
    ($bound:ident, $func:ident, $assign_bound:ident, $assign_func:ident, $($t:ident),*) => (
        impl<T: $bound + Copy> $bound<T> for Angle<T> {
            type Output = Angle<T::Output>;
            #[inline]
            fn $func(self, rhs: T) -> Self::Output {
                match self {
                    Radians(v) => Radians(v.$func(rhs)),
                    Degrees(v) => Degrees(v.$func(rhs))
                }
            }
        }

        impl<T: $assign_bound> $assign_bound<T> for Angle<T> {
            #[inline]
            fn $assign_func(&mut self, rhs: T) {
                match *self {
                    Radians(ref mut v) => { v.$assign_func(rhs) }
                    Degrees(ref mut v) => { v.$assign_func(rhs) }
                }
            }
        }

        $(
            impl $bound<Angle<$t>> for $t {
                type Output = Angle<$t>;
                #[inline]
                fn $func(self, rhs: Angle<$t>) -> Self::Output {
                    match rhs {
                        Radians(v) => Radians(self.$func(v)),
                        Degrees(v) => Degrees(self.$func(v))
                    }
                }
            }
        )*
    );
);

math_multiplicative!(
    Mul, mul, MulAssign, mul_assign, u8, u16, u32, u64, i8, i16, i32, i64, usize, isize, f32, f64
);
math_multiplicative!(
    Div, div, DivAssign, div_assign, u8, u16, u32, u64, i8, i16, i32, i64, usize, isize, f32, f64
);

impl<T: Neg> Neg for Angle<T> {
    type Output = Angle<T::Output>;
    #[inline]
    fn neg(self) -> Self::Output {
        match self {
            Radians(v) => Radians(-v),
            Degrees(v) => Degrees(-v),
        }
    }
}

impl<T: PartialOrd + Copy + NumCast> PartialOrd<Angle<T>> for Angle<T> {
    #[inline]
    fn partial_cmp(&self, other: &Angle<T>) -> Option<Ordering> {
        match (*self, *other) {
            (Radians(ref v0), Radians(ref v1)) => v0.partial_cmp(&v1),
            (_, _) => self.in_degrees().partial_cmp(&other.in_degrees()),
        }
    }
}

impl<T: Ord + Eq + Copy + NumCast> Ord for Angle<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        match (*self, *other) {
            (Radians(ref v0), Radians(ref v1)) => v0.cmp(&v1),
            (_, _) => self.in_degrees().cmp(&other.in_degrees()),
        }
    }
}

impl<T: Display> Display for Angle<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Radians(ref v) => write!(f, "{}rad", v),
            Degrees(ref v) => write!(f, "{}°", v),
        }
    }
}

unsafe impl<T: Send> Send for Angle<T> {}

/// Compute the arcsine of a number. Return value is in the range of
/// [-π/2, π/2] rad or `None` if the number is outside the range [-1, 1].
#[cfg(feature = "std")]
#[inline]
pub fn asin<T: Float>(value: T) -> Option<Angle<T>> {
    let value = value.asin();
    if value.is_nan() {
        None
    } else {
        Some(Radians(value))
    }
}

/// Compute the arccosine of a number. Return value is in the range of
/// [0, π] rad or `None` if the number is outside the range [-1, 1].
#[cfg(feature = "std")]
#[inline]
pub fn acos<T: Float>(value: T) -> Option<Angle<T>> {
    let value = value.acos();
    if value.is_nan() {
        None
    } else {
        Some(Radians(value))
    }
}

/// Compute the arctangent of a number. Return value is in the range of
/// [-π/2, π/2] rad.
#[cfg(feature = "std")]
#[inline]
pub fn atan<T: Float>(value: T) -> Angle<T> {
    Radians(value.atan())
}

/// Compute the four quadrant arctangent of `y` and `x`.
#[cfg(feature = "std")]
#[inline]
pub fn atan2<T: Float>(y: T, x: T) -> Angle<T> {
    Radians(y.atan2(x))
}

/// Compute the approximate mean of a list of angles by averaging the
/// Cartesian coordinates of the angles on the unit circle. Return the
/// normalized angle.
///
/// # Examples
///
/// ```rust
/// # use ang::*;
/// let angles = [Degrees(270.0f64), Degrees(360.0), Degrees(90.0)];
///
/// let mu = mean_angle(&angles);
/// assert!(mu.min_dist(Radians(0.0)).in_radians() < 1.0e-10);
/// ```
#[cfg(feature = "std")]
#[inline]
pub fn mean_angle<'a, T, I>(angles: I) -> Angle<T>
where
    T: 'a + Float,
    I: IntoIterator<Item = &'a Angle<T>>,
{
    let mut x = T::zero();
    let mut y = T::zero();
    let mut n = 0;

    for angle in angles {
        let (sin, cos) = angle.sin_cos();

        x = x + cos;
        y = y + sin;
        n += 1;
    }

    let n = cast(n).unwrap();
    let a = (y / n).atan2(x / n);

    Radians(a).normalized()
}

// re-exports
pub use Angle::{Degrees, Radians};

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use core::f64::consts::PI;
    use hamcrest2::{assert_that, close_to, prelude::*};
    use num_traits::cast::cast;
    use quickcheck::{quickcheck, Arbitrary, Gen};

    #[cfg(feature = "std")]
    use num_traits::Float;

    use super::*;

    #[test]
    fn test_angle_conversions() {
        fn prop(angle: Angle) -> bool {
            are_close(angle.in_radians(), Degrees(angle.in_degrees()).in_radians())
        }
        quickcheck(prop as fn(Angle) -> bool);
    }

    #[test]
    fn test_angle_math_multiplicative() {
        fn prop(a: Angle, x: f64) -> bool {
            match a {
                Radians(v) => {
                    let div_res = {
                        let mut a1 = a.clone();
                        a1 /= x;
                        a1.in_radians() == v / x
                    };
                    let mult_res = {
                        let mut a1 = a.clone();
                        a1 *= x;
                        a1.in_radians() == v * x
                    };
                    (a * x).in_radians() == v * x
                        && (a / x).in_radians() == v / x
                        && div_res
                        && mult_res
                }
                Degrees(v) => {
                    let div_res = {
                        let mut a1 = a.clone();
                        a1 *= x;
                        a1.in_degrees() == v * x
                    };
                    let mult_res = {
                        let mut a1 = a.clone();
                        a1 /= x;
                        a1.in_degrees() == v / x
                    };
                    (a * x).in_degrees() == v * x
                        && (a / x).in_degrees() == v / x
                        && div_res
                        && mult_res
                }
            }
        }
        quickcheck(prop as fn(Angle, f64) -> bool);
    }

    #[test]
    fn test_angle_math_additive() {
        fn prop(a: Angle, b: Angle) -> bool {
            if let (Radians(x), Radians(y)) = (a, b) {
                let add_res = {
                    let mut a1 = a.clone();
                    a1 += b;
                    a1.in_radians() == x + y
                };
                let sub_res = {
                    let mut a1 = a.clone();
                    a1 -= b;
                    a1.in_radians() == x - y
                };
                (a + b).in_radians() == x + y && (a - b).in_radians() == x - y && add_res && sub_res
            } else if let (Degrees(x), Degrees(y)) = (a, b) {
                let add_res = {
                    let mut a1 = a.clone();
                    a1 += b;
                    a1.in_degrees() == x + y
                };
                let sub_res = {
                    let mut a1 = a.clone();
                    a1 -= b;
                    a1.in_degrees() == x - y
                };
                (a + b).in_degrees() == x + y && (a - b).in_degrees() == x - y && add_res && sub_res
            } else {
                let add_res = {
                    let mut a1 = a.clone();
                    a1 += b;
                    a1.in_radians() == a.in_radians() + b.in_radians()
                };
                let sub_res = {
                    let mut a1 = a.clone();
                    a1 -= b;
                    a1.in_radians() == a.in_radians() - b.in_radians()
                };
                (a + b).in_radians() == a.in_radians() + b.in_radians() && add_res && sub_res
            }
        }
        quickcheck(prop as fn(Angle, Angle) -> bool);
    }

    #[test]
    fn test_angle_normalization() {
        fn prop(angle: Angle) -> bool {
            let v = angle.normalized();
            let rad = v.in_radians();
            let deg = v.in_degrees();

            0.0 <= rad
                && rad < 2.0 * PI
                && 0.0 <= deg
                && deg < 360.0
                && are_close(rad.cos(), angle.cos())
        }
        quickcheck(prop as fn(Angle) -> bool);
    }

    #[test]
    fn test_angle_minimal_distance() {
        fn prop(a: Angle, b: Angle) -> bool {
            let d = a.min_dist(b);
            0.0 <= d.in_radians() && d.in_radians() <= PI
        }
        quickcheck(prop as fn(Angle, Angle) -> bool);

        assert_that!(
            Degrees(180.0).min_dist(Degrees(0.0)).in_degrees(),
            close_to(180.0, 0.000001)
        );
        assert_that!(
            Degrees(0.1).min_dist(Degrees(359.9)).in_degrees(),
            close_to(0.2, 0.000001)
        );
        assert_that!(
            Degrees(1.0).min_dist(Degrees(2.0)).in_degrees(),
            close_to(1.0, 0.000001)
        );
    }

    #[test]
    pub fn test_mean_angle() {
        assert_that!(
            mean_angle(&[Degrees(90.0)]).in_degrees(),
            close_to(90.0, 0.000001)
        );
        assert_that!(
            mean_angle(&[Degrees(90.0), Degrees(90.0)]).in_degrees(),
            close_to(90.0, 0.000001)
        );
        assert_that!(
            mean_angle(&[Degrees(90.0), Degrees(180.0), Degrees(270.0)]).in_degrees(),
            close_to(180.0, 0.000001)
        );
        assert_that!(
            mean_angle(&[Degrees(20.0), Degrees(350.0)]).in_degrees(),
            close_to(5.0, 0.000001)
        );
    }

    #[cfg(feature = "std")]
    fn are_close<T: Float>(a: T, b: T) -> bool {
        (a - b).abs() < cast(1.0e-10).unwrap()
    }

    impl<T: Arbitrary> Arbitrary for Angle<T> {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let v = Arbitrary::arbitrary(g);
            if bool::arbitrary(g) {
                Radians(v)
            } else {
                Degrees(v)
            }
        }
    }
}
