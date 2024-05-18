// Taken from
// 1. https://gbdev.io/pandocs/Power_Up_Sequence.html#compatibility-palettes
// 2. https://github.com/LIJI32/SameBoy/blob/master/BootROMs/cgb_boot.asm

pub const PALETTE_ID_LOOKUP_TABLE: [usize; 94] = [
    0,  // Default Palette
    4,  // ALLEY WAY
    5,  // YAKUMAN
    35, // BASEBALL, (Game and Watch 2)
    34, // TENNIS
    3,  // TETRIS
    31, // QIX
    15, // DR.MARIO
    10, // RADARMISSION
    5,  // F1RACE
    19, // YOSSY NO TAMAGO
    36, //
    7,  // X
    37, // MARIOLAND2
    30, // YOSSY NO COOKIE
    44, // ZELDA
    21, //
    32, //
    31, // TETRIS FLASH
    20, // DONKEY KONG
    5,  // MARIO'S PICROSS
    33, //
    13, // POKEMON RED, (GAMEBOYCAMERA G)
    14, // POKEMON GREEN
    5,  // PICROSS 2
    29, // YOSSY NO PANEPON
    5,  // KIRAKIRA KIDS
    18, // GAMEBOY GALLERY
    9,  // POCKETCAMERA
    3,  //
    2,  // BALLOON KID
    26, // KINGOFTHEZOO
    25, // DMG FOOTBALL
    25, // WORLD CUP
    41, // OTHELLO
    42, // SUPER RC PRO-AM
    26, // DYNABLASTER
    45, // BOY AND BLOB GB2
    42, // MEGAMAN
    45, // STAR WARS-NOA
    36, //
    38, // WAVERACE
    26, //
    42, // LOLO2
    30, // YOSHI'S COOKIE
    41, // MYSTIC QUEST
    34, //
    34, // TOPRANKINGTENNIS
    5,  // MANSELL
    42, // MEGAMAN3
    6,  // SPACE INVADERS
    5,  // GAME&WATCH
    33, // DONKEYKONGLAND95
    25, // ASTEROIDS/MISCMD
    42, // STREET FIGHTER 2
    42, // DEFENDER/JOUST
    40, // KILLERINSTINCT95
    2,  // TETRIS BLAST
    16, // PINOCCHIO
    25, //
    42, // BA.TOSHINDEN
    42, // NETTOU KOF 95
    5,  //
    0,  // TETRIS PLUS
    39, // DONKEYKONGLAND 3
    36, //
    22, // SUPER MARIOLAND
    25, // GOLF
    6,  // SOLARSTRIKER
    32, // GBWARS
    12, // KAERUNOTAMENI
    36, //
    11, // POKEMON BLUE
    39, // DONKEYKONGLAND
    18, // GAMEBOY GALLERY2
    39, // DONKEYKONGLAND 2
    24, // KID ICARUS
    31, // TETRIS2
    50, //
    17, // MOGURANYA
    46, //
    6,  // GALAGA&GALAXIAN
    27, // BT2RAGNAROKWORLD
    0,  // KEN GRIFFEY JR
    47, //
    41, // MAGNETIC SOCCER
    41, // VEGAS STAKES
    0,  //
    0,  // MILLI/CENTI/PEDE
    19, // MARIO & YOSHI
    34, // SOCCER
    23, // POKEBOM
    18, // G&W GALLERY
    29, // TETRIS ATTACK
];
