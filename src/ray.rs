use glam::Vec3;

// Define the Ray struct
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

// Define the AABB struct
#[derive(Debug, Copy, Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    // Method to check if the ray intersects with the AABB
    pub fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<f32> {
        let mut t_min = t_min;
        let mut t_max = t_max;

        for i in 0..3 {
            let inv_d = 1.0 / ray.direction[i];
            let mut t0 = (self.min[i] - ray.origin[i]) * inv_d;
            let mut t1 = (self.max[i] - ray.origin[i]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            if t_max <= t_min {
                return None;
            }
        }

        Some(t_min)
    }
}
