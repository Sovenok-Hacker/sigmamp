use sigmamp::{Limb, SigmaUInt};

#[test]
fn add_test() {
    let mut num1 = SigmaUInt::from_usize(60);
    let num2 = SigmaUInt::from_usize(7);

    num1 += &num2;

    assert_eq!(num1, SigmaUInt::from_usize(67));
}

#[test]
fn add_with_carry_test() {
    let mut num1 = SigmaUInt::from_usize(Limb::MAX as usize);
    let num2 = SigmaUInt::from_usize(1);

    num1 += &num2;

    assert_eq!(num1, SigmaUInt::from_limbs(vec![0, 1]))
}

#[test]
fn add_deep_ripple_carry_test() {
    // Create a number: [MAX, MAX, MAX]
    // In hex (64-bit limbs): 0xFFFFFFFFFFFFFFFF_FFFFFFFFFFFFFFFF_FFFFFFFFFFFFFFFF
    let mut num1 = SigmaUInt::from_limbs(vec![Limb::MAX, Limb::MAX, Limb::MAX]);
    let num2 = SigmaUInt::from_usize(1);

    num1 += &num2;

    // The result should be [0, 0, 0, 1]
    // Because:
    // Limb 0: MAX + 1 = 0, carry 1
    // Limb 1: MAX + carry(1) = 0, carry 1
    // Limb 2: MAX + carry(1) = 0, carry 1
    // Final: push carry(1)
    let expected = SigmaUInt::from_limbs(vec![0, 0, 0, 1]);

    assert_eq!(num1, expected, "Carry failed to ripple through all limbs!");
}

#[test]
fn add_different_length_test() {
    // num1: [1]
    // num2: [0, 1] (which is 2^64)
    let mut num1 = SigmaUInt::from_usize(1);
    let num2 = SigmaUInt::from_limbs(vec![0, 1]);

    num1 += &num2;

    // Result: [1, 1]
    assert_eq!(num1, SigmaUInt::from_limbs(vec![1, 1]));
}

#[test]
fn trailing_zeros_invariant_test() {
    // These should be mathematically identical in your library
    let num_clean = SigmaUInt::from_limbs(vec![5]);
    let num_dirty = SigmaUInt::from_limbs(vec![5, 0, 0, 0]);

    // Test PartialEq
    assert_eq!(
        num_clean, num_dirty,
        "from_limbs must strip trailing zeros for equality to work"
    );

    // Test Length (Internal Integrity)
    assert_eq!(
        num_dirty.to_limbs().len(),
        1,
        "Internal vector should have been trimmed to length 1"
    );
}

#[test]
fn comparison_significance_test() {
    // 10 + 1*B^1  (Smaller total value)
    let small = SigmaUInt::from_limbs(vec![10, 1]);
    // 1 + 10*B^1  (Larger total value)
    let large = SigmaUInt::from_limbs(vec![1, 10]);

    assert!(
        large > small,
        "Comparison must check most-significant limbs first"
    );
}

#[test]
fn add_to_equality_test() {
    let mut num = SigmaUInt::from_limbs(vec![Limb::MAX]);
    num += &SigmaUInt::from_usize(1);

    // The result of MAX + 1 should be exactly [0, 1]
    let expected = SigmaUInt::from_limbs(vec![0, 1]);

    assert_eq!(num, expected);
    assert_eq!(num.to_limbs().len(), 2);
}

#[test]
fn zero_representation_test() {
    let zero1 = SigmaUInt::from_usize(0);
    let zero2 = SigmaUInt::from_limbs(vec![0, 0, 0]);
    let zero3 = SigmaUInt::from_limbs(vec![]);

    assert_eq!(zero1, zero2);
    assert_eq!(zero1, zero3);
    // Ensure that even 'zero' doesn't accidentally have trailing zeros like [0, 0]
    assert!(zero1.to_limbs().len() <= 1);
}

#[test]
fn sub_basic_test() {
    let mut a = SigmaUInt::from_usize(100);
    let b = SigmaUInt::from_usize(40);
    a -= &b;
    assert_eq!(a, SigmaUInt::from_usize(60));
}

#[test]
fn sub_deep_borrow_test() {
    // a = [0, 0, 1]  => 1 * B^2
    let mut a = SigmaUInt::from_limbs(vec![0, 0, 1]);
    let b = SigmaUInt::from_usize(1);

    a -= &b;

    // Expected: [MAX, MAX]
    // The high-order 1 becomes 0 and should be trimmed!
    let expected = SigmaUInt::from_limbs(vec![Limb::MAX, Limb::MAX]);

    assert_eq!(
        a, expected,
        "Borrow failed to ripple or high-order zero wasn't trimmed"
    );
    assert_eq!(
        a.to_limbs().len(),
        2,
        "The leading zero limb [0, MAX, MAX] was not popped"
    );
}

#[test]
fn sub_to_zero_test() {
    let mut a = SigmaUInt::from_limbs(vec![1, 2, 3]);
    let b = SigmaUInt::from_limbs(vec![1, 2, 3]);

    a -= &b;

    // Based on your 'zero' logic, this should result in an empty limbs vector
    assert_eq!(
        a.to_limbs().len(),
        0,
        "Subtracting a number from itself should leave no limbs"
    );
}

#[test]
#[should_panic(expected = "Overflow, subtraction resulted in negative number")]
fn sub_underflow_panic_test() {
    let mut a = SigmaUInt::from_usize(10);
    let b = SigmaUInt::from_usize(20);

    // This should trigger your panic
    a -= &b;
}

#[test]
fn sub_varying_length_borrow_test() {
    // a = [0, 1] (2^64)
    // b = [1] (1)
    let mut a = SigmaUInt::from_limbs(vec![0, 1]);
    let b = SigmaUInt::from_usize(1);

    a -= &b;

    // Result should be [MAX]
    assert_eq!(a, SigmaUInt::from_limbs(vec![Limb::MAX]));
}
