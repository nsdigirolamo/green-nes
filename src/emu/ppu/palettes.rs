use sdl2::pixels::Color;

pub const RED: Color = Color::RGB(255, 0, 0);
pub const GREEN: Color = Color::RGB(0, 128, 0);
pub const BLUE: Color = Color::RGB(0, 0, 255);
pub const YELLOW: Color = Color::RGB(255, 255, 0);
pub const CYAN: Color = Color::RGB(0, 255, 255);
pub const MAGENTA: Color = Color::RGB(255, 0, 255);
pub const BLACK: Color = Color::RGB(0, 0, 0);
pub const WHITE: Color = Color::RGB(255, 255, 255);
pub const GRAY: Color = Color::RGB(128, 128, 128);
pub const ORANGE: Color = Color::RGB(255, 165, 0);
pub const PURPLE: Color = Color::RGB(128, 0, 128);
pub const PINK: Color = Color::RGB(255, 192, 203);
pub const BROWN: Color = Color::RGB(165, 42, 42);
pub const LIME: Color = Color::RGB(0, 255, 0);
pub const NAVY: Color = Color::RGB(0, 0, 128);
pub const TEAL: Color = Color::RGB(0, 128, 128);
pub const OLIVE: Color = Color::RGB(128, 128, 0);
pub const MAROON: Color = Color::RGB(128, 0, 0);
pub const SILVER: Color = Color::RGB(192, 192, 192);
pub const GOLD: Color = Color::RGB(255, 215, 0);

pub fn get_pattern_index_debug_color(pattern_index: u8) -> Color {
    // let colors: [Color; 20] = [
    //     RED, GREEN, BLUE, YELLOW, CYAN, MAGENTA, BLACK, WHITE, GRAY, ORANGE, PURPLE, PINK, BROWN,
    //     LIME, NAVY, TEAL, OLIVE, MAROON, SILVER, GOLD,
    // ];

    // colors[(pattern_index % 20) as usize]

    if pattern_index == 0 { BLACK } else { RED }
}

pub const PALETTE_TABLE: [[Color; 16]; 4] = [
    [
        Color::RGB(98, 98, 98),
        Color::RGB(0, 28, 149),
        Color::RGB(25, 4, 172),
        Color::RGB(66, 0, 157),
        Color::RGB(97, 0, 107),
        Color::RGB(110, 0, 37),
        Color::RGB(101, 5, 0),
        Color::RGB(73, 30, 0),
        Color::RGB(34, 55, 0),
        Color::RGB(0, 73, 0),
        Color::RGB(0, 79, 0),
        Color::RGB(0, 72, 22),
        Color::RGB(0, 53, 94),
        Color::RGB(0, 0, 0), // https://www.nesdev.org/wiki/Color_$0D_games
        Color::RGB(0, 0, 0),
        Color::RGB(0, 0, 0),
    ],
    [
        Color::RGB(171, 171, 171),
        Color::RGB(12, 78, 219),
        Color::RGB(61, 46, 255),
        Color::RGB(113, 21, 243),
        Color::RGB(155, 11, 185),
        Color::RGB(176, 18, 98),
        Color::RGB(169, 39, 4),
        Color::RGB(137, 70, 0),
        Color::RGB(87, 102, 0),
        Color::RGB(35, 127, 0),
        Color::RGB(0, 137, 0),
        Color::RGB(0, 131, 50),
        Color::RGB(0, 109, 144),
        Color::RGB(0, 0, 0),
        Color::RGB(0, 0, 0),
        Color::RGB(0, 0, 0),
    ],
    [
        Color::RGB(255, 255, 255),
        Color::RGB(87, 165, 255),
        Color::RGB(130, 135, 255),
        Color::RGB(180, 109, 255),
        Color::RGB(223, 96, 255),
        Color::RGB(248, 99, 198),
        Color::RGB(248, 116, 109),
        Color::RGB(222, 144, 32),
        Color::RGB(179, 174, 0),
        Color::RGB(129, 200, 0),
        Color::RGB(86, 213, 34),
        Color::RGB(61, 211, 111),
        Color::RGB(62, 193, 200),
        Color::RGB(78, 78, 78),
        Color::RGB(0, 0, 0),
        Color::RGB(0, 0, 0),
    ],
    [
        Color::RGB(255, 255, 255),
        Color::RGB(190, 224, 255),
        Color::RGB(205, 212, 255),
        Color::RGB(224, 202, 255),
        Color::RGB(241, 196, 255),
        Color::RGB(252, 196, 239),
        Color::RGB(253, 202, 206),
        Color::RGB(245, 212, 175),
        Color::RGB(230, 223, 156),
        Color::RGB(211, 233, 154),
        Color::RGB(194, 239, 168),
        Color::RGB(183, 239, 196),
        Color::RGB(182, 234, 229),
        Color::RGB(184, 184, 184),
        Color::RGB(0, 0, 0),
        Color::RGB(0, 0, 0),
    ],
];
