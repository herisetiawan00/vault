use rand::Rng;

pub fn key_generator(length: u32) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| if rng.gen_bool(0.5) { 1 } else { 0 })
        .collect()
}
