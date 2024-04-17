use std::f64::consts::PI;

fn main() {
    // Define parameters
    let nu = 0.1;  // kinematic viscosity
    let l = PI;    // length of the domain (changed to PI for sine function)
    let n = 10;    // number of grid points
    let dx = l / (n - 1) as f64;  // space step
    let dt = 0.01; // time step
    let steps = 10; // number of time steps

    // Initial condition y(x, 0) = sin(x)
    let mut y = [0.0; 10];
    for i in 0..n {
        let x = i as f64 * dx;
        y[i] = x.sin();
    }
    

    // Function to compute the slope
    fn f(y: &[f64], nu: f64, dx: f64) -> [f64; 10] {
        let mut dydx = [0.0; 10];
        for i in 1..9 {
            dydx[i] = -0.5 * (y[i+1].powi(2) - y[i-1].powi(2)) / dx +
                      nu * (y[i+1] - 2.0 * y[i] + y[i-1]) / dx.powi(2);
        }
        dydx
    }

    // RK4 solver
    for step in 0..steps {
        let k1 = f(&y, nu, dx);
        let mut k2 = [0.0; 10];
        let mut k3 = [0.0; 10];
        let mut k4 = [0.0; 10];
        for i in 1..9 {
            k2[i] = y[i] + 0.5 * dt * k1[i];
            k3[i] = y[i] + 0.5 * dt * k2[i];
            k4[i] = y[i] + dt * k3[i];
        }
        for i in 1..9 {
            y[i] += (dt / 6.0) * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
            //println!("{:?}", y);
        }
        //println!("{:?}", y);
        println!("Time step {}: {:?}", step+1, y);
        println!();
    }

    // Print the final solution
    println!("Final solution:");
    println!("{:?}", y);
}