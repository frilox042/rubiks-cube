extern crate rubiks_cube;

#[cfg(test)]
mod tests {

    #[test]
    fn test_rotation_in_one_direction_then_opposite_should_give_identity() {
        let my_rubiks_cube = rubiks_cube::build_rubiks_cube();
        for color in rubiks_cube::ColorFacet::iterator() {
            for direction in rubiks_cube::RotationDirection::iterator() {
                let rotated_cube = my_rubiks_cube
                    .rotate(*color, *direction)
                    .rotate(*color, direction.opposite());
                assert!(
                    my_rubiks_cube == rotated_cube,
                    "\n{:?}\n{:?}",
                    color,
                    rotated_cube
                );
            }
        }
    }

    #[test]
    fn test_four_same_rotation_should_give_identity() {
        let my_rubiks_cube = rubiks_cube::build_rubiks_cube();
        for color in rubiks_cube::ColorFacet::iterator() {
            for direction in rubiks_cube::RotationDirection::iterator() {
                let rotated_cube = my_rubiks_cube
                    .rotate(*color, *direction)
                    .rotate(*color, *direction)
                    .rotate(*color, *direction)
                    .rotate(*color, *direction);
                assert!(
                    my_rubiks_cube == rotated_cube,
                    "\n{:?}\n{:?}",
                    color,
                    rotated_cube
                );
            }
        }
    }

    #[test]
    fn test_one_rotation_should_give_same_as_three_opposite_rotation() {
        let my_rubiks_cube = rubiks_cube::build_rubiks_cube();
        for color in rubiks_cube::ColorFacet::iterator() {
            for direction in rubiks_cube::RotationDirection::iterator() {
                let rotated_cube = my_rubiks_cube.rotate(*color, *direction);
                let opposite_rotated_cube = my_rubiks_cube
                    .rotate(*color, direction.opposite())
                    .rotate(*color, direction.opposite())
                    .rotate(*color, direction.opposite());
                assert!(
                    rotated_cube == opposite_rotated_cube,
                    "\n{:?}\n{:?}",
                    color,
                    rotated_cube
                );
            }
        }
    }

}
