//! The deterministic `(u, v)` schedule the offset probes sample on,
//! and the residual they measure on it.

use geom::NurbsSurface;
use geom_brep::offset_fit::offset_point;

/// `nu` x `nv` stations spanning `[0,1]²`, both endpoints included, in
/// u-major order.
///
/// **The counts are the caller's, and they carry an argument.** A
/// schedule is only independent evidence about a fit if its stations do
/// not land on the fit's own cell grid, and that is a property of the
/// numbers passed here, not of this function: `23 x 19` and `41 x 37`
/// are coprime to the cell counts the fitter picks, while `41 x 41`
/// (40 divisions) lands on them. Denser is not stronger for a
/// supremum — landing off the grid is.
pub(crate) fn grid(nu: usize, nv: usize) -> Vec<(f64, f64)> {
    let mut out = Vec::with_capacity(nu * nv);
    for i in 0..nu {
        for j in 0..nv {
            #[allow(clippy::cast_precision_loss)]
            out.push((i as f64 / (nu - 1) as f64, j as f64 / (nv - 1) as f64));
        }
    }
    out
}

/// The largest distance from `candidate`'s surface to the EXACT offset
/// locus of `base` at `d`, over `stations` — the independent truth a
/// certified `hull_sup` has to sit above.
///
/// `Err((u, v))` at the first station where `base` has no offset point
/// (a normal that does not exist there), naming it. **The callers do
/// not agree on what that means and are not made to**, so the three
/// policies in this crate all sit at their own call site, in one line
/// each:
///
/// - `.unwrap()` — the carrier is regular by construction, so a missing
///   normal is a broken premise (`offb_r1_probes.rs`,
///   `cert7_r2_probes.rs`, and the shipped rows in `offset_fit.rs`);
/// - `.unwrap_or(f64::INFINITY)` — the row hands the door hostile
///   rationals on purpose, so the containment assertion above must be
///   what fails, not a panic one frame under it (`cert7_r1_probes.rs`);
/// - `.unwrap_or_else(|(u, v)| panic!(...))` — same broken premise, but
///   the row drives several named carriers through one helper and the
///   message has to say WHICH, and where (`offb_r2_probes.rs`).
///
/// The station travels in the error for the sake of that third one: a
/// panic that cannot say where it happened is a worse message than the
/// loop it replaced.
pub(crate) fn worst_offset_residual(
    base: &NurbsSurface<f64>,
    candidate: &NurbsSurface<f64>,
    d: f64,
    stations: &[(f64, f64)],
) -> Result<f64, (f64, f64)> {
    let mut worst = 0.0f64;
    for &(u, v) in stations {
        let target = offset_point(base, d, u, v).ok_or((u, v))?;
        worst = worst.max((candidate.eval(u, v) - target).norm());
    }
    Ok(worst)
}
