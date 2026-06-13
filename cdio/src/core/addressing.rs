use std::ops::{Add, AddAssign, Sub, SubAssign};

use derive_more::{Display, From, Into};

/// Newtype representing a Logical Block Address (LBA)
///
/// The LBA is the number that a Host uses to reference Logical Blocks on a block storage device.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, PartialOrd, Ord, From, Into)]
pub struct Lba(i32);

// macro_rules! lba {
//     ($e:expr) => {
//         const {
//             match $crate::core::addressing::Lba::try_from_i32($e) {
//                 Ok(v) => v,
//                 Err(_) => panic!("LBA must be in range -451150..=404849"),
//             }
//         }
//     };
// }
// pub(crate) use lba;

// Why derive the LBA's Add/Sub traits for i32 instead of LBA?
// Well, because conceptually, an LBA represents an ADDRESS not some sort of scalar value. It
// doesn't make much sense to ask, "What is Appartment A + Appartment G?" but, "What is 5 doors
// down from Appartment A?" makes perfect sense. You can also think of it like a memory address,
// but pointing to a place on the disc. The same reasons pointer arithmetic is done between memory
// addresses and scalars applies here.

impl Add<i32> for Lba {
    type Output = Self;

    #[inline]
    fn add(self, rhs: i32) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<i32> for Lba {
    #[inline]
    fn add_assign(&mut self, rhs: i32) {
        *self = *self + rhs
    }
}

impl Sub<i32> for Lba {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: i32) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl SubAssign<i32> for Lba {
    #[inline]
    fn sub_assign(&mut self, rhs: i32) {
        *self = *self - rhs
    }
}

// impl From<Msf> for Lba {
//     /* The following comes strainght from the MMC-6 spec (Table 677) */
//     fn from(value: Msf) -> Self {
//         let offset_lba =
//             (value.minute() as i32 * 60 + value.second() as i32) * 75 + value.frame() as i32;

//         // Range from MMC-6: 00:00:00 <= MSF <= 89:59:74
//         if value <= Msf::new_unchecked(89, 59, 74) {
//             Self(offset_lba - PREGAP_OFFSET as i32)
//         }
//         // Range from MMC-6: 90:00:00 <= MSF <= 99:59:74
//         else {
//             Self(offset_lba - 450150)
//         }
//     }
// }

// ! THIS NEEDS TO BE MOVED
// ! In short, this conversion is only technically defined for the WRITE command, and it not
// ! technically, a top-level rule. This conversion should be moved to a module dedicated to
// ! handling the WRITE command. Currently I'm going to proceed under the assumption that in the
// ! typically LBAs and MSFs should not be converted to and from each other.
// impl From<Lba> for Msf {
//     /* The following comes strainght from the MMC-6 spec (Table 677) */
//     fn from(value: Lba) -> Self {
//         let raw_lba: i32 = value.into();

//         if (-150..=404849).contains(&raw_lba) {
//             let m = (raw_lba + PREGAP_OFFSET as i32) / (60 * 75);
//             let s = (raw_lba + PREGAP_OFFSET as i32 - m * 60 * 75) / 75;
//             let f = raw_lba + PREGAP_OFFSET as i32 - m * 60 * 75 - s * 75;

//             // We should be mathmatecially garunteed safe truncation here
//             Msf::new_unchecked(m as u8, s as u8, f as u8)
//         } else {
//             let m = (raw_lba + 450150) / (60 * 75);
//             let s = (raw_lba + 450150 - m * 60 * 75) / 75;
//             let f = raw_lba + 450150 - m * 60 * 75 - s * 75;

//             // We should be mathmatecially garunteed safe truncation here
//             Msf::new_unchecked(m as u8, s as u8, f as u8)
//         }
//     }
// }
