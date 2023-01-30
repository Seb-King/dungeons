use crate::dungeon_generation::room::Orientation::{DOWN, LEFT, RIGHT, UP};
use bevy::math::IVec2;

#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct CollisionBox {
    pub shape: Rectangle,
    pub position: IVec2,
}

#[derive(Clone, Debug)]
pub struct Room {
    pub shape: Rectangle,
    pub position: IVec2,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Orientation {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
pub struct Line {
    pub start: i32,
    pub end: i32,
}

#[derive(Clone, Debug)]
pub struct IShape {
    pub orientation: Orientation,
    pub length: u32,
}

#[derive(Clone, Debug)]
pub struct Corridor {
    pub shape: IShape,
    pub position: IVec2,
}

impl Line {
    fn new(start: i32, end: i32) -> Line {
        Line { start, end }
    }
}

pub trait Collision {
    fn collides_with(&self, rhs: &impl Collision) -> bool {
        let left_box = self.to_collision_box();
        let right_box = rhs.to_collision_box();

        let left_x_line = Line::new(
            left_box.position.x,
            left_box.position.x + left_box.shape.width as i32 - 1,
        );

        let left_y_line = Line::new(
            left_box.position.y,
            left_box.position.y + left_box.shape.height as i32 - 1,
        );

        let right_x_line = Line::new(
            right_box.position.x,
            right_box.position.x + right_box.shape.width as i32 - 1,
        );

        let right_y_line = Line::new(
            right_box.position.y,
            right_box.position.y + right_box.shape.height as i32 - 1,
        );

        let x_line_intersect = lines_intersect(&left_x_line, &right_x_line);
        let y_line_intersect = lines_intersect(&left_y_line, &right_y_line);

        return x_line_intersect && y_line_intersect;
    }

    fn to_collision_box(&self) -> CollisionBox;
}

fn lines_intersect(lhs: &Line, rhs: &Line) -> bool {
    return (rhs.start <= lhs.start && lhs.start <= rhs.end)
        || (rhs.start <= lhs.end && lhs.end <= rhs.end)
        || (lhs.start <= rhs.start && lhs.end >= rhs.end);
}

impl Collision for Room {
    fn to_collision_box(&self) -> CollisionBox {
        CollisionBox {
            shape: self.shape,
            position: self.position,
        }
    }
}

impl Collision for Corridor {
    fn to_collision_box(&self) -> CollisionBox {
        if self.shape.length <= 2 {
            return CollisionBox {
                shape: Rectangle {
                    width: 0,
                    height: 0,
                },
                position: self.position,
            };
        }

        let orientation = match self.shape.orientation {
            UP => UP,
            DOWN => UP,
            RIGHT => RIGHT,
            LEFT => RIGHT,
        };

        let direction: IVec2 = self.shape.orientation.into();

        let flipped_position = match self.shape.orientation {
            UP => self.position,
            DOWN => (self.shape.length as i32 - 1) * direction,
            RIGHT => self.position,
            LEFT => (self.shape.length as i32 - 1) * direction,
        };

        let dir: IVec2 = orientation.clone().into();
        let perp = dir.perp().abs();

        let start_point: IVec2 = flipped_position + dir - perp;
        let end_point: IVec2 = flipped_position + (self.shape.length as i32 - 2) * dir + perp;

        let width = (start_point.x - end_point.x).abs() as u32 + 1;
        let height = (start_point.y - end_point.y).abs() as u32 + 1;

        let coll = CollisionBox {
            shape: Rectangle { width, height },
            position: start_point,
        };

        return coll;
    }
}

impl From<Orientation> for IVec2 {
    fn from(value: Orientation) -> Self {
        match value {
            RIGHT => IVec2::X,
            LEFT => IVec2::NEG_X,
            UP => IVec2::Y,
            DOWN => IVec2::NEG_Y,
        }
    }
}

#[cfg(test)]
mod line_collision_tests {
    use super::*;

    #[test]
    fn line_collides_with_itself() {
        let line = Line::new(0, 10);

        assert_eq!(lines_intersect(&line, &line), true);
    }

    #[test]
    fn line_test_1() {
        let line_1 = Line::new(0, 10);
        let line_2 = Line::new(0, 9);

        assert_eq!(lines_intersect(&line_1, &line_2), true);
    }

    #[test]
    fn line_test_2() {
        let line_1 = Line::new(0, 10);
        let line_2 = Line::new(-1, 10);

        assert_eq!(lines_intersect(&line_1, &line_2), true);
    }

    #[test]
    fn line_test_3() {
        let line_1 = Line::new(0, 10);
        let line_2 = Line::new(1, 9);

        assert_eq!(lines_intersect(&line_1, &line_2), true);
    }

    #[test]
    fn line_test_4() {
        let line_1 = Line::new(0, 10);
        let line_2 = Line::new(2, 8);

        assert_eq!(lines_intersect(&line_1, &line_2), true);
    }

    #[test]
    fn line_test_5() {
        let line_1 = Line::new(0, 10);
        let line_2 = Line::new(2, 12);

        assert_eq!(lines_intersect(&line_1, &line_2), true);
    }

    #[test]
    fn line_test_6() {
        let line_1 = Line::new(1, 3);
        let line_2 = Line::new(-2, 0);

        assert_eq!(lines_intersect(&line_1, &line_2), false);
    }
}

#[cfg(test)]
mod collision_tests {
    use super::*;
    use crate::dungeon_generation::room::Orientation::{LEFT, RIGHT, UP};

    #[test]
    fn room_collides_with_itself() {
        let room = Room {
            shape: Rectangle {
                width: 10,
                height: 10,
            },
            position: IVec2::new(0, 0),
        };

        assert_eq!(room.collides_with(&room), true)
    }

    #[test]
    fn disjoint_rooms_dont_collide() {
        let lhs = Room {
            shape: Rectangle {
                width: 5,
                height: 5,
            },
            position: IVec2::new(0, 0),
        };

        let rhs = Room {
            shape: Rectangle {
                width: 5,
                height: 5,
            },
            position: IVec2::new(10, 10),
        };

        assert_eq!(lhs.collides_with(&rhs), false)
    }

    #[test]
    fn corridor_collides_with_itself() {
        let corridor = Corridor {
            shape: IShape {
                length: 5,
                orientation: RIGHT,
            },
            position: IVec2::new(0, 0),
        };

        assert_eq!(corridor.collides_with(&corridor), true);
    }

    #[test]
    fn barely_intersecting_parallel_corridors_dont_collide() {
        let lhs = Corridor {
            shape: IShape {
                length: 5,
                orientation: RIGHT,
            },
            position: IVec2::new(0, 0),
        };

        let rhs = Corridor {
            shape: IShape {
                length: 5,
                orientation: RIGHT,
            },
            position: IVec2::new(3, 0),
        };

        assert_eq!(lhs.collides_with(&rhs), true);
    }

    #[test]
    fn disjoint_horizontal_corridors_dont_collide() {
        let lhs = Corridor {
            shape: IShape {
                length: 5,
                orientation: RIGHT,
            },
            position: IVec2::new(0, 0),
        };

        let rhs = Corridor {
            shape: IShape {
                length: 5,
                orientation: RIGHT,
            },
            position: IVec2::new(3, 1),
        };

        assert_eq!(lhs.collides_with(&rhs), false);
        assert_eq!(rhs.collides_with(&lhs), false);
    }

    #[test]
    fn disjoint_perpendicular_corridors_dont_collide() {
        let lhs = Corridor {
            shape: IShape {
                length: 5,
                orientation: UP,
            },
            position: IVec2::new(0, 0),
        };

        let rhs = Corridor {
            shape: IShape {
                length: 5,
                orientation: RIGHT,
            },
            position: IVec2::new(3, 1),
        };

        assert_eq!(lhs.collides_with(&rhs), false);
    }

    #[test]
    fn intersecting_perpendicular_corridors_collide() {
        let lhs = Corridor {
            shape: IShape {
                length: 5,
                orientation: UP,
            },
            position: IVec2::new(0, 0),
        };

        let rhs = Corridor {
            shape: IShape {
                length: 5,
                orientation: RIGHT,
            },
            position: IVec2::new(2, -1),
        };

        assert_eq!(lhs.collides_with(&rhs), false);
    }

    #[test]
    fn corridors_collide() {
        let lhs = Corridor {
            shape: IShape {
                length: 5,
                orientation: UP,
            },
            position: IVec2::new(0, 0),
        };

        let rhs = Corridor {
            shape: IShape {
                length: 5,
                orientation: RIGHT,
            },
            position: IVec2::new(0, 1),
        };

        assert_eq!(lhs.collides_with(&rhs), true);
    }

    #[test]
    fn corridors_dont_collide_1() {
        let lhs = Corridor {
            shape: IShape {
                length: 5,
                orientation: UP,
            },
            position: IVec2::new(0, 0),
        };

        let rhs = Corridor {
            shape: IShape {
                length: 5,
                orientation: RIGHT,
            },
            position: IVec2::new(0, -1),
        };

        assert_eq!(lhs.collides_with(&rhs), true);
    }

    #[test]
    fn corridors_collide_1() {
        let lhs = Corridor {
            shape: IShape {
                length: 10,
                orientation: DOWN,
            },
            position: IVec2::new(0, 3),
        };

        let rhs = Corridor {
            shape: IShape {
                length: 10,
                orientation: LEFT,
            },
            position: IVec2::new(2, 0),
        };

        assert_eq!(lhs.collides_with(&rhs), true);
    }

    #[test]
    fn intersecting_room_and_corridor_collide() {
        let corridor = Corridor {
            shape: IShape {
                length: 5,
                orientation: RIGHT,
            },
            position: IVec2::new(2, 2),
        };

        let room = Room {
            shape: Rectangle {
                width: 10,
                height: 10,
            },
            position: IVec2::new(0, 0),
        };

        assert_eq!(corridor.collides_with(&room), true);
    }

    #[test]
    fn disjoint_room_and_corridor_dont_collide() {
        let corridor = Corridor {
            shape: IShape {
                length: 5,
                orientation: RIGHT,
            },
            position: IVec2::new(1, 20),
        };

        let room = Room {
            shape: Rectangle {
                width: 5,
                height: 5,
            },
            position: IVec2::new(0, 0),
        };

        assert_eq!(corridor.collides_with(&room), false);
    }
}
