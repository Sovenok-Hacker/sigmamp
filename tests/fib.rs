use sigmamp::{Limb, SigmaUInt};

fn fib(n: usize) -> SigmaUInt {
    if n == 0 {
        return SigmaUInt::zero();
    }

    let mut a = SigmaUInt::zero(); // F_0
    let mut b = SigmaUInt::from_usize(1); // F_1

    for _ in 0..(n - 1) {
        let temp = a.clone();
        a = b.clone();
        b += &temp;
    }
    b
}

#[test]
fn fib_1000() {
    #[cfg(target_pointer_width = "64")]
    const EXPECTED_LIMBS: [Limb; 11] = [
        817770325994397771,
        5516466794786742215,
        662231354435186987,
        3329585264503644336,
        2884134661581877441,
        8288363948632357276,
        13157175199224117635,
        8249032568704518803,
        12747505868792900890,
        6283290113689269178,
        9527040750744258,
    ];

    #[cfg(target_pointer_width = "32")]
    const EXPECTED_LIMBS: [Limb; 22] = [
        1556111435, 190401991, 2256560071, 1284402514, 2151428395, 154187752, 1008558256,
        775229480, 2751115457, 671514929, 4284660124, 1929785112, 1297430915, 3063393570,
        2118306451, 1920627562, 1771810074, 2968009996, 1865167802, 1462942481, 129331906, 2218187,
    ];

    assert_eq!(fib(1000), SigmaUInt::from_limbs(EXPECTED_LIMBS.to_vec()))
}
