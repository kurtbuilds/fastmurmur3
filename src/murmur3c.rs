use core::ffi::c_void;


#[link(name = "murmur3")]
extern "C" {
    fn MurmurHash3_x86_32(key: *const c_void, len: i32, seed: i32, out: *const c_void) -> ();
    // fn MurmurHash3_x86_128(const void *key, int len, uint32_t seed, void *out);
    fn MurmurHash3_x64_128(key: *const c_void, len: i32, seed: i32, out: *const c_void) -> ();
}

#[cfg(target_pointer_width = "64")]
pub fn hash(key: &[u8]) -> u128 {
    let mut buf = [0u8; 16];
    unsafe {
        MurmurHash3_x64_128(key.as_ptr() as *const c_void, key.len() as i32, 0, &mut buf as *mut _ as *mut c_void);
    }
    if cfg!(target_endian = "big") {
        u128::from_be_bytes(buf)
    } else {
        u128::from_le_bytes(buf)
    }
}