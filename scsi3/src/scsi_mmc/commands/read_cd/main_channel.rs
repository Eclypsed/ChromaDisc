use core::{
    mem::{offset_of, size_of},
    num::NonZeroU32,
};

use arbitrary_int::u3;
use derive_where::derive_where;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, LittleEndian, Unaligned, U32};

use crate::{
    core::util::{Absent, Presence, Present},
    mmc::device_models::cd::main_channel::{
        edc,
        mode2::{Mode2Form, Mode2FormedSubHeader},
        DataMode, SectorHeader, SYNC_PATTERN,
    },
};

mod sealed {
    pub trait SectorModeSeal {}
    pub trait SelectionSeal {
        // Hiding these for now. May want/need to make them public as part of `Selection` in the future
        type Sync: super::Presence;
        type Header: super::Presence;
        type SubHeader: super::Presence;
        type UserData: super::Presence;
        type EdcEcc: super::Presence;
    }
    pub trait SelectionOfSeal<M: super::SectorMode> {}
}

pub trait SectorMode: sealed::SectorModeSeal + Sized + 'static {
    const EXPECTED_SECTOR_TYPE: u3;

    // Leaving these public for now. May want to hide them as part of `SectorModeSeal` in the future
    type Sync: FromBytes + IntoBytes + Immutable + KnownLayout + Unaligned;
    type Header: FromBytes + IntoBytes + Immutable + KnownLayout + Unaligned;
    type SubHeader: FromBytes + IntoBytes + Immutable + KnownLayout + Unaligned;
    type UserData: FromBytes + IntoBytes + Immutable + KnownLayout + Unaligned;
    type EdcEcc: FromBytes + IntoBytes + Immutable + KnownLayout + Unaligned;

    type Full: SelectionOf<Self>;
}

pub trait DataSectorMode: SectorMode<Header = SectorHeader> {
    const DATA_MODE: DataMode;
}

// TODO: Change SubHeader type and figure out how to implement a mode2_form_matches method that
// reconciles the two copies of Mode2FormedSubHeader.
pub trait Mode2FormedSectorMode: DataSectorMode<SubHeader = [Mode2FormedSubHeader; 2]> {
    const MODE2_FORM: Mode2Form;

    // Rust gods please give me a complete implementation of `generic_const_exprs` feature
    #[doc(hidden)]
    const ASSERT_MODE2: () = assert!(
        matches!(Self::DATA_MODE, DataMode::Mode2),
        "Mode2FormedSectorMode requires DATA_MODE == DataMode::Mode2"
    );
}

pub trait Selection: sealed::SelectionSeal {}

#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a legal `READ CD` selection for `{M}`",
    label = "these subsections are non-contiguous, or absent from this mode",
    note = "e.g. SyncHeaderUserData (B0h) skips the Mode 2 sub-header"
)]
pub trait SelectionOf<M: SectorMode>: Selection + sealed::SelectionOfSeal<M> + 'static {}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[derive_where(Debug, Clone, Copy, PartialEq, Eq;
    <L::Sync as Presence>::Output<M::Sync>,
    <L::Header as Presence>::Output<M::Header>,
    <L::SubHeader as Presence>::Output<M::SubHeader>,
    <L::UserData as Presence>::Output<M::UserData>,
    <L::EdcEcc as Presence>::Output<M::EdcEcc>)]
#[repr(C)]
pub struct MainChannel<M: SectorMode, L: SelectionOf<M>> {
    sync: <L::Sync as Presence>::Output<M::Sync>,
    header: <L::Header as Presence>::Output<M::Header>,
    sub_header: <L::SubHeader as Presence>::Output<M::SubHeader>,
    user_data: <L::UserData as Presence>::Output<M::UserData>,
    edc_ecc: <L::EdcEcc as Presence>::Output<M::EdcEcc>,
}

pub type Sector<T> = MainChannel<T, <T as SectorMode>::Full>;

#[inline]
const fn bit<P: Presence>(pos: u8) -> u8 {
    (P::PRESENT as u8) << pos
}

impl<M: SectorMode, L: SelectionOf<M>> MainChannel<M, L> {
    pub(super) const SELECTION_VALUE: u8 = bit::<L::Sync>(7)
        | bit::<L::SubHeader>(6)
        | bit::<L::Header>(5)
        | bit::<L::UserData>(4)
        | bit::<L::EdcEcc>(3);
}

impl<M: SectorMode, L: SelectionOf<M, Sync = Present>> MainChannel<M, L> {
    #[inline]
    pub const fn sync(&self) -> &M::Sync {
        &self.sync
    }

    #[inline]
    pub const fn sync_mut(&mut self) -> &mut M::Sync {
        &mut self.sync
    }
}

impl<M: SectorMode<Sync = [u8; 12]>, L: SelectionOf<M, Sync = Present>> MainChannel<M, L> {
    #[inline]
    pub fn sync_is_valid(&self) -> bool {
        *self.sync() == SYNC_PATTERN
    }
}

impl<M: SectorMode, L: SelectionOf<M, Header = Present>> MainChannel<M, L> {
    #[inline]
    pub const fn header(&self) -> &M::Header {
        &self.header
    }

    #[inline]
    pub const fn header_mut(&mut self) -> &mut M::Header {
        &mut self.header
    }
}

impl<M: DataSectorMode, L: SelectionOf<M, Header = Present>> MainChannel<M, L> {
    #[inline]
    pub fn data_mode_matches(&self) -> bool {
        self.header().mode.data_mode() == M::DATA_MODE
    }
}

impl<M: SectorMode, L: SelectionOf<M, SubHeader = Present>> MainChannel<M, L> {
    #[inline]
    pub const fn sub_header(&self) -> &M::SubHeader {
        &self.sub_header
    }

    #[inline]
    pub const fn sub_header_mut(&mut self) -> &mut M::SubHeader {
        &mut self.sub_header
    }
}

impl<M: SectorMode, L: SelectionOf<M, UserData = Present>> MainChannel<M, L> {
    #[inline]
    pub const fn user_data(&self) -> &M::UserData {
        &self.user_data
    }

    #[inline]
    pub const fn user_data_mut(&mut self) -> &mut M::UserData {
        &mut self.user_data
    }
}

impl<M: SectorMode, L: SelectionOf<M, EdcEcc = Present>> MainChannel<M, L> {
    #[inline]
    pub const fn edc_ecc(&self) -> &M::EdcEcc {
        &self.edc_ecc
    }

    #[inline]
    pub const fn edc_ecc_mut(&mut self) -> &mut M::EdcEcc {
        &mut self.edc_ecc
    }
}

impl<L: SelectionOf<Mode1, Sync = Present, Header = Present, UserData = Present>>
    MainChannel<Mode1, L>
{
    #[inline]
    pub fn calculate_edc(&self) -> u32 {
        edc(&self.as_bytes()[(offset_of!(Self, sync))..(offset_of!(Self, edc_ecc))])
    }
}

impl<
        L: SelectionOf<Mode1, Sync = Present, Header = Present, UserData = Present, EdcEcc = Present>,
    > MainChannel<Mode1, L>
{
    #[inline]
    pub fn edc_matches(&self) -> bool {
        self.calculate_edc() == self.edc_ecc().edc.get()
    }
}

impl<L: SelectionOf<Mode2Form1, SubHeader = Present, UserData = Present>>
    MainChannel<Mode2Form1, L>
{
    #[inline]
    pub fn calculate_edc(&self) -> u32 {
        edc(&self.as_bytes()[(offset_of!(Self, sub_header))..(offset_of!(Self, edc_ecc))])
    }
}

impl<L: SelectionOf<Mode2Form1, SubHeader = Present, UserData = Present, EdcEcc = Present>>
    MainChannel<Mode2Form1, L>
{
    #[inline]
    pub fn edc_matches(&self) -> bool {
        self.calculate_edc() == self.edc_ecc().edc.get()
    }
}

impl<L: SelectionOf<Mode2Form2, SubHeader = Present, UserData = Present>>
    MainChannel<Mode2Form2, L>
{
    #[inline]
    pub fn calculate_edc(&self) -> u32 {
        edc(&self.as_bytes()[(offset_of!(Self, sub_header))..(offset_of!(Self, edc_ecc))])
    }
}

impl<L: SelectionOf<Mode2Form2, SubHeader = Present, UserData = Present, EdcEcc = Present>>
    MainChannel<Mode2Form2, L>
{
    #[inline]
    pub fn edc_matches(&self) -> Option<bool> {
        Some(self.calculate_edc() == self.edc_ecc().edc()?.into())
    }
}

pub mod selections {
    use super::{sealed::SelectionSeal, Absent, Present, Selection};

    // ---- field lookups -------------------------------------------------------
    // Each answers "is <name> in this field list?" with Present or Absent.
    // macro_rules! can't compare two idents, so the name has to appear literally
    // in a matcher — hence one small macro per field. The literal arm must come
    // before the catch-all, or the catch-all swallows it.

    macro_rules! has_sync {
        () => { Absent };
        (sync $($rest:ident)*) => { Present };
        ($skip:ident $($rest:ident)*) => { has_sync!($($rest)*) };
    }

    macro_rules! has_header {
        () => { Absent };
        (header $($rest:ident)*) => { Present };
        ($skip:ident $($rest:ident)*) => { has_header!($($rest)*) };
    }

    macro_rules! has_sub_header {
        () => { Absent };
        (sub_header $($rest:ident)*) => { Present };
        ($skip:ident $($rest:ident)*) => { has_sub_header!($($rest)*) };
    }

    macro_rules! has_user_data {
        () => { Absent };
        (user_data $($rest:ident)*) => { Present };
        ($skip:ident $($rest:ident)*) => { has_user_data!($($rest)*) };
    }

    macro_rules! has_edc_ecc {
        () => { Absent };
        (edc_ecc $($rest:ident)*) => { Present };
        ($skip:ident $($rest:ident)*) => { has_edc_ecc!($($rest)*) };
    }

    // typo guard
    macro_rules! check_field {
        (sync) => {};
        (header) => {};
        (sub_header) => {};
        (user_data) => {};
        (edc_ecc) => {};
        ($other:ident) => {
            compile_error!(concat!(
                "unknown selection field `",
                stringify!($other),
                "`; expected one of: sync, header, sub_header, user_data, edc_ecc"
            ))
        };
    }

    macro_rules! impl_selections {
        ($($marker:ident = [$($field:ident),* $(,)?];)*) => {$(
            pub struct $marker;

            impl SelectionSeal for $marker {
                type Sync      = has_sync!($($field)*);
                type Header    = has_header!($($field)*);
                type SubHeader = has_sub_header!($($field)*);
                type UserData  = has_user_data!($($field)*);
                type EdcEcc    = has_edc_ecc!($($field)*);
            }

            impl Selection for $marker {}

            const _: () = { $(check_field!($field);)* };
        )*};
    }

    impl_selections! {
        NoFields                     = []; // 0x00
        EdcEcc                       = [edc_ecc]; // 0x08
        UserData                     = [user_data]; // 0x10
        UserDataEdcEcc               = [user_data, edc_ecc]; // 0x18
        Header                       = [header]; // 0x20
        HeaderEdcEcc                 = [header, edc_ecc]; // 0x28
        HeaderUserData               = [header, user_data]; // 0x30
        HeaderUserDataEdcEcc         = [header, user_data, edc_ecc]; // 0x38
        SubHeader                    = [sub_header]; // 0x40
        SubHeaderEdcEcc              = [sub_header, edc_ecc]; // 0x48
        SubHeaderUserData            = [sub_header, user_data]; // 0x50
        SubHeaderUserDataEdcEcc      = [sub_header, user_data, edc_ecc]; // 0x58
        AllHeaders                   = [header, sub_header]; // 0x60
        AllHeadersEdcEcc             = [header, sub_header, edc_ecc]; // 0x68
        AllHeadersUserData           = [header, sub_header, user_data]; // 0x70
        AllHeadersUserDataEdcEcc     = [header, sub_header, user_data, edc_ecc]; // 0x78
        Sync                         = [sync]; // 0x80
        SyncEdcEcc                   = [sync, edc_ecc]; // 0x88
        SyncUserData                 = [sync, user_data]; // 0x90
        SyncUserDataEdcEcc           = [sync, user_data, edc_ecc]; // 0x98
        SyncHeader                   = [sync, header]; // 0xA0
        SyncHeaderEdcEcc             = [sync, header, edc_ecc]; // 0xA8
        SyncHeaderUserData           = [sync, header, user_data]; // 0xB0
        SyncHeaderUserDataEdcEcc     = [sync, header, user_data, edc_ecc]; // 0xB8
        SyncSubHeader                = [sync, sub_header]; // 0xC0
        SyncSubHeaderEdcEcc          = [sync, sub_header, edc_ecc]; // 0xC8
        SyncSubHeaderUserData        = [sync, sub_header, user_data]; // 0xD0
        SyncSubHeaderUserDataEdcEcc  = [sync, sub_header, user_data, edc_ecc]; // 0xD8
        SyncAllHeaders               = [sync, header, sub_header]; // 0xE0
        SyncAllHeadersEdcEcc         = [sync, header, sub_header, edc_ecc]; // 0xE8
        SyncAllHeadersUserData       = [sync, header, sub_header, user_data]; // 0xF0
        SyncAllHeadersUserDataEdcEcc = [sync, header, sub_header, user_data, edc_ecc]; // 0xF8
    }
}

use selections::*;

// ---- legality of a (mode, selection) pair -------------------------------
//
// Both halves of the rule are reconstructed from consts that already exist:
// `Presence::PRESENT` says what the selection asks for, and a zero-sized field
// type says the mode has no such subsection. Since `impl_layout_of!` expands
// with concrete types, the `const _` blocks below are evaluated eagerly rather
// than post-monomorphisation.

/// What the selection asks for, in **physical order within the sector**.
///
/// Note this is not the order used by `SELECTION_VALUE`, where the sub-header
/// outranks the header. Indices: 0 sync, 1 header, 2 sub-header, 3 user data,
/// 4 EDC/ECC.
const fn requested<L: Selection>() -> [bool; 5] {
    [
        <L::Sync as Presence>::PRESENT,
        <L::Header as Presence>::PRESENT,
        <L::SubHeader as Presence>::PRESENT,
        <L::UserData as Presence>::PRESENT,
        <L::EdcEcc as Presence>::PRESENT,
    ]
}

/// Which subsections the mode actually has, same order.
///
/// A mode marks an absent subsection with `()`, so zero size means absent. No
/// real field type is zero-sized.
const fn available<M: SectorMode>() -> [bool; 5] {
    [
        size_of::<M::Sync>() != 0,
        size_of::<M::Header>() != 0,
        size_of::<M::SubHeader>() != 0,
        size_of::<M::UserData>() != 0,
        size_of::<M::EdcEcc>() != 0,
    ]
}

/// Nothing is requested that this mode does not have.
const fn mode_has_all(requested: [bool; 5], available: [bool; 5]) -> bool {
    let mut i = 0;
    while i < 5 {
        if requested[i] && !available[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// The requested subsections form one unbroken run, so a drive can return them
/// as a single contiguous span.
///
/// Subsections the mode does not have are transparent: Mode 1 stays contiguous
/// across the sub-header slot, which is why 30h and B0h are legal there and not
/// for the formed Mode 2 variants.
const fn is_contiguous(requested: [bool; 5], available: [bool; 5]) -> bool {
    let mut i = 0;
    let mut state = 0u8; // 0 = before the run, 1 = inside it, 2 = past it
    while i < 5 {
        if available[i] {
            if requested[i] {
                if state == 2 {
                    return false; // run resumed after a gap
                }
                state = 1;
            } else if state == 1 {
                state = 2;
            }
        }
        i += 1;
    }
    true
}

macro_rules! impl_selection_of {
    ($sector_mode:ty, [$($selection:ty),* $(,)?]) => {$(
        impl sealed::SelectionOfSeal<$sector_mode> for $selection {}
        impl SelectionOf<$sector_mode> for $selection {}

        const _: () = {
            let requested = requested::<$selection>();
            let available = available::<$sector_mode>();
            assert!(
                mode_has_all(requested, available),
                concat!(
                    "`", stringify!($layout), "` requests a subsection that `",
                    stringify!($sector_mode), "` sectors do not have"
                )
            );
            assert!(
                is_contiguous(requested, available),
                concat!(
                    "`", stringify!($layout), "` is not contiguous for `",
                    stringify!($sector_mode),
                    "`, so a drive cannot return it in one span"
                )
            );
        };
    )*};
}

const SECTOR_SIZE: usize = 2352;

pub struct CdDa;
impl sealed::SectorModeSeal for CdDa {}
impl SectorMode for CdDa {
    const EXPECTED_SECTOR_TYPE: u3 = u3::new(0b001);
    type Sync = ();
    type Header = ();
    type SubHeader = ();
    type UserData = [u8; SECTOR_SIZE];
    type EdcEcc = ();

    type Full = UserData;
}

impl_selection_of!(CdDa, [NoFields, UserData]);

const _: () = assert!(size_of::<Sector<CdDa>>() == SECTOR_SIZE);

pub struct Mode1;
impl sealed::SectorModeSeal for Mode1 {}
impl SectorMode for Mode1 {
    const EXPECTED_SECTOR_TYPE: u3 = u3::new(0b010);
    type Sync = [u8; 12];
    type Header = SectorHeader;
    type SubHeader = ();
    type UserData = [u8; 2048];
    type EdcEcc = Mode1EdcEcc;

    type Full = SyncHeaderUserDataEdcEcc;
}
impl DataSectorMode for Mode1 {
    const DATA_MODE: DataMode = DataMode::Mode1;
}

impl_selection_of!(
    Mode1,
    [
        NoFields,
        EdcEcc,
        UserData,
        UserDataEdcEcc,
        Header,
        HeaderUserData,
        HeaderUserDataEdcEcc,
        Sync,
        SyncHeader,
        SyncHeaderUserData,
        SyncHeaderUserDataEdcEcc
    ]
);

const _: () = assert!(size_of::<Sector<Mode1>>() == SECTOR_SIZE);

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned,
)]
#[repr(C)]
pub struct Mode1EdcEcc {
    pub edc: U32<LittleEndian>,
    zero_fill: [u8; 8],
    pub p_parity: [u8; 172],
    pub q_parity: [u8; 104],
}

const _: () = assert!(size_of::<Mode1EdcEcc>() == 288);

pub struct Mode2Formless;
impl sealed::SectorModeSeal for Mode2Formless {}
impl SectorMode for Mode2Formless {
    const EXPECTED_SECTOR_TYPE: u3 = u3::new(0b011);
    type Sync = [u8; 12];
    type Header = SectorHeader;
    type SubHeader = ();
    type UserData = [u8; 2336];
    type EdcEcc = ();

    type Full = SyncHeaderUserData;
}
impl DataSectorMode for Mode2Formless {
    const DATA_MODE: DataMode = DataMode::Mode2;
}

impl_selection_of!(
    Mode2Formless,
    [
        NoFields,
        UserData,
        Header,
        HeaderUserData,
        Sync,
        SyncHeader,
        SyncHeaderUserData
    ]
);

const _: () = assert!(size_of::<Sector<Mode2Formless>>() == SECTOR_SIZE);

pub struct Mode2Form1;
impl sealed::SectorModeSeal for Mode2Form1 {}
impl SectorMode for Mode2Form1 {
    const EXPECTED_SECTOR_TYPE: u3 = u3::new(0b100);
    type Sync = [u8; 12];
    type Header = SectorHeader;
    type SubHeader = [Mode2FormedSubHeader; 2];
    type UserData = [u8; 2048];
    type EdcEcc = Mode2Form1EdcEcc;

    type Full = SyncAllHeadersUserDataEdcEcc;
}
impl DataSectorMode for Mode2Form1 {
    const DATA_MODE: DataMode = DataMode::Mode2;
}
impl Mode2FormedSectorMode for Mode2Form1 {
    const MODE2_FORM: Mode2Form = Mode2Form::Form1;
}
const _: () = <Mode2Form1 as Mode2FormedSectorMode>::ASSERT_MODE2;

impl_selection_of!(
    Mode2Form1,
    [
        NoFields,
        EdcEcc,
        UserData,
        UserDataEdcEcc,
        Header,
        SubHeader,
        SubHeaderUserData,
        SubHeaderUserDataEdcEcc,
        AllHeaders,
        AllHeadersUserData,
        AllHeadersUserDataEdcEcc,
        Sync,
        SyncHeader,
        SyncAllHeaders,
        SyncAllHeadersUserData,
        SyncAllHeadersUserDataEdcEcc,
    ]
);

const _: () = assert!(size_of::<Sector<Mode2Form1>>() == SECTOR_SIZE);

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned,
)]
#[repr(C)]
pub struct Mode2Form1EdcEcc {
    pub edc: U32<LittleEndian>,
    pub p_parity: [u8; 172],
    pub q_parity: [u8; 104],
}

const _: () = assert!(size_of::<Mode2Form1EdcEcc>() == 280);

pub struct Mode2Form2;
impl sealed::SectorModeSeal for Mode2Form2 {}
impl SectorMode for Mode2Form2 {
    const EXPECTED_SECTOR_TYPE: u3 = u3::new(0b101);
    type Sync = [u8; 12];
    type Header = SectorHeader;
    type SubHeader = [Mode2FormedSubHeader; 2];
    type UserData = [u8; 2324];
    type EdcEcc = Mode2Form2EdcEcc;

    type Full = SyncAllHeadersUserDataEdcEcc;
}
impl DataSectorMode for Mode2Form2 {
    const DATA_MODE: DataMode = DataMode::Mode2;
}
impl Mode2FormedSectorMode for Mode2Form2 {
    const MODE2_FORM: Mode2Form = Mode2Form::Form2;
}
const _: () = <Mode2Form2 as Mode2FormedSectorMode>::ASSERT_MODE2;

impl_selection_of!(
    Mode2Form2,
    [
        NoFields,
        EdcEcc,
        UserData,
        UserDataEdcEcc,
        Header,
        SubHeader,
        SubHeaderUserData,
        SubHeaderUserDataEdcEcc,
        AllHeaders,
        AllHeadersUserData,
        AllHeadersUserDataEdcEcc,
        Sync,
        SyncHeader,
        SyncAllHeaders,
        SyncAllHeadersUserData,
        SyncAllHeadersUserDataEdcEcc,
    ]
);

const _: () = assert!(size_of::<Sector<Mode2Form2>>() == SECTOR_SIZE);

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned,
)]
#[repr(C)]
pub struct Mode2Form2EdcEcc {
    edc: U32<LittleEndian>,
}

const _: () = assert!(size_of::<Mode2Form2EdcEcc>() == 4);

impl Mode2Form2EdcEcc {
    pub const fn edc(&self) -> Option<NonZeroU32> {
        NonZeroU32::new(self.edc.get())
    }
}
