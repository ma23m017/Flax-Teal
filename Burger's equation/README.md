## Description

This Rust program simulates the one-dimensional advection-diffusion equation using the Runge-Kutta 4th order (RK4) method. The equation is given by:

\[
\frac{\partial y}{\partial t} = -\frac{1}{2}\left(y_{i+1}^2 - y_{i-1}^2\right) + \nu\left(y_{i+1} - 2y_i + y_{i-1}\right)
\]

where:
- \(y(x, t)\) is the dependent variable representing the state of the system,
- \(t\) is time,
- \(x\) is the spatial coordinate,
- \(\nu\) is the kinematic viscosity,
- \(l\) is the length of the domain,
- \(n\) is the number of grid points,
- \(dx\) is the space step,
- \(dt\) is the time step, and
- \(steps\) is the number of time steps.

The initial condition \(y(x, 0) = \sin(x)\) is given, and the RK4 method is used to solve the equation numerically.

