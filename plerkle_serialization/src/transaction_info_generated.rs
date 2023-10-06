// automatically generated by the FlatBuffers compiler, do not modify

// @generated

use crate::common_generated::*;
use crate::compiled_instruction_generated::*;
use core::cmp::Ordering;
use core::mem;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
pub const ENUM_MIN_TRANSACTION_VERSION: i8 = 0;
#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
pub const ENUM_MAX_TRANSACTION_VERSION: i8 = 1;
#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_TRANSACTION_VERSION: [TransactionVersion; 2] =
    [TransactionVersion::Legacy, TransactionVersion::V0];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct TransactionVersion(pub i8);
#[allow(non_upper_case_globals)]
impl TransactionVersion {
    pub const Legacy: Self = Self(0);
    pub const V0: Self = Self(1);

    pub const ENUM_MIN: i8 = 0;
    pub const ENUM_MAX: i8 = 1;
    pub const ENUM_VALUES: &'static [Self] = &[Self::Legacy, Self::V0];
    /// Returns the variant's name or "" if unknown.
    pub fn variant_name(self) -> Option<&'static str> {
        match self {
            Self::Legacy => Some("Legacy"),
            Self::V0 => Some("V0"),
            _ => None,
        }
    }
}
impl core::fmt::Debug for TransactionVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if let Some(name) = self.variant_name() {
            f.write_str(name)
        } else {
            f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
        }
    }
}
impl<'a> flatbuffers::Follow<'a> for TransactionVersion {
    type Inner = Self;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let b = flatbuffers::read_scalar_at::<i8>(buf, loc);
        Self(b)
    }
}

impl flatbuffers::Push for TransactionVersion {
    type Output = TransactionVersion;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<i8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for TransactionVersion {
    type Scalar = i8;
    #[inline]
    fn to_little_endian(self) -> i8 {
        self.0.to_le()
    }
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    fn from_little_endian(v: i8) -> Self {
        let b = i8::from_le(v);
        Self(b)
    }
}

impl<'a> flatbuffers::Verifiable for TransactionVersion {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        i8::run_verifier(v, pos)
    }
}

impl flatbuffers::SimpleToVerifyInSlice for TransactionVersion {}
pub enum TransactionInfoOffset {}
#[derive(Copy, Clone, PartialEq, Eq)]

pub struct TransactionInfo<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TransactionInfo<'a> {
    type Inner = TransactionInfo<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> TransactionInfo<'a> {
    pub const VT_IS_VOTE: flatbuffers::VOffsetT = 4;
    pub const VT_ACCOUNT_KEYS: flatbuffers::VOffsetT = 6;
    pub const VT_LOG_MESSAGES: flatbuffers::VOffsetT = 8;
    pub const VT_INNER_INSTRUCTIONS: flatbuffers::VOffsetT = 10;
    pub const VT_OUTER_INSTRUCTIONS: flatbuffers::VOffsetT = 12;
    pub const VT_SLOT: flatbuffers::VOffsetT = 14;
    pub const VT_SLOT_INDEX: flatbuffers::VOffsetT = 16;
    pub const VT_SEEN_AT: flatbuffers::VOffsetT = 18;
    pub const VT_SIGNATURE: flatbuffers::VOffsetT = 20;
    pub const VT_COMPILED_INNER_INSTRUCTIONS: flatbuffers::VOffsetT = 22;
    pub const VT_VERSION: flatbuffers::VOffsetT = 24;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TransactionInfo { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TransactionInfoArgs<'args>,
    ) -> flatbuffers::WIPOffset<TransactionInfo<'bldr>> {
        let mut builder = TransactionInfoBuilder::new(_fbb);
        builder.add_seen_at(args.seen_at);
        builder.add_slot(args.slot);
        if let Some(x) = args.compiled_inner_instructions {
            builder.add_compiled_inner_instructions(x);
        }
        if let Some(x) = args.signature {
            builder.add_signature(x);
        }
        if let Some(x) = args.slot_index {
            builder.add_slot_index(x);
        }
        if let Some(x) = args.outer_instructions {
            builder.add_outer_instructions(x);
        }
        if let Some(x) = args.inner_instructions {
            builder.add_inner_instructions(x);
        }
        if let Some(x) = args.log_messages {
            builder.add_log_messages(x);
        }
        if let Some(x) = args.account_keys {
            builder.add_account_keys(x);
        }
        builder.add_version(args.version);
        builder.add_is_vote(args.is_vote);
        builder.finish()
    }

    #[inline]
    pub fn is_vote(&self) -> bool {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<bool>(TransactionInfo::VT_IS_VOTE, Some(false))
                .unwrap()
        }
    }
    #[inline]
    pub fn account_keys(&self) -> Option<flatbuffers::Vector<'a, Pubkey>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, Pubkey>>>(
                    TransactionInfo::VT_ACCOUNT_KEYS,
                    None,
                )
        }
    }
    #[inline]
    pub fn log_messages(
        &self,
    ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>,
            >>(TransactionInfo::VT_LOG_MESSAGES, None)
        }
    }
    #[inline]
    pub fn inner_instructions(
        &self,
    ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<InnerInstructions<'a>>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<InnerInstructions>>,
            >>(TransactionInfo::VT_INNER_INSTRUCTIONS, None)
        }
    }
    #[inline]
    pub fn outer_instructions(
        &self,
    ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<CompiledInstruction<'a>>>>
    {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<CompiledInstruction>>,
            >>(TransactionInfo::VT_OUTER_INSTRUCTIONS, None)
        }
    }
    #[inline]
    pub fn slot(&self) -> u64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u64>(TransactionInfo::VT_SLOT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn slot_index(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<&str>>(TransactionInfo::VT_SLOT_INDEX, None)
        }
    }
    #[inline]
    pub fn seen_at(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(TransactionInfo::VT_SEEN_AT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn signature(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<&str>>(TransactionInfo::VT_SIGNATURE, None)
        }
    }
    #[inline]
    pub fn compiled_inner_instructions(
        &self,
    ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<CompiledInnerInstructions<'a>>>>
    {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<CompiledInnerInstructions>>,
            >>(TransactionInfo::VT_COMPILED_INNER_INSTRUCTIONS, None)
        }
    }
    #[inline]
    pub fn version(&self) -> TransactionVersion {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<TransactionVersion>(
                    TransactionInfo::VT_VERSION,
                    Some(TransactionVersion::Legacy),
                )
                .unwrap()
        }
    }
}

impl flatbuffers::Verifiable for TransactionInfo<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<bool>("is_vote", Self::VT_IS_VOTE, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, Pubkey>>>(
                "account_keys",
                Self::VT_ACCOUNT_KEYS,
                false,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<&'_ str>>,
            >>("log_messages", Self::VT_LOG_MESSAGES, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<InnerInstructions>>,
            >>("inner_instructions", Self::VT_INNER_INSTRUCTIONS, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<CompiledInstruction>>,
            >>("outer_instructions", Self::VT_OUTER_INSTRUCTIONS, false)?
            .visit_field::<u64>("slot", Self::VT_SLOT, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<&str>>(
                "slot_index",
                Self::VT_SLOT_INDEX,
                false,
            )?
            .visit_field::<i64>("seen_at", Self::VT_SEEN_AT, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<&str>>(
                "signature",
                Self::VT_SIGNATURE,
                false,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<CompiledInnerInstructions>>,
            >>(
                "compiled_inner_instructions",
                Self::VT_COMPILED_INNER_INSTRUCTIONS,
                false,
            )?
            .visit_field::<TransactionVersion>("version", Self::VT_VERSION, false)?
            .finish();
        Ok(())
    }
}
pub struct TransactionInfoArgs<'a> {
    pub is_vote: bool,
    pub account_keys: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, Pubkey>>>,
    pub log_messages: Option<
        flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>,
    >,
    pub inner_instructions: Option<
        flatbuffers::WIPOffset<
            flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<InnerInstructions<'a>>>,
        >,
    >,
    pub outer_instructions: Option<
        flatbuffers::WIPOffset<
            flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<CompiledInstruction<'a>>>,
        >,
    >,
    pub slot: u64,
    pub slot_index: Option<flatbuffers::WIPOffset<&'a str>>,
    pub seen_at: i64,
    pub signature: Option<flatbuffers::WIPOffset<&'a str>>,
    pub compiled_inner_instructions: Option<
        flatbuffers::WIPOffset<
            flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<CompiledInnerInstructions<'a>>>,
        >,
    >,
    pub version: TransactionVersion,
}
impl<'a> Default for TransactionInfoArgs<'a> {
    #[inline]
    fn default() -> Self {
        TransactionInfoArgs {
            is_vote: false,
            account_keys: None,
            log_messages: None,
            inner_instructions: None,
            outer_instructions: None,
            slot: 0,
            slot_index: None,
            seen_at: 0,
            signature: None,
            compiled_inner_instructions: None,
            version: TransactionVersion::Legacy,
        }
    }
}

pub struct TransactionInfoBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TransactionInfoBuilder<'a, 'b> {
    #[inline]
    pub fn add_is_vote(&mut self, is_vote: bool) {
        self.fbb_
            .push_slot::<bool>(TransactionInfo::VT_IS_VOTE, is_vote, false);
    }
    #[inline]
    pub fn add_account_keys(
        &mut self,
        account_keys: flatbuffers::WIPOffset<flatbuffers::Vector<'b, Pubkey>>,
    ) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            TransactionInfo::VT_ACCOUNT_KEYS,
            account_keys,
        );
    }
    #[inline]
    pub fn add_log_messages(
        &mut self,
        log_messages: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<&'b str>>,
        >,
    ) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            TransactionInfo::VT_LOG_MESSAGES,
            log_messages,
        );
    }
    #[inline]
    pub fn add_inner_instructions(
        &mut self,
        inner_instructions: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<InnerInstructions<'b>>>,
        >,
    ) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            TransactionInfo::VT_INNER_INSTRUCTIONS,
            inner_instructions,
        );
    }
    #[inline]
    pub fn add_outer_instructions(
        &mut self,
        outer_instructions: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<CompiledInstruction<'b>>>,
        >,
    ) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            TransactionInfo::VT_OUTER_INSTRUCTIONS,
            outer_instructions,
        );
    }
    #[inline]
    pub fn add_slot(&mut self, slot: u64) {
        self.fbb_
            .push_slot::<u64>(TransactionInfo::VT_SLOT, slot, 0);
    }
    #[inline]
    pub fn add_slot_index(&mut self, slot_index: flatbuffers::WIPOffset<&'b str>) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            TransactionInfo::VT_SLOT_INDEX,
            slot_index,
        );
    }
    #[inline]
    pub fn add_seen_at(&mut self, seen_at: i64) {
        self.fbb_
            .push_slot::<i64>(TransactionInfo::VT_SEEN_AT, seen_at, 0);
    }
    #[inline]
    pub fn add_signature(&mut self, signature: flatbuffers::WIPOffset<&'b str>) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            TransactionInfo::VT_SIGNATURE,
            signature,
        );
    }
    #[inline]
    pub fn add_compiled_inner_instructions(
        &mut self,
        compiled_inner_instructions: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<CompiledInnerInstructions<'b>>>,
        >,
    ) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            TransactionInfo::VT_COMPILED_INNER_INSTRUCTIONS,
            compiled_inner_instructions,
        );
    }
    #[inline]
    pub fn add_version(&mut self, version: TransactionVersion) {
        self.fbb_.push_slot::<TransactionVersion>(
            TransactionInfo::VT_VERSION,
            version,
            TransactionVersion::Legacy,
        );
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TransactionInfoBuilder<'a, 'b> {
        let start = _fbb.start_table();
        TransactionInfoBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<TransactionInfo<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for TransactionInfo<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("TransactionInfo");
        ds.field("is_vote", &self.is_vote());
        ds.field("account_keys", &self.account_keys());
        ds.field("log_messages", &self.log_messages());
        ds.field("inner_instructions", &self.inner_instructions());
        ds.field("outer_instructions", &self.outer_instructions());
        ds.field("slot", &self.slot());
        ds.field("slot_index", &self.slot_index());
        ds.field("seen_at", &self.seen_at());
        ds.field("signature", &self.signature());
        ds.field(
            "compiled_inner_instructions",
            &self.compiled_inner_instructions(),
        );
        ds.field("version", &self.version());
        ds.finish()
    }
}
pub enum InnerInstructionsOffset {}
#[derive(Copy, Clone, PartialEq, Eq)]

pub struct InnerInstructions<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for InnerInstructions<'a> {
    type Inner = InnerInstructions<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> InnerInstructions<'a> {
    pub const VT_INDEX: flatbuffers::VOffsetT = 4;
    pub const VT_INSTRUCTIONS: flatbuffers::VOffsetT = 6;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        InnerInstructions { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args InnerInstructionsArgs<'args>,
    ) -> flatbuffers::WIPOffset<InnerInstructions<'bldr>> {
        let mut builder = InnerInstructionsBuilder::new(_fbb);
        if let Some(x) = args.instructions {
            builder.add_instructions(x);
        }
        builder.add_index(args.index);
        builder.finish()
    }

    #[inline]
    pub fn index(&self) -> u8 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u8>(InnerInstructions::VT_INDEX, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn instructions(
        &self,
    ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<CompiledInstruction<'a>>>>
    {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<CompiledInstruction>>,
            >>(InnerInstructions::VT_INSTRUCTIONS, None)
        }
    }
}

impl flatbuffers::Verifiable for InnerInstructions<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<u8>("index", Self::VT_INDEX, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<CompiledInstruction>>,
            >>("instructions", Self::VT_INSTRUCTIONS, false)?
            .finish();
        Ok(())
    }
}
pub struct InnerInstructionsArgs<'a> {
    pub index: u8,
    pub instructions: Option<
        flatbuffers::WIPOffset<
            flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<CompiledInstruction<'a>>>,
        >,
    >,
}
impl<'a> Default for InnerInstructionsArgs<'a> {
    #[inline]
    fn default() -> Self {
        InnerInstructionsArgs {
            index: 0,
            instructions: None,
        }
    }
}

pub struct InnerInstructionsBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> InnerInstructionsBuilder<'a, 'b> {
    #[inline]
    pub fn add_index(&mut self, index: u8) {
        self.fbb_
            .push_slot::<u8>(InnerInstructions::VT_INDEX, index, 0);
    }
    #[inline]
    pub fn add_instructions(
        &mut self,
        instructions: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<CompiledInstruction<'b>>>,
        >,
    ) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            InnerInstructions::VT_INSTRUCTIONS,
            instructions,
        );
    }
    #[inline]
    pub fn new(
        _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> InnerInstructionsBuilder<'a, 'b> {
        let start = _fbb.start_table();
        InnerInstructionsBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<InnerInstructions<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for InnerInstructions<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("InnerInstructions");
        ds.field("index", &self.index());
        ds.field("instructions", &self.instructions());
        ds.finish()
    }
}
pub enum CompiledInnerInstructionsOffset {}
#[derive(Copy, Clone, PartialEq, Eq)]

pub struct CompiledInnerInstructions<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for CompiledInnerInstructions<'a> {
    type Inner = CompiledInnerInstructions<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> CompiledInnerInstructions<'a> {
    pub const VT_INDEX: flatbuffers::VOffsetT = 4;
    pub const VT_INSTRUCTIONS: flatbuffers::VOffsetT = 6;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        CompiledInnerInstructions { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args CompiledInnerInstructionsArgs<'args>,
    ) -> flatbuffers::WIPOffset<CompiledInnerInstructions<'bldr>> {
        let mut builder = CompiledInnerInstructionsBuilder::new(_fbb);
        if let Some(x) = args.instructions {
            builder.add_instructions(x);
        }
        builder.add_index(args.index);
        builder.finish()
    }

    #[inline]
    pub fn index(&self) -> u8 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u8>(CompiledInnerInstructions::VT_INDEX, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn instructions(
        &self,
    ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<CompiledInnerInstruction<'a>>>>
    {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<CompiledInnerInstruction>>,
            >>(CompiledInnerInstructions::VT_INSTRUCTIONS, None)
        }
    }
}

impl flatbuffers::Verifiable for CompiledInnerInstructions<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<u8>("index", Self::VT_INDEX, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<CompiledInnerInstruction>>,
            >>("instructions", Self::VT_INSTRUCTIONS, false)?
            .finish();
        Ok(())
    }
}
pub struct CompiledInnerInstructionsArgs<'a> {
    pub index: u8,
    pub instructions: Option<
        flatbuffers::WIPOffset<
            flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<CompiledInnerInstruction<'a>>>,
        >,
    >,
}
impl<'a> Default for CompiledInnerInstructionsArgs<'a> {
    #[inline]
    fn default() -> Self {
        CompiledInnerInstructionsArgs {
            index: 0,
            instructions: None,
        }
    }
}

pub struct CompiledInnerInstructionsBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> CompiledInnerInstructionsBuilder<'a, 'b> {
    #[inline]
    pub fn add_index(&mut self, index: u8) {
        self.fbb_
            .push_slot::<u8>(CompiledInnerInstructions::VT_INDEX, index, 0);
    }
    #[inline]
    pub fn add_instructions(
        &mut self,
        instructions: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<CompiledInnerInstruction<'b>>>,
        >,
    ) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            CompiledInnerInstructions::VT_INSTRUCTIONS,
            instructions,
        );
    }
    #[inline]
    pub fn new(
        _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> CompiledInnerInstructionsBuilder<'a, 'b> {
        let start = _fbb.start_table();
        CompiledInnerInstructionsBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<CompiledInnerInstructions<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for CompiledInnerInstructions<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("CompiledInnerInstructions");
        ds.field("index", &self.index());
        ds.field("instructions", &self.instructions());
        ds.finish()
    }
}
#[inline]
/// Verifies that a buffer of bytes contains a `TransactionInfo`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_transaction_info_unchecked`.
pub fn root_as_transaction_info(
    buf: &[u8],
) -> Result<TransactionInfo, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root::<TransactionInfo>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `TransactionInfo` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_transaction_info_unchecked`.
pub fn size_prefixed_root_as_transaction_info(
    buf: &[u8],
) -> Result<TransactionInfo, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root::<TransactionInfo>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `TransactionInfo` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_transaction_info_unchecked`.
pub fn root_as_transaction_info_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<TransactionInfo<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root_with_opts::<TransactionInfo<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `TransactionInfo` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_transaction_info_unchecked`.
pub fn size_prefixed_root_as_transaction_info_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<TransactionInfo<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root_with_opts::<TransactionInfo<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a TransactionInfo and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `TransactionInfo`.
pub unsafe fn root_as_transaction_info_unchecked(buf: &[u8]) -> TransactionInfo {
    flatbuffers::root_unchecked::<TransactionInfo>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed TransactionInfo and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `TransactionInfo`.
pub unsafe fn size_prefixed_root_as_transaction_info_unchecked(buf: &[u8]) -> TransactionInfo {
    flatbuffers::size_prefixed_root_unchecked::<TransactionInfo>(buf)
}
#[inline]
pub fn finish_transaction_info_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<TransactionInfo<'a>>,
) {
    fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_transaction_info_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<TransactionInfo<'a>>,
) {
    fbb.finish_size_prefixed(root, None);
}
