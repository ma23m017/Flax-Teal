fn main() {
    // Define the function representing the ODE: y' = 1 - t^2 + y
    fn f(t: f64, y: f64) -> f64 {
        1.0 - t.powi(2) + y
    }

    // Define the Fourth-Order Runge-Kutta (RK4) method
    fn rk4(t: f64, y: f64, h: f64, f: fn(f64, f64) -> f64) -> f64 {
        let k1 = h * f(t, y);
        let k2 = h * f(t + h / 2.0, y + k1 / 2.0);
        let k3 = h * f(t + h / 2.0, y + k2 / 2.0);
        let k4 = h * f(t + h, y + k3);
        y + (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0
    }

    // Define the initial condition and range of t
    let y0 = 0.5;
    let t0 = 0.0;
    let tn = 2.0;
    let n = 10;
    let h = (tn - t0) / n as f64;

    // Perform RK4 iterations to obtain the numerical solution
    let mut t = t0;
    let mut y = y0;
    for _ in 0..n {
        y = rk4(t, y, h, f);
        t += h;
        println!("t = {:.2}, y = {:.6}", t, y);
    }
}
