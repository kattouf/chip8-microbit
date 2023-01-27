#[allow(dead_code)]

pub const IBM_LOGO: [u8; 132] = [
    0x00, 0xe0,
    0xa2, 0x2a,
    0x60, 0x0c,
    0x61, 0x08,
    0xd0, 0x1f,
    0x70, 0x09,
    0xa2, 0x39,
    0xd0, 0x1f,
    0xa2, 0x48,
    0x70, 0x08,
    0xd0, 0x1f,
    0x70, 0x04,
    0xa2, 0x57,
    0xd0, 0x1f,
    0x70, 0x08,
    0xa2, 0x66,
    0xd0, 0x1f,
    0x70, 0x08,
    0xa2, 0x75,
    0xd0, 0x1f,
    0x12, 0x28,
    0xff, 0x00,
    0xff, 0x00,
    0x3c, 0x00,
    0x3c, 0x00,
    0x3c, 0x00,
    0x3c, 0x00,
    0xff, 0x00,
    0xff, 0xff,
    0x00, 0xff,
    0x00, 0x38,
    0x00, 0x3f,
    0x00, 0x3f,
    0x00, 0x38,
    0x00, 0xff,
    0x00, 0xff,
    0x80, 0x00,
    0xe0, 0x00,
    0xe0, 0x00,
    0x80, 0x00,
    0x80, 0x00,
    0xe0, 0x00,
    0xe0, 0x00,
    0x80, 0xf8,
    0x00, 0xfc,
    0x00, 0x3e,
    0x00, 0x3f,
    0x00, 0x3b,
    0x00, 0x39,
    0x00, 0xf8,
    0x00, 0xf8,
    0x03, 0x00,
    0x07, 0x00,
    0x0f, 0x00,
    0xbf, 0x00,
    0xfb, 0x00,
    0xf3, 0x00,
    0xe3, 0x00,
    0x43, 0xe0,
    0x00, 0xe0,
    0x00, 0x80,
    0x00, 0x80,
    0x00, 0x80,
    0x00, 0x80,
    0x00, 0xe0,
    0x00, 0xe0,
];

pub const TEST_OPCODE: [u8; 478] = [
    0x12,
    0x4e,
    0xea,
    0xac,
    0xaa,
    0xea,
    0xce,
    0xaa,
    0xaa,
    0xae,
    0xe0,
    0xa0,
    0xa0,
    0xe0,
    0xc0,
    0x40,
    0x40,
    0xe0,
    0xe0,
    0x20,
    0xc0,
    0xe0,
    0xe0,
    0x60,
    0x20,
    0xe0,
    0xa0,
    0xe0,
    0x20,
    0x20,
    0x60,
    0x40,
    0x20,
    0x40,
    0xe0,
    0x80,
    0xe0,
    0xe0,
    0xe0,
    0x20,
    0x20,
    0x20,
    0xe0,
    0xe0,
    0xa0,
    0xe0,
    0xe0,
    0xe0,
    0x20,
    0xe0,
    0x40,
    0xa0,
    0xe0,
    0xa0,
    0xe0,
    0xc0,
    0x80,
    0xe0,
    0xe0,
    0x80,
    0xc0,
    0x80,
    0xa0,
    0x40,
    0xa0,
    0xa0,
    0xa2,
    0x02,
    0xda,
    0xb4,
    0x00,
    0xee,
    0xa2,
    0x02,
    0xda,
    0xb4,
    0x13,
    0xdc,
    0x68,
    0x01,
    0x69,
    0x05,
    0x6a,
    0x0a,
    0x6b,
    0x01,
    0x65,
    0x2a,
    0x66,
    0x2b,
    0xa2,
    0x16,
    0xd8,
    0xb4,
    0xa2,
    0x3e,
    0xd9,
    0xb4,
    0xa2,
    0x02,
    0x36,
    0x2b,
    0xa2,
    0x06,
    0xda,
    0xb4,
    0x6b,
    0x06,
    0xa2,
    0x1a,
    0xd8,
    0xb4,
    0xa2,
    0x3e,
    0xd9,
    0xb4,
    0xa2,
    0x06,
    0x45,
    0x2a,
    0xa2,
    0x02,
    0xda,
    0xb4,
    0x6b,
    0x0b,
    0xa2,
    0x1e,
    0xd8,
    0xb4,
    0xa2,
    0x3e,
    0xd9,
    0xb4,
    0xa2,
    0x06,
    0x55,
    0x60,
    0xa2,
    0x02,
    0xda,
    0xb4,
    0x6b,
    0x10,
    0xa2,
    0x26,
    0xd8,
    0xb4,
    0xa2,
    0x3e,
    0xd9,
    0xb4,
    0xa2,
    0x06,
    0x76,
    0xff,
    0x46,
    0x2a,
    0xa2,
    0x02,
    0xda,
    0xb4,
    0x6b,
    0x15,
    0xa2,
    0x2e,
    0xd8,
    0xb4,
    0xa2,
    0x3e,
    0xd9,
    0xb4,
    0xa2,
    0x06,
    0x95,
    0x60,
    0xa2,
    0x02,
    0xda,
    0xb4,
    0x6b,
    0x1a,
    0xa2,
    0x32,
    0xd8,
    0xb4,
    0xa2,
    0x3e,
    0xd9,
    0xb4,
    0x22,
    0x42,
    0x68,
    0x17,
    0x69,
    0x1b,
    0x6a,
    0x20,
    0x6b,
    0x01,
    0xa2,
    0x0a,
    0xd8,
    0xb4,
    0xa2,
    0x36,
    0xd9,
    0xb4,
    0xa2,
    0x02,
    0xda,
    0xb4,
    0x6b,
    0x06,
    0xa2,
    0x2a,
    0xd8,
    0xb4,
    0xa2,
    0x0a,
    0xd9,
    0xb4,
    0xa2,
    0x06,
    0x87,
    0x50,
    0x47,
    0x2a,
    0xa2,
    0x02,
    0xda,
    0xb4,
    0x6b,
    0x0b,
    0xa2,
    0x2a,
    0xd8,
    0xb4,
    0xa2,
    0x0e,
    0xd9,
    0xb4,
    0xa2,
    0x06,
    0x67,
    0x2a,
    0x87,
    0xb1,
    0x47,
    0x2b,
    0xa2,
    0x02,
    0xda,
    0xb4,
    0x6b,
    0x10,
    0xa2,
    0x2a,
    0xd8,
    0xb4,
    0xa2,
    0x12,
    0xd9,
    0xb4,
    0xa2,
    0x06,
    0x66,
    0x78,
    0x67,
    0x1f,
    0x87,
    0x62,
    0x47,
    0x18,
    0xa2,
    0x02,
    0xda,
    0xb4,
    0x6b,
    0x15,
    0xa2,
    0x2a,
    0xd8,
    0xb4,
    0xa2,
    0x16,
    0xd9,
    0xb4,
    0xa2,
    0x06,
    0x66,
    0x78,
    0x67,
    0x1f,
    0x87,
    0x63,
    0x47,
    0x67,
    0xa2,
    0x02,
    0xda,
    0xb4,
    0x6b,
    0x1a,
    0xa2,
    0x2a,
    0xd8,
    0xb4,
    0xa2,
    0x1a,
    0xd9,
    0xb4,
    0xa2,
    0x06,
    0x66,
    0x8c,
    0x67,
    0x8c,
    0x87,
    0x64,
    0x47,
    0x18,
    0xa2,
    0x02,
    0xda,
    0xb4,
    0x68,
    0x2c,
    0x69,
    0x30,
    0x6a,
    0x34,
    0x6b,
    0x01,
    0xa2,
    0x2a,
    0xd8,
    0xb4,
    0xa2,
    0x1e,
    0xd9,
    0xb4,
    0xa2,
    0x06,
    0x66,
    0x8c,
    0x67,
    0x78,
    0x87,
    0x65,
    0x47,
    0xec,
    0xa2,
    0x02,
    0xda,
    0xb4,
    0x6b,
    0x06,
    0xa2,
    0x2a,
    0xd8,
    0xb4,
    0xa2,
    0x22,
    0xd9,
    0xb4,
    0xa2,
    0x06,
    0x66,
    0xe0,
    0x86,
    0x6e,
    0x46,
    0xc0,
    0xa2,
    0x02,
    0xda,
    0xb4,
    0x6b,
    0x0b,
    0xa2,
    0x2a,
    0xd8,
    0xb4,
    0xa2,
    0x36,
    0xd9,
    0xb4,
    0xa2,
    0x06,
    0x66,
    0x0f,
    0x86,
    0x66,
    0x46,
    0x07,
    0xa2,
    0x02,
    0xda,
    0xb4,
    0x6b,
    0x10,
    0xa2,
    0x3a,
    0xd8,
    0xb4,
    0xa2,
    0x1e,
    0xd9,
    0xb4,
    0xa3,
    0xe8,
    0x60,
    0x00,
    0x61,
    0x30,
    0xf1,
    0x55,
    0xa3,
    0xe9,
    0xf0,
    0x65,
    0xa2,
    0x06,
    0x40,
    0x30,
    0xa2,
    0x02,
    0xda,
    0xb4,
    0x6b,
    0x15,
    0xa2,
    0x3a,
    0xd8,
    0xb4,
    0xa2,
    0x16,
    0xd9,
    0xb4,
    0xa3,
    0xe8,
    0x66,
    0x89,
    0xf6,
    0x33,
    0xf2,
    0x65,
    0xa2,
    0x02,
    0x30,
    0x01,
    0xa2,
    0x06,
    0x31,
    0x03,
    0xa2,
    0x06,
    0x32,
    0x07,
    0xa2,
    0x06,
    0xda,
    0xb4,
    0x6b,
    0x1a,
    0xa2,
    0x0e,
    0xd8,
    0xb4,
    0xa2,
    0x3e,
    0xd9,
    0xb4,
    0x12,
    0x48,
    0x13,
    0xdc,
];
