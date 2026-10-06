//! A sketch arc as the chord lowering mints it from a chord and a
//! quarter-tangent ([`Arc2::from_chord`]).
//!
//! At a scalar whose endpoints are wide, the carrier the lowering
//! derives carries the chord's relative width amplified by the radius.
//! That derived-carrier width is what the arc-evaluation rows
//! (`arc_eval_anchor.rs`, `review_arceval_r1_probes.rs`) are about, so
//! both build their arcs here, through the sketch layers' one spelling
//! of the lowering.
//!
//! **What this module does NOT absorb.** The two suites' `width`
//! meters, which each keeps for its own reason (see `mod.rs`), and the
//! literal carriers the f64 rows spell by hand (a quarter of the unit
//! circle needs no derivation).

use geom_brep::SketchSegment;
use geom_core::{Arc2, Point2, Real};

/// The arc from `a` to `b` with quarter-tangent `bulge`, lowered from
/// its chord.
pub(crate) fn lowered_arc<T: Real>(a: Point2<T>, b: Point2<T>, bulge: T) -> SketchSegment<T> {
    SketchSegment::Arc {
        a,
        b,
        arc: Arc2::from_chord(a, b, bulge),
    }
}
