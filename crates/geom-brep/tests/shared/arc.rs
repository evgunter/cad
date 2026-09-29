//! A sketch arc as the profile's arc lowering mints it from a chord and
//! a bulge.
//!
//! The profile crate lowers bulge input to the canonical segment
//! through `seg::arc_carrier` — the centre at apothem
//! `L·(1 − b²)/(4b)` along the chord's left normal from its midpoint,
//! the radius `|L·(1 + b²)/(4b)|` — and `Δθ = 4·atan b`; at a certified
//! scalar the lift re-runs that derivation from the embedded endpoints,
//! so the carrier's enclosure is the chord's relative width amplified
//! by the radius. That derived-carrier width is what the arc-evaluation
//! rows (`arc_eval_anchor.rs`, `review_arceval_r1_probes.rs`) are
//! about, and both built it by hand, so the construction lives here.
//! `geom-brep` does not depend on `profile`, so the derivation is
//! restated in the lowering's operation order.
//!
//! **What this module does NOT absorb.** The two suites' `width`
//! meters, which each keeps for its own reason (see `mod.rs`), and the
//! literal carriers the f64 rows spell by hand (a quarter of the unit
//! circle needs no derivation).

use geom_brep::SketchSegment;
use geom_core::{Point2, Real, Vec2};

/// The arc from `a` to `b` with `bulge`, lowered as the profile lowers
/// it.
pub(crate) fn lowered_arc<T: Real>(a: Point2<T>, b: Point2<T>, bulge: T) -> SketchSegment<T> {
    let len = a.distance(b);
    let unit = (b - a) / len;
    let normal = Vec2::new(-unit.y, unit.x);
    let mid = a.lerp(b, T::from_f64(0.5));
    let four_bulge = T::from_f64(4.0) * bulge;
    let b2 = bulge.powi(2);
    let apothem = len * (T::one() - b2) / four_bulge;
    let signed_radius = len * (T::one() + b2) / four_bulge;
    SketchSegment::Arc {
        a,
        b,
        centre: mid + normal * apothem,
        radius: signed_radius.abs(),
        sweep: T::from_f64(4.0) * bulge.atan(),
    }
}
