use bevy::math::IVec2;

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

pub trait Collision {
    fn collides_with(&self, rhs: &Self) -> bool;
}

impl Collision for Room {
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
