use crate::ColorFacet;
use crate::Facet;
use crate::RotationDirection;

#[derive(PartialEq, Clone, Copy)]
pub struct Face {
    pub center: Facet,
    pub left_top: Facet,
    pub top: Facet,
    pub right_top: Facet,
    pub left: Facet,
    pub right: Facet,
    pub left_bottom: Facet,
    pub bottom: Facet,
    pub right_bottom: Facet,
}

impl std::fmt::Debug for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?}{:?}{:?}\n{:?}{:?}{:?}\n{:?}{:?}{:?}",
            self.left_top,
            self.top,
            self.right_top,
            self.left,
            self.center,
            self.right,
            self.left_bottom,
            self.bottom,
            self.right_bottom
        )
    }
}

impl Face {
    pub fn new(color: ColorFacet) -> Face {
        Face {
            left_top: Facet::new(color, 1),
            top: Facet::new(color, 2),
            right_top: Facet::new(color, 3),
            left: Facet::new(color, 4),
            center: Facet::new(color, 5),
            right: Facet::new(color, 6),
            left_bottom: Facet::new(color, 7),
            bottom: Facet::new(color, 8),
            right_bottom: Facet::new(color, 9),
        }
    }

    pub fn rotate(self, direction: RotationDirection) -> Face {
        match direction {
            RotationDirection::Clockwise => Face::rotate_clockwise(self),
            RotationDirection::Anticlockwise => Face::rotate_anticlockwise(self),
        }
    }

    pub fn rotate_clockwise(self) -> Face {
        Face {
            left_top: self.left_bottom,
            top: self.left,
            right_top: self.left_top,
            left: self.bottom,
            right: self.top,
            left_bottom: self.right_bottom,
            bottom: self.right,
            right_bottom: self.right_top,
            ..self
        }
    }

    pub fn rotate_anticlockwise(self) -> Face {
        Face {
            left_top: self.right_top,
            top: self.right,
            right_top: self.right_bottom,
            left: self.top,
            right: self.bottom,
            left_bottom: self.left_top,
            bottom: self.left,
            right_bottom: self.left_bottom,
            ..self
        }
    }
}
