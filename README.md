# video2binframe

Dump frames from video into black & white binaries

## How binaries encoded

### Signature

Use 24 bytes to store the size, encoded in UTF-8. Remaining padding are fill with 0x00

For example

```plaintext
480x360@30fps
34 38 30 78 33 36 30 40 33 30 66 70 73 00 00 00 00 00 00 00 00 00 00 00
```

### Frame Information

Use 4 bytes to store the frame information. The first 2 bits is used to represent the frame type, and the remaining 30 bits store the data length in bytes.

- **KEY(0)**: Full frame data (uncompressed bits)
- **PREDICT(1)**: Delta from previous frame *(TBD)*
- **RUNLENGTH(2)**: Run-length encoded frame

```plaintext
KEY 21600
00 00 54 60

PREDICT 256
40 00 01 00

RUNLENGTH 63
80 00 00 3F
```

### Key Frame

Key frame's data will encode directly into bits, 1 means white, and 0 means black

```plaintext
BWBBBBWB
01000010
42
```

### Predict Frame

Predict frame's data store the next bit to flip

> ***Detail TBD.***

### Run-length

Stores pointers where color switches in u32. Starts with WHITE by default.

```plaintext
WWWWWWWWBBBBBBBBWWWWWWWW
00 00 00 08 | 00 00 00 10 | 00 00 00 18
```

## File Format Structure

File:

```plaintext
[16-byte signature][frame1][frame2][frame3]...
```

Frame:

```plaintext
[4-byte header][variable-length data]
```
