use std::slice::Iter;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum RotationDirection {
    Clockwise,
    Anticlockwise,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ColorFacet {
    Red,
    Blue,
    Green,
    Orange,
    White,
    Yellow,
}

#[derive(PartialEq, Clone, Copy)]
pub struct Facet {
    pub color: ColorFacet,
    pub index: u8,
}

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

#[derive(PartialEq, Clone, Copy)]
pub struct RubiksCube {
    pub red: Face,
    pub blue: Face,
    pub green: Face,
    pub orange: Face,
    pub white: Face,
    pub yellow: Face,
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
        COLOR_FACET.into_iter()
    }
}

impl RotationDirection {
    pub fn iterator() -> Iter<'static, RotationDirection> {
        static ROTATION_DIRECTION: [RotationDirection; 2] = [
            RotationDirection::Clockwise,
            RotationDirection::Anticlockwise,
        ];
        ROTATION_DIRECTION.into_iter()
    }

    pub fn opposite(&self) -> RotationDirection {
        match self {
            RotationDirection::Clockwise => RotationDirection::Anticlockwise,
            RotationDirection::Anticlockwise => RotationDirection::Clockwise,
        }
    }
}

impl std::fmt::Debug for Facet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}{} ", self.color, self.index)
    }
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

impl std::fmt::Debug for RubiksCube {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?}\n\n{:?}\n\n{:?}\n\n{:?}\n\n{:?}\n\n{:?}",
            self.red, self.blue, self.green, self.white, self.orange, self.yellow
        )
    }
}

impl Face {
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

impl RubiksCube {
    pub fn rotate(self, face: ColorFacet, direction: RotationDirection) -> RubiksCube {
        match face {
            ColorFacet::Blue => RubiksCube::rotate_blue(self, direction),
            ColorFacet::Red => RubiksCube::rotate_red(self, direction),
            ColorFacet::Green => RubiksCube::rotate_green(self, direction),
            ColorFacet::Orange => RubiksCube::rotate_orange(self, direction),
            ColorFacet::White => RubiksCube::rotate_white(self, direction),
            ColorFacet::Yellow => RubiksCube::rotate_yellow(self, direction),
        }
    }

    pub fn rotate_blue(self, direction: RotationDirection) -> RubiksCube {
        match direction {
            RotationDirection::Clockwise => RubiksCube::rotate_blue_clockwise(self),
            RotationDirection::Anticlockwise => RubiksCube::rotate_blue_anticlockwise(self),
        }
    }
    pub fn rotate_red(self, direction: RotationDirection) -> RubiksCube {
        match direction {
            RotationDirection::Clockwise => RubiksCube::rotate_red_clockwise(self),
            RotationDirection::Anticlockwise => RubiksCube::rotate_red_anticlockwise(self),
        }
    }
    pub fn rotate_green(self, direction: RotationDirection) -> RubiksCube {
        match direction {
            RotationDirection::Clockwise => RubiksCube::rotate_green_clockwise(self),
            RotationDirection::Anticlockwise => RubiksCube::rotate_green_anticlockwise(self),
        }
    }
    pub fn rotate_orange(self, direction: RotationDirection) -> RubiksCube {
        match direction {
            RotationDirection::Clockwise => RubiksCube::rotate_orange_clockwise(self),
            RotationDirection::Anticlockwise => RubiksCube::rotate_orange_anticlockwise(self),
        }
    }
    pub fn rotate_white(self, direction: RotationDirection) -> RubiksCube {
        match direction {
            RotationDirection::Clockwise => RubiksCube::rotate_white_clockwise(self),
            RotationDirection::Anticlockwise => RubiksCube::rotate_white_anticlockwise(self),
        }
    }
    pub fn rotate_yellow(self, direction: RotationDirection) -> RubiksCube {
        match direction {
            RotationDirection::Clockwise => RubiksCube::rotate_yellow_clockwise(self),
            RotationDirection::Anticlockwise => RubiksCube::rotate_yellow_anticlockwise(self),
        }
    }

    pub fn rotate_red_clockwise(self) -> RubiksCube {
        RubiksCube {
            red: Face::rotate(self.red, RotationDirection::Clockwise),
            white: Face {
                left_bottom: self.green.right_bottom,
                bottom: self.green.right,
                right_bottom: self.green.right_top,
                ..self.white
            },
            blue: Face {
                left_top: self.white.left_bottom,
                left: self.white.bottom,
                left_bottom: self.white.right_bottom,
                ..self.blue
            },
            yellow: Face {
                right_top: self.blue.left_top,
                top: self.blue.left,
                left_top: self.blue.left_bottom,
                ..self.yellow
            },
            green: Face {
                right_bottom: self.yellow.right_top,
                right: self.yellow.top,
                right_top: self.yellow.left_top,
                ..self.green
            },
            ..self
        }
    }
    pub fn rotate_red_anticlockwise(self) -> RubiksCube {
        RubiksCube {
            red: Face::rotate(self.red, RotationDirection::Anticlockwise),
            white: Face {
                left_bottom: self.blue.left_top,
                bottom: self.blue.left,
                right_bottom: self.blue.left_bottom,
                ..self.white
            },
            blue: Face {
                left_top: self.yellow.right_top,
                left: self.yellow.top,
                left_bottom: self.yellow.left_top,
                ..self.blue
            },
            yellow: Face {
                right_top: self.green.right_bottom,
                top: self.green.right,
                left_top: self.green.right_top,
                ..self.yellow
            },
            green: Face {
                right_bottom: self.white.left_bottom,
                right: self.white.bottom,
                right_top: self.white.right_bottom,
                ..self.green
            },
            ..self
        }
    }
    pub fn rotate_blue_clockwise(self) -> RubiksCube {
        RubiksCube {
            blue: Face::rotate(self.blue, RotationDirection::Clockwise),
            red: Face {
                right_bottom: self.yellow.right_bottom,
                right: self.yellow.right,
                right_top: self.yellow.right_top,
                ..self.red
            },
            white: Face {
                right_bottom: self.red.right_bottom,
                right: self.red.right,
                right_top: self.red.right_top,
                ..self.white
            },
            orange: Face {
                left_bottom: self.white.right_bottom,
                left: self.white.right,
                left_top: self.white.right_top,
                ..self.orange
            },
            yellow: Face {
                right_bottom: self.orange.left_bottom,
                right: self.orange.left,
                right_top: self.orange.left_top,
                ..self.yellow
            },
            ..self
        }
    }
    pub fn rotate_blue_anticlockwise(self) -> RubiksCube {
        RubiksCube {
            blue: Face::rotate(self.blue, RotationDirection::Anticlockwise),
            red: Face {
                right_bottom: self.white.right_bottom,
                right: self.white.right,
                right_top: self.white.right_top,
                ..self.red
            },
            white: Face {
                right_bottom: self.orange.left_bottom,
                right: self.orange.left,
                right_top: self.orange.left_top,
                ..self.white
            },
            orange: Face {
                left_bottom: self.yellow.right_bottom,
                left: self.yellow.right,
                left_top: self.yellow.right_top,
                ..self.orange
            },
            yellow: Face {
                right_bottom: self.red.right_bottom,
                right: self.red.right,
                right_top: self.red.right_top,
                ..self.yellow
            },
            ..self
        }
    }
    pub fn rotate_green_clockwise(self) -> RubiksCube {
        RubiksCube {
            green: Face::rotate(self.green, RotationDirection::Clockwise),
            red: Face {
                left_bottom: self.white.left_bottom,
                left: self.white.left,
                left_top: self.white.left_top,
                ..self.red
            },
            white: Face {
                left_bottom: self.orange.right_top,
                left: self.orange.right,
                left_top: self.orange.right_bottom,
                ..self.white
            },
            orange: Face {
                right_bottom: self.yellow.left_top,
                right: self.yellow.left,
                right_top: self.yellow.left_bottom,
                ..self.orange
            },
            yellow: Face {
                left_bottom: self.red.left_bottom,
                left: self.red.left,
                left_top: self.red.left_top,
                ..self.yellow
            },
            ..self
        }
    }
    pub fn rotate_green_anticlockwise(self) -> RubiksCube {
        RubiksCube {
            green: Face::rotate(self.green, RotationDirection::Anticlockwise),
            red: Face {
                left_bottom: self.yellow.left_bottom,
                left: self.yellow.left,
                left_top: self.yellow.left_top,
                ..self.red
            },
            white: Face {
                left_bottom: self.red.left_bottom,
                left: self.red.left,
                left_top: self.red.left_top,
                ..self.white
            },
            orange: Face {
                right_bottom: self.white.left_top,
                right: self.white.left,
                right_top: self.white.left_bottom,
                ..self.orange
            },
            yellow: Face {
                left_bottom: self.orange.right_top,
                left: self.orange.right,
                left_top: self.orange.right_bottom,
                ..self.yellow
            },
            ..self
        }
    }
    pub fn rotate_orange_clockwise(self) -> RubiksCube {
        RubiksCube {
            orange: Face::rotate(self.orange, RotationDirection::Clockwise),
            green: Face {
                left_bottom: self.white.left_top,
                left: self.white.top,
                left_top: self.white.right_top,
                ..self.green
            },
            white: Face {
                left_top: self.blue.right_top,
                top: self.blue.right,
                right_top: self.blue.right_bottom,
                ..self.white
            },
            blue: Face {
                right_bottom: self.yellow.left_bottom,
                right: self.yellow.bottom,
                right_top: self.yellow.right_bottom,
                ..self.blue
            },
            yellow: Face {
                left_bottom: self.green.left_top,
                bottom: self.green.left,
                right_bottom: self.green.left_bottom,
                ..self.yellow
            },
            ..self
        }
    }
    pub fn rotate_orange_anticlockwise(self) -> RubiksCube {
        RubiksCube {
            orange: Face::rotate(self.orange, RotationDirection::Anticlockwise),
            green: Face {
                left_bottom: self.yellow.right_bottom,
                left: self.yellow.bottom,
                left_top: self.yellow.left_bottom,
                ..self.green
            },
            white: Face {
                left_top: self.green.left_bottom,
                top: self.green.left,
                right_top: self.green.left_top,
                ..self.white
            },
            blue: Face {
                right_bottom: self.white.right_top,
                right: self.white.top,
                right_top: self.white.left_top,
                ..self.blue
            },
            yellow: Face {
                left_bottom: self.blue.right_bottom,
                bottom: self.blue.right,
                right_bottom: self.blue.right_top,
                ..self.yellow
            },
            ..self
        }
    }
    pub fn rotate_white_clockwise(self) -> RubiksCube {
        RubiksCube {
            white: Face::rotate(self.white, RotationDirection::Clockwise),
            green: Face {
                left_top: self.red.left_top,
                top: self.red.top,
                right_top: self.red.right_top,
                ..self.green
            },
            orange: Face {
                left_top: self.green.left_top,
                top: self.green.top,
                right_top: self.green.right_top,
                ..self.orange
            },
            blue: Face {
                left_top: self.orange.left_top,
                top: self.orange.top,
                right_top: self.orange.right_top,
                ..self.blue
            },
            red: Face {
                left_top: self.blue.left_top,
                top: self.blue.top,
                right_top: self.blue.right_top,
                ..self.red
            },
            ..self
        }
    }
    pub fn rotate_white_anticlockwise(self) -> RubiksCube {
        RubiksCube {
            white: Face::rotate(self.white, RotationDirection::Anticlockwise),
            green: Face {
                left_top: self.orange.left_top,
                top: self.orange.top,
                right_top: self.orange.right_top,
                ..self.green
            },
            orange: Face {
                left_top: self.blue.left_top,
                top: self.blue.top,
                right_top: self.blue.right_top,
                ..self.orange
            },
            blue: Face {
                left_top: self.red.left_top,
                top: self.red.top,
                right_top: self.red.right_top,
                ..self.blue
            },
            red: Face {
                left_top: self.green.left_top,
                top: self.green.top,
                right_top: self.green.right_top,
                ..self.red
            },
            ..self
        }
    }
    pub fn rotate_yellow_clockwise(self) -> RubiksCube {
        RubiksCube {
            yellow: Face::rotate(self.yellow, RotationDirection::Clockwise),
            green: Face {
                left_bottom: self.orange.left_bottom,
                bottom: self.orange.bottom,
                right_bottom: self.orange.right_bottom,
                ..self.green
            },
            orange: Face {
                left_bottom: self.blue.left_bottom,
                bottom: self.blue.bottom,
                right_bottom: self.blue.right_bottom,
                ..self.orange
            },
            blue: Face {
                left_bottom: self.red.left_bottom,
                bottom: self.red.bottom,
                right_bottom: self.red.right_bottom,
                ..self.blue
            },
            red: Face {
                left_bottom: self.green.left_bottom,
                bottom: self.green.bottom,
                right_bottom: self.green.right_bottom,
                ..self.red
            },
            ..self
        }
    }
    pub fn rotate_yellow_anticlockwise(self) -> RubiksCube {
        RubiksCube {
            yellow: Face::rotate(self.yellow, RotationDirection::Anticlockwise),
            green: Face {
                left_bottom: self.red.left_bottom,
                bottom: self.red.bottom,
                right_bottom: self.red.right_bottom,
                ..self.green
            },
            orange: Face {
                left_bottom: self.green.left_bottom,
                bottom: self.green.bottom,
                right_bottom: self.green.right_bottom,
                ..self.orange
            },
            blue: Face {
                left_bottom: self.orange.left_bottom,
                bottom: self.orange.bottom,
                right_bottom: self.orange.right_bottom,
                ..self.blue
            },
            red: Face {
                left_bottom: self.blue.left_bottom,
                bottom: self.blue.bottom,
                right_bottom: self.blue.right_bottom,
                ..self.red
            },
            ..self
        }
    }
}

pub fn build_facet(color: ColorFacet, index: u8) -> Facet {
    Facet {
        color: color,
        index: index,
    }
}

pub fn build_face(color: ColorFacet) -> Face {
    Face {
        left_top: build_facet(color, 1),
        top: build_facet(color, 2),
        right_top: build_facet(color, 3),
        left: build_facet(color, 4),
        center: build_facet(color, 5),
        right: build_facet(color, 6),
        left_bottom: build_facet(color, 7),
        bottom: build_facet(color, 8),
        right_bottom: build_facet(color, 9),
    }
}

pub fn build_rubiks_cube() -> RubiksCube {
    RubiksCube {
        red: build_face(ColorFacet::Red),
        blue: build_face(ColorFacet::Blue),
        green: build_face(ColorFacet::Green),
        white: build_face(ColorFacet::White),
        orange: build_face(ColorFacet::Orange),
        yellow: build_face(ColorFacet::Yellow),
    }
}