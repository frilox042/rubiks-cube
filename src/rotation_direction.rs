use std::slice::Iter;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum RotationDirection {
    Clockwise,
    Anticlockwise,
}

impl RotationDirection {
    pub fn iterator() -> Iter<'static, RotationDirection> {
        static ROTATION_DIRECTION: [RotationDirection; 2] = [
            RotationDirection::Clockwise,
            RotationDirection::Anticlockwise,
        ];
        ROTATION_DIRECTION.iter()
    }

    pub fn opposite(self) -> RotationDirection {
        match self {
            RotationDirection::Clockwise => RotationDirection::Anticlockwise,
            RotationDirection::Anticlockwise => RotationDirection::Clockwise,
        }
    }
}
