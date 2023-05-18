#[derive(Copy, Clone)]
#[allow(unused)]
pub enum DisplayMode {
    Light,
    Dark,
}
// TODO: Use DisplayMode for light mode/dark mode. For light mode, we will also want to consider decreasing from the top when using a list of colors since that will be darker
#[derive(Debug)]
#[allow(unused)]
pub enum Color {
    Green,
    Blue,
    Purple,
    Grey,
    Black,
    White,
}

#[allow(unused)]
pub enum Style {
    Lines(LineEmphasis),
    Markers(MarkerEmphasis),
}

#[allow(unused)]
pub enum LineEmphasis {
    Light,
    Heavy,
    Dashed,
}

#[allow(unused)]
pub enum MarkerEmphasis {
    Light,
    Heavy,
}

#[allow(unused)]
pub struct RegionDesign {
    pub color: Color,
    pub color_slot: usize,
}

#[allow(unused)]
pub struct CurveDesign {
    pub color: Color,
    pub color_slot: usize,
    pub style: Style,
}

pub const MAIN_COLOR_SLOT: usize = 5;

pub const PRIMITIVE_GREENS: [&str; 10] = [
    "E1FDEA", "BCF2CD", "95E8AF", "6CDD90", "45D471", "2BBA58", "1F9143", "136730", "063F1A",
    "001703",
];

pub const PRIMITIVE_BLUES: [&str; 10] = [
    "C2E8FF", "AED4FF", "9AC0FF", "86ACFF", "7298EB", "5E84D7", "4A70C3", "365CAF", "22489B",
    "002073",
];

pub const PRIMITIVE_PURPLES: [&str; 10] = [
    "FEF4FF", "EAE0FF", "D6CCFF", "C2B8FF", "AEA4FF", "9A90F2", "867CDE", "7268CA", "5E54B6",
    "362C8E",
];

pub const PRIMITIVE_GREYS: [&str; 10] = [
    "F1F1FC", "D6D8DF", "BBBEC3", "A2A4AA", "878A91", "6D7077", "55575E", "3D3E44", "23252B",
    "0B0B15",
];

pub const PRIMITIVE_BLACK: &str = "151718";
pub const PRIMITIVE_WHITE: &str = "FFFFFF";
