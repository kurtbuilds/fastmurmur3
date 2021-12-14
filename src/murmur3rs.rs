#![allow(unreachable_code)]
use std::ops::Shl;
use crate::{
    match_fallthrough,
    match_fallthrough_reverse_branches,
    match_fallthrough_make_loops,
    match_fallthrough_make_match
};


#[inline]
pub fn hash(data: &[u8]) -> u128 {
    murmur3_x64_128(data, 0)
}

/// This macro only prints if we're in test mode.
macro_rules! test_println {
    ($($arg:tt)*) => {
        if cfg!(test) {
            println!($($arg)*)
        }
    }
}


#[inline]
pub fn murmur3_x64_128(data: &[u8], salt: u32) -> u128 {
    const C1: u64 = 0x87c3_7b91_1142_53d5;
    const C2: u64 = 0x4cf5_ad43_2745_937f;
    const C3: u64 = 0x52dc_e729;
    const C4: u64 = 0x3849_5ab5;
    const R1: u32 = 27;
    const R2: u32 = 31;
    const R3: u32 = 33;
    const M: u64 = 5;
    const BLOCK_SIZE: usize = 16;
    const HALF_BLOCK_SIZE: usize = BLOCK_SIZE / 2;
    let len = data.len();
    let full_block_len = data.len() / BLOCK_SIZE * BLOCK_SIZE;

    let mut h1: u64 = salt as u64;
    let mut h2: u64 = salt as u64;

    for slice in data[..full_block_len].chunks(BLOCK_SIZE) {
        let k1 = u64::from_le_bytes(unsafe {*(
            slice.as_ptr() as *const [u8; HALF_BLOCK_SIZE]
        )});
        let k2 = u64::from_le_bytes(unsafe {*(
            slice.as_ptr().offset(HALF_BLOCK_SIZE as isize) as *const [u8; HALF_BLOCK_SIZE]
        )});
        h1 ^= k1.wrapping_mul(C1).rotate_left(R2).wrapping_mul(C2);
        h1 = h1
            .rotate_left(R1)
            .wrapping_add(h2)
            .wrapping_mul(M)
            .wrapping_add(C3);
        h2 ^= k2.wrapping_mul(C2).rotate_left(R3).wrapping_mul(C1);
        h2 = h2
            .rotate_left(R2)
            .wrapping_add(h1)
            .wrapping_mul(M)
            .wrapping_add(C4);
    }

    let buf = &data[full_block_len..];
    let trailing_len = buf.len();
    let mut k1 = 0;
    let mut k2 = 0;
    // this macro saves about 8% on performance compared to a huge if statement.
    match_fallthrough!(trailing_len, {
        15 => k2 ^= (buf[14] as u64).shl(48),
        14 => k2 ^= (buf[13] as u64).shl(40),
        13 => k2 ^= (buf[12] as u64).shl(32),
        12 => k2 ^= (buf[11] as u64).shl(24),
        11 => k2 ^= (buf[10] as u64).shl(16),
        10 => k2 ^= (buf[9] as u64).shl(8),
        9 => {
            k2 ^= buf[8] as u64;
            k2 = k2.wrapping_mul(C2)
                .rotate_left(33)
                .wrapping_mul(C1);
            h2 ^= k2;
        },
        8 => k1 ^= (buf[7] as u64).shl(56),
        7 => k1 ^= (buf[6] as u64).shl(48),
        6 => k1 ^= (buf[5] as u64).shl(40),
        5 => k1 ^= (buf[4] as u64).shl(32),
        4 => k1 ^= (buf[3] as u64).shl(24),
        3 => k1 ^= (buf[2] as u64).shl(16),
        2 => k1 ^= (buf[1] as u64).shl(8),
        1 => k1 ^= buf[0] as u64,
        0 => {
            k1 = k1.wrapping_mul(C1)
                .rotate_left(R2)
                .wrapping_mul(C2);
            h1 ^= k1;
            break;
        },
        _ => unreachable!()
    });

    h1 ^= len as u64;
    h2 ^= len as u64;
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);
    h1 = fmix64(h1);
    h2 = fmix64(h2);
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);
    u128::from_ne_bytes(unsafe {*([h1, h2].as_ptr() as *const [u8; 16])})
}


trait XorShift {
    fn xor_shr(&self, shift: u32) -> Self;
}


impl XorShift for u64 {
    fn xor_shr(&self, shift: u32) -> Self {
        self ^ (self >> shift)
    }
}


fn fmix64(k: u64) -> u64 {
    const C1: u64 = 0xff51_afd7_ed55_8ccd;
    const C2: u64 = 0xc4ce_b9fe_1a85_ec53;
    const R: u32 = 33;
    k
        .xor_shr(R)
        .wrapping_mul(C1)
        .xor_shr(R)
        .wrapping_mul(C2)
        .xor_shr(R)
}