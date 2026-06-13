# Q Sub-Channel

## Overview

The Q sub-channel is a 96-bit per sector channel within the CD subcode block. It carries timing, track identification, and disc metadata used during playback and navigation.

### Record Format

| Field | Size | Description |
|-------|------|-------------|
| S0, S1 | 2 bits | Sub-channel synchronization |
| CONTROL | 4 bits | Frame type (audio channels, pre-emphasis, copy permission) |
| ADR | 4 bits | DATA-Q mode identifier |
| DATA-Q | 72 bits | Payload; interpretation depends on disc area and ADR mode |
| EDC | 16 bits | Error detection code over CONTROL + ADR + DATA-Q; bits are inverted on disc; remainder must be zero |

After subtracting the 2 sync bits and 2 EDC bytes, the usable Q payload is **10 bytes**.

#### CONTROL Field Bit Patterns

| Value | Meaning |
|-------|---------|
| `00x0b` | 2 audio channels, no pre-emphasis |
| `00x1b` | 2 audio channels, 50/15 μs pre-emphasis |
| `10x0b` | 4 audio channels, no pre-emphasis |
| `10x1b` | 4 audio channels, 50/15 μs pre-emphasis |
| `01x0b` | Data track, recorded uninterrupted |
| `01x1b` | Data track, recorded incrementally |
| `11xxb` | Reserved |
| `xx0xb` | Digital copy prohibited |
| `xx1xb` | Digital copy permitted |

> **Note:** CONTROL bits (except the copy bit) may only change during a pause (`X=00`) of at least 2 seconds, or within the Lead-in Area.
>
> **Orange Book caveat — Copy Bit (bit 1):** In the Program Area of Orange Book discs (CD-R / CD-RW), the copy bit has a third state and may change during a track:
>
> | State | Meaning |
> |-------|---------|
> | Continuous `0` | Track is copy-protected |
> | Continuous `1` | Track is not copy-protected; copying is permitted |
> | Alternating `1`/`0` | Track is a first- or higher-generation copy of a copy-protected track |
>
> The alternating state runs at 9.375 Hz with a 50% duty cycle: four successive Subcode frames at `1`, followed by four frames at `0`.

---

## DATA-Q Interpretation by Disc Area

The 9 bytes of DATA-Q are interpreted differently depending on the disc area and ADR mode.

**Disc areas:**

- **Lead-in** — present on all disc types
- **Program** — present on all disc types
- **Lead-out** — present on all disc types
- **Program Memory Area (PMA)** — Orange Book only (CD-R / CD-RW); temporary store for partially recorded discs

---

## Program Memory Area (PMA) — Orange Book Only

### DATA-Q Layout

```text
+-----------------------------------------------------------------------+
|                                DATA-Q                                 |
+--------+-------+------+------+-------+------+------+------+-----------+
| TNO=00 | POINT | MIN  | SEC  | FRAME | ZERO | PMIN | PSEC | PFRAME    |
+--------+-------+------+------+-------+------+------+------+-----------+
```

- `TNO` is always `00` in the PMA.
- `ZERO` (`00`–`09`) is a frame counter within a Unity of ten Subcode frames; the first frame is labeled 0, the last 9.

### ADR Modes

#### ADR = 1 — Table of Contents

Lists track numbers and start/stop times for all tracks.

- `POINT = 01..99`: Value equals the track number `n`.
  - `PMIN`, `PSEC`, `PFRAME`: start time of the track indicated by POINT.
  - `MIN`, `SEC`, `FRAME`: stop time of the track indicated by POINT.
  - If the track is an Incomplete Track, `MIN`/`SEC`/`FRAME` = `FF FF FF` (hex) as a dummy stop time. Once completed, this entry is overwritten with the actual stop time.

#### ADR = 2 — Disc Identification

Mandatory. Records a statistically unique 24-bit binary disc identifier.

- `MIN`, `SEC`, `FRAME`: together form the 24-bit Disc Identification number, chosen randomly per disc.
- `PSEC`: specifies the format of all Data Sessions on the disc (all sessions must share the same format):

  | PSEC (hex) | Session Format |
  |------------|----------------|
  | `00` | CD-DA or CD-ROM |
  | `10` | CD-i |
  | `20` | CD-ROM XA |
  | Other | Reserved |

- `POINT`, `PMIN`, `PFRAME`: reserved, set to `00`.

If the session format changes (e.g., a CD-ROM XA session is added after an audio session), a new Disc Identification Item must be written with the same 24-bit number and the updated `PSEC`.

#### ADR = 3 — Skip Track (Audio Sessions Only)

Optional. Up to 6 track numbers per item may be marked for skipping during playback. Not permitted in Data Sessions.

- `POINT = 01..21`: J-th Skip Track assignment.
- `MIN`, `SEC`, `FRAME`, `PMIN`, `PSEC`, `PFRAME`: each may contain a track number to skip. Unused bytes are set to `00`.

#### ADR = 4 — Unskip Track

Optional. Cancels a previously recorded Skip Track assignment.

- `POINT = 01..21`: K-th Unskip Track assignment.
- `MIN`, `SEC`, `FRAME`, `PMIN`, `PSEC`, `PFRAME`: each may contain a track number to restore to normal playback. Unused bytes are set to `00`.

#### ADR = 5 — Skip Time Interval (Audio Sessions Only)

Optional. Marks a time interval in the Program Area to be skipped during playback. Not permitted in Data Sessions.

- `POINT = 01..40`: M-th Skip Time Interval assignment.
- `PMIN`, `PSEC`, `PFRAME`: start time of Skip Time Interval M.
- `MIN`, `SEC`, `FRAME`: stop time of Skip Time Interval M.

#### ADR = 6 — Unskip Time Interval

Optional. Cancels a previously recorded Skip Time Interval.

- `POINT = 01..40`: N-th Unskip Time Interval assignment.
- `MIN`, `SEC`, `FRAME`, `PMIN`, `PSEC`, `PFRAME`: each contains a value M referencing a previously defined Skip Time Interval. Unused bytes set to `00`.

#### ADR = 7..F — Reserved

> **Remark:** In the PMA, the net result of skipped and unskipped tracks must never exceed 21.

---

### Erasing the PMA with Mode 0 (CD-RW Only)

A PMA sequence can be overwritten and terminated with a **mode 0 Unity**: ten successive Subcode-Q mode 0 frames (labeled 0–9). The count sequence of overwritten frames must be synchronized to previously written frames.

**Mode 0 Unity field values:**

| Field | Value | Notes |
|-------|-------|-------|
| CONTROL | `0000` | All bits zero |
| ADR | `0` | Indicates mode 0 |
| TNO | `00` | |
| POINT | `00` | |
| MIN, SEC, FRAME | `00, 00, 00` | |
| ZERO | `00..09` | Frame counter within the Unity |
| PMIN, PSEC, PFRAME | `00, 00, 00` | |

---

### Program Memory Area Example

Encoding of a CD-R disc (Disc ID: 201514) with 5 audio tracks in the Program Area:

| Frame | CONTROL & ADR | TNO | POINT | MIN | SEC | FRM | ZERO | PMIN | PSEC | PFRM |
|-------|--------------|-----|-------|-----|-----|-----|------|------|------|------|
| 1  | 02 | 00 | 00 | 20 | 15 | 14 | 00 | 00 | 00 | 00 |
| 2  | 02 | 00 | 00 | 20 | 15 | 14 | 01 | 00 | 00 | 00 |
| 3  | 02 | 00 | 00 | 20 | 15 | 14 | 02 | 00 | 00 | 00 |
| 4  | 02 | 00 | 00 | 20 | 15 | 14 | 03 | 00 | 00 | 00 |
| 5  | 02 | 00 | 00 | 20 | 15 | 14 | 04 | 00 | 00 | 00 |
| 6  | 02 | 00 | 00 | 20 | 15 | 14 | 05 | 00 | 00 | 00 |
| 7  | 02 | 00 | 00 | 20 | 15 | 14 | 06 | 00 | 00 | 00 |
| 8  | 02 | 00 | 00 | 20 | 15 | 14 | 07 | 00 | 00 | 00 |
| 9  | 02 | 00 | 00 | 20 | 15 | 14 | 08 | 00 | 00 | 00 |
| 10 | 02 | 00 | 00 | 20 | 15 | 14 | 09 | 00 | 00 | 00 |
| 11 | 01 | 00 | 01 | 05 | 45 | 67 | 00 | 00 | 02 | 01 |
| 12 | 01 | 00 | 01 | 05 | 45 | 67 | 01 | 00 | 02 | 01 |
| 13 | 01 | 00 | 01 | 05 | 45 | 67 | 02 | 00 | 02 | 01 |
| 14 | 01 | 00 | 01 | 05 | 45 | 67 | 03 | 00 | 02 | 01 |
| 15 | 01 | 00 | 01 | 05 | 45 | 67 | 04 | 00 | 02 | 01 |
| 16 | 01 | 00 | 02 | 12 | 01 | 09 | 05 | 05 | 45 | 67 |
| 17 | 01 | 00 | 02 | 12 | 01 | 09 | 06 | 05 | 45 | 67 |
| 18 | 01 | 00 | 02 | 12 | 01 | 09 | 07 | 05 | 45 | 67 |
| 19 | 01 | 00 | 02 | 12 | 01 | 09 | 08 | 05 | 45 | 67 |
| 20 | 01 | 00 | 02 | 12 | 01 | 09 | 09 | 05 | 45 | 67 |
| 21 | 01 | 00 | 03 | 30 | 17 | 42 | 00 | 12 | 04 | 09 |
| 22 | 01 | 00 | 03 | 30 | 17 | 42 | 01 | 12 | 04 | 09 |
| 23 | 01 | 00 | 03 | 30 | 17 | 42 | 02 | 12 | 04 | 09 |
| 24 | 01 | 00 | 03 | 30 | 17 | 42 | 03 | 12 | 04 | 09 |
| 25 | 01 | 00 | 03 | 30 | 17 | 42 | 04 | 12 | 04 | 09 |
| 26 | 01 | 00 | 04 | 37 | 50 | 18 | 05 | 30 | 19 | 52 |
| 27 | 01 | 00 | 04 | 37 | 50 | 18 | 06 | 30 | 19 | 52 |
| 28 | 01 | 00 | 04 | 37 | 50 | 18 | 07 | 30 | 19 | 52 |
| 29 | 01 | 00 | 04 | 37 | 50 | 18 | 08 | 30 | 19 | 52 |
| 30 | 01 | 00 | 04 | 37 | 50 | 18 | 09 | 30 | 19 | 52 |
| 31 | 03 | 00 | 01 | 02 | 03 | 04 | 00 | 00 | 00 | 00 |
| 32 | 03 | 00 | 01 | 02 | 03 | 04 | 01 | 00 | 00 | 00 |
| 33 | 03 | 00 | 01 | 02 | 03 | 04 | 02 | 00 | 00 | 00 |
| 34 | 03 | 00 | 01 | 02 | 03 | 04 | 03 | 00 | 00 | 00 |
| 35 | 03 | 00 | 01 | 02 | 03 | 04 | 04 | 00 | 00 | 00 |
| 36 | 05 | 00 | 01 | 05 | 45 | 67 | 05 | 05 | 42 | 67 |
| 37 | 05 | 00 | 01 | 05 | 45 | 67 | 06 | 05 | 42 | 67 |
| 38 | 05 | 00 | 01 | 05 | 45 | 67 | 07 | 05 | 42 | 67 |
| 39 | 05 | 00 | 01 | 05 | 45 | 67 | 08 | 05 | 42 | 67 |
| 40 | 05 | 00 | 01 | 05 | 45 | 67 | 09 | 05 | 42 | 67 |
| 41 | 01 | 00 | 05 | 42 | 16 | 32 | 00 | 37 | 50 | 18 |
| 42 | 01 | 00 | 05 | 42 | 16 | 32 | 01 | 37 | 50 | 18 |
| 43 | 01 | 00 | 05 | 42 | 16 | 32 | 02 | 37 | 50 | 18 |
| 44 | 01 | 00 | 05 | 42 | 16 | 32 | 03 | 37 | 50 | 18 |
| 45 | 01 | 00 | 05 | 42 | 16 | 32 | 04 | 37 | 50 | 18 |
| 46 | 04 | 00 | 01 | 03 | 04 | 00 | 05 | 00 | 00 | 00 |
| 47 | 04 | 00 | 01 | 03 | 04 | 00 | 06 | 00 | 00 | 00 |
| 48 | 04 | 00 | 01 | 03 | 04 | 00 | 07 | 00 | 00 | 00 |
| 49 | 04 | 00 | 01 | 03 | 04 | 00 | 08 | 00 | 00 | 00 |
| 50 | 04 | 00 | 01 | 03 | 04 | 00 | 09 | 00 | 00 | 00 |

**Interpretation:**

- Frames 1–10: Disc Identification recorded (repeated 10 times as a standalone item).
- Frames 11–30: Start and stop times of Tracks 1–4.
- Frames 31–35: Tracks 2, 3, and 4 marked as Skip Track.
- Frames 36–40: Time Interval 1 marked as Skip Time Interval.
- Frames 41–45: Start and stop time of Track 5.
- Frames 46–50: Tracks 3 and 4 unskipped.

**Result:** Tracks 1, 3, 4, and 5 play back normally. Track 2 and the last three seconds of Track 1 are skipped.

---

## Lead-in Area

### DATA-Q Layouts

**ADR = 1, 5, and 6:**

```text
+-----------------------------------------------------------------------+
|                       DATA-Q (ADR = 1, 5, and 6)                     |
+--------+-------+------+------+-------+------+------+------+-----------+
| TNO=00 | POINT | MIN  | SEC  | FRAME | ZERO | PMIN | PSEC | PFRAME    |
+--------+-------+------+------+-------+------+------+------+-----------+
```

**ADR = 2:**

```text
+------------------------------------------------------------------------------------+
|                                 DATA-Q (ADR = 2)                                   |
+----+----+----+----+----+----+----+----+----+-----+-----+-----+-----+------+--------+
| N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 | N13 | ZERO | AFRAME |
+----+----+----+----+----+----+----+----+----+-----+-----+-----+-----+------+--------+
```

`TNO = 00` in the Lead-in Area.

> **Note:** The Red Book and MMC-6 use the term "Running time" for fields where the Orange Book uses "Absolute time." Whether these values are numerically identical across CD-DA and CD-R/CD-RW disc types is currently unconfirmed.

### ADR Modes

#### ADR = 1 — Track Timing and TOC (Red Book & Orange Book)

- `MIN`, `SEC`, `FRAME`: Running time within the track, expressed as 6 BCD digits (MM:SS:FF). Starts at zero at track start; increases during audio; decreases during pause (reaching zero at pause end). Increases during Lead-in and Lead-out. On Orange Book discs, must match ATIP time.
- `ZERO` = `00`
- `POINT = 01..99`: `PMIN`, `PSEC`, `PFRAME` give the start position of the track indicated by POINT.
- `POINT = A0`:
  - `PMIN`: first recorded track number in the Program Area.
  - `PFRAME`: `00`.
  - `PSEC`: Session format:

    | PSEC (hex) | Format |
    |------------|--------|
    | `00` | CD-DA or CD-ROM |
    | `10` | CD-i |
    | `20` | CD-ROM XA |

- `POINT = A1`:
  - `PMIN`: last recorded track number in the Program Area.
  - `PSEC`, `PFRAME`: `00`.
- `POINT = A2`: `PMIN`, `PSEC`, `PFRAME` give the start position of the Lead-out Area.

#### ADR = 2 — Media Catalog Number (Red Book Only)

The first 52 bits of DATA-Q are organized as 13 nibbles (N1–N13), each a single BCD digit forming the Media Catalog Number (MCN). The MCN does not change on a disc. If no UPC/EAN catalog number is encoded, N1–N13 are all zero (or Mode-2 may be absent). `ZERO` is 12 bits of zero. `AFRAME` encodes two BCD digits (00–74); during Lead-in (`TNO = 00`), these 8 bits are zero.

#### ADR = 5 — Skip Intervals and Multisession Pointers (Orange Book & Blue Book)

- `POINT = 01..40` — Skip Interval Pointers: indicate time intervals in the Program Area to skip during playback. Intervals must be recorded chronologically. The count N is given in `PMIN` of `POINT = B1`; if N = 0, these pointers are absent.
  - `PMIN`, `PSEC`, `PFRAME`: start time of the skip interval.
  - `MIN`, `SEC`, `FRAME`: stop time of the skip interval.
  - `ZERO = 00`: reserved.

- `POINT = B0` — Multisession next-area pointer: present in each session's Lead-in Area on a Multisession disc; absent on Single Session discs.
  - `MIN`, `SEC`, `FRAME`: start time of the next possible Program Area in the Recordable Area. Set to `FF FF FF` (hex) if this is the Final Session (alternatively, `POINT = B0` may be omitted in the Final Session's Lead-in).
  - `PMIN`, `PSEC`, `PFRAME`: start time of the Additional Capacity & Lead-out Area (copied from ATIP).
  - `ZERO`: total number of distinct pointers present in mode 5, including any Audio Skip pointers.

- `POINT = B1` — Audio Skip summary pointer: indicates that an Audio Session contains intervals or tracks to skip (not allowed in Data Sessions).
  - `MIN`, `SEC`, `FRAME`, `ZERO`, `PFRAME` = `00`.
  - `PMIN`: number N (N ≤ 40) of Skip Interval Pointers (`POINT = 01..N`).
  - `PSEC`: number M (M ≤ 21) of Skip Track assignments (`POINT = B2..B4`).
  - If no Skip Interval Pointers and no Skip Track assignments are used, `POINT = B1` is absent.

- `POINT = B2..B4` — Skip Track lists: each pointer holds up to 7 track numbers to skip. The count M is given in `PSEC` of `POINT = B1`; if M = 0, these pointers are absent.
  - `MIN`, `SEC`, `FRAME`, `ZERO`, `PMIN`, `PSEC`, `PFRAME`: each holds a track number to skip. Unused bytes = `00`.

- `POINT = C0` — Multisession first-session indicator: present only in the first Lead-in Area of a Multisession disc; absent on Single Session discs. `MIN`, `SEC`, `FRAME` are copies of the corresponding ATIP fields from frames with MSB combination `101`:
  - `MIN`: copied from ATIP Minutes byte (MSB combo `101`). Bits 7..1: W1..W3, X1, V1..V3. Bit 0 = 0.
  - `SEC`: copied from ATIP Seconds byte (MSB combo `101`). Bits 7..1: U1..U7. Bit 0 = 0.
  - `FRAME`: recommended copy of ATIP Frames byte (MSB combo `101`). Bits 7..1: D1, B1..B3, A1..A3. Bit 0 = 0. If not copied, all bits set to 0. (Note: A1..A3 indicate ATIP Additional Information 1/2/3 presence and are unrelated to TOC pointers C1/C2/C3.)
  - `ZERO`: reserved, set to `00`.
  - `PMIN`, `PSEC`, `PFRAME`: start time of the first Lead-in Area on the disc.

- `POINT = C1` — CD-RW Additional Information 1 (CD-RW Only): present only in the first Lead-in Area; present only if Additional Information 1 exists in ATIP. Contents are a copy of ATIP Additional Information 1. `MIN`, `SEC`, `FRAME` copy the ATIP fields from frames with MSB combination `001`:
  - `MIN`: copied from ATIP Minutes byte (MSB combo `001`). Bits 7..1: L1..L3, H1..H4. Bit 0 = 0.
  - `SEC`: copied from ATIP Seconds byte (MSB combo `001`). Bits 7..1: P1..P3, G1..G3, Y1. Bit 0 = 0.
  - `FRAME`: copied from ATIP Frames byte (MSB combo `001`). Bits 7..1: E1..E3, Z1..Z4. Bit 0 = 0.
  - `ZERO`, `PMIN`, `PSEC`, `PFRAME`: reserved, set to `00`.

- `POINT = C2, C3`: reserved for future use; shall not be used.

- `POINT = C0` — Multisession first-session indicator: present only in the first Lead-in Area of a Multisession disc; absent on Single Session discs.

> **Remark:** Skip Intervals must not overlap each other, and Skip Intervals must not overlap with Skip Track assignments.

#### ADR = 6 — Disc Identification in Lead-in (Orange Book Only)

- `POINT = 00`: Identifies the disc with the same unique 24-bit number recorded in the PMA (ADR = 2). Present only in the first Lead-in Area (including Single Session discs).
  - `MIN`, `SEC`, `FRAME`: absolute time on the disc; must match the ATIP time.
  - `ZERO = 00`
  - `PMIN`: equals the `MIN` field of the corresponding PMA ADR = 2 block.
  - `PSEC`: equals the `SEC` field of the corresponding PMA ADR = 2 block.
  - `PFRAME`: equals the `FRAME` field of the corresponding PMA ADR = 2 block.
- `POINT = 01..99`: reserved for future use.

---

## Program Area

### DATA-Q Layouts

**ADR = 1:**

```text
+-----------------------------------------------------------------------+
|                           DATA-Q (ADR = 1)                            |
+--------+-------+------+------+-------+------+------+------+-----------+
|  TNO   | INDEX | MIN  | SEC  | FRAME | ZERO | AMIN | ASEC | AFRAME    |
+--------+-------+------+------+-------+------+------+------+-----------+
```

**ADR = 3:**

```text
+----------------------------------------------------------------------------------------+
|                                    DATA-Q (ADR = 3)                                    |
+----+----+----+----+----+----+----+----+----+----+----+-----+-----+-----+------+--------+
| I1 | I2 | I3 | I4 | I5 | C1 | C2 | I6 | I7 | I8 | I9 | I10 | I11 | I12 | ZERO | AFRAME |
+----+----+----+----+----+----+----+----+----+----+----+-----+-----+-----+------+--------+
```

### ADR Modes

#### ADR = 1 — Track Position (Red Book & Orange Book)

- `TNO = 01..99bcd`: track number.
- `INDEX = 00..99bcd`: index within the track. An audio track may have up to 99 indexed sections, starting from index `01`. Most discs have only one indexed section per track. During a pre-gap (the portion of a track-to-track gap belonging to the following track), TNO is the following track number and INDEX = `00`.
- `MIN`, `SEC`, `FRAME`: relative time within the track as 6 BCD digits (MM:SS:FF). Starts at `00:00:00` at track start; increases through the track; decreases during the pre-gap.
- `ZERO` = `00000000b` (8 bits).
- `AMIN`, `ASEC`, `AFRAME`: absolute time address in the Program Area, expressed as 6 BCD digits.

#### ADR = 2 — Media Catalog Number (Red Book Only)

Same definition as ADR = 2 for the Lead-in Area.

#### ADR = 3 — ISRC / RID / TBD Code (Red Book & Orange Book)

Mode 3 carries one of three 60-bit codes encoded across 12 bit-groups:

- `I1..I5`: 5 groups × 6 bits each, occupying bit positions 0–29.
- `C1`, `C2`: 2 bits at positions 30–31; identify the code type.
- `I6..I12`: 7 groups × 4 bits each, occupying bit positions 32–59.
- `ZERO`: 4 bits = `0000`.
- `AFRAME`: 8 bits; frame value of the Absolute Time.

**C1C2 Code Type Selection:**

| C1C2 | Code |
|------|------|
| `00` | ISRC |
| `11` | RID |
| `01` | TBD (reserved; all bits = 0) |
| `10` | Not used |

**ISRC Code:** The ISRC may only change immediately after the Track Number (TNO) changes.

| Fields | Content |
|--------|---------|
| I1–I2 | Country Code (6-bit encoded per Table 18) |
| I3–I5 | Owner Code (6-bit encoded per Table 18) |
| I6–I7 | Year of recording (4-bit BCD) |
| I8–I12 | Serial number (4-bit BCD) |

**RID Code:** Identifies the recording device.

| Fields | Encoding | Content |
|--------|----------|---------|
| I1–I5 | 6-bit alphanumeric | Groups 1 and 2 (partial) |
| I6–I7 | 4-bit BCD | Group 2 (partial) |
| I8–I12 | 20-bit unsigned binary (MSB first) | Group 3 |

Code groups:

- Group 1 — `I1`, `I2`, `I3`: Manufacturer Code (example: `PHI`)
- Group 2 — `I4`, `I5`, `I6`, `I7`: Type Code (example: `CR 27`)
- Group 3 — `I8..I12`: Recorder Unique Number (example: `87532`)

Full RID example: `PHI CR 27 87532`

---

## Lead-out Area

- **ADR = 1:** Same as ADR = 1 for the Program Area, except `TNO = AAh` and `INDEX = 01bcd`.
- **ADR = 2:** Same as ADR = 2 for the Program and Lead-in Areas.
