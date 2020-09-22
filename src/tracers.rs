use rand::Rng;




// ============================================================================
#[repr(C)]
#[derive(hdf5::H5Type)]
pub struct Tracer
{
    pub id: usize,
    pub x : f64,
    pub y : f64,
}




// ============================================================================
impl Tracer
{
    pub fn default() -> Tracer
    {
        return Tracer{x: 0.0, y: 0.0, id: 0};
    }

    pub fn randomize(domain_radius: f64, id: usize) -> Tracer
    {
        let mut rng = rand::thread_rng();
        let rand_x = rng.gen_range(-domain_radius, domain_radius);
        let rand_y = rng.gen_range(-domain_radius, domain_radius);
        return Tracer{x: rand_x, y: rand_y, id: id};
    }

    pub fn update(&self, grid: &crate::Grid, vfields: &crate::Velocities, dt: f64) -> Tracer
    {
        let (ix, iy) = grid.get_cell_index(self.x, self.y);
        let dx = (grid.x1 - grid.x0) / grid.nx as f64;
        let dy = (grid.y1 - grid.y0) / grid.ny as f64;
        let wx = (self.x - grid.face_center(ix + 1, iy, 'x').0) / dx; 
        let wy = (self.y - grid.face_center(ix, iy + 1, 'y').1) / dy; 
        let vx = (1.0 - wx) * vfields.face_vx[[ix, iy]] + wx * vfields.face_vx[[ix + 1, iy]];
        let vy = (1.0 - wy) * vfields.face_vy[[ix, iy]] + wy * vfields.face_vy[[ix, iy + 1]];
        return Tracer{x : self.x + vx * dt,
                      y : self.y + vy * dt,
                      id: self.id};
    }
}




// ============================================================================
pub fn apply_boundary_condition(tracer: &Tracer, domain_radius: f64) -> Tracer
{
    let mut x = tracer.x;
    let mut y = tracer.y;

    if x >= domain_radius {
        x -= 2.0 * domain_radius;
    }
    if x < -domain_radius {
        x += 2.0 * domain_radius;
    }
    if y >= domain_radius {
        y -= 2.0 * domain_radius;
    }
    if y < -domain_radius {
        y += 2.0 * domain_radius;
    }

    Tracer{
        x: x,
        y: y,
        id: tracer.id,
    }
}




