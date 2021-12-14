mod murmur3rs;
mod fallthrough;

pub use murmur3rs::{hash, murmur3_x64_128};

#[cfg(feature = "murmur3c")]
pub mod murmur3c;


#[cfg(test)]
mod test {
    use super::*;
    use rand::{Rng, RngCore};

    static SOURCE: &'static [u8] = b"The quick brown fox jumps over the lazy dog";

    #[test]
    fn test_agreement_basic() {
        let a = murmur3rs::murmur3_x64_128(SOURCE, 0);
        let b = murmur3c::hash(SOURCE);
        assert_eq!(a, b);
    }

    #[test]
    fn test_agreement_fuzzed() {
        let mut rng = rand::thread_rng();

        for i in 0..10000 {
            let len: u8 = rng.gen();
            let mut buf = vec![0; len as usize];
            rng.fill_bytes(&mut buf[..]);
            let salt: u32 = rng.gen();
            let a = murmur3rs::murmur3_x64_128(&buf, salt);
            let b = murmur3c::murmur3_x64_128(&buf, salt);
            assert_eq!(
                a, b,
                "Failed after {} iterations. salt={} data={}",
                i,
                salt,
                buf.iter()
                    .map(|b| format!("{:x}", b))
                    .collect::<String>(),
            );
        }
    }
}