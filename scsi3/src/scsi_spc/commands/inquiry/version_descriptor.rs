use derive_more::Debug;
use zerocopy::{BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, U16, Unaligned};

/// A version descriptor: a standard the device claims conformance to.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(transparent)]
pub struct VersionDescriptor(#[debug("{_0:04X}h")] U16<BigEndian>);

impl VersionDescriptor {
    #[inline]
    pub const fn from_u16(code: u16) -> Self {
        Self(U16::new(code))
    }

    #[inline]
    pub const fn into_u16(self) -> u16 {
        self.0.get()
    }
}

// The consts below are generated from the list T10 maintains,
// https://www.t10.org/lists/stds-alph.txt, as of 2026-10-13. Each doc comment is the entry's text
// in that list.
//
// Names leave out the project designation before a revision (T10/1416-D, BSR INCITS 513), and
// write revision 37a as R37A and revision 1.04 as R1_04. A standard published with amendments ends
// in AM1, AM1_AM2 and so on. A few entries are named by hand: ATA/ATAPI-8's parts, amendments,
// IEEE, USB and 0000h.
// NOTE: The list also gives 0961h to 097Fh to iSCSI, as described by RFC 7144. Those have no
// consts.
// NOTE: SAS_R4 and SAS_R04 are both in the list, as "revision 4" and "revision 04".
impl VersionDescriptor {
    /// ACS-2 (no version claimed)
    pub const ACS_2: Self = Self::from_u16(0x1761);
    /// ACS-2 INCITS 482-2013
    pub const ACS_2_INCITS_482_2013: Self = Self::from_u16(0x1762);
    /// ACS-3 (no version claimed)
    pub const ACS_3: Self = Self::from_u16(0x1765);
    /// ACS-3 INCITS 522-2014
    pub const ACS_3_INCITS_522_2014: Self = Self::from_u16(0x1766);
    /// ACS-4 (no version claimed)
    pub const ACS_4: Self = Self::from_u16(0x1768);
    /// ACS-4 INCITS 529-2018
    pub const ACS_4_INCITS_529_2018: Self = Self::from_u16(0x1767);
    /// ACS-5 (no version claimed)
    pub const ACS_5: Self = Self::from_u16(0x1769);
    /// ACS-5 INCITS 558-2021
    pub const ACS_5_INCITS_558_2021: Self = Self::from_u16(0x176A);
    /// ACS-6 (no version claimed)
    pub const ACS_6: Self = Self::from_u16(0x176E);
    /// ACS-2 ISO/IEC 17760-102
    pub const ACS_2_ISO_IEC_17760_102: Self = Self::from_u16(0x1778);
    /// ACS-3 ISO/IEC 17760-103
    pub const ACS_3_ISO_IEC_17760_103: Self = Self::from_u16(0x1779);
    /// ACS-5 ISO/IEC 17760-105
    pub const ACS_5_ISO_IEC_17760_105: Self = Self::from_u16(0x177B);
    /// ADC (no version claimed)
    pub const ADC: Self = Self::from_u16(0x03C0);
    /// ADC INCITS 403-2005
    pub const ADC_INCITS_403_2005: Self = Self::from_u16(0x03D7);
    /// ADC T10/1558-D revision 7
    pub const ADC_R7: Self = Self::from_u16(0x03D6);
    /// ADC T10/1558-D revision 6
    pub const ADC_R6: Self = Self::from_u16(0x03D5);
    /// ADC-2 (no version claimed)
    pub const ADC_2: Self = Self::from_u16(0x04A0);
    /// ADC-2 INCITS 441-2008
    pub const ADC_2_INCITS_441_2008: Self = Self::from_u16(0x04AC);
    /// ADC-2 T10/1741-D revision 7
    pub const ADC_2_R7: Self = Self::from_u16(0x04A7);
    /// ADC-2 T10/1741-D revision 8
    pub const ADC_2_R8: Self = Self::from_u16(0x04AA);
    /// ADC-3 (no version claimed)
    pub const ADC_3: Self = Self::from_u16(0x0500);
    /// ADC-3 INCITS 497-2012
    pub const ADC_3_INCITS_497_2012: Self = Self::from_u16(0x050A);
    /// ADC-3 T10/1895-D revision 04
    pub const ADC_3_R04: Self = Self::from_u16(0x0502);
    /// ADC-3 T10/1895-D revision 05
    pub const ADC_3_R05: Self = Self::from_u16(0x0504);
    /// ADC-3 T10/1895-D revision 05a
    pub const ADC_3_R05A: Self = Self::from_u16(0x0506);
    /// ADC-4 (no version claimed)
    pub const ADC_4: Self = Self::from_u16(0x0640);
    /// ADC-4 INCITS 541-2023
    pub const ADC_4_INCITS_541_2023: Self = Self::from_u16(0x0642);
    /// ADC-4 BSR INCITS 541 revision 05
    pub const ADC_4_R05: Self = Self::from_u16(0x064C);
    /// ADC-4 BSR INCITS 541 revision 04
    pub const ADC_4_R04: Self = Self::from_u16(0x064B);
    /// ADP (no version claimed)
    pub const ADP: Self = Self::from_u16(0x09C0);
    /// ADT (no version claimed)
    pub const ADT: Self = Self::from_u16(0x09E0);
    /// ADT INCITS 406-2005
    pub const ADT_INCITS_406_2005: Self = Self::from_u16(0x09FD);
    /// ADT T10/1557-D revision 14
    pub const ADT_R14: Self = Self::from_u16(0x09FA);
    /// ADT T10/1557-D revision 11
    pub const ADT_R11: Self = Self::from_u16(0x09F9);
    /// ADT-2 (no version claimed)
    pub const ADT_2: Self = Self::from_u16(0x0A20);
    /// ADT-2 INCITS 472-2011
    pub const ADT_2_INCITS_472_2011: Self = Self::from_u16(0x0A2B);
    /// ADT-2 T10/1742-D revision 06
    pub const ADT_2_R06: Self = Self::from_u16(0x0A22);
    /// ADT-2 T10/1742-D revision 08
    pub const ADT_2_R08: Self = Self::from_u16(0x0A27);
    /// ADT-2 T10/1742-D revision 09
    pub const ADT_2_R09: Self = Self::from_u16(0x0A28);
    /// ADT-3 (no version claimed)
    pub const ADT_3: Self = Self::from_u16(0x0A60);
    /// ADT-3 INCITS 542-2022
    pub const ADT_3_INCITS_542_2022: Self = Self::from_u16(0x0A62);
    /// ADT-3 BSR INCITS 542 revision 03
    pub const ADT_3_R03: Self = Self::from_u16(0x0A6B);
    /// ADT-4 (no version claimed)
    pub const ADT_4: Self = Self::from_u16(0x2460);
    /// ADT-4 INCITS 541-2023
    pub const ADT_4_INCITS_541_2023: Self = Self::from_u16(0x246B);
    /// ATA/ATAPI-6 (no version claimed)
    pub const ATA_ATAPI_6: Self = Self::from_u16(0x15E0);
    /// ATA/ATAPI-6 INCITS 361-2002
    pub const ATA_ATAPI_6_INCITS_361_2002: Self = Self::from_u16(0x15FD);
    /// ATA/ATAPI-7 (no version claimed)
    pub const ATA_ATAPI_7: Self = Self::from_u16(0x1600);
    /// ATA/ATAPI-7 ISO/IEC 24739
    pub const ATA_ATAPI_7_ISO_IEC_24739: Self = Self::from_u16(0x161E);
    /// ATA/ATAPI-7 INCITS 397-2005
    pub const ATA_ATAPI_7_INCITS_397_2005: Self = Self::from_u16(0x161C);
    /// ATA/ATAPI-7 T13/1532-D revision 3
    pub const ATA_ATAPI_7_R3: Self = Self::from_u16(0x1602);
    /// ATA/ATAPI-8 ATA8-AAM (no version claimed)
    pub const ATA8_AAM: Self = Self::from_u16(0x1620);
    /// ATA/ATAPI-8 ATA8-ACS ATA/ATAPI Command Set ISO/IEC 17760-101
    pub const ATA8_ACS_ISO_IEC_17760_101: Self = Self::from_u16(0x1630);
    /// ATA/ATAPI-8 ATA8-AAM INCITS 451-2008
    pub const ATA8_AAM_INCITS_451_2008: Self = Self::from_u16(0x1628);
    /// ATA/ATAPI-8 ATA8-APT Parallel Transport (no version claimed)
    pub const ATA8_APT: Self = Self::from_u16(0x1621);
    /// ATA/ATAPI-8 ATA8-AST Serial Transport (no version claimed)
    pub const ATA8_AST: Self = Self::from_u16(0x1622);
    /// ATA/ATAPI-8 ATA8-ACS ATA/ATAPI Command Set (no version claimed)
    pub const ATA8_ACS: Self = Self::from_u16(0x1623);
    /// ATA/ATAPI-8 ATA8-ACS INCITS 452-2009 w/ Amendment 1
    pub const ATA8_ACS_INCITS_452_2009_AM1: Self = Self::from_u16(0x162A);
    /// BCC (no version claimed)
    pub const BCC: Self = Self::from_u16(0x0380);
    /// EPI (no version claimed)
    pub const EPI: Self = Self::from_u16(0x0B20);
    /// EPI INCITS TR-23 1999
    pub const EPI_INCITS_TR_23_1999: Self = Self::from_u16(0x0B3C);
    /// EPI T10/1134 revision 16
    pub const EPI_R16: Self = Self::from_u16(0x0B3B);
    /// Fast-20 (no version claimed)
    pub const FAST_20: Self = Self::from_u16(0x0AC0);
    /// Fast-20 INCITS 277-1996
    pub const FAST_20_INCITS_277_1996: Self = Self::from_u16(0x0ADC);
    /// Fast-20 T10/1071 revision 06
    pub const FAST_20_R06: Self = Self::from_u16(0x0ADB);
    /// FC 10GFC (no version claimed)
    pub const FC_10GFC: Self = Self::from_u16(0x0EA0);
    /// FC 10GFC ISO/IEC 14165-116
    pub const FC_10GFC_ISO_IEC_14165_116: Self = Self::from_u16(0x0EA3);
    /// FC 10GFC INCITS 364-2003
    pub const FC_10GFC_INCITS_364_2003: Self = Self::from_u16(0x0EA2);
    /// FC 10GFC ISO/IEC 14165-116 with AM1
    pub const FC_10GFC_ISO_IEC_14165_116_AM1: Self = Self::from_u16(0x0EA5);
    /// FC 10GFC INCITS 364-2003 with AM1 INCITS 364/AM1-2007
    pub const FC_10GFC_INCITS_364_2003_AM1: Self = Self::from_u16(0x0EA6);
    /// FC-AL (no version claimed)
    pub const FC_AL: Self = Self::from_u16(0x0D40);
    /// FC-AL INCITS 272-1996
    pub const FC_AL_INCITS_272_1996: Self = Self::from_u16(0x0D5C);
    /// FC-AL-2 (no version claimed)
    pub const FC_AL_2: Self = Self::from_u16(0x0D60);
    /// FC-AL-2 ISO/IEC 14165-122 with AM1 & AM2
    pub const FC_AL_2_ISO_IEC_14165_122_AM1_AM2: Self = Self::from_u16(0x0D65);
    /// FC-AL-2 INCITS 332-1999 with AM1-2003 & AM2-2006
    pub const FC_AL_2_INCITS_332_1999_AM1_AM2: Self = Self::from_u16(0x0D63);
    /// FC-AL-2 INCITS 332-1999 with Amnd 2 AM2-2006
    pub const FC_AL_2_INCITS_332_1999_AM2: Self = Self::from_u16(0x0D64);
    /// FC-AL-2 INCITS 332-1999 with Amnd 1 AM1-2003
    pub const FC_AL_2_INCITS_332_1999_AM1: Self = Self::from_u16(0x0D7D);
    /// FC-AL-2 INCITS 332-1999
    pub const FC_AL_2_INCITS_332_1999: Self = Self::from_u16(0x0D7C);
    /// FC-AL-2 T11/1133-D revision 7.0
    pub const FC_AL_2_R7_0: Self = Self::from_u16(0x0D61);
    /// FC-DA (no version claimed)
    pub const FC_DA: Self = Self::from_u16(0x12E0);
    /// FC-DA ISO/IEC 14165-341
    pub const FC_DA_ISO_IEC_14165_341: Self = Self::from_u16(0x12E9);
    /// FC-DA INCITS TR-36 2004
    pub const FC_DA_INCITS_TR_36_2004: Self = Self::from_u16(0x12E8);
    /// FC-DA T11/1513-DT revision 3.1
    pub const FC_DA_R3_1: Self = Self::from_u16(0x12E2);
    /// FC-DA-2 (no version claimed)
    pub const FC_DA_2: Self = Self::from_u16(0x12C0);
    /// FC-DA-2 INCITS TR-49 2012
    pub const FC_DA_2_INCITS_TR_49_2012: Self = Self::from_u16(0x12C9);
    /// FC-DA-2 T11/1870DT revision 1.04
    pub const FC_DA_2_R1_04: Self = Self::from_u16(0x12C3);
    /// FC-DA-2 T11/1870DT revision 1.06
    pub const FC_DA_2_R1_06: Self = Self::from_u16(0x12C5);
    /// FC-FLA (no version claimed)
    pub const FC_FLA: Self = Self::from_u16(0x1320);
    /// FC-FLA INCITS TR-20 1998
    pub const FC_FLA_INCITS_TR_20_1998: Self = Self::from_u16(0x133C);
    /// FC-FLA T11/1235 revision 7
    pub const FC_FLA_R7: Self = Self::from_u16(0x133B);
    /// FC-FS (no version claimed)
    pub const FC_FS: Self = Self::from_u16(0x0DA0);
    /// FC-FS ISO/IEC 14165-251
    pub const FC_FS_ISO_IEC_14165_251: Self = Self::from_u16(0x0DBD);
    /// FC-FS INCITS 373-2003
    pub const FC_FS_INCITS_373_2003: Self = Self::from_u16(0x0DBC);
    /// FC-FS T11/1331-D revision 1.2
    pub const FC_FS_R1_2: Self = Self::from_u16(0x0DB7);
    /// FC-FS T11/1331-D revision 1.7
    pub const FC_FS_R1_7: Self = Self::from_u16(0x0DB8);
    /// FC-FS-2 (no version claimed)
    pub const FC_FS_2: Self = Self::from_u16(0x0E00);
    /// FC-FS-2 INCITS 242-2007 with AM1 INCITS 242/AM1-2007
    pub const FC_FS_2_INCITS_242_2007_AM1: Self = Self::from_u16(0x0E03);
    /// FC-FS-2 INCITS 242-2007
    pub const FC_FS_2_INCITS_242_2007: Self = Self::from_u16(0x0E02);
    /// FC-FS-3 (no version claimed)
    pub const FC_FS_3: Self = Self::from_u16(0x0EE0);
    /// FC-FS-3 INCITS 470-2011
    pub const FC_FS_3_INCITS_470_2011: Self = Self::from_u16(0x0EEB);
    /// FC-FS-3 T11/1861-D revision 0.9
    pub const FC_FS_3_R0_9: Self = Self::from_u16(0x0EE2);
    /// FC-FS-3 T11/1861-D revision 1.0
    pub const FC_FS_3_R1_0: Self = Self::from_u16(0x0EE7);
    /// FC-FS-3 T11/1861-D revision 1.10
    pub const FC_FS_3_R1_10: Self = Self::from_u16(0x0EE9);
    /// FC-FS-4 (no version claimed)
    pub const FC_FS_4: Self = Self::from_u16(0x0F60);
    /// FC-LS (no version claimed)
    pub const FC_LS: Self = Self::from_u16(0x0E20);
    /// FC-LS INCITS 433-2007
    pub const FC_LS_INCITS_433_2007: Self = Self::from_u16(0x0E29);
    /// FC-LS T11/1620-D revision 1.62
    pub const FC_LS_R1_62: Self = Self::from_u16(0x0E21);
    /// FC-LS-2 (no version claimed)
    pub const FC_LS_2: Self = Self::from_u16(0x0F00);
    /// FC-LS-2 INCITS 477-2011
    pub const FC_LS_2_INCITS_477_2011: Self = Self::from_u16(0x0F07);
    /// FC-LS-2 T11/2103-D revision 2.11
    pub const FC_LS_2_R2_11: Self = Self::from_u16(0x0F03);
    /// FC-LS-2 T11/2103-D revision 2.21
    pub const FC_LS_2_R2_21: Self = Self::from_u16(0x0F05);
    /// FC-LS-3 (no version claimed)
    pub const FC_LS_3: Self = Self::from_u16(0x0F80);
    /// FC-PH (no version claimed)
    pub const FC_PH: Self = Self::from_u16(0x0D20);
    /// FC-PH INCITS 230-1994
    pub const FC_PH_INCITS_230_1994: Self = Self::from_u16(0x0D3B);
    /// FC-PH INCITS 230-1994 with Amnd 1 INCITS 230/AM1-1996
    pub const FC_PH_INCITS_230_1994_AM1: Self = Self::from_u16(0x0D3C);
    /// FC-PH-3 (no version claimed)
    pub const FC_PH_3: Self = Self::from_u16(0x0D80);
    /// FC-PH-3 INCITS 303-1998
    pub const FC_PH_3_INCITS_303_1998: Self = Self::from_u16(0x0D9C);
    /// FC-PI (no version claimed)
    pub const FC_PI: Self = Self::from_u16(0x0DC0);
    /// FC-PI INCITS 352-2002
    pub const FC_PI_INCITS_352_2002: Self = Self::from_u16(0x0DDC);
    /// FC-PI-2 (no version claimed)
    pub const FC_PI_2: Self = Self::from_u16(0x0DE0);
    /// FC-PI-2 INCITS 404-2006
    pub const FC_PI_2_INCITS_404_2006: Self = Self::from_u16(0x0DE4);
    /// FC-PI-2 T11/1506-D revision 5.0
    pub const FC_PI_2_R5_0: Self = Self::from_u16(0x0DE2);
    /// FC-PI-3 (no version claimed)
    pub const FC_PI_3: Self = Self::from_u16(0x0E60);
    /// FC-PI-3 INCITS 460-2011
    pub const FC_PI_3_INCITS_460_2011: Self = Self::from_u16(0x0E6E);
    /// FC-PI-3 T11/1625-D revision 2.0
    pub const FC_PI_3_R2_0: Self = Self::from_u16(0x0E62);
    /// FC-PI-3 T11/1625-D revision 2.1
    pub const FC_PI_3_R2_1: Self = Self::from_u16(0x0E68);
    /// FC-PI-3 T11/1625-D revision 4.0
    pub const FC_PI_3_R4_0: Self = Self::from_u16(0x0E6A);
    /// FC-PI-4 (no version claimed)
    pub const FC_PI_4: Self = Self::from_u16(0x0E80);
    /// FC-PI-4 INCITS 450-2009
    pub const FC_PI_4_INCITS_450_2009: Self = Self::from_u16(0x0E88);
    /// FC-PI-4 T11/1647-D revision 8.0
    pub const FC_PI_4_R8_0: Self = Self::from_u16(0x0E82);
    /// FC-PI-5 (no version claimed)
    pub const FC_PI_5: Self = Self::from_u16(0x0F20);
    /// FC-PI-5 INCITS 479-2011
    pub const FC_PI_5_INCITS_479_2011: Self = Self::from_u16(0x0F2E);
    /// FC-PI-5 T11/2118-D revision 2.00
    pub const FC_PI_5_R2_00: Self = Self::from_u16(0x0F27);
    /// FC-PI-5 T11/2118-D revision 3.00
    pub const FC_PI_5_R3_00: Self = Self::from_u16(0x0F28);
    /// FC-PI-5 T11/2118-D revision 6.00
    pub const FC_PI_5_R6_00: Self = Self::from_u16(0x0F2A);
    /// FC-PI-5 T11/2118-D revision 6.10
    pub const FC_PI_5_R6_10: Self = Self::from_u16(0x0F2B);
    /// FC-PI-6 (no version claimed)
    pub const FC_PI_6: Self = Self::from_u16(0x0F40);
    /// FC-PLDA (no version claimed)
    pub const FC_PLDA: Self = Self::from_u16(0x1340);
    /// FC-PLDA INCITS TR-19 1998
    pub const FC_PLDA_INCITS_TR_19_1998: Self = Self::from_u16(0x135C);
    /// FC-PLDA T11/1162 revision 2.1
    pub const FC_PLDA_R2_1: Self = Self::from_u16(0x135B);
    /// FC-SCM (no version claimed)
    pub const FC_SCM: Self = Self::from_u16(0x12A0);
    /// FC-SCM INCITS TR-47 2012
    pub const FC_SCM_INCITS_TR_47_2012: Self = Self::from_u16(0x12AA);
    /// FC-SCM T11/1824DT revision 1.0
    pub const FC_SCM_R1_0: Self = Self::from_u16(0x12A3);
    /// FC-SCM T11/1824DT revision 1.1
    pub const FC_SCM_R1_1: Self = Self::from_u16(0x12A5);
    /// FC-SCM T11/1824DT revision 1.4
    pub const FC_SCM_R1_4: Self = Self::from_u16(0x12A7);
    /// FC-SP (no version claimed)
    pub const FC_SP: Self = Self::from_u16(0x0E40);
    /// FC-SP INCITS 426-2007
    pub const FC_SP_INCITS_426_2007: Self = Self::from_u16(0x0E45);
    /// FC-SP T11/1570-D revision 1.6
    pub const FC_SP_R1_6: Self = Self::from_u16(0x0E42);
    /// FC-SP-2 (no version claimed)
    pub const FC_SP_2: Self = Self::from_u16(0x0EC0);
    /// FC-Tape (no version claimed)
    pub const FC_TAPE: Self = Self::from_u16(0x1300);
    /// FC-Tape INCITS TR-24 1999
    pub const FC_TAPE_INCITS_TR_24_1999: Self = Self::from_u16(0x131C);
    /// FC-Tape T11/1315 revision 1.17
    pub const FC_TAPE_R1_17: Self = Self::from_u16(0x131B);
    /// FC-Tape T11/1315 revision 1.16
    pub const FC_TAPE_R1_16: Self = Self::from_u16(0x1301);
    /// FCP (no version claimed)
    pub const FCP: Self = Self::from_u16(0x08C0);
    /// FCP INCITS 269-1996
    pub const FCP_INCITS_269_1996: Self = Self::from_u16(0x08DC);
    /// FCP T10/0993-D revision 12
    pub const FCP_R12: Self = Self::from_u16(0x08DB);
    /// FCP-2 (no version claimed)
    pub const FCP_2: Self = Self::from_u16(0x0900);
    /// FCP-2 ISO/IEC 14776-222
    pub const FCP_2_ISO_IEC_14776_222: Self = Self::from_u16(0x091A);
    /// FCP-2 INCITS 350-2003
    pub const FCP_2_INCITS_350_2003: Self = Self::from_u16(0x0917);
    /// FCP-2 T10/1144-D revision 8
    pub const FCP_2_R8: Self = Self::from_u16(0x0918);
    /// FCP-2 T10/1144-D revision 4
    pub const FCP_2_R4: Self = Self::from_u16(0x0901);
    /// FCP-2 T10/1144-D revision 7
    pub const FCP_2_R7: Self = Self::from_u16(0x0915);
    /// FCP-2 T10/1144-D revision 7a
    pub const FCP_2_R7A: Self = Self::from_u16(0x0916);
    /// FCP-3 (no version claimed)
    pub const FCP_3: Self = Self::from_u16(0x0A00);
    /// FCP-3 ISO/IEC 14776-223
    pub const FCP_3_ISO_IEC_14776_223: Self = Self::from_u16(0x0A1C);
    /// FCP-3 INCITS 416-2006
    pub const FCP_3_INCITS_416_2006: Self = Self::from_u16(0x0A11);
    /// FCP-3 T10/1560-D revision 4
    pub const FCP_3_R4: Self = Self::from_u16(0x0A0F);
    /// FCP-3 T10/1560-D revision 3f
    pub const FCP_3_R3F: Self = Self::from_u16(0x0A07);
    /// FCP-4 (no version claimed)
    pub const FCP_4: Self = Self::from_u16(0x0A40);
    /// FCP-4 ISO/IEC 14776-224
    pub const FCP_4_ISO_IEC_14776_224: Self = Self::from_u16(0x0A50);
    /// FCP-4 INCITS 481-2011
    pub const FCP_4_INCITS_481_2011: Self = Self::from_u16(0x0A46);
    /// FCP-4 AM1 INCITS 481-2011/AM1-2018
    pub const FCP_4_INCITS_481_2011_AM1_2018: Self = Self::from_u16(0x0A52);
    /// FCP-4 T10/1828-D revision 01
    pub const FCP_4_R01: Self = Self::from_u16(0x0A42);
    /// FCP-4 T10/1828-D revision 02
    pub const FCP_4_R02: Self = Self::from_u16(0x0A44);
    /// FCP-4 T10/1828-D revision 02b
    pub const FCP_4_R02B: Self = Self::from_u16(0x0A45);
    /// FCP-5 (no version claimed)
    pub const FCP_5: Self = Self::from_u16(0x0A80);
    /// FCP-5 INCITS 563-2023
    pub const FCP_5_INCITS_563_2023: Self = Self::from_u16(0x0A82);
    /// FCP-5 BSR INCITS 563 revision 04
    pub const FCP_5_R04: Self = Self::from_u16(0x0A8B);
    /// IEEE 1394 (no version claimed)
    pub const IEEE_1394: Self = Self::from_u16(0x14A0);
    /// IEEE 1394-1995
    pub const IEEE_1394_1995: Self = Self::from_u16(0x14BD);
    /// IEEE 1394a (no version claimed)
    pub const IEEE_1394A: Self = Self::from_u16(0x14C0);
    /// IEEE 1394b (no version claimed)
    pub const IEEE_1394B: Self = Self::from_u16(0x14E0);
    /// IEEE 1667 (no version claimed)
    pub const IEEE_1667: Self = Self::from_u16(0xFFC0);
    /// IEEE 1667-2006
    pub const IEEE_1667_2006: Self = Self::from_u16(0xFFC1);
    /// IEEE 1667-2009
    pub const IEEE_1667_2009: Self = Self::from_u16(0xFFC2);
    /// IEEE 1667-2015
    pub const IEEE_1667_2015: Self = Self::from_u16(0xFFC3);
    /// IEEE 1667-2018
    pub const IEEE_1667_2018: Self = Self::from_u16(0xFFC4);
    /// iSCSI (no version claimed)
    pub const ISCSI: Self = Self::from_u16(0x0960);
    /// MMC (no version claimed)
    pub const MMC: Self = Self::from_u16(0x0140);
    /// MMC INCITS 304-1997
    pub const MMC_INCITS_304_1997: Self = Self::from_u16(0x015C);
    /// MMC T10/1048-D revision 10a
    pub const MMC_R10A: Self = Self::from_u16(0x015B);
    /// MMC-2 (no version claimed)
    pub const MMC_2: Self = Self::from_u16(0x0240);
    /// MMC-2 INCITS 333-2000
    pub const MMC_2_INCITS_333_2000: Self = Self::from_u16(0x025C);
    /// MMC-2 T10/1228-D revision 11a
    pub const MMC_2_R11A: Self = Self::from_u16(0x025B);
    /// MMC-2 T10/1228-D revision 11
    pub const MMC_2_R11: Self = Self::from_u16(0x0255);
    /// MMC-3 (no version claimed)
    pub const MMC_3: Self = Self::from_u16(0x02A0);
    /// MMC-3 INCITS 360-2002
    pub const MMC_3_INCITS_360_2002: Self = Self::from_u16(0x02B8);
    /// MMC-3 T10/1363-D revision 10g
    pub const MMC_3_R10G: Self = Self::from_u16(0x02B6);
    /// MMC-3 T10/1363-D revision 9
    pub const MMC_3_R9: Self = Self::from_u16(0x02B5);
    /// MMC-4 (no version claimed)
    pub const MMC_4: Self = Self::from_u16(0x03A0);
    /// MMC-4 INCITS 401-2005
    pub const MMC_4_INCITS_401_2005: Self = Self::from_u16(0x03BF);
    /// MMC-4 T10/1545-D revision 3
    pub const MMC_4_R3: Self = Self::from_u16(0x03BD);
    /// MMC-4 T10/1545-D revision 3d
    pub const MMC_4_R3D: Self = Self::from_u16(0x03BE);
    /// MMC-4 T10/1545-D revision 5
    pub const MMC_4_R5: Self = Self::from_u16(0x03B0);
    /// MMC-4 T10/1545-D revision 5a
    pub const MMC_4_R5A: Self = Self::from_u16(0x03B1);
    /// MMC-5 (no version claimed)
    pub const MMC_5: Self = Self::from_u16(0x0420);
    /// MMC-5 INCITS 430-2007
    pub const MMC_5_INCITS_430_2007: Self = Self::from_u16(0x0434);
    /// MMC-5 T10/1675-D revision 04
    pub const MMC_5_R04: Self = Self::from_u16(0x0432);
    /// MMC-5 T10/1675-D revision 03
    pub const MMC_5_R03: Self = Self::from_u16(0x042F);
    /// MMC-5 T10/1675-D revision 03b
    pub const MMC_5_R03B: Self = Self::from_u16(0x0431);
    /// MMC-6 (no version claimed)
    pub const MMC_6: Self = Self::from_u16(0x04E0);
    /// MMC-6 INCITS 468-2010 + MMC-6/AM1 INCITS 468-2010/AM 1
    pub const MMC_6_INCITS_468_2010_AM1: Self = Self::from_u16(0x04E7);
    /// MMC-6 INCITS 468-2010
    pub const MMC_6_INCITS_468_2010: Self = Self::from_u16(0x04E6);
    /// MMC-6 T10/1836-D revision 02b
    pub const MMC_6_R02B: Self = Self::from_u16(0x04E3);
    /// MMC-6 T10/1836-D revision 02g
    pub const MMC_6_R02G: Self = Self::from_u16(0x04E5);
    /// OCRW (no version claimed)
    pub const OCRW: Self = Self::from_u16(0x0280);
    /// OCRW ISO/IEC 14776-381
    pub const OCRW_ISO_IEC_14776_381: Self = Self::from_u16(0x029E);
    /// OSD (no version claimed)
    pub const OSD: Self = Self::from_u16(0x0340);
    /// OSD INCITS 400-2004
    pub const OSD_INCITS_400_2004: Self = Self::from_u16(0x0356);
    /// OSD T10/1355-D revision 10
    pub const OSD_R10: Self = Self::from_u16(0x0355);
    /// OSD T10/1355-D revision 0
    pub const OSD_R0: Self = Self::from_u16(0x0341);
    /// OSD T10/1355-D revision 7a
    pub const OSD_R7A: Self = Self::from_u16(0x0342);
    /// OSD T10/1355-D revision 8
    pub const OSD_R8: Self = Self::from_u16(0x0343);
    /// OSD T10/1355-D revision 9
    pub const OSD_R9: Self = Self::from_u16(0x0344);
    /// OSD-2 (no version claimed)
    pub const OSD_2: Self = Self::from_u16(0x0440);
    /// OSD-2 INCITS 458-2011
    pub const OSD_2_INCITS_458_2011: Self = Self::from_u16(0x0448);
    /// OSD-2 T10/1729-D revision 4
    pub const OSD_2_R4: Self = Self::from_u16(0x0444);
    /// OSD-2 T10/1729-D revision 5
    pub const OSD_2_R5: Self = Self::from_u16(0x0446);
    /// OSD-3 (no version claimed)
    pub const OSD_3: Self = Self::from_u16(0x0560);
    /// PQI (no version claimed)
    pub const PQI: Self = Self::from_u16(0x2200);
    /// PQI INCITS 490-2014
    pub const PQI_INCITS_490_2014: Self = Self::from_u16(0x2208);
    /// PQI T10/BSR INCITS 490 revision 6
    pub const PQI_R6: Self = Self::from_u16(0x2204);
    /// PQI T10/BSR INCITS 490 revision 7
    pub const PQI_R7: Self = Self::from_u16(0x2206);
    /// PQI-2 (no version claimed)
    pub const PQI_2: Self = Self::from_u16(0x2240);
    /// PQI-2 INCITS 507-2016
    pub const PQI_2_INCITS_507_2016: Self = Self::from_u16(0x2244);
    /// PQI-2 T10/BSR INCITS 507 revision 01
    pub const PQI_2_R01: Self = Self::from_u16(0x2242);
    /// RBC (no version claimed)
    pub const RBC: Self = Self::from_u16(0x0220);
    /// RBC ISO/IEC 14776-326
    pub const RBC_ISO_IEC_14776_326: Self = Self::from_u16(0x023E);
    /// RBC INCITS 330-2000
    pub const RBC_INCITS_330_2000: Self = Self::from_u16(0x023C);
    /// RBC T10/1240-D revision 10a
    pub const RBC_R10A: Self = Self::from_u16(0x0238);
    /// SAM (no version claimed)
    pub const SAM: Self = Self::from_u16(0x0020);
    /// SAM ISO/IEC 14776-411
    pub const SAM_ISO_IEC_14776_411: Self = Self::from_u16(0x003D);
    /// SAM INCITS 270-1996
    pub const SAM_INCITS_270_1996: Self = Self::from_u16(0x003C);
    /// SAM T10/0994-D revision 18
    pub const SAM_R18: Self = Self::from_u16(0x003B);
    /// SAM-2 (no version claimed)
    pub const SAM_2: Self = Self::from_u16(0x0040);
    /// SAM-2 ISO/IEC 14776-412
    pub const SAM_2_ISO_IEC_14776_412: Self = Self::from_u16(0x005E);
    /// SAM-2 INCITS 366-2003
    pub const SAM_2_INCITS_366_2003: Self = Self::from_u16(0x005C);
    /// SAM-2 T10/1157-D revision 24
    pub const SAM_2_R24: Self = Self::from_u16(0x0055);
    /// SAM-2 T10/1157-D revision 23
    pub const SAM_2_R23: Self = Self::from_u16(0x0054);
    /// SAM-3 (no version claimed)
    pub const SAM_3: Self = Self::from_u16(0x0060);
    /// SAM-3 ISO/IEC 14776-413
    pub const SAM_3_ISO_IEC_14776_413: Self = Self::from_u16(0x0079);
    /// SAM-3 INCITS 402-2005
    pub const SAM_3_INCITS_402_2005: Self = Self::from_u16(0x0077);
    /// SAM-3 T10/1561-D revision 14
    pub const SAM_3_R14: Self = Self::from_u16(0x0076);
    /// SAM-3 T10/1561-D revision 7
    pub const SAM_3_R7: Self = Self::from_u16(0x0062);
    /// SAM-3 T10/1561-D revision 13
    pub const SAM_3_R13: Self = Self::from_u16(0x0075);
    /// SAM-4 (no version claimed)
    pub const SAM_4: Self = Self::from_u16(0x0080);
    /// SAM-4 ISO/IEC 14776-414
    pub const SAM_4_ISO_IEC_14776_414: Self = Self::from_u16(0x0092);
    /// SAM-4 INCITS 447-2008
    pub const SAM_4_INCITS_447_2008: Self = Self::from_u16(0x0090);
    /// SAM-4 T10/1683-D revision 13
    pub const SAM_4_R13: Self = Self::from_u16(0x0087);
    /// SAM-4 T10/1683-D revision 14
    pub const SAM_4_R14: Self = Self::from_u16(0x008B);
    /// SAM-5 (no version claimed)
    pub const SAM_5: Self = Self::from_u16(0x00A0);
    /// SAM-5 ISO/IEC 14776-415
    pub const SAM_5_ISO_IEC_14776_415: Self = Self::from_u16(0x00AA);
    /// SAM-5 INCITS 515-2016
    pub const SAM_5_INCITS_515_2016: Self = Self::from_u16(0x00A8);
    /// SAM-5 T10/2104-D revision 4
    pub const SAM_5_R4: Self = Self::from_u16(0x00A2);
    /// SAM-5 T10/2104-D revision 20
    pub const SAM_5_R20: Self = Self::from_u16(0x00A4);
    /// SAM-5 T10/2104-D revision 21
    pub const SAM_5_R21: Self = Self::from_u16(0x00A6);
    /// SAM-6 (no version claimed)
    pub const SAM_6: Self = Self::from_u16(0x00C0);
    /// SAM-6 INCITS 546-2021
    pub const SAM_6_INCITS_546_2021: Self = Self::from_u16(0x00C2);
    /// SAM-6 BSR INCITS 546 revision 10
    pub const SAM_6_R10: Self = Self::from_u16(0x00D4);
    /// SAS (no version claimed)
    pub const SAS: Self = Self::from_u16(0x0BE0);
    /// SAS ISO/IEC 14776-150
    pub const SAS_ISO_IEC_14776_150: Self = Self::from_u16(0x0BFE);
    /// SAS INCITS 376-2003
    pub const SAS_INCITS_376_2003: Self = Self::from_u16(0x0BFD);
    /// SAS T10/1562-D revision 05
    pub const SAS_R05: Self = Self::from_u16(0x0BFC);
    /// SAS T10/1562-D revision 01
    pub const SAS_R01: Self = Self::from_u16(0x0BE1);
    /// SAS T10/1562-D revision 03
    pub const SAS_R03: Self = Self::from_u16(0x0BF5);
    /// SAS T10/1562-D revision 4
    pub const SAS_R4: Self = Self::from_u16(0x0BFA);
    /// SAS T10/1562-D revision 04
    pub const SAS_R04: Self = Self::from_u16(0x0BFB);
    /// SAS-1.1 (no version claimed)
    pub const SAS_1_1: Self = Self::from_u16(0x0C00);
    /// SAS-1.1 ISO/IEC 14776-151
    pub const SAS_1_1_ISO_IEC_14776_151: Self = Self::from_u16(0x0C12);
    /// SAS-1.1 INCITS 417-2006
    pub const SAS_1_1_INCITS_417_2006: Self = Self::from_u16(0x0C11);
    /// SAS-1.1 T10/1601-D revision 9
    pub const SAS_1_1_R9: Self = Self::from_u16(0x0C07);
    /// SAS-1.1 T10/1601-D revision 10
    pub const SAS_1_1_R10: Self = Self::from_u16(0x0C0F);
    /// SAS-2 (no version claimed)
    pub const SAS_2: Self = Self::from_u16(0x0C20);
    /// SAS-2 INCITS 457-2010
    pub const SAS_2_INCITS_457_2010: Self = Self::from_u16(0x0C2A);
    /// SAS-2 T10/1760-D revision 14
    pub const SAS_2_R14: Self = Self::from_u16(0x0C23);
    /// SAS-2 T10/1760-D revision 15
    pub const SAS_2_R15: Self = Self::from_u16(0x0C27);
    /// SAS-2 T10/1760-D revision 16
    pub const SAS_2_R16: Self = Self::from_u16(0x0C28);
    /// SAS-2.1 (no version claimed)
    pub const SAS_2_1: Self = Self::from_u16(0x0C40);
    /// SAS-2.1 ISO/IEC 14776-153
    pub const SAS_2_1_ISO_IEC_14776_153: Self = Self::from_u16(0x0C52);
    /// SAS-2.1 INCITS 478-2011
    pub const SAS_2_1_INCITS_478_2011: Self = Self::from_u16(0x0C4E);
    /// SAS-2.1 INCITS 478-2011 w/ Amnd 1 INCITS 478/AM1-2014
    pub const SAS_2_1_INCITS_478_2011_AM1: Self = Self::from_u16(0x0C4F);
    /// SAS-2.1 T10/2125-D revision 04
    pub const SAS_2_1_R04: Self = Self::from_u16(0x0C48);
    /// SAS-2.1 T10/2125-D revision 06
    pub const SAS_2_1_R06: Self = Self::from_u16(0x0C4A);
    /// SAS-2.1 T10/2125-D revision 07
    pub const SAS_2_1_R07: Self = Self::from_u16(0x0C4B);
    /// SAS-3 (no version claimed)
    pub const SAS_3: Self = Self::from_u16(0x0C60);
    /// SAS-3 ISO/IEC 14776-154
    pub const SAS_3_ISO_IEC_14776_154: Self = Self::from_u16(0x0C6A);
    /// SAS-3 INCITS 519-2014
    pub const SAS_3_INCITS_519_2014: Self = Self::from_u16(0x0C68);
    /// SAS-3 T10/BSR INCITS 519 revision 05a
    pub const SAS_3_R05A: Self = Self::from_u16(0x0C63);
    /// SAS-3 T10/BSR INCITS 519 revision 06
    pub const SAS_3_R06: Self = Self::from_u16(0x0C65);
    /// SAS-4 (no version claimed)
    pub const SAS_4: Self = Self::from_u16(0x0C80);
    /// SAS-4 INCITS 534-2019
    pub const SAS_4_INCITS_534_2019: Self = Self::from_u16(0x0C92);
    /// SAS-4 T10/BSR INCITS 534 revision 09
    pub const SAS_4_R09: Self = Self::from_u16(0x0C84);
    /// SAS-4 T10/BSR INCITS 534 revision 08a
    pub const SAS_4_R08A: Self = Self::from_u16(0x0C82);
    /// SAS-4.1 (no version claimed)
    pub const SAS_4_1: Self = Self::from_u16(0x0CA0);
    /// SAS-4.1 INCITS 567-2023
    pub const SAS_4_1_INCITS_567_2023: Self = Self::from_u16(0x0CA2);
    /// SAS-4.1 BSR INCITS 567 revision 04
    pub const SAS_4_1_R04: Self = Self::from_u16(0x0CB0);
    /// SAS-4.1 BSR INCITS 567 revision 03
    pub const SAS_4_1_R03: Self = Self::from_u16(0x0CAF);
    /// SAS-5 (no version claimed)
    pub const SAS_5: Self = Self::from_u16(0x0CC0);
    /// SAS-5 BSR INCITS 561 revision 02
    pub const SAS_5_R02: Self = Self::from_u16(0x0CCB);
    /// SAT (no version claimed)
    pub const SAT: Self = Self::from_u16(0x1EA0);
    /// SAT INCITS 431-2007
    pub const SAT_INCITS_431_2007: Self = Self::from_u16(0x1EAD);
    /// SAT T10/1711-D revision 9
    pub const SAT_R9: Self = Self::from_u16(0x1EAB);
    /// SAT T10/1711-D revision 8
    pub const SAT_R8: Self = Self::from_u16(0x1EA7);
    /// SAT-2 (no version claimed)
    pub const SAT_2: Self = Self::from_u16(0x1EC0);
    /// SAT-2 INCITS 465-2010
    pub const SAT_2_INCITS_465_2010: Self = Self::from_u16(0x1ECA);
    /// SAT-2 T10/1826-D revision 06
    pub const SAT_2_R06: Self = Self::from_u16(0x1EC4);
    /// SAT-2 T10/1826-D revision 09
    pub const SAT_2_R09: Self = Self::from_u16(0x1EC8);
    /// SAT-3 (no version claimed)
    pub const SAT_3: Self = Self::from_u16(0x1EE0);
    /// SAT-3 INCITS 517-2015
    pub const SAT_3_INCITS_517_2015: Self = Self::from_u16(0x1EE8);
    /// SAT-3 T10/BSR INCITS 517 revision 4
    pub const SAT_3_R4: Self = Self::from_u16(0x1EE2);
    /// SAT-3 T10/BSR INCITS 517 revision 7
    pub const SAT_3_R7: Self = Self::from_u16(0x1EE4);
    /// SAT-4 (no version claimed)
    pub const SAT_4: Self = Self::from_u16(0x1F00);
    /// SAT-4 INCITS 491-2018
    pub const SAT_4_INCITS_491_2018: Self = Self::from_u16(0x1F0C);
    /// SAT-4 T10/BSR INCITS 491 revision 5
    pub const SAT_4_R5: Self = Self::from_u16(0x1F02);
    /// SAT-4 T10/BSR INCITS 491 revision 6
    pub const SAT_4_R6: Self = Self::from_u16(0x1F04);
    /// SAT-5 (no version claimed)
    pub const SAT_5: Self = Self::from_u16(0x1F20);
    /// SAT-5 INCITS 557-2023
    pub const SAT_5_INCITS_557_2023: Self = Self::from_u16(0x1F22);
    /// SAT-5 BSR INCITS 577 revision 10
    pub const SAT_5_R10: Self = Self::from_u16(0x1F25);
    /// SAT-6 (no version claimed)
    pub const SAT_6: Self = Self::from_u16(0x1F40);
    /// SBC (no version claimed)
    pub const SBC: Self = Self::from_u16(0x0180);
    /// SBC ISO/IEC 14776-321
    pub const SBC_ISO_IEC_14776_321: Self = Self::from_u16(0x019E);
    /// SBC INCITS 306-1998
    pub const SBC_INCITS_306_1998: Self = Self::from_u16(0x019C);
    /// SBC T10/0996-D revision 08c
    pub const SBC_R08C: Self = Self::from_u16(0x019B);
    /// SBC-2 (no version claimed)
    pub const SBC_2: Self = Self::from_u16(0x0320);
    /// SBC-2 ISO/IEC 14776-322
    pub const SBC_2_ISO_IEC_14776_322: Self = Self::from_u16(0x033E);
    /// SBC-2 INCITS 405-2005
    pub const SBC_2_INCITS_405_2005: Self = Self::from_u16(0x033D);
    /// SBC-2 T10/1417-D revision 16
    pub const SBC_2_R16: Self = Self::from_u16(0x033B);
    /// SBC-2 T10/1417-D revision 5a
    pub const SBC_2_R5A: Self = Self::from_u16(0x0322);
    /// SBC-2 T10/1417-D revision 15
    pub const SBC_2_R15: Self = Self::from_u16(0x0324);
    /// SBC-3 (no version claimed)
    pub const SBC_3: Self = Self::from_u16(0x04C0);
    /// SBC-3 ISO/IEC 14776-323
    pub const SBC_3_ISO_IEC_14776_323: Self = Self::from_u16(0x04CA);
    /// SBC-3 INCITS 514-2014
    pub const SBC_3_INCITS_514_2014: Self = Self::from_u16(0x04C8);
    /// SBC-3 T10/BSR INCITS 514 revision 35
    pub const SBC_3_R35: Self = Self::from_u16(0x04C3);
    /// SBC-3 T10/BSR INCITS 514 revision 36
    pub const SBC_3_R36: Self = Self::from_u16(0x04C5);
    /// SBC-4 (no version claimed)
    pub const SBC_4: Self = Self::from_u16(0x0600);
    /// SBC-4 INCITS 506-2021
    pub const SBC_4_INCITS_506_2021: Self = Self::from_u16(0x0602);
    /// SBC-4 BSR INCITS 506 revision 22
    pub const SBC_4_R22: Self = Self::from_u16(0x0610);
    /// SBC-4 BSR INCITS 506 revision 20a
    pub const SBC_4_R20A: Self = Self::from_u16(0x060F);
    /// SBC-5 (no version claimed)
    pub const SBC_5: Self = Self::from_u16(0x06C0);
    /// SBC-5 INCITS 571-2025
    pub const SBC_5_INCITS_571_2025: Self = Self::from_u16(0x06C2);
    /// SBC-5 BSR INCITS 571 revision 08
    pub const SBC_5_R08: Self = Self::from_u16(0x06C9);
    /// SBC-5 BSR INCITS 571 revision 07
    pub const SBC_5_R07: Self = Self::from_u16(0x06C8);
    /// SBC-6 (no version claimed)
    pub const SBC_6: Self = Self::from_u16(0x0720);
    /// SBP-2 (no version claimed)
    pub const SBP_2: Self = Self::from_u16(0x08E0);
    /// SBP-2 INCITS 325-1998
    pub const SBP_2_INCITS_325_1998: Self = Self::from_u16(0x08FC);
    /// SBP-2 T10/1155-D revision 04
    pub const SBP_2_R04: Self = Self::from_u16(0x08FB);
    /// SBP-3 (no version claimed)
    pub const SBP_3: Self = Self::from_u16(0x0980);
    /// SBP-3 INCITS 375-2004
    pub const SBP_3_INCITS_375_2004: Self = Self::from_u16(0x099C);
    /// SBP-3 T10/1467-D revision 5
    pub const SBP_3_R5: Self = Self::from_u16(0x099B);
    /// SBP-3 T10/1467-D revision 1f
    pub const SBP_3_R1F: Self = Self::from_u16(0x0982);
    /// SBP-3 T10/1467-D revision 3
    pub const SBP_3_R3: Self = Self::from_u16(0x0994);
    /// SBP-3 T10/1467-D revision 4
    pub const SBP_3_R4: Self = Self::from_u16(0x099A);
    /// SCC (no version claimed)
    pub const SCC: Self = Self::from_u16(0x0160);
    /// SCC INCITS 276-1997
    pub const SCC_INCITS_276_1997: Self = Self::from_u16(0x017C);
    /// SCC T10/1047-D revision 06c
    pub const SCC_R06C: Self = Self::from_u16(0x017B);
    /// SCC-2 (no version claimed)
    pub const SCC_2: Self = Self::from_u16(0x01E0);
    /// SCC-2 INCITS 318-1998
    pub const SCC_2_INCITS_318_1998: Self = Self::from_u16(0x01FC);
    /// SCC-2 T10/1125-D revision 04
    pub const SCC_2_R04: Self = Self::from_u16(0x01FB);
    /// SES (no version claimed)
    pub const SES: Self = Self::from_u16(0x01C0);
    /// SES INCITS 305-1998 w/ Amendment INCITS 305/AM1-2000
    pub const SES_INCITS_305_1998_AM1: Self = Self::from_u16(0x01DE);
    /// SES INCITS 305-1998
    pub const SES_INCITS_305_1998: Self = Self::from_u16(0x01DC);
    /// SES T10/1212-D revision 08b
    pub const SES_R08B: Self = Self::from_u16(0x01DB);
    /// SES T10/1212 revision 08b w/ Amendment INCITS 305/AM1-2000
    pub const SES_R08B_AM1: Self = Self::from_u16(0x01DD);
    /// SES-2 (no version claimed)
    pub const SES_2: Self = Self::from_u16(0x03E0);
    /// SES-2 ISO/IEC 14776-372
    pub const SES_2_ISO_IEC_14776_372: Self = Self::from_u16(0x03F2);
    /// SES-2 INCITS 448-2008
    pub const SES_2_INCITS_448_2008: Self = Self::from_u16(0x03F0);
    /// SES-2 T10/1559-D revision 16
    pub const SES_2_R16: Self = Self::from_u16(0x03E1);
    /// SES-2 T10/1559-D revision 19
    pub const SES_2_R19: Self = Self::from_u16(0x03E7);
    /// SES-2 T10/1559-D revision 20
    pub const SES_2_R20: Self = Self::from_u16(0x03EB);
    /// SES-3 (no version claimed)
    pub const SES_3: Self = Self::from_u16(0x0580);
    /// SES-3 INCITS 518-2017
    pub const SES_3_INCITS_518_2017: Self = Self::from_u16(0x0591);
    /// SES-3 T10/BSR INCITS 518 revision 14
    pub const SES_3_R14: Self = Self::from_u16(0x0584);
    /// SES-3 T10/BSR INCITS 518 revision 13
    pub const SES_3_R13: Self = Self::from_u16(0x0582);
    /// SES-4 (no version claimed)
    pub const SES_4: Self = Self::from_u16(0x0680);
    /// SES-4 INCITS 555-2020
    pub const SES_4_INCITS_555_2020: Self = Self::from_u16(0x0682);
    /// SES-4 BSR INCITS 555 revision 05
    pub const SES_4_R05: Self = Self::from_u16(0x0690);
    /// SES-4 BSR INCITS 555 revision 03
    pub const SES_4_R03: Self = Self::from_u16(0x068F);
    /// SFSC (no version claimed)
    pub const SFSC: Self = Self::from_u16(0x05E0);
    /// SFSC ISO/IEC 14776-481
    pub const SFSC_ISO_IEC_14776_481: Self = Self::from_u16(0x05EA);
    /// SFSC INCITS 501-2016
    pub const SFSC_INCITS_501_2016: Self = Self::from_u16(0x05E8);
    /// SFSC BSR INCITS 501 revision 01
    pub const SFSC_R01: Self = Self::from_u16(0x05E3);
    /// SFSC BSR INCITS 501 revision 02
    pub const SFSC_R02: Self = Self::from_u16(0x05E5);
    /// SIP (no version claimed)
    pub const SIP: Self = Self::from_u16(0x08A0);
    /// SIP INCITS 292-1997
    pub const SIP_INCITS_292_1997: Self = Self::from_u16(0x08BC);
    /// SIP T10/0856-D revision 10
    pub const SIP_R10: Self = Self::from_u16(0x08BB);
    /// SMC (no version claimed)
    pub const SMC: Self = Self::from_u16(0x01A0);
    /// SMC ISO/IEC 14776-351
    pub const SMC_ISO_IEC_14776_351: Self = Self::from_u16(0x01BE);
    /// SMC INCITS 314-1998
    pub const SMC_INCITS_314_1998: Self = Self::from_u16(0x01BC);
    /// SMC T10/0999-D revision 10a
    pub const SMC_R10A: Self = Self::from_u16(0x01BB);
    /// SMC-2 (no version claimed)
    pub const SMC_2: Self = Self::from_u16(0x02E0);
    /// SMC-2 INCITS 382-2004
    pub const SMC_2_INCITS_382_2004: Self = Self::from_u16(0x02FE);
    /// SMC-2 T10/1383-D revision 7
    pub const SMC_2_R7: Self = Self::from_u16(0x02FD);
    /// SMC-2 T10/1383-D revision 5
    pub const SMC_2_R5: Self = Self::from_u16(0x02F5);
    /// SMC-2 T10/1383-D revision 6
    pub const SMC_2_R6: Self = Self::from_u16(0x02FC);
    /// SMC-3 (no version claimed)
    pub const SMC_3: Self = Self::from_u16(0x0480);
    /// SMC-3 INCITS 484-2012
    pub const SMC_3_INCITS_484_2012: Self = Self::from_u16(0x0486);
    /// SMC-3 T10/1730-D revision 15
    pub const SMC_3_R15: Self = Self::from_u16(0x0482);
    /// SMC-3 T10/1730-D revision 16
    pub const SMC_3_R16: Self = Self::from_u16(0x0484);
    /// SNT (no version claimed)
    pub const SNT: Self = Self::from_u16(0x1F60);
    /// SOP (no version claimed)
    pub const SOP: Self = Self::from_u16(0x21E0);
    /// SOP INCITS 489-2014
    pub const SOP_INCITS_489_2014: Self = Self::from_u16(0x21E8);
    /// SOP T10/BSR INCITS 489 revision 4
    pub const SOP_R4: Self = Self::from_u16(0x21E4);
    /// SOP T10/BSR INCITS 489 revision 5
    pub const SOP_R5: Self = Self::from_u16(0x21E6);
    /// SOP-2 (no draft published)
    pub const SOP_2: Self = Self::from_u16(0x2220);
    /// SPC (no version claimed)
    pub const SPC: Self = Self::from_u16(0x0120);
    /// SPC INCITS 301-1997
    pub const SPC_INCITS_301_1997: Self = Self::from_u16(0x013C);
    /// SPC T10/0995-D revision 11a
    pub const SPC_R11A: Self = Self::from_u16(0x013B);
    /// SPC-2 (no version claimed)
    pub const SPC_2: Self = Self::from_u16(0x0260);
    /// SPC-2 ISO/IEC 14776-452
    pub const SPC_2_ISO_IEC_14776_452: Self = Self::from_u16(0x0278);
    /// SPC-2 INCITS 351-2001
    pub const SPC_2_INCITS_351_2001: Self = Self::from_u16(0x0277);
    /// SPC-2 T10/1236-D revision 20
    pub const SPC_2_R20: Self = Self::from_u16(0x0276);
    /// SPC-2 T10/1236-D revision 12
    pub const SPC_2_R12: Self = Self::from_u16(0x0267);
    /// SPC-2 T10/1236-D revision 18
    pub const SPC_2_R18: Self = Self::from_u16(0x0269);
    /// SPC-2 T10/1236-D revision 19
    pub const SPC_2_R19: Self = Self::from_u16(0x0275);
    /// SPC-3 (no version claimed)
    pub const SPC_3: Self = Self::from_u16(0x0300);
    /// SPC-3 ISO/IEC 14776-453
    pub const SPC_3_ISO_IEC_14776_453: Self = Self::from_u16(0x0316);
    /// SPC-3 INCITS 408-2005
    pub const SPC_3_INCITS_408_2005: Self = Self::from_u16(0x0314);
    /// SPC-3 T10/1416-D revision 7
    pub const SPC_3_R7: Self = Self::from_u16(0x0301);
    /// SPC-3 T10/1416-D revision 21
    pub const SPC_3_R21: Self = Self::from_u16(0x0307);
    /// SPC-3 T10/1416-D revision 22
    pub const SPC_3_R22: Self = Self::from_u16(0x030F);
    /// SPC-3 T10/1416-D revision 23
    pub const SPC_3_R23: Self = Self::from_u16(0x0312);
    /// SPC-4 (no version claimed)
    pub const SPC_4: Self = Self::from_u16(0x0460);
    /// SPC-4 ISO/IEC 14776-454
    pub const SPC_4_ISO_IEC_14776_454: Self = Self::from_u16(0x046E);
    /// SPC-4 INCITS 513-2015
    pub const SPC_4_INCITS_513_2015: Self = Self::from_u16(0x046C);
    /// SPC-4 T10/BSR INCITS 513 revision 16
    pub const SPC_4_R16: Self = Self::from_u16(0x0461);
    /// SPC-4 T10/BSR INCITS 513 revision 18
    pub const SPC_4_R18: Self = Self::from_u16(0x0462);
    /// SPC-4 T10/BSR INCITS 513 revision 23
    pub const SPC_4_R23: Self = Self::from_u16(0x0463);
    /// SPC-4 T10/BSR INCITS 513 revision 36
    pub const SPC_4_R36: Self = Self::from_u16(0x0466);
    /// SPC-4 T10/BSR INCITS 513 revision 37
    pub const SPC_4_R37: Self = Self::from_u16(0x0468);
    /// SPC-4 T10/BSR INCITS 513 revision 37a
    pub const SPC_4_R37A: Self = Self::from_u16(0x0469);
    /// SPC-5 (no version claimed)
    pub const SPC_5: Self = Self::from_u16(0x05C0);
    /// SPC-5 INCITS 502-2019
    pub const SPC_5_INCITS_502_2019: Self = Self::from_u16(0x05C2);
    /// SPC-5 BSR INCITS 502 revision 22
    pub const SPC_5_R22: Self = Self::from_u16(0x05CB);
    /// SPC-6 (no version claimed)
    pub const SPC_6: Self = Self::from_u16(0x06E0);
    /// SPC-6 INCITS 566-2025
    pub const SPC_6_INCITS_566_2025: Self = Self::from_u16(0x06E2);
    /// SPC-6 SCSI/INCITS 566 revision 13
    pub const SPC_6_R13: Self = Self::from_u16(0x06E5);
    /// SPC-6 SCSI/INCITS 566 revision 12
    pub const SPC_6_R12: Self = Self::from_u16(0x06E8);
    /// SPC-7 (no version claimed)
    pub const SPC_7: Self = Self::from_u16(0x0700);
    /// SPI (no version claimed)
    pub const SPI: Self = Self::from_u16(0x0AA0);
    /// SPI INCITS 253-1995
    pub const SPI_INCITS_253_1995: Self = Self::from_u16(0x0ABA);
    /// SPI T10/0855-D revision 15a
    pub const SPI_R15A: Self = Self::from_u16(0x0AB9);
    /// SPI INCITS 253-1995 with SPI Amnd INCITS 253/AM1-1998
    pub const SPI_INCITS_253_1995_AM1: Self = Self::from_u16(0x0ABC);
    /// SPI T10/0855-D revision 15a with SPI Amnd revision 3a
    pub const SPI_R15A_AM1_R3A: Self = Self::from_u16(0x0ABB);
    /// SPI-2 (no version claimed)
    pub const SPI_2: Self = Self::from_u16(0x0AE0);
    /// SPI-2 INCITS 302-1999
    pub const SPI_2_INCITS_302_1999: Self = Self::from_u16(0x0AFC);
    /// SPI-2 T10/1142-D revision 20b
    pub const SPI_2_R20B: Self = Self::from_u16(0x0AFB);
    /// SPI-3 (no version claimed)
    pub const SPI_3: Self = Self::from_u16(0x0B00);
    /// SPI-3 INCITS 336-2000
    pub const SPI_3_INCITS_336_2000: Self = Self::from_u16(0x0B1C);
    /// SPI-3 T10/1302-D revision 14
    pub const SPI_3_R14: Self = Self::from_u16(0x0B1A);
    /// SPI-3 T10/1302-D revision 10
    pub const SPI_3_R10: Self = Self::from_u16(0x0B18);
    /// SPI-3 T10/1302-D revision 13a
    pub const SPI_3_R13A: Self = Self::from_u16(0x0B19);
    /// SPI-4 (no version claimed)
    pub const SPI_4: Self = Self::from_u16(0x0B40);
    /// SPI-4 INCITS 362-2002
    pub const SPI_4_INCITS_362_2002: Self = Self::from_u16(0x0B56);
    /// SPI-4 T10/1365-D revision 7
    pub const SPI_4_R7: Self = Self::from_u16(0x0B54);
    /// SPI-4 T10/1365-D revision 9
    pub const SPI_4_R9: Self = Self::from_u16(0x0B55);
    /// SPI-4 T10/1365-D revision 10
    pub const SPI_4_R10: Self = Self::from_u16(0x0B59);
    /// SPI-5 (no version claimed)
    pub const SPI_5: Self = Self::from_u16(0x0B60);
    /// SPI-5 INCITS 367-2003
    pub const SPI_5_INCITS_367_2003: Self = Self::from_u16(0x0B7C);
    /// SPI-5 T10/1525-D revision 6
    pub const SPI_5_R6: Self = Self::from_u16(0x0B7B);
    /// SPI-5 T10/1525-D revision 3
    pub const SPI_5_R3: Self = Self::from_u16(0x0B79);
    /// SPI-5 T10/1525-D revision 5
    pub const SPI_5_R5: Self = Self::from_u16(0x0B7A);
    /// SPL (no version claimed)
    pub const SPL: Self = Self::from_u16(0x20A0);
    /// SPL ISO/IEC 14776-261
    pub const SPL_ISO_IEC_14776_261: Self = Self::from_u16(0x20AA);
    /// SPL INCITS 476-2011
    pub const SPL_INCITS_476_2011: Self = Self::from_u16(0x20A7);
    /// SPL INCITS 476-2011 + SPL AM1 INCITS 476/AM1 2012
    pub const SPL_INCITS_476_2011_AM1: Self = Self::from_u16(0x20A8);
    /// SPL T10/2124-D revision 6a
    pub const SPL_R6A: Self = Self::from_u16(0x20A3);
    /// SPL T10/2124-D revision 7
    pub const SPL_R7: Self = Self::from_u16(0x20A5);
    /// SPL-2 (no version claimed)
    pub const SPL_2: Self = Self::from_u16(0x20C0);
    /// SPL-2 ISO/IEC 14776-262
    pub const SPL_2_ISO_IEC_14776_262: Self = Self::from_u16(0x20C9);
    /// SPL-2 INCITS 505-2013
    pub const SPL_2_INCITS_505_2013: Self = Self::from_u16(0x20C8);
    /// SPL-2 T10/BSR INCITS 505 revision 4
    pub const SPL_2_R4: Self = Self::from_u16(0x20C2);
    /// SPL-2 T10/BSR INCITS 505 revision 5
    pub const SPL_2_R5: Self = Self::from_u16(0x20C4);
    /// SPL-3 (no version claimed)
    pub const SPL_3: Self = Self::from_u16(0x20E0);
    /// SPL-3 ISO/IEC 14776-263
    pub const SPL_3_ISO_IEC_14776_263: Self = Self::from_u16(0x20E9);
    /// SPL-3 INCITS 492-2015
    pub const SPL_3_INCITS_492_2015: Self = Self::from_u16(0x20E8);
    /// SPL-3 T10/BSR INCITS 492 revision 6
    pub const SPL_3_R6: Self = Self::from_u16(0x20E4);
    /// SPL-3 T10/BSR INCITS 492 revision 7
    pub const SPL_3_R7: Self = Self::from_u16(0x20E6);
    /// SPL-4 (no version claimed)
    pub const SPL_4: Self = Self::from_u16(0x2100);
    /// SPL-4 INCITS 538-2018
    pub const SPL_4_INCITS_538_2018: Self = Self::from_u16(0x2110);
    /// SPL-4 T10/BSR INCITS 538 revision 13
    pub const SPL_4_R13: Self = Self::from_u16(0x2107);
    /// SPL-4 T10/BSR INCITS 538 revision 08a
    pub const SPL_4_R08A: Self = Self::from_u16(0x2102);
    /// SPL-4 T10/BSR INCITS 538 revision 10
    pub const SPL_4_R10: Self = Self::from_u16(0x2104);
    /// SPL-4 T10/BSR INCITS 538 revision 11
    pub const SPL_4_R11: Self = Self::from_u16(0x2105);
    /// SPL-5 (no version claimed)
    pub const SPL_5: Self = Self::from_u16(0x2120);
    /// SPL-5 INCITS 554-2023
    pub const SPL_5_INCITS_554_2023: Self = Self::from_u16(0x2122);
    /// SPL-5 BSR INCITS 554 revision 15
    pub const SPL_5_R15: Self = Self::from_u16(0x212F);
    /// SPL-5 BSR INCITS 554 revision 14
    pub const SPL_5_R14: Self = Self::from_u16(0x212E);
    /// SRP (no version claimed)
    pub const SRP: Self = Self::from_u16(0x0940);
    /// SRP INCITS 365-2002
    pub const SRP_INCITS_365_2002: Self = Self::from_u16(0x095C);
    /// SRP T10/1415-D revision 16a
    pub const SRP_R16A: Self = Self::from_u16(0x0955);
    /// SRP T10/1415-D revision 10
    pub const SRP_R10: Self = Self::from_u16(0x0954);
    /// SRP-2 (no version claimed)
    pub const SRP_2: Self = Self::from_u16(0x09A0);
    /// SRP-2 INCITS 551-2019
    pub const SRP_2_INCITS_551_2019: Self = Self::from_u16(0x09BC);
    /// SSA-PH2 (no version claimed)
    pub const SSA_PH2: Self = Self::from_u16(0x1360);
    /// SSA-PH2 INCITS 293-1996
    pub const SSA_PH2_INCITS_293_1996: Self = Self::from_u16(0x137C);
    /// SSA-PH2 T10.1/1145-D revision 09c
    pub const SSA_PH2_R09C: Self = Self::from_u16(0x137B);
    /// SSA-PH3 (no version claimed)
    pub const SSA_PH3: Self = Self::from_u16(0x1380);
    /// SSA-PH3 INCITS 307-1998
    pub const SSA_PH3_INCITS_307_1998: Self = Self::from_u16(0x139C);
    /// SSA-PH3 T10.1/1146-D revision 05b
    pub const SSA_PH3_R05B: Self = Self::from_u16(0x139B);
    /// SSA-S2P (no version claimed)
    pub const SSA_S2P: Self = Self::from_u16(0x0880);
    /// SSA-S2P INCITS 294-1996
    pub const SSA_S2P_INCITS_294_1996: Self = Self::from_u16(0x089C);
    /// SSA-S2P T10.1/1121-D revision 07b
    pub const SSA_S2P_R07B: Self = Self::from_u16(0x089B);
    /// SSA-S3P (no version claimed)
    pub const SSA_S3P: Self = Self::from_u16(0x0860);
    /// SSA-S3P INCITS 309-1998
    pub const SSA_S3P_INCITS_309_1998: Self = Self::from_u16(0x087C);
    /// SSA-S3P T10.1/1051-D revision 05b
    pub const SSA_S3P_R05B: Self = Self::from_u16(0x087B);
    /// SSA-TL1 (no version claimed)
    pub const SSA_TL1: Self = Self::from_u16(0x0840);
    /// SSA-TL1 INCITS 295-1996
    pub const SSA_TL1_INCITS_295_1996: Self = Self::from_u16(0x085C);
    /// SSA-TL1 T10.1/0989-D revision 10b
    pub const SSA_TL1_R10B: Self = Self::from_u16(0x085B);
    /// SSA-TL2 (no version claimed)
    pub const SSA_TL2: Self = Self::from_u16(0x0820);
    /// SSA-TL2 INCITS 308-1998
    pub const SSA_TL2_INCITS_308_1998: Self = Self::from_u16(0x083C);
    /// SSA-TL2 T10.1/1147-D revision 05b
    pub const SSA_TL2_R05B: Self = Self::from_u16(0x083B);
    /// SSC (no version claimed)
    pub const SSC: Self = Self::from_u16(0x0200);
    /// SSC ISO/IEC 14776-331
    pub const SSC_ISO_IEC_14776_331: Self = Self::from_u16(0x021E);
    /// SSC INCITS 335-2000
    pub const SSC_INCITS_335_2000: Self = Self::from_u16(0x021C);
    /// SSC T10/0997-D revision 22
    pub const SSC_R22: Self = Self::from_u16(0x0207);
    /// SSC T10/0997-D revision 17
    pub const SSC_R17: Self = Self::from_u16(0x0201);
    /// SSC-2 (no version claimed)
    pub const SSC_2: Self = Self::from_u16(0x0360);
    /// SSC-2 ISO/IEC 14776-342
    pub const SSC_2_ISO_IEC_14776_342: Self = Self::from_u16(0x037E);
    /// SSC-2 INCITS 380-2003
    pub const SSC_2_INCITS_380_2003: Self = Self::from_u16(0x037D);
    /// SSC-2 T10/1434-D revision 9
    pub const SSC_2_R9: Self = Self::from_u16(0x0375);
    /// SSC-2 T10/1434-D revision 7
    pub const SSC_2_R7: Self = Self::from_u16(0x0374);
    /// SSC-3 (no version claimed)
    pub const SSC_3: Self = Self::from_u16(0x0400);
    /// SSC-3 ISO/IEC 14776-333
    pub const SSC_3_ISO_IEC_14776_333: Self = Self::from_u16(0x040B);
    /// SSC-3 INCITS 467-2011
    pub const SSC_3_INCITS_467_2011: Self = Self::from_u16(0x0409);
    /// SSC-3 T10/1611-D revision 04a
    pub const SSC_3_R04A: Self = Self::from_u16(0x0403);
    /// SSC-3 T10/1611-D revision 05
    pub const SSC_3_R05: Self = Self::from_u16(0x0407);
    /// SSC-4 (no version claimed)
    pub const SSC_4: Self = Self::from_u16(0x0520);
    /// SSC-4 INCITS 516-2013
    pub const SSC_4_INCITS_516_2013: Self = Self::from_u16(0x0527);
    /// SSC-4 T10/BSR INCITS 516 revision 2
    pub const SSC_4_R2: Self = Self::from_u16(0x0523);
    /// SSC-4 T10/BSR INCITS 516 revision 3
    pub const SSC_4_R3: Self = Self::from_u16(0x0525);
    /// SSC-5 (no version claimed)
    pub const SSC_5: Self = Self::from_u16(0x05A0);
    /// SSC-5 BSR INCITS 503-2022
    pub const SSC_5_INCITS_503_2022: Self = Self::from_u16(0x05A2);
    /// SSC-5 BSR INCITS 503 AM1-2025
    pub const SSC_5_INCITS_503_AM1_2025: Self = Self::from_u16(0x05B1);
    /// SSC-5 BSR INCITS 503 revision 06
    pub const SSC_5_R06: Self = Self::from_u16(0x05AB);
    /// SSC-5 AM1 (no version claimed)
    pub const SSC_5_AM1: Self = Self::from_u16(0x05AF);
    /// SSC-5 BSR INCITS 503 AM1 revision 00
    pub const SSC_5_AM1_R00: Self = Self::from_u16(0x05B0);
    /// SST (no version claimed)
    pub const SST: Self = Self::from_u16(0x0920);
    /// SST T10/1380-D revision 8b
    pub const SST_R8B: Self = Self::from_u16(0x0935);
    /// UAS (no version claimed)
    pub const UAS: Self = Self::from_u16(0x1740);
    /// UAS ISO/IEC 14776-251
    pub const UAS_ISO_IEC_14776_251: Self = Self::from_u16(0x1749);
    /// UAS INCITS 471-2010
    pub const UAS_INCITS_471_2010: Self = Self::from_u16(0x1748);
    /// UAS T10/2095-D revision 02
    pub const UAS_R02: Self = Self::from_u16(0x1743);
    /// UAS T10/2095-D revision 04
    pub const UAS_R04: Self = Self::from_u16(0x1747);
    /// UAS-2 (no version claimed)
    pub const UAS_2: Self = Self::from_u16(0x1780);
    /// UAS-3 (no version claimed)
    pub const UAS_3: Self = Self::from_u16(0x17C0);
    /// UAS-3 INCITS 572-2021
    pub const UAS_3_INCITS_572_2021: Self = Self::from_u16(0x17C2);
    /// UAS-3 BSR INCITS 572 revision 05
    pub const UAS_3_R05: Self = Self::from_u16(0x17C5);
    /// UAS-3 ISO/IEC 14776-253
    pub const UAS_3_ISO_IEC_14776_253: Self = Self::from_u16(0x1807);
    /// Universal Serial Bus Specification, Revision 1.1
    pub const USB_1_1: Self = Self::from_u16(0x1728);
    /// Universal Serial Bus Specification, Revision 2.0
    pub const USB_2_0: Self = Self::from_u16(0x1729);
    /// Universal Serial Bus 3.2 Specification Revision 1.0
    pub const USB_3_2_R1_0: Self = Self::from_u16(0x172A);
    /// Universal Serial Bus 4 Specification Version 1.0
    pub const USB4_V1_0: Self = Self::from_u16(0x172B);
    /// Universal Serial Bus 4 Specification Version 2.0
    pub const USB4_V2_0: Self = Self::from_u16(0x172C);
    /// USB Mass Storage Class Bulk-Only Transport, Revision 1.0
    pub const USB_MSC_BOT_R1_0: Self = Self::from_u16(0x1730);
    /// ZAC (no version claimed)
    pub const ZAC: Self = Self::from_u16(0x17A3);
    /// ZAC INCITS 537-2016
    pub const ZAC_INCITS_537_2016: Self = Self::from_u16(0x17A4);
    /// ZAC AM1 INCITS 537-2016/AM1-2019
    pub const ZAC_INCITS_537_2016_AM1_2019: Self = Self::from_u16(0x17BC);
    /// ZAC-2 (no version claimed)
    pub const ZAC_2: Self = Self::from_u16(0x17A6);
    /// ZAC-2 INCITS 549-2022
    pub const ZAC_2_INCITS_549_2022: Self = Self::from_u16(0x17A5);
    /// ZAC-3 (no version claimed)
    pub const ZAC_3: Self = Self::from_u16(0x17A7);
    /// ZBC (no version claimed)
    pub const ZBC: Self = Self::from_u16(0x0620);
    /// ZBC INCITS 536-2016
    pub const ZBC_INCITS_536_2016: Self = Self::from_u16(0x0628);
    /// ZBC AM1 INCITS 536-2016/AM1-2019
    pub const ZBC_INCITS_536_2016_AM1_2019: Self = Self::from_u16(0x0629);
    /// ZBC BSR INCITS 536 revision 02
    pub const ZBC_R02: Self = Self::from_u16(0x0622);
    /// ZBC BSR INCITS 536 revision 05
    pub const ZBC_R05: Self = Self::from_u16(0x0624);
    /// ZBC-2 (no version claimed)
    pub const ZBC_2: Self = Self::from_u16(0x0660);
    /// ZBC-2 INCITS 550-2023
    pub const ZBC_2_INCITS_550_2023: Self = Self::from_u16(0x0662);
    /// ZBC-2 BSR INCITS 550 revision 13
    pub const ZBC_2_R13: Self = Self::from_u16(0x066B);
    /// ZBC-3 (no version claimed)
    pub const ZBC_3: Self = Self::from_u16(0x06A0);
    /// Version Descriptor Not Supported or No Standard Identified
    pub const NOT_SUPPORTED: Self = Self::from_u16(0x0000);
}
