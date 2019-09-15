use std::slice::Iter;

#[derive(PartialEq, Clone, Copy)]
pub enum ColorFacet {
    Red,
    Blue,
    Green,
    Orange,
    White,
    Yellow,
}

impl ColorFacet {
    pub fn iterator() -> Iter<'static, ColorFacet> {
        static COLOR_FACET: [ColorFacet; 6] = [
            ColorFacet::Red,
            ColorFacet::Blue,
            ColorFacet::Green,
            ColorFacet::Orange,
            ColorFacet::White,
            ColorFacet::Yellow,
        ];
        COLOR_FACET.iter()
    }
}

impl std::fmt::Debug for ColorFacet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ColorFacet::Blue => write!(f, "B"),
            ColorFacet::Red => write!(f, "R"),
            ColorFacet::Green => write!(f, "G"),
            ColorFacet::Orange => write!(f, "O"),
            ColorFacet::White => write!(f, "W"),
            ColorFacet::Yellow => write!(f, "Y"),
        }
    }
}
