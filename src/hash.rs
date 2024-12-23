use sha2::{Sha256, Digest};

pub fn hash_value(num: usize) -> String {
    let mut hasher = Sha256::new();
    hasher.update(num.to_string().as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}