use crate::ColorFacet;

#[derive(PartialEq, Clone, Copy)]
pub struct Facet {
    pub color: ColorFacet,
    pub index: u8,
}

impl std::fmt::Debug for Facet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}{} ", self.color, self.index)
    }
}

impl Facet {
    pub fn new(color: ColorFacet, index: u8) -> Facet {
        Facet { color, index }
    }
}
