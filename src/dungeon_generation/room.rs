use bevy::math::IVec2;

#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

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
    VERTICAL,
    HORIZONTAL,
}

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

        return lines_collide(&left_x_line, &right_x_line)
            && lines_collide(&left_y_line, &right_y_line);
    }

    fn to_collision_box(&self) -> CollisionBox;
}

fn lines_collide(lhs: &Line, rhs: &Line) -> bool {
    return (rhs.start <= lhs.start && lhs.start <= rhs.end)
        || (rhs.start <= lhs.end && lhs.end <= rhs.end)
        || (lhs.start < rhs.start && rhs.end < lhs.end);
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
        let dir: IVec2 = self.shape.orientation.clone().into();
        let end_point: IVec2 = self.position + (self.shape.length as i32) * dir;

        let width = (self.position.x - end_point.x).abs() as u32 + 1;
        let height = (self.position.y - end_point.y).abs() as u32 + 1;

        CollisionBox {
            shape: Rectangle { width, height },
            position: self.position,
        }
    }
}

impl From<Orientation> for IVec2 {
    fn from(value: Orientation) -> Self {
        match value {
            Orientation::HORIZONTAL => IVec2::X,
            Orientation::VERTICAL => IVec2::Y,
        }
    }
}

#[cfg(test)]
mod collision_tests {
    use super::*;
    use crate::dungeon_generation::room::Orientation::{HORIZONTAL, VERTICAL};

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
                orientation: HORIZONTAL,
            },
            position: IVec2::new(0, 0),
        };

        assert_eq!(corridor.collides_with(&corridor), true);
    }

    #[test]
    fn intersecting_parallel_corridors_collide() {
        let lhs = Corridor {
            shape: IShape {
                length: 5,
                orientation: HORIZONTAL,
            },
            position: IVec2::new(0, 0),
        };

        let rhs = Corridor {
            shape: IShape {
                length: 5,
                orientation: HORIZONTAL,
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
                orientation: HORIZONTAL,
            },
            position: IVec2::new(0, 0),
        };

        let rhs = Corridor {
            shape: IShape {
                length: 5,
                orientation: HORIZONTAL,
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
                orientation: VERTICAL,
            },
            position: IVec2::new(0, 0),
        };

        let rhs = Corridor {
            shape: IShape {
                length: 5,
                orientation: HORIZONTAL,
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
                orientation: VERTICAL,
            },
            position: IVec2::new(0, 0),
        };

        let rhs = Corridor {
            shape: IShape {
                length: 5,
                orientation: HORIZONTAL,
            },
            position: IVec2::new(2, -1),
        };

        assert_eq!(lhs.collides_with(&rhs), false);
    }

    #[test]
    fn intersecting_room_and_corridor_collide() {
        let corridor = Corridor {
            shape: IShape {
                length: 5,
                orientation: HORIZONTAL,
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
                orientation: HORIZONTAL,
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
