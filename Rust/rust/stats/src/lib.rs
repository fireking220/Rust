// Copyright © 2019 Scott Patterson
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

///! Functions to compute various statistics on a slice of
///! floating-point numbers.

/// Type of statistics function. If the statistic
/// is ill-defined, `None` will be returned.
pub type StatFn = fn(&[f64]) -> Option<f64>;
use num::pow;
/// Arithmetic mean of input values. The mean of an empty
/// list is 0.0.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), mean(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), mean(&[-1.0, 1.0]));
/// ```
pub fn mean(nums: &[f64]) -> Option<f64> {
    //unimplemented!("no mean yet")
    //Some(1.3_f64)
    if nums.is_empty() {
        return Some(0.0_f64);
    }
    let mut my_mean: f64 = 0.0;
    let len = nums.len() as f64;
    for x in nums {
        my_mean += x;
    }
    Some(my_mean / len)
}

#[test]
fn test_mean_1() {
    assert_eq!(Some(2.5), mean(&[1.0, 2.0, 3.0, 4.0]));
}

/// Population standard deviation of input values. The
/// standard deviation of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(None, stddev(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), stddev(&[1.0, 1.0]));
/// ```
pub fn stddev(nums: &[f64]) -> Option<f64> {
    if nums.is_empty() {
        return None;
    }
    let len = nums.len() as f64;
    let mean = mean(nums).unwrap();
    let mut sum = 0.0_f64;
    // sum of squares
    for x in nums {
        sum += pow(x - mean, 2);
    }
    sum /= len;
    sum = sum.sqrt();
    Some(sum)
}

#[test]
fn test_stddev_1() {
    assert_eq!(Some(0.0), stddev(&[2.0]));
}

/// Median value of input values, taking the value closer
/// to the beginning to break ties. The median
/// of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(None, median(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), median(&[0.0, 0.5, -1.0, 1.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(6.0), median(&[6.0, 2.0, 20.5]));
/// ```
pub fn median(nums: &[f64]) -> Option<f64> {
    // Make a sorted copy of the input floats.
    let mut nums = nums.to_owned();
    // https://users.rust-lang.org/t/how-to-sort-a-vec-of-floats/2838/2
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = nums.len() as i64;
    let mut ret = 0.0_f64;
    // if array is empty
    if len == 0 {
        return None;
    }
    //if array length is even
    else if len % 2 == 0 {
        let even = ((len / 2) - 1) as usize;
        ret = nums[even] as f64;
    }
    //if array length is odd
    else if len % 2 == 1 {
        let odd = (len / 2) as usize;
        ret = nums[odd] as f64;
    }
    Some(ret)
}

#[test]
fn test_median_odd() {
    assert_eq!(Some(6.0), median(&[6.0, 2.0, 20.5]));
}

#[test]
fn test_median_even() {
    assert_eq!(Some(6.0), median(&[6.0, 2.0, 20.5, 7.25]));
}

/// L2 norm (Euclidean norm) of input values. The L2
/// norm of an empty list is 0.0.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), l2(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(5.0), l2(&[-3.0, 4.0]));
/// ```
pub fn l2(nums: &[f64]) -> Option<f64> {
    if nums.is_empty() {
        return Some(0.0_f64);
    }
    let mut sum = 0.0_f64;
    // x gets the address of the element in nums
    for x in nums {
        // * dereferences the addess that x points to
        sum += pow(*x, 2);
    }
    sum = sum.sqrt();
    Some(sum)
}

#[test]
fn test_l2_1() {
    assert_eq!(Some(2.0), l2(&[2.0]));
}
