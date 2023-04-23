pub(crate) mod rand {
    use rand::distributions::Standard;
    pub use rand::*;

    pub fn random_range<T>(min: T, max: T) -> T
    where
        T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + std::ops::Sub<Output = T> + Copy,
        Standard: rand::distributions::Distribution<T>,
    {
        min + (max - min) * rand::random::<T>()
    }
}
