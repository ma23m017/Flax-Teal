fn main() {
    // Define the function representing the ODE: y' = 1 - t^2 + y
    fn f(t: f64, y: f64) -> f64 {
        1.0 - t.powi(2) + y
    }

    // Define the Fourth-Order Runge-Kutta (RK4) method
    fn rk4(t: f64, y: f64, h: f64, f: fn(f64, f64) -> f64) -> f64 {
        let k1 = h * f(t, y); // Calculate slope at the start of the interval
        let k2 = h * f(t + h / 2.0, y + k1 / 2.0); // Calculate slope at the midpoint of the interval
        let k3 = h * f(t + h / 2.0, y + k2 / 2.0); // Calculate slope at another midpoint
        let k4 = h * f(t + h, y + k3); // Calculate slope at the end of the interval
        y + (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0 // Weighted average of slopes to find new y
    }

    // Define the initial condition and range of t
    let y0 = 0.5; // Initial value of y
    let t0 = 0.0; // Initial value of t
    let tn = 2.0; // Final value of t
    let n = 10; // Number of steps
    let h = (tn - t0) / n as f64; // Step size

    // Perform RK4 iterations to obtain the numerical solution
    let mut t = t0; // Initialize t
    let mut y = y0; // Initialize y
    for _ in 0..n {
        y = rk4(t, y, h, f); // Compute new y using RK4 method
        t += h; // Move to the next time step
        println!("t = {:.2}, y = {:.6}", t, y); // Print current values of t and y
    }
}
