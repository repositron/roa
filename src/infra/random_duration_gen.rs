use rand::Rng;

pub fn random_duration(from: i32, to: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(from..=to)
}