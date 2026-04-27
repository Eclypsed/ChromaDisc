# ATIP Descriptor

```text
     7   6   5   4   3   2   1   0
   +---+---+---+---+---+---+---+---+
 0 |                               |
 1 |     Special Information 1     |
 2 |                               |
   +---+---+---+---+---+---+---+---+
 3 |           Reserved            |
   +---+---+---+---+---+---+---+---+
 4 |                               |
 5 |     Special Information 2     |
 6 |                               |
   +---+---+---+---+---+---+---+---+
 7 |           Reserved            |
   +---+---+---+---+---+---+---+---+
 8 |                               |
 9 |     Special Information 3     |
10 |                               |
   +---+---+---+---+---+---+---+---+
11 |           Reserved            |
   +---+---+---+---+---+---+---+---+
12 |                               |
13 |   Additional Information 1    |
14 |                               |
   +---+---+---+---+---+---+---+---+
15 |           Reserved            |
   +---+---+---+---+---+---+---+---+
16 |                               |
17 |   Additional Information 2    |
18 |                               |
   +---+---+---+---+---+---+---+---+
19 |           Reserved            |
   +---+---+---+---+---+---+---+---+
20 |                               |
21 |   Additional Information 3    |
22 |                               |
   +---+---+---+---+---+---+---+---+
23 |           Reserved            |
   +---+---+---+---+---+---+---+---+
```

## Special Information 1

### Byte diagram (CD-R, CD-RW Vol.1, CD-RW Vol.2, CD-RW Vol.3)

```text
  7   6   5   4   3   2   1   0
+---+---+---+---+---+---+---+---+
| 1 | W1 W2 W3  | X1| V1 V2 V3  |
+---+---+---+---+---+---+---+---+
| 0 |    U1 U2 U3 U4 U5 U6 U7   |
+---+---+---+---+---+---+---+---+
| 1 | D1| B1 B2 B3  | A1 A2 A3  |
+---+---+---+---+---+---+---+---+
```

W1..W3 : Indicative Target Writing Power
X1 : Reserved and set to zero
V1..V3 : Reference Speed
U1..U7 : Disc Application Code
D1 : Disc type
B1..B3 : Disc sub-type
A1..A3 : Presence of Additional Information

## Special Information 2

### Byte diagram (CD-R, CD-RW Vol.1, CD-RW Vol.2, CD-RW Vol.3)

```text
  7   6   5   4   3   2   1   0
+---+---+---+---+---+---+---+---+
| 1 |   M2 M3 M4 M5 M6 M7 M8    |
+---+---+---+---+---+---+---+---+
| 1 |   S2 S3 S4 S5 S6 S7 S8    |
+---+---+---+---+---+---+---+---+
| 0 |   F2 F3 F4 F5 F6 F7 F8    |
+---+---+---+---+---+---+---+---+
```

M1,M2..M8 S1,S2..S7 F1,F2..F7 : Minutes, Seconds, Frames

## Special Information 3

### Byte diagram (CD-R, CD-RW Vol.1, CD-RW Vol.2, CD-RW Vol.3)

```text
  7   6   5   4   3   2   1   0
+---+---+---+---+---+---+---+---+
| 1 |   M2 M3 M4 M5 M6 M7 M8    |
+---+---+---+---+---+---+---+---+
| 1 |   S2 S3 S4 S5 S6 S7 S8    |
+---+---+---+---+---+---+---+---+
| 1 |   F2 F3 F4 F5 F6 F7 F8    |
+---+---+---+---+---+---+---+---+
```

M1,M2..M7 S1,S2..S7 F1,F2..F7 : Minutes, Seconds, Frames

## Additional Information 1

### Byte Diagram (CD-R)

```text
  7   6   5   4   3   2   1   0
+---+---+---+---+---+---+---+---+
| 0 | L1 L2 L3  |  H1 H2 H3 H4  |
+---+---+---+---+---+---+---+---+
| 0 | I1 I2 I3  | Y1 Y2 | C1 C2 |
+---+---+---+---+---+---+---+---+
| 1 | N1 N2 N3  |  E1 E2 E3 E4  |
+---+---+---+---+---+---+---+---+
```

L1..L3 : Lowest Test Speed
H1..H4 : Highest Test Speed
I1..I3 : High-Speed subtype
Y1..Y2 : Reserved and set to zero
C1..C2 : Optimum Beta-range
N1..N3 : Optimum pulse length
E1..E4 : Length of Additional Capacity & Lead-out Area

### Byte Diagram (CD-RW Vol.1)

```text
  7   6   5   4   3   2   1   0
+---+---+---+---+---+---+---+---+
| 0 | L1 L2 L3  |  H1 H2 H3 H4  |
+---+---+---+---+---+---+---+---+
| 0 | P1 P2 P3  | G1 G2 G3  | Y1|
+---+---+---+---+---+---+---+---+
| 1 | E1 E2 E3  |  Z1 Z2 Z3 Z4  |
+---+---+---+---+---+---+---+---+
```

L1..L3 : Lowest Usable CLV Recording Speed
H1..H4 : Highest Usable CLV Recording Speed
P1..P3 : Power Multiplication Factor ρ at Reference Speed
G1..G3 : Target gamma value of the modulation/power function for all speeds
Y1 : Reserved for future extensions (= 0)
E1..E3 : Recommended erase/write power ratio epsilon at Reference Speed
Z1..Z4 : Reserved for future extensions (=0000)

### Byte Diagram (CD-RW Vol.2)

```text
  7   6   5   4   3   2   1   0
+---+---+---+---+---+---+---+---+
| 0 | L1 L2 L3  |  H1 H2 H3 H4  |
+---+---+---+---+---+---+---+---+
| 0 | P1 P2 P3  | G1 G2 G3  | Y1|
+---+---+---+---+---+---+---+---+
| 1 | E1 E2 E3  |  C1 C2 C3 C4  |
+---+---+---+---+---+---+---+---+
```

L1..L3 : Lowest Usable CLV Recording Speed
H1..H4 : Highest Usable CLV Recording Speed
P1..P3 : Power Multiplication Factor ρ at Reference Speed
G1..G3 : Target γ value of the modulation/power function for all speeds
Y1 : Reserved and set to zero
E1..E3 : Recommended erase/write power ratio ε at Reference Speed
C1..C4 : Write strategy optimization

### Byte Diagram (CD-RW Vol.3)

```text
  7   6   5   4   3   2   1   0
+---+---+---+---+---+---+---+---+
| 0 | L1 L2 L3  |  H1 H2 H3 H4  |
+---+---+---+---+---+---+---+---+
| 0 | P1 P2 P3  | G1 G2 G3  | Y1|
+---+---+---+---+---+---+---+---+
| 1 | E1 E2 E3  |  C1 C2 C3 C4  |
+---+---+---+---+---+---+---+---+
```

L1..L3 : Lowest ‘1T’ Test Speed
H1..H4 : Highest ‘1T’ Test Speed
P1..P3 : Power Multiplication Factor ρ at ‘1T’ Test Speed, ‘1T’ Write-strategy applied
G1..G3 : Target γ value of the modulation/power function at ‘1T’ Test Speed, ‘1T’ Write-strategy applied
Y1 : Reserved and set to zero
E1..E3 : Recommended erase/write power ratio ε at ‘1T’ Test Speed, ‘1T’ Write-strategy applied
C1..C4 : Write-strategy optimization at ‘1T’ Test Speed, ‘1T’ Write-strategy applied

## Additional Information 2

### Byte Diagram (CD-R)

```text
  7   6   5   4   3   2   1   0
+---+---+---+---+---+---+---+---+
| 0 | W4 W5 W6  | W7 W8 W9 W10  |
+---+---+---+---+---+---+---+---+
| 1 | P1 P2 P3  | T1 T2 | Y1 Y2 |
+---+---+---+---+---+---+---+---+
| 0 |   Z1 Z2 Z3 Z4 Z5 Z6 Z7    |
+---+---+---+---+---+---+---+---+
```

W4..W6 : Indicative Optimum Writing Power at Lowest Test Speed
W7..W10 : Indicative Optimum Writing Power at Highest Test Speed
P1..P3 : Power boost for I3 pits
T1..T2 : Pulse length correction after I3 lands
Y1..Y2 : Reserved and set to zero
Z1..Z7 : Reserved and set to zero

### Byte Diagram (CD-RW Vol.1)

```text
  7   6   5   4   3   2   1   0
+---+---+---+---+---+---+---+---+
| 0 | W4 W5 W6  | W7 W8 W9  | X1|
+---+---+---+---+---+---+---+---+
| 1 | P4 P5 P6  | P7 P8 P9  | Y1|
+---+---+---+---+---+---+---+---+
| 0 | E4 E5 E6  | E7 E8 E9  | Z1|
+---+---+---+---+---+---+---+---+
```

W4..W6 : Indicative Target Writing Power at Lowest Usable Recording Speed (P ind,L )
W7..W9 : Indicative Target Writing Power at Highest Usable Recording Speed (P ind,H)
X1 : Reserved for future extensions (= 0)
P4..P6 : Power Multiplication Factor ρ at Lowest Usable Recording Speed (ρL )
P7..P9 : Power Multiplication Factor ρ at Highest Usable Recording Speed (ρH)
Y1 : Reserved for future extensions (= 0)
E4..E6 : Recommended erase/write power ratio at Lowest Usable Recording Speed (εL )
E7..E9 : Recommended erase/write power ratio at Highest Usable Recording Speed (εH)
Z1 : Reserved for future extensions (= 0)

### Byte Diagram (CD-RW Vol.2)

```text
  7   6   5   4   3   2   1   0
+---+---+---+---+---+---+---+---+
| 0 | W4 W5 W6  | W7 W8 W9  | X1|
+---+---+---+---+---+---+---+---+
| 1 | P4 P5 P6  | P7 P8 P9  | Y1|
+---+---+---+---+---+---+---+---+
| 0 | E4 E5 E6  | E7 E8 E9  | Z1|
+---+---+---+---+---+---+---+---+
```

W4..W6 : Indicative Target Writing Power at Lowest Usable Recording Speed (P ind,L )
W7..W9 : Indicative Target Writing Power at Highest Usable Recording Speed (P ind,H)
X1 : Reserved and set to zero
P4..P6 : Power Multiplication Factor ρ at Lowest Usable Recording Speed (ρL )
P7..P9 : Power Multiplication Factor ρ at Highest Usable Recording Speed (ρH)
Y1 : Reserved and set to zero
E4..E6 : Recommended erase/write power ratio at Lowest Usable Recording Speed (εL )
E7..E9 : Recommended erase/write power ratio at Highest Usable Recording Speed (εH)
Z1 : Reserved and set to zero

### Byte Diagram (CD-RW Vol.3)

```text
  7   6   5   4   3   2   1   0
+---+---+---+---+---+---+---+---+
| 0 | L4 L5 L6  |  H5 H6 H7 H8  |
+---+---+---+---+---+---+---+---+
| 1 | P4 P5 P6  | P7 P8 P9  | Y1|
+---+---+---+---+---+---+---+---+
| 0 | E4 E5 E6  | E7 E8 E9  | Z1|
+---+---+---+---+---+---+---+---+
```

L4.. L6 : Lowest ‘2T’ Write-strategy Test Speed
H5..H8 : Highest ‘2T’ Write-strategy Test Speed
P4..P6 : Optimum ‘2T’ Write-strategy write power indication at 16x
P7..P9 : Optimum ‘2T’ Write-strategy write power indication at HTS
Y1 : Reserved and set to zero
E4..E6 : Optimum ‘2T’ Write-strategy erase power indication at 16x
E7..E9 : Optimum ‘2T’ Write-strategy erase poser indication at HTS
Z1 : Reserved and set to zero

## Additional Information 3

### Byte Diagram (CD-R, CD-RW Vol.1, CD-RW Vol.2, CD-RW Vol.3)

```text
  7   6   5   4   3   2   1   0
+---+---+---+---+---+---+---+---+
| 0 | J1 J2 |  Q1 Q2 Q3 Q4 Q5   |
+---+---+---+---+---+---+---+---+
| 1 |  Q6 Q7 Q8 Q9 Q10 Q11 Q12  |
+---+---+---+---+---+---+---+---+
| 1 |Q13 Q14 Q15 Q16| R1 R2 R3  |
+---+---+---+---+---+---+---+---+
```

J1..J2 : Media technology type
Q1..Q12 : Media IDentification (MID) code first part
Q13..Q16 : Media IDentification (MID) code second part
R1..R3 : Product revision number
