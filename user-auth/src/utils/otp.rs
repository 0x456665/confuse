use rand::Rng;

pub fn generate_otp(length: usize) -> String {
    let mut rng = rand::rng();
    (0..length)
        .map(|_| char::from_digit(rng.random_range(0..10), 10).unwrap())
        .collect()
}
