// use ndarray::{Axis, Array, Ix2};
use rand::Rng;




// ============================================================================
pub struct Tracer
{
    pub x: f64,
    pub y: f64,
}




// ============================================================================
impl Tracer
{
    pub fn default() -> Tracer
    {
        return Tracer{x: 0.0, y: 0.0};
    }


    pub fn randomize(domain_radius: f64) -> Tracer
    {
        let mut rng = rand::thread_rng();
        let rand_x = rng.gen_range(-domain_radius, domain_radius);
        let rand_y = rng.gen_range(-domain_radius, domain_radius);
        return Tracer{x: rand_x, y: rand_y};
    }

    // fn find_cell(&self, grid) -> index {}

    // fn compute_velocity(index) -> (vx, vy) {}

    // fn update((vx, vy), dt) -> Tracer {}

}