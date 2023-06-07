use crate::geometries::Geometry;
use crate::materials::Material;

pub struct Object {
    pub geometry: Box<dyn Geometry>,
    pub material: Box<dyn Material>,
}
