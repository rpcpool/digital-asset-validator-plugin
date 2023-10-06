#![allow(clippy::all)]
// automatically generated by the FlatBuffers compiler, do not modify

// @generated

use core::{cmp::Ordering, mem};

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

// struct Pubkey, aligned to 1
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pubkey(pub [u8; 32]);
impl Default for Pubkey {
    fn default() -> Self {
        Self([0; 32])
    }
}
impl core::fmt::Debug for Pubkey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pubkey").field("key", &self.key()).finish()
    }
}

impl flatbuffers::SimpleToVerifyInSlice for Pubkey {}
impl<'a> flatbuffers::Follow<'a> for Pubkey {
    type Inner = &'a Pubkey;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        <&'a Pubkey>::follow(buf, loc)
    }
}
impl<'a> flatbuffers::Follow<'a> for &'a Pubkey {
    type Inner = &'a Pubkey;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        flatbuffers::follow_cast_ref::<Pubkey>(buf, loc)
    }
}
impl<'b> flatbuffers::Push for Pubkey {
    type Output = Pubkey;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = ::core::slice::from_raw_parts(self as *const Pubkey as *const u8, Self::size());
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for Pubkey {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.in_buffer::<Self>(pos)
    }
}

impl<'a> Pubkey {
    #[allow(clippy::too_many_arguments)]
    pub fn new(key: &[u8; 32]) -> Self {
        let mut s = Self([0; 32]);
        s.set_key(key);
        s
    }

    pub fn key(&'a self) -> flatbuffers::Array<'a, u8, 32> {
        // Safety:
        // Created from a valid Table for this object
        // Which contains a valid array in this slot
        unsafe { flatbuffers::Array::follow(&self.0, 0) }
    }

    pub fn set_key(&mut self, items: &[u8; 32]) {
        // Safety:
        // Created from a valid Table for this object
        // Which contains a valid array in this slot
        unsafe { flatbuffers::emplace_scalar_array(&mut self.0, 0, items) };
    }
}
