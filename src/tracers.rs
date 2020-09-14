use rand::Rng;




// ============================================================================
pub struct Tracer
{
    pub x : f64,
    pub y : f64,
    pub id: usize,
}




// ============================================================================
impl Tracer
{
    pub fn default() -> Tracer
    {
        return Tracer{x: 0.0, y: 0.0, id: 0};
    }


    pub fn randomize(domain_radius: f64) -> Tracer
    {
        let mut rng = rand::thread_rng();
        let rand_x = rng.gen_range(-domain_radius, domain_radius);
        let rand_y = rng.gen_range(-domain_radius, domain_radius);
        return Tracer{x: rand_x, y: rand_y, id: rng.gen::<usize>()};
    }


    pub fn update(&self, grid: &crate::Grid, vfields: &crate::Velocities, dt: f64) -> Tracer
    {
        let (ix, iy) = get_cell_indexes(&(self.x, self.y), grid);
        let vx = (vfields.face_vx[[ix, iy]] + vfields.face_vx[[ix + 1, iy]]) / 2.;
        let vy = (vfields.face_vy[[ix, iy]] + vfields.face_vy[[ix, iy + 1]]) / 2.;
        return Tracer{x: boundary(self.x + vx * dt, grid.domain_radius), 
                      y: boundary(self.y + vy * dt, grid.domain_radius), 
                      id: self.id};
    }
}




// ============================================================================
fn get_cell_indexes(xy: &(f64, f64), grid: &crate::Grid) -> (usize, usize)
{
    let (x, y) = xy;
    let n      = grid.block_size;
    let r      = grid.domain_radius;
    let dr     = 2.0 * r / n as f64;

    let ix = ((x + r) / dr) as usize;
    let iy = ((y + r) / dr) as usize;
    return (ix, iy);
}


fn boundary(x: f64, domain_radius: f64) -> f64
{
    if x > domain_radius
    {
        return x - 2.0 * domain_radius;
    }
    if x < -domain_radius
    {
        return x + 2.0 * domain_radius;
    }
    return x;
}


// fn search_for_index(target: f64, array: Array<f64>) -> usize
// {
//     // non-functional binary search for the index of the array element left-adjacent to target
//     let mut size = array.len();
//     if size == 0 
//     {
//         return Err(0);
//     }
//     let mut base : usize = 0;

//     while size > 1
//     {
//         let half = size / 2;
//         let mid  = base + half;
        
//         if array[mid] <= target
//         {
//             base = mid
//         }
//         size -= half;
//     }
//     return base;
// }







