//! Algebra-authored profile helpers (LIB-U2 PR-2): the tour's polygon
//! loops said through the PATHS algebra (`pncad::prelude::{Open,
//! Start}`) instead of the raw `ProfileLoop::polygon` constructor.
//!
//! The lowering emits every authored point verbatim (bulge 0, no
//! declared joints) — the exact `ProfileLoop` the raw constructor
//! produced — so this changes how the loops are SAID, not what they
//! are. What it adds is the algebra's authoring-time junction
//! classification: a polygon with a within-band-tangent (or reverse/
//! cusp) corner now refuses at authoring, before validate ever sees
//! it. The tour's polygons are all definitely-sharp, so the chains
//! run clean at every supported ε row (see
//! `tests/eps_regression.rs`).

use pncad::prelude::{Open, Point2, ProfileLoop, Start};

use crate::scalar::Scalar;
use pncad::geom_core::Tol;

/// A closed polygon authored through the algebra: `Open.at(p0)`, then
/// `line_to` each subsequent vertex, `line_to(Start)` as the sharp
/// seam (the seam's two junction checks run with both directions
/// known). Mirrors `pncad::authoring::polygon`'s `(f64, f64)` slice
/// signature; panics on refusal — fail-loud demo style, and every
/// caller's corners are definitely-sharp by construction.
pub fn path_polygon<S: Scalar>(pts: &[(f64, f64)], tol: Tol) -> ProfileLoop<S> {
    let p = |&(x, y): &(f64, f64)| Point2::new(S::from_f64(x), S::from_f64(y));
    let (first, rest) = pts.split_first().expect("polygon needs vertices");
    let (second, rest) = rest.split_first().expect("polygon needs >= 3 vertices");
    let mut tip = Open
        .at(p(first))
        .line_to(p(second), tol)
        .expect("polygon leg refused");
    for q in rest {
        tip = tip.line_to(p(q), tol).expect("polygon leg refused");
    }
    tip.line_to(Start, tol)
        .expect("polygon seam refused")
        .into()
}
