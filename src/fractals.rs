use crate::point::Point;
use crate::sdf::SDF;

pub struct SDFMengerSponge {
    pub iterations: u8,
}

// See https://iquilezles.org/articles/menger/
impl SDF for SDFMengerSponge {
    fn distance(&self, point: Point) -> f64 {
        let p = point.into_vector();

        // Start with cube
        let q = p.abs() - 0.5;
        let mut d = q.max(0.).norm().min(q.maxcomp());

        // Remove crosses
        let mut s = 1.;
        for _ in 0..self.iterations {
            // Transform point into relevant subcube
            let a = (2. * p * s).rem_euclid(2.) - 1.;
            let r = (1. - 3. * a.abs()).abs();

            // Smaller grid next iteration
            s *= 3.;

            // Compute distance to cutout cross
            let da = r.x.max(r.y);
            let db = r.y.max(r.z);
            let dc = r.z.max(r.x);
            let c = (1. - da.min(db).min(dc)) / (2. * s);

            // Subtract cross
            d = d.max(-c)
        }

        return d;
    }
}
