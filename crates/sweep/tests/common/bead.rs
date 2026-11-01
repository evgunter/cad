//! **The drilled bead**: the lens a bore line `ρ = r` and the arc of the
//! radius-`R` circle through `ρ = R` enclose, revolved about the
//! sketch's `y` axis. A full turn gives two faces, each the whole turn
//! of its carrier — a bore cylinder with a self-mated seam and a sphere
//! zone — bounded by the two rims they share. It is the smallest valid
//! body whose faces wrap the azimuth alone, so the full-turn rows of
//! two suites (`full_turn_wall`, `verbs_sphsph_chart`) build it here
//! (body authoring, the module's routing rule).
//!
//! What this module deliberately did NOT absorb, as the whole list:
//!
//! - `full_turn_wall`'s `Pose`: the frames the oracle row sweeps are
//!   that suite's question, not the body's.

use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::Body;

/// The sphere radius `big_r`, bore radius `r` bead in `plane`, swept
/// through `turn` about the sketch's `y` axis.
pub fn bead(plane: SketchPlane<f64>, big_r: f64, r: f64, turn: Revolution<f64>) -> Body<f64> {
    let h = (big_r * big_r - r * r).sqrt();
    // The arc from (r, −h) to (r, h) through ρ = R turns CCW about the
    // origin by 2φ, φ = atan(h/r): bulge tan(φ/2).
    let phi = (h / r).atan();
    let lp = bulge_loop(vec![
        (Point2::new(r, -h), (phi / 2.0).tan()),
        (Point2::new(r, h), 0.0),
    ]);
    let vp = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&vp, axis, turn, Tol::witness()).unwrap().body
}
