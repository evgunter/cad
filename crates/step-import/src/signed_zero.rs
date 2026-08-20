//! Signed-zero hygiene: the importer's **one** flush of `-0.0` to
//! `+0.0`. Every site that states a representative for a zero calls
//! this module; the argument for doing so is here, once.
//!
//! `-0.` is pervasive in the foreign dialect
//! (`DIRECTION('',(1.,0.,-0.))` on nearly every FreeCAD placement) and
//! is minted in-tree wherever an exact zero is negated or cancelled.
//! It denotes the same real number as `0.`, but it is a different f64
//! bit pattern, and this importer reads bit patterns twice:
//!
//! * it compares surface records **bitwise** to restore writer-side
//!   key sharing (`adopt::surface_sig`), so two records identical as
//!   geometry would fail to share a key on a printed sign alone; and
//! * the writer prints `-0.0` verbatim while a re-import normalizes it
//!   back, so an unflushed mint makes the adoption pass fail to be a
//!   fixed point of the writer over that one printed sign.
//!
//! Stating one representative at the mint closes both and moves no
//! value (`-0.0 == 0.0`).
//!
//! **Where it is called, and where it is not.** The reader's numeric
//! path, per parsed literal (`entities::as_real`); the periodic-face
//! normalizations, whose half turns negate exact zeros
//! (`normalize::half_turn` and the frames it re-mints); and every frame
//! RECOGNITION derives — `center`/`axis`/`u_ref`/`origin` on a promoted
//! analytic surface or curve, which are minted, not read.
//!
//! `Resolver::placement` and `Resolver::direction` DO arithmetic —
//! `candidate - axis * along`, then `perpendicular / norm` and
//! `ratios / norm` — and still do not call it, for a stronger reason
//! than "they only copy". Under round-to-nearest a difference of zeros
//! is `-0.0` only when the minuend is `-0.0` and the subtrahend `+0.0`;
//! the minuend here is a component the reader already flushed, so it
//! never is. The `axis * along` product CAN be `-0.0` (an exact zero
//! times a negative), but it appears as the SUBTRAHEND, and
//! `+0.0 − (−0.0)` is `+0.0`. Division by a strictly positive norm
//! carries the sign through unchanged. So no sign is minted there.
//!
//! That is a property of those three operations, not a licence for
//! those functions: a negation, a cross product, or a multiply whose
//! result is kept added to either resolver mints `-0.0` from an exact
//! zero and owes the flush.
//!
//! The recognition half is pinned by the promoted one-cycle fixed point
//! (`tests/corpus_fold.rs:130`): without the flush, `nonuniform_loft`'s
//! two promoted planes state `u_ref` as `(1.0, 0.0, -0.0)` and
//! `(1.0, -0.0, -0.0)`, and the second re-export differs from the
//! first on exactly those tokens.

use geom_core::{Point3, Vec3};

/// Negative zero normalized to `+0.0`, componentwise (module docs).
pub(crate) fn plus_zero(v: Vec3<f64>) -> Vec3<f64> {
    Vec3::new(
        plus_zero_scalar(v.x),
        plus_zero_scalar(v.y),
        plus_zero_scalar(v.z),
    )
}

/// [`plus_zero`] for a point.
pub(crate) fn plus_zero_point(p: Point3<f64>) -> Point3<f64> {
    Point3::new(
        plus_zero_scalar(p.x),
        plus_zero_scalar(p.y),
        plus_zero_scalar(p.z),
    )
}

/// [`plus_zero`] on one component — the predicate itself, which the
/// reader's numeric path calls per parsed literal. `NaN` is not a zero
/// and gets no representative here; one reaching this door is an
/// upstream defect, not a value this module normalizes.
pub(crate) fn plus_zero_scalar(x: f64) -> f64 {
    if x == 0.0 { 0.0 } else { x }
}
