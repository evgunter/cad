//! Closed forms more than one scene checks its bodies against, spelled
//! once.

use core::f64::consts::PI;

/// The volume of an `a × b × c` box with every edge rounded at radius
/// `r` (every corner a sphere octant) — the Steiner form of the core box
/// `a − 2r × b − 2r × c − 2r` grown by `r`: core, plus a slab of
/// thickness `r` on each face, plus twelve quarter-cylinders along the
/// edges, plus eight octants that sum to one ball.
pub fn rounded_box_volume([a, b, c]: [f64; 3], r: f64) -> f64 {
    let [x, y, z] = [a - 2.0 * r, b - 2.0 * r, c - 2.0 * r];
    x * y * z
        + 2.0 * r * (x * y + y * z + z * x)
        + PI * r * r * (x + y + z)
        + (4.0 / 3.0) * PI * r.powi(3)
}
