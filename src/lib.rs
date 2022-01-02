#![feature(new_uninit, const_type_id, vec_retain_mut)]
#[macro_use]
pub mod shadow_stack;
#[macro_use]
pub mod utils;
pub mod alloc;
pub mod api;
pub mod bitmap;
pub mod bump_pointer_space;
pub mod card_table;
pub mod concurrent_semispace;
pub mod gc_base;
pub mod generational;
pub mod immix;
pub mod large_space;
pub mod marksweep;
pub mod mutator;
pub mod rosalloc_space;
pub mod safepoint;
pub mod semispace;
pub mod shenandoah;
pub mod space;
pub mod tlab;
pub mod waitlists;
use std::any::TypeId;

pub use mopa;

const FNV_OFFSET_BASIS_32: u32 = 0x811c9dc5;

const FNV_PRIME_32: u32 = 0x01000193;

/// Computes 32-bits fnv1a hash of the given slice, or up-to limit if provided.
/// If limit is zero or exceeds slice length, slice length is used instead.
#[inline(always)]
const fn fnv1a_hash_32(bytes: &[u8], limit: Option<usize>) -> u32 {
    let mut hash = FNV_OFFSET_BASIS_32;

    let mut i = 0;
    let len = match limit {
        Some(v) => {
            if v <= bytes.len() && v > 0 {
                v
            } else {
                bytes.len()
            }
        }
        None => bytes.len(),
    };

    while i < len {
        hash ^= bytes[i] as u32;
        hash = hash.wrapping_mul(FNV_PRIME_32);
        i += 1;
    }
    hash
}

/*
/// Computes 32-bits fnv1a hash and XORs higher and lower 16-bits.
/// This results in a 16-bits hash value.
/// Up to limit if provided, otherwise slice length.
/// If limit is zero or exceeds slice length, slice length is used instead.
#[inline(always)]
const fn fnv1a_hash_16_xor(bytes: &[u8], limit: Option<usize>) -> u16 {
    let bytes = fnv1a_hash_32(bytes, limit).to_ne_bytes();
    let upper: u16 = u16::from_ne_bytes([bytes[0], bytes[1]]);
    let lower: u16 = u16::from_ne_bytes([bytes[2], bytes[3]]);
    upper ^ lower
}
*/

#[inline(always)]
pub(crate) const fn small_type_id<T: 'static>() -> u32 {
    unsafe {
        let bytes: [u8; std::mem::size_of::<TypeId>()] = std::mem::transmute(TypeId::of::<T>());
        fnv1a_hash_32(&bytes, None)
    }
}
