use glam::Vec3;

mod ray;
mod grid;

fn main() {
    // Example usage
    let ray = ray::Ray {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(1.0, 0.12, 0.6).normalize(),
    };

    let aabb = ray::AABB {
        min: Vec3::new(1.0, -1.0, -1.0),
        max: Vec3::new(3.0, 3.0, 3.0),
    };

    let intersects = aabb.intersect(&ray, 0.0, f32::INFINITY);

    println!("Ray intersects AABB: {:#?}", intersects);
}
