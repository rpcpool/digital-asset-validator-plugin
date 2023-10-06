#![allow(clippy::all)]
// automatically generated by the FlatBuffers compiler, do not modify

// @generated

use core::{cmp::Ordering, mem};

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
pub const ENUM_MIN_REWARD_TYPE: u8 = 0;
#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
pub const ENUM_MAX_REWARD_TYPE: u8 = 3;
#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_REWARD_TYPE: [RewardType; 4] = [
    RewardType::Fee,
    RewardType::Rent,
    RewardType::Staking,
    RewardType::Voting,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct RewardType(pub u8);
#[allow(non_upper_case_globals)]
impl RewardType {
    pub const Fee: Self = Self(0);
    pub const Rent: Self = Self(1);
    pub const Staking: Self = Self(2);
    pub const Voting: Self = Self(3);

    pub const ENUM_MIN: u8 = 0;
    pub const ENUM_MAX: u8 = 3;
    pub const ENUM_VALUES: &'static [Self] = &[Self::Fee, Self::Rent, Self::Staking, Self::Voting];
    /// Returns the variant's name or "" if unknown.
    pub fn variant_name(self) -> Option<&'static str> {
        match self {
            Self::Fee => Some("Fee"),
            Self::Rent => Some("Rent"),
            Self::Staking => Some("Staking"),
            Self::Voting => Some("Voting"),
            _ => None,
        }
    }
}
impl core::fmt::Debug for RewardType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if let Some(name) = self.variant_name() {
            f.write_str(name)
        } else {
            f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
        }
    }
}
impl<'a> flatbuffers::Follow<'a> for RewardType {
    type Inner = Self;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
        Self(b)
    }
}

impl flatbuffers::Push for RewardType {
    type Output = RewardType;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for RewardType {
    type Scalar = u8;
    #[inline]
    fn to_little_endian(self) -> u8 {
        self.0.to_le()
    }
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    fn from_little_endian(v: u8) -> Self {
        let b = u8::from_le(v);
        Self(b)
    }
}

impl<'a> flatbuffers::Verifiable for RewardType {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        u8::run_verifier(v, pos)
    }
}

impl flatbuffers::SimpleToVerifyInSlice for RewardType {}
pub enum RewardOffset {}
#[derive(Copy, Clone, PartialEq, Eq)]

pub struct Reward<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Reward<'a> {
    type Inner = Reward<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> Reward<'a> {
    pub const VT_PUBKEY: flatbuffers::VOffsetT = 4;
    pub const VT_LAMPORTS: flatbuffers::VOffsetT = 6;
    pub const VT_POST_BALANCE: flatbuffers::VOffsetT = 8;
    pub const VT_REWARD_TYPE: flatbuffers::VOffsetT = 10;
    pub const VT_COMMISSION: flatbuffers::VOffsetT = 12;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Reward { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args RewardArgs<'args>,
    ) -> flatbuffers::WIPOffset<Reward<'bldr>> {
        let mut builder = RewardBuilder::new(_fbb);
        builder.add_post_balance(args.post_balance);
        builder.add_lamports(args.lamports);
        if let Some(x) = args.pubkey {
            builder.add_pubkey(x);
        }
        if let Some(x) = args.commission {
            builder.add_commission(x);
        }
        if let Some(x) = args.reward_type {
            builder.add_reward_type(x);
        }
        builder.finish()
    }

    #[inline]
    pub fn pubkey(&self) -> Option<flatbuffers::Vector<'a, u8>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                    Reward::VT_PUBKEY,
                    None,
                )
        }
    }
    #[inline]
    pub fn lamports(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<i64>(Reward::VT_LAMPORTS, Some(0)).unwrap() }
    }
    #[inline]
    pub fn post_balance(&self) -> u64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u64>(Reward::VT_POST_BALANCE, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn reward_type(&self) -> Option<RewardType> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<RewardType>(Reward::VT_REWARD_TYPE, None) }
    }
    #[inline]
    pub fn commission(&self) -> Option<u8> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<u8>(Reward::VT_COMMISSION, None) }
    }
}

impl flatbuffers::Verifiable for Reward<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                "pubkey",
                Self::VT_PUBKEY,
                false,
            )?
            .visit_field::<i64>("lamports", Self::VT_LAMPORTS, false)?
            .visit_field::<u64>("post_balance", Self::VT_POST_BALANCE, false)?
            .visit_field::<RewardType>("reward_type", Self::VT_REWARD_TYPE, false)?
            .visit_field::<u8>("commission", Self::VT_COMMISSION, false)?
            .finish();
        Ok(())
    }
}
pub struct RewardArgs<'a> {
    pub pubkey: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    pub lamports: i64,
    pub post_balance: u64,
    pub reward_type: Option<RewardType>,
    pub commission: Option<u8>,
}
impl<'a> Default for RewardArgs<'a> {
    #[inline]
    fn default() -> Self {
        RewardArgs {
            pubkey: None,
            lamports: 0,
            post_balance: 0,
            reward_type: None,
            commission: None,
        }
    }
}

pub struct RewardBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> RewardBuilder<'a, 'b> {
    #[inline]
    pub fn add_pubkey(&mut self, pubkey: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(Reward::VT_PUBKEY, pubkey);
    }
    #[inline]
    pub fn add_lamports(&mut self, lamports: i64) {
        self.fbb_.push_slot::<i64>(Reward::VT_LAMPORTS, lamports, 0);
    }
    #[inline]
    pub fn add_post_balance(&mut self, post_balance: u64) {
        self.fbb_
            .push_slot::<u64>(Reward::VT_POST_BALANCE, post_balance, 0);
    }
    #[inline]
    pub fn add_reward_type(&mut self, reward_type: RewardType) {
        self.fbb_
            .push_slot_always::<RewardType>(Reward::VT_REWARD_TYPE, reward_type);
    }
    #[inline]
    pub fn add_commission(&mut self, commission: u8) {
        self.fbb_
            .push_slot_always::<u8>(Reward::VT_COMMISSION, commission);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> RewardBuilder<'a, 'b> {
        let start = _fbb.start_table();
        RewardBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<Reward<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for Reward<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("Reward");
        ds.field("pubkey", &self.pubkey());
        ds.field("lamports", &self.lamports());
        ds.field("post_balance", &self.post_balance());
        ds.field("reward_type", &self.reward_type());
        ds.field("commission", &self.commission());
        ds.finish()
    }
}
pub enum BlockInfoOffset {}
#[derive(Copy, Clone, PartialEq, Eq)]

pub struct BlockInfo<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for BlockInfo<'a> {
    type Inner = BlockInfo<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> BlockInfo<'a> {
    pub const VT_SLOT: flatbuffers::VOffsetT = 4;
    pub const VT_BLOCKHASH: flatbuffers::VOffsetT = 6;
    pub const VT_REWARDS: flatbuffers::VOffsetT = 8;
    pub const VT_BLOCK_TIME: flatbuffers::VOffsetT = 10;
    pub const VT_BLOCK_HEIGHT: flatbuffers::VOffsetT = 12;
    pub const VT_SEEN_AT: flatbuffers::VOffsetT = 14;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        BlockInfo { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args BlockInfoArgs<'args>,
    ) -> flatbuffers::WIPOffset<BlockInfo<'bldr>> {
        let mut builder = BlockInfoBuilder::new(_fbb);
        builder.add_seen_at(args.seen_at);
        if let Some(x) = args.block_height {
            builder.add_block_height(x);
        }
        if let Some(x) = args.block_time {
            builder.add_block_time(x);
        }
        builder.add_slot(args.slot);
        if let Some(x) = args.rewards {
            builder.add_rewards(x);
        }
        if let Some(x) = args.blockhash {
            builder.add_blockhash(x);
        }
        builder.finish()
    }

    #[inline]
    pub fn slot(&self) -> u64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<u64>(BlockInfo::VT_SLOT, Some(0)).unwrap() }
    }
    #[inline]
    pub fn blockhash(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<&str>>(BlockInfo::VT_BLOCKHASH, None)
        }
    }
    #[inline]
    pub fn rewards(
        &self,
    ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Reward<'a>>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Reward>>,
            >>(BlockInfo::VT_REWARDS, None)
        }
    }
    #[inline]
    pub fn block_time(&self) -> Option<i64> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<i64>(BlockInfo::VT_BLOCK_TIME, None) }
    }
    #[inline]
    pub fn block_height(&self) -> Option<u64> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<u64>(BlockInfo::VT_BLOCK_HEIGHT, None) }
    }
    #[inline]
    pub fn seen_at(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(BlockInfo::VT_SEEN_AT, Some(0))
                .unwrap()
        }
    }
}

impl flatbuffers::Verifiable for BlockInfo<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<u64>("slot", Self::VT_SLOT, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<&str>>(
                "blockhash",
                Self::VT_BLOCKHASH,
                false,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Reward>>,
            >>("rewards", Self::VT_REWARDS, false)?
            .visit_field::<i64>("block_time", Self::VT_BLOCK_TIME, false)?
            .visit_field::<u64>("block_height", Self::VT_BLOCK_HEIGHT, false)?
            .visit_field::<i64>("seen_at", Self::VT_SEEN_AT, false)?
            .finish();
        Ok(())
    }
}
pub struct BlockInfoArgs<'a> {
    pub slot: u64,
    pub blockhash: Option<flatbuffers::WIPOffset<&'a str>>,
    pub rewards: Option<
        flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Reward<'a>>>>,
    >,
    pub block_time: Option<i64>,
    pub block_height: Option<u64>,
    pub seen_at: i64,
}
impl<'a> Default for BlockInfoArgs<'a> {
    #[inline]
    fn default() -> Self {
        BlockInfoArgs {
            slot: 0,
            blockhash: None,
            rewards: None,
            block_time: None,
            block_height: None,
            seen_at: 0,
        }
    }
}

pub struct BlockInfoBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> BlockInfoBuilder<'a, 'b> {
    #[inline]
    pub fn add_slot(&mut self, slot: u64) {
        self.fbb_.push_slot::<u64>(BlockInfo::VT_SLOT, slot, 0);
    }
    #[inline]
    pub fn add_blockhash(&mut self, blockhash: flatbuffers::WIPOffset<&'b str>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(BlockInfo::VT_BLOCKHASH, blockhash);
    }
    #[inline]
    pub fn add_rewards(
        &mut self,
        rewards: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<Reward<'b>>>,
        >,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(BlockInfo::VT_REWARDS, rewards);
    }
    #[inline]
    pub fn add_block_time(&mut self, block_time: i64) {
        self.fbb_
            .push_slot_always::<i64>(BlockInfo::VT_BLOCK_TIME, block_time);
    }
    #[inline]
    pub fn add_block_height(&mut self, block_height: u64) {
        self.fbb_
            .push_slot_always::<u64>(BlockInfo::VT_BLOCK_HEIGHT, block_height);
    }
    #[inline]
    pub fn add_seen_at(&mut self, seen_at: i64) {
        self.fbb_
            .push_slot::<i64>(BlockInfo::VT_SEEN_AT, seen_at, 0);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> BlockInfoBuilder<'a, 'b> {
        let start = _fbb.start_table();
        BlockInfoBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<BlockInfo<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for BlockInfo<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("BlockInfo");
        ds.field("slot", &self.slot());
        ds.field("blockhash", &self.blockhash());
        ds.field("rewards", &self.rewards());
        ds.field("block_time", &self.block_time());
        ds.field("block_height", &self.block_height());
        ds.field("seen_at", &self.seen_at());
        ds.finish()
    }
}
#[inline]
/// Verifies that a buffer of bytes contains a `BlockInfo`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_block_info_unchecked`.
pub fn root_as_block_info(buf: &[u8]) -> Result<BlockInfo, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root::<BlockInfo>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `BlockInfo` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_block_info_unchecked`.
pub fn size_prefixed_root_as_block_info(
    buf: &[u8],
) -> Result<BlockInfo, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root::<BlockInfo>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `BlockInfo` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_block_info_unchecked`.
pub fn root_as_block_info_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<BlockInfo<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root_with_opts::<BlockInfo<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `BlockInfo` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_block_info_unchecked`.
pub fn size_prefixed_root_as_block_info_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<BlockInfo<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root_with_opts::<BlockInfo<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a BlockInfo and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `BlockInfo`.
pub unsafe fn root_as_block_info_unchecked(buf: &[u8]) -> BlockInfo {
    flatbuffers::root_unchecked::<BlockInfo>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed BlockInfo and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `BlockInfo`.
pub unsafe fn size_prefixed_root_as_block_info_unchecked(buf: &[u8]) -> BlockInfo {
    flatbuffers::size_prefixed_root_unchecked::<BlockInfo>(buf)
}
#[inline]
pub fn finish_block_info_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<BlockInfo<'a>>,
) {
    fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_block_info_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<BlockInfo<'a>>,
) {
    fbb.finish_size_prefixed(root, None);
}
