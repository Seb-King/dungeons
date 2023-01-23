use bevy::math::IVec2;
use std::ops::Mul;

#[derive(Clone, Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug)]
pub struct Room {
    pub shape: Rectangle,
    pub position: IVec2,
}

#[derive(Clone, PartialEq)]
pub enum Orientation {
    VERTICAL,
    HORIZONTAL,
}

#[derive(Clone)]
pub struct IShape {
    orientation: Orientation,
    length: u32,
}

#[derive(Clone)]
pub struct Corridor {
    shape: IShape,
    position: IVec2,
}

pub trait Collision<T> {
    fn collides_with(&self, rhs: &T) -> bool;
}

impl Collision<Room> for Room {
    fn collides_with(&self, rhs: &Self) -> bool {
        let left_coords: (i32, i32, i32, i32) = (
            self.position.x,
            self.position.x + self.shape.width as i32 - 1,
            self.position.y,
            self.position.y + self.shape.height as i32 - 1,
        );

        let right_coords: (i32, i32, i32, i32) = (
            rhs.position.x,
            rhs.position.x + rhs.shape.width as i32 - 1,
            rhs.position.y,
            rhs.position.y + rhs.shape.height as i32 - 1,
        );

        return (left_coords.0 <= right_coords.0 && left_coords.1 >= right_coords.0
            || left_coords.1 >= right_coords.1 && left_coords.0 <= right_coords.1)
            && (left_coords.2 <= right_coords.2 && left_coords.3 >= right_coords.2
                || left_coords.3 >= right_coords.3 && left_coords.2 <= right_coords.3);
    }
}

impl Collision<Corridor> for Corridor {
    fn collides_with(&self, rhs: &Self) -> bool {
        let left_dir: IVec2 = self.shape.orientation.clone().into();
        let left_end_point: IVec2 = self.position + (self.shape.length as i32) * left_dir;

        let left_coords: (i32, i32, i32, i32) = (
            self.position.x,
            left_end_point.x,
            self.position.y,
            left_end_point.y,
        );

        let right_dir: IVec2 = rhs.shape.orientation.clone().into();
        let right_end_point: IVec2 = rhs.position + (rhs.shape.length as i32) * right_dir;

        let right_coords: (i32, i32, i32, i32) = (
            rhs.position.x,
            right_end_point.x,
            rhs.position.y,
            right_end_point.y,
        );

        return (left_coords.0 <= right_coords.0 && left_coords.1 >= right_coords.0
            || left_coords.1 >= right_coords.1 && left_coords.0 <= right_coords.1)
            && (left_coords.2 <= right_coords.2 && left_coords.3 >= right_coords.2
                || left_coords.3 >= right_coords.3 && left_coords.2 <= right_coords.3);
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
}
