use crate::point::Point;
use crate::vector::Vector;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}
