use sigmamp::SigmaUInt;

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

fn main() {
    println!("{:?}", fib(1_00_000));
}
