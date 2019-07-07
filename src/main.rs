
mod rubiks_cube;

fn main() {
    let my_rubiks_cube = rubiks_cube::build_rubiks_cube();
    for color in rubiks_cube::ColorFacet::iterator() {
        for direction in rubiks_cube::RotationDirection::iterator() {
            let test_1 = my_rubiks_cube
                .rotate(*color, *direction)
                .rotate(*color, direction.opposite());
            assert!(my_rubiks_cube == test_1, "\n{:?}\n{:?}", color, test_1);
            let test_2 = my_rubiks_cube
                .rotate(*color, *direction)
                .rotate(*color, *direction)
                .rotate(*color, *direction)
                .rotate(*color, *direction);
            assert!(
                my_rubiks_cube == test_2,
                "\n{:?} {:?}\n{:?}",
                color,
                direction,
                test_2
            );
            let test_3_1 = my_rubiks_cube.rotate(*color, *direction);
            let test_3_2 = my_rubiks_cube
                .rotate(*color, direction.opposite())
                .rotate(*color, direction.opposite())
                .rotate(*color, direction.opposite());
            assert!(test_3_1 == test_3_2, "\n{:?}", color);
        }
    }
}
