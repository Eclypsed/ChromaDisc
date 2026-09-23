# Quirks

This is a place for me to make note of all drive quirks I should be aware of when developing the application side of ChromaDisc.

A quirk is any way in which a drive diverges from the specfications that define it.

## Test environment

- **Drive:** ASUS DRW-24B1ST j, firmware 1.11 (INQUIRY vendor-specific: `2016/10/11 20:37`). Chipset unconfirmed; other DRW-24B1ST revisions are reported as Lite-On iHAS clones, and the "i" revision uses a MediaTek MT1882N.
- **Connection:** SATA drive in a USB enclosure (USB–SATA bridge), accessed through Linux `SG_IO` on `/dev/sr0`.
- **Transfer limit:** `max_hw_sectors_kb` = 120 (122,880 bytes, i.e. 52 × 2352-byte sectors).
- **Host note:** vendor opcodes such as `0xD5` fail with `EPERM` under `SG_IO` unless the process has `CAP_SYS_RAWIO`. This is the kernel's command filter, not the drive.
- **Test disc:** Enhanced CD (Blue Book). Session 1: 11 CD-DA tracks with ISRCs. Session 2: one CD-ROM XA Mode 2 data track starting at LBA 251949.

## READ CD / READ CD MSF

### 1. Sub-channel returned before C2 error information

**Status:** Confirmed.
**Commands:** READ CD (`0xBE`), READ CD MSF (`0xB9`).
**Sectors:** CD-DA and CD-ROM XA.

**Spec (MMC-6):** C2 error information follows the main channel, and sub-channel data follows the C2 information.

**Observed:** sub-channel data comes directly after the main channel, and C2 information comes last. Seen with formatted Q (16 bytes) and raw P-W (96 bytes), and with both C2 error codes (01b and 10b).

Example: 1 sector, main channel + C2 01b + raw P-W.

| Field              | Spec offset | Observed offset |
|--------------------|-------------|-----------------|
| Main channel (2352) | 0          | 0               |
| C2 01b (294)       | 2352        | 2448            |
| Raw P-W (96)       | 2646        | 2352            |

**Workaround:** parse responses as main → sub → C2 on this drive.

### 2. Transfers rounded up to a multiple of 4 bytes (transport, not drive)

**Status:** Explained. Attributed to the USB–SATA bridge; not yet re-tested on a native SATA port.

**Spec (MMC-6):** C2 error code 01b returns 294 bytes per sector, so the transfer length is the sum of the requested fields.

**Observed:** when the expected length isn't a multiple of 4, the transfer is padded up to the next multiple of 4. With C2 01b this happens whenever an odd number of sectors is requested, because 294 ≡ 2 (mod 4) while 2352, 96, 16 and 296 are all multiples of 4. Pad bytes have always been zero. C2 10b is never affected, since 296 bytes per sector keeps every transfer aligned.

| Request                              | Expected | Received |
|--------------------------------------|----------|----------|
| 1 sector, C2 01b                     | 294      | 296      |
| 1 sector, main + C2 01b              | 2646     | 2648     |
| 1 sector, main + C2 01b + formatted Q | 2662    | 2664     |
| 2 sectors, C2 01b                    | 588      | 588      |
| 5 sectors, C2 01b                    | 1470     | 1472     |

Residual behavior, 5 sectors with C2 01b:

| Buffer size | Bytes received | `resid` |
|-------------|----------------|---------|
| 1470        | 1470           | 0       |
| 1472        | 1472           | 0       |
| 1473        | 1472           | 1       |

The pad is silently dropped when the buffer is exact and delivered as data when the buffer is larger. No overrun is reported either way.

**Likely cause:** SATA moves data in 4-byte dwords, so a 1470-byte transfer is padded to 1472 on the link, and the bridge passes the padding through to the host.

**Workaround:** compute the expected payload length from the CDB instead of from `dxfer_len - resid`, or size buffers exactly.

### 3. C2 10b: nonzero last byte and inconsistent block error byte in XA sectors

**Status:** Open. Needs a sector with real C2 errors to resolve.

**Spec (MMC-6):** 296 bytes per sector: block error byte (logical OR of the 294 C2 bytes), then a pad byte of 0, then the 294 C2 error bytes. Earlier MMC revisions only say the block error byte is followed by an undefined pad byte, without fixing the pair's position relative to the C2 bytes.

**Observed:**

| Byte(s) | Spec (MMC-6)                                   | CD-DA sectors | CD-ROM XA sectors |
|---------|------------------------------------------------|---------------|-------------------|
| 0       | Block error byte (OR of bytes 2–295)           | `0x00`        | `0x00`            |
| 1       | Pad, `0x00`                                    | `0x00`        | `0x00`            |
| 2–294   | C2 error bits                                  | all `0x00`    | all `0x00`        |
| 295     | C2 error bits for main-channel bytes 2344–2351 | `0x00`        | `0x1F`            |

The same pattern appears in every XA sector tested, regardless of main-channel and sub-channel selection.

**Spec conflict:** under the MMC-6 layout, byte 295 is nonzero, so byte 0 should be nonzero too. It isn't.

**Evidence the `0x1F` isn't a C2 flag:** C2 01b on the same sectors returns 294 zero bytes. If `0x1F` were C2 flags, it would mark main-channel bytes 2347–2351 and would also appear as the last byte of the 01b data.

**Hypothesis:** the firmware uses a trailing layout consistent with the older wording: 294 C2 bytes, then the block error byte, then an undefined pad byte. Under that reading the C2 bytes are all zero, the block error byte (byte 294) is correctly zero, and `0x1F` is the pad. That would still be non-compliant with MMC-6. Why the pad is nonzero only in XA sectors is unknown.

**To resolve:** find a sector with real C2 errors (a damaged disc, a marker-stripe test disc, or a disc with deliberate C2 errors from copy protection) and compare the 01b and 10b output for it.

- If the nonzero bits from 01b line up with 10b bytes 0–293, the layout is trailing.
- If they line up with bytes 2–295, the layout is MMC-6 and the block error byte is wrong.

### 4. Read cache served the wrong sector

**Status:** Seen once. Trigger unknown, not reproduced.

**Spec:** READ CD returns the addressed sector. Its CDB has no FUA bit, so the host can't force a media read, and caching is expected to be transparent.

**Observed:** READ CD for LBA 28635 (CD-DA, main channel + raw P-W) returned a main channel starting with the data sync pattern (`00 FF FF FF FF FF FF FF FF FF FF 00`) followed by garbled data, and the raw Q was also wrong. Repeating the identical request returned the same bytes. It cleared only after reading a different LBA (1000); after that, LBA 28635 returned correct audio and Q.

The persistence implies that repeated reads of the same LBA are served from cache rather than re-read from the disc.

**Workarounds:**

- Before retrying a sector (for C2 errors or verification), read a distant LBA to evict it from cache.
- When reading CD-DA with sub-channel, check that Q's absolute time matches the requested LBA.

**If it recurs:** record the exact command sequence leading up to it.
