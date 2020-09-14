
use ndarray::{Array, Axis, Ix2};
use ndarray_ops::*;
mod tracers;

static TAU: f64 = 2.0 * std::f64::consts::PI;




// ============================================================================
pub struct Grid
{
    pub cell_centers    : Array<(f64, f64), Ix2>,
    pub face_centers_x  : Array<(f64, f64), Ix2>,
    pub face_centers_y  : Array<(f64, f64), Ix2>,
    pub velocity_fields : Velocities,
}

pub struct Velocities
{
    pub face_vx: Array<f64, Ix2>, 
    pub face_vy: Array<f64, Ix2>,
}

impl Velocities
{
    fn initialize_sine(grid: Grid) -> Velocities
    {
        return Velocities{
            face_vx: grid.face_centers_x.mapv(|xy: (f64, f64)| -> f64 {let (x, _) = xy; x.sin()}),
            face_vy: grid.face_centers_y.mapv(|xy: (f64, f64)| -> f64 {let (_, y) = xy; y.sin()}),            
        };
    }
}




// ============================================================================
pub fn cell_centers(domain_radius: f64, block_size: usize) -> Array<(f64, f64), Ix2>
{
    let xv = Array::linspace(-domain_radius, domain_radius, block_size + 1);
    let yv = Array::linspace(-domain_radius, domain_radius, block_size + 1);
    let xc = adjacent_mean(&xv, Axis(0));
    let yc = adjacent_mean(&yv, Axis(0));
    return cartesian_product2(xc, yc);
}

pub fn face_centers_x(domain_radius: f64, block_size: usize) -> Array<(f64, f64), Ix2>
{
    let xv = Array::linspace(-domain_radius, domain_radius, block_size + 1);
    let yv = Array::linspace(-domain_radius, domain_radius, block_size + 1);
    let yc = adjacent_mean(&yv, Axis(0));
    return cartesian_product2(xv, yc);
}

pub fn face_centers_y(domain_radius: f64, block_size: usize) -> Array<(f64, f64), Ix2>
{
    let xv = Array::linspace(-domain_radius, domain_radius, block_size + 1);
    let yv = Array::linspace(-domain_radius, domain_radius, block_size + 1);
    let xc = adjacent_mean(&xv, Axis(0));
    return cartesian_product2(xc, yv);
}

pub fn init_tracer_list(domain_radius: f64, ntracers: usize) -> Vec<tracers::Tracer>
{
    return (0..ntracers).map(|_| tracers::Tracer::randomize(domain_radius)).collect();
}




// ============================================================================
fn update(tracers: Vec<tracers::Tracer>, grid: Grid, dt: f64) -> Vec<tracers::Tracer>
{
    return tracers.map(|t| t.update(grid, dt));
}




// ============================================================================
fn run(domain_radius: f64, block_size: usize, ntracers: usize) -> () 
{
    let grid = Grid{
        cell_centers  : cell_centers  (domain_radius, block_size),
        face_centers_x: face_centers_x(domain_radius, block_size),
        face_centers_y: face_centers_y(domain_radius, block_size),
        vfields       : Velocities::initialize_sine(grid),
    };

    let tracers = init_tracer_list(domain_radius, ntracers);

    let dt = 0.01;
    let tf = 1.0;

    new_tracers = update(tracers, Grid, dt);
}




// ============================================================================
fn main() 
{
    println!("Hello, world!");

    let domain_radius      = TAU;
    let block_size: usize  = 64;
    let num_tracers: usize = 100;
    run(domain_radius, block_size, num_tracers);
}
