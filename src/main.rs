use ndarray::{Array, Axis, Ix2};
use ndarray_ops::*;
use hdf5::File;
mod tracers;

static TAU: f64 = 2.0 * std::f64::consts::PI;




// ============================================================================
pub struct Grid
{
    nx: usize,
    ny: usize,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
}

impl Grid
{
    pub fn cell_centers(&self) -> Array<(f64, f64), Ix2>
    {
        let xv = Array::linspace(self.x0, self.x1, self.nx + 1);
        let yv = Array::linspace(self.y0, self.y1, self.ny + 1);
        let xc = adjacent_mean(&xv, Axis(0));
        let yc = adjacent_mean(&yv, Axis(0));
        return cartesian_product2(xc, yc);
    }

    pub fn face_centers_x(&self) -> Array<(f64, f64), Ix2>
    {
        let xv = Array::linspace(self.x0, self.x1, self.nx + 1);
        let yv = Array::linspace(self.y0, self.y1, self.ny + 1);
        let yc = adjacent_mean(&yv, Axis(0));
        return cartesian_product2(xv, yc);
    }

    pub fn face_centers_y(&self) -> Array<(f64, f64), Ix2>
    {
        let xv = Array::linspace(self.x0, self.x1, self.nx + 1);
        let yv = Array::linspace(self.y0, self.y1, self.ny + 1);
        let xc = adjacent_mean(&xv, Axis(0));
        return cartesian_product2(xc, yv);
    }

    pub fn face_center(&self, i: usize, j: usize, dir: char) -> (f64, f64)
    {
        let dx = (self.x1 - self.x0) / (self.nx + 1) as f64;
        let dy = (self.y1 - self.y0) / (self.ny + 1) as f64;
        if dir == 'x'
        {
            return ((i as f64) * dx, (j as f64 + 0.5) * dy);
        }
        if dir == 'y'
        {
            return ((i as f64 + 0.5) * dx, (j as f64) * dy);
        }
        // Should handle this case so it doesn't continue
        println!("Grid::face_center : bad direction!");
        return (0.0, 0.0);
    }

    pub fn get_cell_index(&self, x: f64, y: f64) -> (usize, usize)
    {
        let float_i = (x - self.x0) / (self.x1 - self.x0);
        let float_j = (y - self.y0) / (self.y1 - self.y0);

        let i = (float_i * (self.nx as f64)) as usize;
        let j = (float_j * (self.ny as f64)) as usize;
        
        if x > self.x1
        {
            println!("Beyond x-boudnary!: {}", i);
        }
        if y > self.y1
        {
            println!("Beyond y-boudnary!: {}", j);
        }
        return (i, j);
    }
}




// ============================================================================
pub struct Velocities
{
    pub face_vx: Array<f64, Ix2>, 
    pub face_vy: Array<f64, Ix2>,
}

impl Velocities
{
    pub fn initialize_sine(grid: &Grid) -> Velocities
    {
        return Velocities{
            face_vx: grid.face_centers_x().mapv(|xy: (f64, f64)| -> f64 {let (x, _) = xy;  x.sin()}),
            face_vy: grid.face_centers_y().mapv(|xy: (f64, f64)| -> f64 {let (_, y) = xy; -y.sin()}),            
        };
    }

    pub fn initialize_div_free(grid: &Grid) -> Velocities
    {
        return Velocities{
            face_vx: grid.face_centers_x().mapv(|xy: (f64, f64)| -> f64 {let (_, y) = xy; y.sin()}),
            face_vy: grid.face_centers_y().mapv(|xy: (f64, f64)| -> f64 {let (x, _) = xy; x.cos()}),
        }
    }

    pub fn initialize_rot_flow(grid: &Grid) -> Velocities
    {
        let omega = 1.0;
        return Velocities{
            face_vx: grid.face_centers_x().mapv(|xy: (f64, f64)| -> f64 {let (_, y) = xy; -omega * y}),
            face_vy: grid.face_centers_y().mapv(|xy: (f64, f64)| -> f64 {let (x, _) = xy;  omega * x}),
        }
    }
}





// ============================================================================
pub struct Tasks
{
    pub tracer_output_interval : usize,
    pub tracer_output_count    : usize,
}

impl Tasks
{
    pub fn write_tracers(&mut self, tracers: &Vec<tracers::Tracer>, t: &f64) -> Result<(), hdf5::Error>
    {
        let fname = format!("./chkpt.{:04}.h5", self.tracer_output_count);
        self.tracer_output_count +=1;

        println!("Writing tracers {}", fname);
        write_tracers_to_h5(&fname, tracers, t)?;
        Ok(())
    }
}




// ============================================================================
fn initial_tracer_list(domain_radius: f64, ntracers: usize) -> Vec<tracers::Tracer>
{
    return (0..ntracers).map(|i| tracers::Tracer::randomize(domain_radius, i)).collect();
}

fn initial_tasks() -> Tasks
{
    return Tasks{tracer_output_interval: 10, tracer_output_count: 0};
}




// ============================================================================
fn update(tracers: &Vec<tracers::Tracer>, grid: &Grid, vfields: &Velocities, domain_radius: f64, dt: f64) -> Vec<tracers::Tracer>
{
    return tracers.into_iter()
        .map(|t| tracers::apply_boundary_condition(&t, domain_radius))
        .map(|t| t.update(grid, vfields, dt))
        .collect();
}




// ============================================================================
fn write_tracers_to_h5(fname: &str, tracers: &Vec<tracers::Tracer>, t: &f64) -> Result<(), hdf5::Error>
{
    let file = File::create(fname)?;
    file.new_dataset::<f64>().create("t", ())?.write_scalar(t)?;
    file.new_dataset::<tracers::Tracer>().create("tracers", tracers.len())?.write(&tracers)?;
    Ok(())
}




// ============================================================================
fn run(domain_radius: f64, block_size: usize, ntracers: usize) -> Result<(), hdf5::Error> 
{
    let grid = Grid{
        nx:  block_size,
        ny:  block_size,
        x0: -domain_radius,
        x1:  domain_radius,
        y0: -domain_radius,
        y1:  domain_radius,
    };

    let tf          = 10.0;
    let dt          = 0.01;
    let vfields     = Velocities::initialize_rot_flow(&grid);
    let mut tracers = initial_tracer_list(domain_radius, ntracers);
    let mut tasks   = initial_tasks();
    let mut t = 0.0;

    while t < tf
    {
        println!("t: {:.2}", t);
        tracers = update(&tracers, &grid, &vfields, domain_radius, dt);
        tasks.write_tracers(&tracers, &t)?;
        t += dt;
    }
    Ok(())
}




// ============================================================================
fn main() 
{
    println!("Tracers Toy Model!");

    let domain_radius      = TAU;
    let block_size: usize  = 64;
    let num_tracers: usize = 100;

    run(domain_radius, block_size, num_tracers).unwrap_or_else(|e| println!("{}", e));
}
