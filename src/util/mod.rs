pub mod iter;

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    let gcdab = gcd(a, b);
    (a / gcdab) * b
}
