//! Signed-zero hygiene: where the importer flushes `-0.0` to `+0.0`
//! (`geom_core::fold_signed_zero`), and why. Every site that states a
//! representative for a zero calls this module; the argument for doing
//! so is here, once.
//!
//! `-0.` is pervasive in the foreign dialect
//! (`DIRECTION('',(1.,0.,-0.))` on nearly every FreeCAD placement) and
//! is minted in-tree wherever an exact zero is negated or cancelled.
//! It denotes the same real number as `0.`, but it is a different f64
//! bit pattern, and bit patterns are read:
//!
//! * the importer compares floats **bitwise** in several places (its
//!   `to_bits` uses) — among them `adopt::surface_sig`, which restores
//!   writer-side key sharing, so two records identical as geometry
//!   would fail to share a key on a printed sign alone; and
//! * the writer prints `-0.0` verbatim while a re-import normalizes it
//!   back, so an unflushed mint makes the adoption pass fail to be a
//!   fixed point of the writer over that one printed sign.
//!
//! Every such reading is a reason. Stating one representative at the
//! mint answers all of them and moves no value (`-0.0 == 0.0`).
//!
//! **Where it is called, and where it is not.** On the reader's numeric
//! path, per parsed literal (`entities::as_real`), and wherever the
//! importer MINTS a coordinate rather than copying one it read (a
//! recognized frame, a periodic face's minted seam, a derived pole),
//! since that arithmetic can negate or cancel an exact zero. The
//! crate's references to `signed_zero` lead to every call site.
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
//! (`tests/corpus_fold.rs`,
//! `the_folds_divergence_is_exactly_the_reported_promotion`): without
//! the flush, `nonuniform_loft`'s two promoted planes state `u_ref` as
//! `(1.0, 0.0, -0.0)` and `(1.0, -0.0, -0.0)`, and the second re-export
//! differs from the first on exactly those tokens.

use geom_core::{Point3, Vec3};

pub(crate) use geom_core::fold_signed_zero;

/// Negative zero normalized to `+0.0`, componentwise (module docs).
pub(crate) fn plus_zero(v: Vec3<f64>) -> Vec3<f64> {
    Vec3::new(
        fold_signed_zero(v.x),
        fold_signed_zero(v.y),
        fold_signed_zero(v.z),
    )
}

/// [`plus_zero`] for a point.
pub(crate) fn plus_zero_point(p: Point3<f64>) -> Point3<f64> {
    Point3::new(
        fold_signed_zero(p.x),
        fold_signed_zero(p.y),
        fold_signed_zero(p.z),
    )
}

/// A minted carrier with its signed zeros made positive
/// (this module's docs state why): a line's, a circle's or an ellipse's
/// placed fields, anything else as it is.
pub(crate) fn plus_zero_curve(c: geom::Curve3<f64>) -> geom::Curve3<f64> {
    use geom::Curve3;
    match c {
        Curve3::Line(geom::Line { origin, dir }) => {
            Curve3::line(plus_zero_point(origin), plus_zero(dir))
        }
        Curve3::Circle(
            conic_k @ geom::Circle {
                frame:
                    geom::Frame {
                        origin: center,
                        axis,
                        u_ref,
                    },
                ..
            },
        ) => {
            let radius = conic_k.radius();
            Curve3::circle(
                plus_zero_point(center),
                plus_zero(axis),
                radius,
                plus_zero(u_ref),
            )
        }
        Curve3::Ellipse(
            ellipse @ geom::Ellipse {
                frame:
                    geom::Frame {
                        origin: center,
                        axis,
                        u_ref,
                    },
                ..
            },
        ) => {
            let (semi_u, semi_v) = ellipse.semi_axes();
            Curve3::Ellipse(geom::Ellipse::new(
                geom::Frame {
                    origin: plus_zero_point(center),
                    axis: plus_zero(axis),
                    u_ref: plus_zero(u_ref),
                },
                semi_u,
                semi_v,
            ))
        }
        other => other,
    }
}
