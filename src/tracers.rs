use ndarray::{Axis, Array, Ix2};
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


    fn update(&self, grid: Grid, dt: f64) -> Tracer
    {
        let vfields = grid.velocity_fields;

        let ix = get_cell_index(x, grid, 'X');
        let iy = get_cell_index(y, grid, 'Y');
        let vx = (vfields.face_vx[ix, iy] + vfields.face_vx[ix + 1, iy]) / 2.
        let vy = (vfields.face_vy[ix, iy] + vfields.face_vy[ix, iy + 1]) / 2.

        return Tracer{x: self.x + vx * dt, y: self.y + vy * dt};
    }

}

fn get_cell_index(x: f64, grid: Grid, dir: Direction) -> usize
{
    use Direction::{X, Y};
    match (dir)
    {
        X => //do search for x index in Grid
        Y => //do search for y index in Grid
    }

}

fn search_for_index(target: f64, array: Array<f64>) -> usize
{
    // non-functional binary search for the index of the array element left-adjacent to target
    let mut size = array.len();
    if size == 0 
    {
        return Err(0);
    }
    let mut base : usize = 0;

    while size > 1
    {
        let half = size / 2;
        let mid  = base + half;
        
        if array[mid] <= target
        {
            base = mid
        }
        size -= half;
    }
    return base;
}







