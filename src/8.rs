use std::ops::Mul;

fn multiply<T>(a: T, b: T) -> T
where
    T: Mul<Output = T> + Copy,
{
    a * b
}
