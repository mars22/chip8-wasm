const TICTAC: &[u8] = &[
    18, 24, 84, 73, 67, 84, 65, 67, 32, 98, 121, 32, 68, 97, 118, 105, 100, 32, 87, 73, 78, 84, 69,
    82, 107, 0, 108, 0, 128, 176, 129, 192, 163, 230, 241, 85, 163, 196, 255, 101, 163, 180, 255,
    85, 163, 230, 241, 101, 139, 0, 140, 16, 0, 224, 110, 1, 96, 19, 97, 3, 163, 154, 208, 17, 112,
    8, 48, 43, 18, 62, 96, 19, 113, 8, 49, 35, 18, 62, 96, 19, 97, 3, 163, 155, 208, 31, 112, 8,
    48, 51, 18, 84, 96, 19, 113, 15, 208, 26, 112, 8, 48, 51, 18, 96, 35, 102, 240, 10, 129, 0,
    163, 180, 240, 30, 240, 101, 64, 0, 18, 138, 34, 124, 18, 106, 96, 16, 240, 24, 240, 21, 240,
    7, 48, 0, 18, 130, 0, 238, 96, 2, 142, 3, 128, 224, 240, 85, 163, 212, 128, 16, 112, 255, 128,
    4, 240, 30, 241, 101, 163, 170, 62, 3, 163, 175, 208, 21, 34, 200, 58, 0, 18, 28, 163, 180, 97,
    0, 98, 0, 99, 1, 240, 101, 48, 0, 113, 1, 243, 30, 114, 1, 50, 16, 18, 180, 49, 16, 18, 106,
    18, 28, 106, 0, 163, 180, 96, 1, 240, 30, 248, 101, 105, 0, 137, 4, 35, 68, 137, 20, 35, 68,
    137, 36, 35, 74, 105, 0, 137, 52, 35, 68, 137, 68, 35, 68, 137, 84, 35, 74, 105, 0, 137, 100,
    35, 68, 137, 116, 35, 68, 137, 132, 35, 74, 105, 0, 137, 100, 35, 68, 137, 52, 35, 68, 137, 4,
    35, 74, 105, 0, 137, 116, 35, 68, 137, 68, 35, 68, 137, 20, 35, 74, 105, 0, 137, 132, 35, 68,
    137, 84, 35, 68, 137, 36, 35, 74, 105, 0, 137, 132, 35, 68, 137, 68, 35, 68, 137, 4, 35, 74,
    105, 0, 137, 100, 35, 68, 137, 68, 35, 68, 137, 36, 35, 74, 0, 238, 137, 14, 137, 14, 0, 238,
    73, 21, 19, 84, 73, 63, 19, 90, 0, 238, 35, 102, 123, 1, 19, 94, 35, 102, 124, 1, 35, 102, 106,
    1, 240, 10, 0, 238, 99, 5, 100, 10, 163, 175, 211, 69, 99, 2, 116, 6, 163, 230, 251, 51, 35,
    136, 99, 50, 100, 10, 163, 170, 211, 69, 99, 47, 116, 6, 163, 230, 252, 51, 242, 101, 240, 41,
    35, 148, 241, 41, 35, 148, 242, 41, 211, 69, 115, 5, 0, 238, 127, 128, 128, 128, 128, 128, 128,
    128, 128, 128, 128, 128, 128, 128, 128, 128, 28, 34, 34, 34, 28, 34, 20, 8, 20, 34, 1, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 19, 5, 27,
    5, 35, 5, 19, 13, 27, 13, 35, 13, 19, 21, 27, 21, 35, 21,
];
const INVADERS: &[u8] = &[
    18, 37, 83, 80, 65, 67, 69, 32, 73, 78, 86, 65, 68, 69, 82, 83, 32, 48, 46, 57, 49, 32, 66,
    121, 32, 68, 97, 118, 105, 100, 32, 87, 73, 78, 84, 69, 82, 96, 0, 97, 0, 98, 8, 163, 221, 208,
    24, 113, 8, 242, 30, 49, 32, 18, 45, 112, 8, 97, 0, 48, 64, 18, 45, 105, 5, 108, 21, 110, 0,
    35, 145, 96, 10, 240, 21, 240, 7, 48, 0, 18, 75, 35, 145, 126, 1, 18, 69, 102, 0, 104, 28, 105,
    0, 106, 4, 107, 10, 108, 4, 109, 60, 110, 15, 0, 224, 35, 117, 35, 81, 253, 21, 96, 4, 224,
    158, 18, 125, 35, 117, 56, 0, 120, 255, 35, 117, 96, 6, 224, 158, 18, 139, 35, 117, 56, 57,
    120, 1, 35, 117, 54, 0, 18, 159, 96, 5, 224, 158, 18, 233, 102, 1, 101, 27, 132, 128, 163, 217,
    212, 81, 163, 217, 212, 81, 117, 255, 53, 255, 18, 173, 102, 0, 18, 233, 212, 81, 63, 1, 18,
    233, 212, 81, 102, 0, 131, 64, 115, 3, 131, 181, 98, 248, 131, 34, 98, 8, 51, 0, 18, 201, 35,
    125, 130, 6, 67, 8, 18, 211, 51, 16, 18, 213, 35, 125, 130, 6, 51, 24, 18, 221, 35, 125, 130,
    6, 67, 32, 18, 231, 51, 40, 18, 233, 35, 125, 62, 0, 19, 7, 121, 6, 73, 24, 105, 0, 106, 4,
    107, 10, 108, 4, 125, 244, 110, 15, 0, 224, 35, 81, 35, 117, 253, 21, 18, 111, 247, 7, 55, 0,
    18, 111, 253, 21, 35, 81, 139, 164, 59, 18, 19, 27, 124, 2, 106, 252, 59, 2, 19, 35, 124, 2,
    106, 4, 35, 81, 60, 24, 18, 111, 0, 224, 164, 221, 96, 20, 97, 8, 98, 15, 208, 31, 112, 8, 242,
    30, 48, 44, 19, 51, 96, 255, 240, 21, 240, 7, 48, 0, 19, 65, 240, 10, 0, 224, 167, 6, 254, 101,
    18, 37, 163, 193, 249, 30, 97, 8, 35, 105, 129, 6, 35, 105, 129, 6, 35, 105, 129, 6, 35, 105,
    123, 208, 0, 238, 128, 224, 128, 18, 48, 0, 219, 198, 123, 12, 0, 238, 163, 217, 96, 28, 216,
    4, 0, 238, 35, 81, 142, 35, 35, 81, 96, 5, 240, 24, 240, 21, 240, 7, 48, 0, 19, 137, 0, 238,
    106, 0, 141, 224, 107, 4, 233, 161, 18, 87, 166, 12, 253, 30, 240, 101, 48, 255, 19, 175, 106,
    0, 107, 4, 109, 1, 110, 1, 19, 151, 165, 10, 240, 30, 219, 198, 123, 8, 125, 1, 122, 1, 58, 7,
    19, 151, 0, 238, 60, 126, 255, 255, 153, 153, 126, 255, 255, 36, 36, 231, 126, 255, 60, 60,
    126, 219, 129, 66, 60, 126, 255, 219, 16, 56, 124, 254, 0, 0, 127, 0, 63, 0, 127, 0, 0, 0, 1,
    1, 1, 3, 3, 3, 3, 0, 0, 63, 32, 32, 32, 32, 32, 32, 32, 32, 63, 8, 8, 255, 0, 0, 254, 0, 252,
    0, 254, 0, 0, 0, 126, 66, 66, 98, 98, 98, 98, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0,
    255, 0, 125, 0, 65, 125, 5, 125, 125, 0, 0, 194, 194, 198, 68, 108, 40, 56, 0, 0, 255, 0, 0, 0,
    0, 0, 0, 0, 0, 255, 0, 0, 255, 0, 247, 16, 20, 247, 247, 4, 4, 0, 0, 124, 68, 254, 194, 194,
    194, 194, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 255, 0, 239, 32, 40, 232, 232, 47, 47,
    0, 0, 249, 133, 197, 197, 197, 197, 249, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 255, 0,
    190, 0, 32, 48, 32, 190, 190, 0, 0, 247, 4, 231, 133, 133, 132, 244, 0, 0, 255, 0, 0, 0, 0, 0,
    0, 0, 0, 255, 0, 0, 255, 0, 0, 127, 0, 63, 0, 127, 0, 0, 0, 239, 40, 239, 0, 224, 96, 111, 0,
    0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 255, 0, 0, 254, 0, 252, 0, 254, 0, 0, 0, 192, 0,
    192, 192, 192, 192, 192, 0, 0, 252, 4, 4, 4, 4, 4, 4, 4, 4, 252, 16, 16, 255, 249, 129, 185,
    139, 154, 154, 250, 0, 250, 138, 154, 154, 155, 153, 248, 230, 37, 37, 244, 52, 52, 52, 0, 23,
    20, 52, 55, 54, 38, 199, 223, 80, 80, 92, 216, 216, 223, 0, 223, 17, 31, 18, 27, 25, 217, 124,
    68, 254, 134, 134, 134, 252, 132, 254, 130, 130, 254, 254, 128, 192, 192, 192, 254, 252, 130,
    194, 194, 194, 252, 254, 128, 248, 192, 192, 254, 254, 128, 240, 192, 192, 192, 254, 128, 190,
    134, 134, 254, 134, 134, 254, 134, 134, 134, 16, 16, 16, 16, 16, 16, 24, 24, 24, 72, 72, 120,
    156, 144, 176, 192, 176, 156, 128, 128, 192, 192, 192, 254, 238, 146, 146, 134, 134, 134, 254,
    130, 134, 134, 134, 134, 124, 130, 134, 134, 134, 124, 254, 130, 254, 192, 192, 192, 124, 130,
    194, 202, 196, 122, 254, 134, 254, 144, 156, 132, 254, 192, 254, 2, 2, 254, 254, 16, 48, 48,
    48, 48, 130, 130, 194, 194, 194, 254, 130, 130, 130, 238, 56, 16, 134, 134, 150, 146, 146, 238,
    130, 68, 56, 56, 68, 130, 130, 130, 254, 48, 48, 48, 254, 2, 30, 240, 128, 254, 0, 0, 0, 0, 6,
    6, 0, 0, 0, 96, 96, 192, 0, 0, 0, 0, 0, 0, 24, 24, 24, 24, 0, 24, 124, 198, 12, 24, 0, 24, 0,
    0, 254, 254, 0, 0, 254, 130, 134, 134, 134, 254, 8, 8, 8, 24, 24, 24, 254, 2, 254, 192, 192,
    254, 254, 2, 30, 6, 6, 254, 132, 196, 196, 254, 4, 4, 254, 128, 254, 6, 6, 254, 192, 192, 192,
    254, 130, 254, 254, 2, 2, 6, 6, 6, 124, 68, 254, 134, 134, 254, 254, 130, 254, 6, 6, 6, 68,
    254, 68, 68, 254, 68, 168, 168, 168, 168, 168, 168, 168, 108, 90, 0, 12, 24, 168, 48, 78, 126,
    0, 18, 24, 102, 108, 168, 90, 102, 84, 36, 102, 0, 72, 72, 24, 18, 168, 6, 144, 168, 18, 0,
    126, 48, 18, 168, 132, 48, 78, 114, 24, 102, 168, 168, 168, 168, 168, 168, 144, 84, 120, 168,
    72, 120, 108, 114, 168, 18, 24, 108, 114, 102, 84, 144, 168, 114, 42, 24, 168, 48, 78, 126, 0,
    18, 24, 102, 108, 168, 114, 84, 168, 90, 102, 24, 126, 24, 78, 114, 168, 114, 42, 24, 48, 102,
    168, 48, 78, 126, 0, 108, 48, 84, 78, 156, 168, 168, 168, 168, 168, 168, 168, 72, 84, 126, 24,
    168, 144, 84, 120, 102, 168, 108, 42, 48, 90, 168, 132, 48, 114, 42, 168, 216, 168, 0, 78, 18,
    168, 228, 162, 168, 0, 78, 18, 168, 108, 42, 84, 84, 114, 168, 132, 48, 114, 42, 168, 222, 156,
    168, 114, 42, 24, 168, 12, 84, 72, 90, 120, 114, 24, 102, 168, 102, 24, 90, 84, 102, 114, 108,
    168, 114, 42, 0, 114, 168, 114, 42, 24, 168, 48, 78, 126, 0, 18, 24, 102, 108, 168, 0, 102, 24,
    168, 48, 78, 12, 102, 24, 0, 108, 48, 78, 36, 168, 114, 42, 24, 48, 102, 168, 30, 84, 102, 12,
    24, 156, 168, 36, 84, 84, 18, 168, 66, 120, 12, 60, 168, 174, 168, 168, 168, 168, 168, 168,
    168, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
const TETRIS: &[u8] = &[
    162, 180, 35, 230, 34, 182, 112, 1, 208, 17, 48, 37, 18, 6, 113, 255, 208, 17, 96, 26, 208, 17,
    96, 37, 49, 0, 18, 14, 196, 112, 68, 112, 18, 28, 195, 3, 96, 30, 97, 3, 34, 92, 245, 21, 208,
    20, 63, 1, 18, 60, 208, 20, 113, 255, 208, 20, 35, 64, 18, 28, 231, 161, 34, 114, 232, 161, 34,
    132, 233, 161, 34, 150, 226, 158, 18, 80, 102, 0, 246, 21, 246, 7, 54, 0, 18, 60, 208, 20, 113,
    1, 18, 42, 162, 196, 244, 30, 102, 0, 67, 1, 102, 4, 67, 2, 102, 8, 67, 3, 102, 12, 246, 30, 0,
    238, 208, 20, 112, 255, 35, 52, 63, 1, 0, 238, 208, 20, 112, 1, 35, 52, 0, 238, 208, 20, 112,
    1, 35, 52, 63, 1, 0, 238, 208, 20, 112, 255, 35, 52, 0, 238, 208, 20, 115, 1, 67, 4, 99, 0, 34,
    92, 35, 52, 63, 1, 0, 238, 208, 20, 115, 255, 67, 255, 99, 3, 34, 92, 35, 52, 0, 238, 128, 0,
    103, 5, 104, 6, 105, 4, 97, 31, 101, 16, 98, 7, 0, 238, 64, 224, 0, 0, 64, 192, 64, 0, 0, 224,
    64, 0, 64, 96, 64, 0, 64, 64, 96, 0, 32, 224, 0, 0, 192, 64, 64, 0, 0, 224, 128, 0, 64, 64,
    192, 0, 0, 224, 32, 0, 96, 64, 64, 0, 128, 224, 0, 0, 64, 192, 128, 0, 192, 96, 0, 0, 64, 192,
    128, 0, 192, 96, 0, 0, 128, 192, 64, 0, 0, 96, 192, 0, 128, 192, 64, 0, 0, 96, 192, 0, 192,
    192, 0, 0, 192, 192, 0, 0, 192, 192, 0, 0, 192, 192, 0, 0, 64, 64, 64, 64, 0, 240, 0, 0, 64,
    64, 64, 64, 0, 240, 0, 0, 208, 20, 102, 53, 118, 255, 54, 0, 19, 56, 0, 238, 162, 180, 140, 16,
    60, 30, 124, 1, 60, 30, 124, 1, 60, 30, 124, 1, 35, 94, 75, 10, 35, 114, 145, 192, 0, 238, 113,
    1, 19, 80, 96, 27, 107, 0, 208, 17, 63, 0, 123, 1, 208, 17, 112, 1, 48, 37, 19, 98, 0, 238, 96,
    27, 208, 17, 112, 1, 48, 37, 19, 116, 142, 16, 141, 224, 126, 255, 96, 27, 107, 0, 208, 225,
    63, 0, 19, 144, 208, 225, 19, 148, 208, 209, 123, 1, 112, 1, 48, 37, 19, 134, 75, 0, 19, 166,
    125, 255, 126, 255, 61, 1, 19, 130, 35, 192, 63, 1, 35, 192, 122, 1, 35, 192, 128, 160, 109, 7,
    128, 210, 64, 4, 117, 254, 69, 2, 101, 4, 0, 238, 167, 0, 242, 85, 168, 4, 250, 51, 242, 101,
    240, 41, 109, 50, 110, 0, 221, 229, 125, 5, 241, 41, 221, 229, 125, 5, 242, 41, 221, 229, 167,
    0, 242, 101, 162, 180, 0, 238, 106, 0, 96, 25, 0, 238, 55, 35,
];
const MAZE: &[u8] = &[
    96, 0, 97, 0, 162, 34, 194, 1, 50, 1, 162, 30, 208, 20, 112, 4, 48, 64, 18, 4, 96, 0, 113, 4,
    49, 32, 18, 4, 18, 28, 128, 64, 32, 16, 32, 64, 128, 16,
];
const PONG: &[u8] = &[
    106, 2, 107, 12, 108, 63, 109, 12, 162, 234, 218, 182, 220, 214, 110, 0, 34, 212, 102, 3, 104,
    2, 96, 96, 240, 21, 240, 7, 48, 0, 18, 26, 199, 23, 119, 8, 105, 255, 162, 240, 214, 113, 162,
    234, 218, 182, 220, 214, 96, 1, 224, 161, 123, 254, 96, 4, 224, 161, 123, 2, 96, 31, 139, 2,
    218, 182, 96, 12, 224, 161, 125, 254, 96, 13, 224, 161, 125, 2, 96, 31, 141, 2, 220, 214, 162,
    240, 214, 113, 134, 132, 135, 148, 96, 63, 134, 2, 97, 31, 135, 18, 70, 2, 18, 120, 70, 63, 18,
    130, 71, 31, 105, 255, 71, 0, 105, 1, 214, 113, 18, 42, 104, 2, 99, 1, 128, 112, 128, 181, 18,
    138, 104, 254, 99, 10, 128, 112, 128, 213, 63, 1, 18, 162, 97, 2, 128, 21, 63, 1, 18, 186, 128,
    21, 63, 1, 18, 200, 128, 21, 63, 1, 18, 194, 96, 32, 240, 24, 34, 212, 142, 52, 34, 212, 102,
    62, 51, 1, 102, 3, 104, 254, 51, 1, 104, 2, 18, 22, 121, 255, 73, 254, 105, 255, 18, 200, 121,
    1, 73, 2, 105, 1, 96, 4, 240, 24, 118, 1, 70, 64, 118, 254, 18, 108, 162, 242, 254, 51, 242,
    101, 241, 41, 100, 20, 101, 0, 212, 85, 116, 21, 242, 41, 212, 85, 0, 238, 128, 128, 128, 128,
    128, 128, 128, 0, 0, 0, 0, 0,
];
const PONG2: &[u8] = &[
    34, 252, 107, 12, 108, 63, 109, 12, 162, 234, 218, 182, 220, 214, 110, 0, 34, 212, 102, 3, 104,
    2, 96, 96, 240, 21, 240, 7, 48, 0, 18, 26, 199, 23, 119, 8, 105, 255, 162, 240, 214, 113, 162,
    234, 218, 182, 220, 214, 96, 1, 224, 161, 123, 254, 96, 4, 224, 161, 123, 2, 96, 31, 139, 2,
    218, 182, 96, 12, 224, 161, 125, 254, 96, 13, 224, 161, 125, 2, 96, 31, 141, 2, 220, 214, 162,
    240, 214, 113, 134, 132, 135, 148, 96, 63, 134, 2, 97, 31, 135, 18, 70, 0, 18, 120, 70, 63, 18,
    130, 71, 31, 105, 255, 71, 0, 105, 1, 214, 113, 18, 42, 104, 2, 99, 1, 128, 112, 128, 181, 18,
    138, 104, 254, 99, 10, 128, 112, 128, 213, 63, 1, 18, 162, 97, 2, 128, 21, 63, 1, 18, 186, 128,
    21, 63, 1, 18, 200, 128, 21, 63, 1, 18, 194, 96, 32, 240, 24, 34, 212, 142, 52, 34, 212, 102,
    62, 51, 1, 102, 3, 104, 254, 51, 1, 104, 2, 18, 22, 121, 255, 73, 254, 105, 255, 18, 200, 121,
    1, 73, 2, 105, 1, 96, 4, 240, 24, 118, 1, 70, 64, 118, 254, 18, 108, 162, 242, 254, 51, 242,
    101, 241, 41, 100, 20, 101, 2, 212, 85, 116, 21, 242, 41, 212, 85, 0, 238, 128, 128, 128, 128,
    128, 128, 128, 0, 0, 0, 0, 0, 192, 192, 192, 0, 255, 0, 107, 32, 108, 0, 162, 246, 219, 196,
    124, 4, 60, 32, 19, 2, 106, 0, 107, 0, 108, 31, 162, 250, 218, 177, 218, 193, 122, 8, 58, 64,
    19, 18, 162, 246, 106, 0, 107, 32, 219, 161, 0, 238,
];

pub fn get_data(game_name: &str) -> &[u8] {
    match game_name {
        "PONG" => PONG,
        "PONG2" => PONG2,
        "TETRIS" => TETRIS,
        "MAZE" => MAZE,
        "INVADERS" => INVADERS,
        "TICTAC" => TICTAC,
        _ => PONG,
    }
}

// pub fn available_games() -> Vec<&str> {
//     vec!["PONG", "PONG2", "TETRIS", "MAZE", "INVADERS", "TICTAC"]
// }