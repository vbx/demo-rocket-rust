use rand::Rng;

pub fn generate_random_number(max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..max)
}
