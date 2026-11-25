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
//!   `to_bits` uses) — among them `on_built::bitwise_twin`, which reads
//!   a printed spline carrier as the wall column it repeats bit for bit
//!   (`build`'s `column_match`), so a carrier identical to its column as
//!   geometry would fail to match it on a printed sign alone; and
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
//! **The recognition half is posture, and no row guards it.** A
//! promoted plane's frame (the net's normal and its own `∂u` made
//! perpendicular to it) could mint `-0.0` — its orientation negates and
//! its residual subtracts — but no input found reaches one: not the
//! corpus, not a flat patch in any axis-aligned orientation. A test
//! needs an input that yields `-0.0` without the flush, and none is
//! known, so none can go red; the flush is kept unguarded.

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
        // A conic's frame is carried with each `−0` entry folded: the
        // stored rotation within one kind, never squared again. The
        // placement is the one thing a conic stores of where it is, so
        // it is re-stated in place and an ellipse stays held as it was.
        Curve3::Circle(circle) => Curve3::Circle(circle.placed(|f| f.map_scalar(fold_signed_zero))),
        Curve3::Ellipse(ellipse) => {
            Curve3::Ellipse(ellipse.mapped(|f| f.map_scalar(fold_signed_zero)))
        }
        other => other,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use geom::Curve3;
    use geom_core::{Point3, Vec3};

    /// The stored motion's entries, as bits.
    fn bits(frame: &geom::Frame<f64>) -> Vec<u64> {
        let m = frame.motion();
        let o = m.origin();
        [o.x, o.y, o.z]
            .into_iter()
            .chain(
                [m.u().get(), m.v().get(), m.w().get()]
                    .into_iter()
                    .flat_map(|c| [c.x, c.y, c.z]),
            )
            .map(f64::to_bits)
            .collect()
    }

    /// **A rebuilt import conic keeps its stored frame, each `−0` folded**:
    /// on tilted frames a rounding off rigid (`Frame::new` of skew
    /// directions, with `−0` origin and axis entries), the circle's and
    /// the ellipse's frames come back entry for entry, each `−0` read
    /// `+0` and nothing else moved. Red under the frame re-squared from
    /// its folded directions (`Frame::new`, the merge's arm), and under
    /// the circle rebuilt through `Curve3::circle`.
    #[test]
    fn a_rebuilt_import_conic_keeps_its_stored_frame() {
        let mut moved = 0;
        for k in 0..512_u32 {
            let t = f64::from(k) * 0.0123;
            let frame = geom::Frame::new(
                Point3::new(-0.0, 1e3 * t.sin(), -0.0),
                Vec3::new(-0.0, t.cos() + 0.3, t.sin() + 1.7),
                Vec3::new(1.0 + 0.1 * t.cos(), 0.2 * t.sin(), -0.0),
            );
            let want: Vec<u64> = bits(&frame)
                .into_iter()
                .map(|b| if b == (-0.0_f64).to_bits() { 0 } else { b })
                .collect();
            let circle = super::plus_zero_curve(Curve3::Circle(geom::Circle::new(frame, 2.0)));
            let ellipse =
                super::plus_zero_curve(Curve3::Ellipse(geom::Ellipse::new(frame, 3.0, 2.0)));
            for c in [circle, ellipse] {
                let got = match &c {
                    Curve3::Circle(c) => bits(&c.frame()),
                    Curve3::Ellipse(e) => bits(&e.frame()),
                    _ => unreachable!(),
                };
                assert_eq!(got, want, "pose {k}: {c:?}");
            }
            moved += usize::from(bits(&frame) != want);
        }
        assert!(moved > 0, "some pose carries a −0 to fold");
    }
}
