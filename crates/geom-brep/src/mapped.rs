//! The **sketch pushforward**: one authoritative sketch entity and the
//! map that carries it into 3-space ([`MappedCurve`]).
//!
//! A pushforward is not a class of locus. It is a statement about who
//! DETERMINED the locus — a modeler's sketch entity under a sweep map,
//! never two peer representations — and U2
//! (`docs/PCURVE-UNIFY-DESIGN.md`) puts it where that statement
//! belongs: it is the payload of [`crate::EdgeAuthority::Declared`],
//! the per-edge record tier 3's prefer-intrinsic rules read, and the
//! payload of [`crate::EdgeDescription::Scaffold`], the fenced door
//! through which an edge whose surfaces do not exist yet is described
//! at all. It is no longer a description arm competing with a chart
//! image: `Seam`, `IsoCurve` and `MappedCurve` collapsed into
//! [`crate::ChartCurve`], and what survives here is the sketch data
//! and its evaluation.
//!
//! # The payload
//!
//! The source/map pairs are combined per variant so that incoherent
//! pairings (a 1-D source under a 1-parameter motion, which would
//! describe a surface, not a curve) are unrepresentable:
//!
//! - [`MappedCurve::PlacedSegment`] — a sketch-plane segment under a
//!   rigid placement: profile rim edges of caps, revolve meridians (the
//!   placement pre-composes any rotation/translation of the sweep).
//! - [`MappedCurve::ExtrudedPoint`] — a sketch point's trajectory under
//!   a translation family: extrude side struts (lines).
//! - [`MappedCurve::RevolvedPoint`] — a sketch point's trajectory under
//!   a rotation family: revolve latitude arcs (circles).
//!
//! Sketch segments use the ratified zero-redundancy bulge form
//! (endpoints + bulge = tan(θ/4), the `profile` crate's convention,
//! restated here without a dependency on that crate — the sweep maps
//! `profile`'s validated segments into this form field-for-field). The
//! **line/arc split is structural** ([`SketchSegment`]), mirroring the
//! upstream trilean classification: by the time a description exists,
//! straightness was already *decided* (profile validation), so
//! evaluation here never re-decides it (no value branch, and the arc
//! closed forms may divide by the definitely-nonzero bulge).
//!
//! # Natural parameterization (the certification contract)
//!
//! Every description evaluates over the **normalized parameter
//! s ∈ [0, 1]** ([`MappedCurve::eval`]), affinely aligned with the
//! cached carrier's parameter interval: sample i of the certification
//! schedule compares `carrier(t₀ + (t₁ − t₀)·s)` against
//! `description(s)`. The sweep constructs both sides, so the alignment
//! is a construction invariant — and certification is exactly what
//! makes it checked rather than trusted.

use geom_core::{Affine3, Point2, Point3, Real, Vec2, Vec3};

/// A 2-D sketch-plane segment in the zero-redundancy bulge form (module
/// docs). The line/arc split is structural — decided upstream, never
/// re-decided here.
#[derive(Clone, Copy, Debug)]
pub enum SketchSegment<T: Real> {
    /// The straight chord from `a` to `b`; `s` sweeps it affinely.
    Line {
        /// Start point (s = 0), sketch-plane meters.
        a: Point2<T>,
        /// End point (s = 1).
        b: Point2<T>,
    },
    /// The circular arc from `a` to `b` with bulge = tan(θ/4)
    /// (DXF-compatible; positive sweeps counterclockwise — the
    /// `profile` crate's ratified semantics). The bulge is definitely
    /// nonzero by upstream classification; evaluation divides by it.
    Arc {
        /// Start point (s = 0).
        a: Point2<T>,
        /// End point (s = 1).
        b: Point2<T>,
        /// tan(θ/4) of the signed included angle θ ∈ (−2π, 2π) \ {0}.
        bulge: T,
    },
}

impl<T: Real> SketchSegment<T> {
    /// The sub-segment covering `[s0, s1]` of this segment,
    /// reparameterized to `[0, 1]` (M3 PR 1, for `split_edge`):
    /// endpoints by [`SketchSegment::eval`]; an arc's bulge becomes
    /// `tan(atan(bulge)·(s1 − s0))` — the sub-arc's `tan(θ′/4)` with
    /// `θ′ = θ·(s1 − s0)` (finite and nonzero for `0 ≤ s0 < s1 ≤ 1`,
    /// since `|θ′/4| < π/2` and `θ ≠ 0`). Fixed evaluation order (D9);
    /// total — degenerate inputs yield degenerate data, caught by the
    /// caller's certification.
    ///
    /// **Coverage**: the arc lane runs end-to-end. Curved booleans and
    /// the fillet verbs split revolve meridians mid-operation, before
    /// the prefer-intrinsic pass can re-describe them, so a boolean
    /// crossing insertion or a meridian split restricts a
    /// `MappedCurve` over an `Arc` and re-certifies the result against
    /// `carrier_matches_mapped_source`. Whole-body rows in `sweep`
    /// exercise that path at both scalars; the formula's own unit
    /// tests are the narrow check, not the only one.
    ///
    /// Each restriction re-derives the endpoints through
    /// [`SketchSegment::eval`], so at `T = Interval` the sub-arc's
    /// stored endpoints inherit that evaluation's enclosure width and
    /// successive splits compound it — see `eval`'s anchoring note for
    /// why the evaluation is written to keep that width at the
    /// endpoints' own scale.
    pub fn restrict(&self, s0: T, s1: T) -> Self {
        match *self {
            SketchSegment::Line { .. } => SketchSegment::Line {
                a: self.eval(s0),
                b: self.eval(s1),
            },
            SketchSegment::Arc { bulge, .. } => SketchSegment::Arc {
                a: self.eval(s0),
                b: self.eval(s1),
                bulge: (bulge.atan() * (s1 - s0)).tan(),
            },
        }
    }

    /// The point at normalized parameter `s ∈ [0, 1]` (module docs).
    ///
    /// Line: `lerp(a, b, s)`. Arc: rotate `a` about the bulge-derived
    /// center by `s·θ`, θ = 4·atan(bulge) (the closed forms of the
    /// profile conventions: center = midpoint + n̂·(L·(1 − b²)/(4b)),
    /// n̂ the left normal of the chord direction). Fixed orders as
    /// written (D9); total — degenerate data (coincident endpoints)
    /// yields poison values, caught by certification.
    ///
    /// **The rotation is anchored on `a`, not on the center**: the
    /// evaluated form is `a + (R − I)·v` (v = a − center, R the
    /// rotation by s·θ), which is the identity `center + R·v` over the
    /// reals — the same locus, the same closed forms — but does not
    /// mention `center` outside a factor that vanishes with the
    /// rotation. `cos − 1` is spelled `−2·sin²(s·θ/2)` so it carries no
    /// cancellation of its own. The center-anchored form adds and
    /// subtracts `center`, and at `T = Interval` that cancellation does
    /// not happen: the enclosure pays `width(center)` twice, and
    /// `width(center)` itself carries the chord's relative width
    /// amplified by the radius — a factor ∝ 1/sin(θ/2), unbounded for
    /// short sub-arcs, which [`SketchSegment::restrict`] then stores
    /// back into the endpoints so successive splits compound it. The
    /// anchored form is exactly `width(a)` wide at s = 0 (R − I is
    /// identically zero there), never wider than the center-anchored
    /// form at s = 0, and tighter wherever `|s·θ|` is small, because
    /// `|R − I| = 2·|sin(s·θ/2)|` scales the center's width down
    /// instead of doubling it.
    pub fn eval(&self, s: T) -> Point2<T> {
        match *self {
            SketchSegment::Line { a, b } => a.lerp(b, s),
            SketchSegment::Arc { a, b, bulge } => {
                let half = T::from_f64(0.5);
                let two = T::from_f64(2.0);
                let four = T::from_f64(4.0);
                let chord = b - a;
                let len = chord.norm();
                let unit = chord / len;
                let n = Vec2::new(-unit.y, unit.x); // left normal
                let mid = a.lerp(b, half);
                let apothem = len * (T::one() - bulge.powi(2)) / (four * bulge);
                let center = mid + n * apothem;
                let theta = four * bulge.atan();
                let sin = (s * theta).sin();
                // cos(s·θ) − 1, in the half-angle form that is exact at
                // s = 0 and free of the 1 − cos cancellation.
                let cos_m1 = -(two * (s * theta * half).sin().powi(2));
                let v = a - center;
                a + Vec2::new(v.x * cos_m1 - v.y * sin, v.x * sin + v.y * cos_m1)
            }
        }
    }
}

/// The sketch pushforward (module docs): one authoritative sketch source plus the map that carries it
/// into 3-space, combined per variant so incoherent pairings are
/// unrepresentable.
///
/// Placements map sketch coordinates `(x, y)` to
/// `place · (x, y, 0)`; rigidity of the placement (orthonormal linear
/// part) is conventional data, unchecked — exactly the `profile`
/// crate's `SketchPlane` posture.
#[derive(Clone, Copy, Debug)]
pub enum MappedCurve<T: Real> {
    /// A sketch segment under a rigid placement — cap rims, revolve
    /// meridians (any sweep rotation/translation is pre-composed into
    /// `place`).
    PlacedSegment {
        /// The authoritative sketch-plane source.
        segment: SketchSegment<T>,
        /// The rigid placement of the sketch plane in 3-space.
        place: Affine3<T>,
    },
    /// A sketch point's trajectory under a translation family — extrude
    /// side struts: `s ↦ place(point) + vec·s`.
    ExtrudedPoint {
        /// The authoritative sketch point.
        point: Point2<T>,
        /// The rigid placement of the sketch plane in 3-space.
        place: Affine3<T>,
        /// The **full** extrusion vector (meters): s = 1 lands at the
        /// far end — no separate length, no unit convention.
        vec: Vec3<T>,
    },
    /// A sketch point's trajectory under a rotation family — revolve
    /// latitude arcs: `s ↦ rotate(place(point))` about the axis by
    /// `s·angle`.
    RevolvedPoint {
        /// The authoritative sketch point.
        point: Point2<T>,
        /// The rigid placement of the sketch plane in 3-space.
        place: Affine3<T>,
        /// A point on the revolution axis.
        axis_origin: Point3<T>,
        /// The axis direction (normalized internally by the rotation —
        /// `Affine3::rotation_about_axis`'s documented posture).
        axis_dir: Vec3<T>,
        /// The **full** signed revolve angle (radians, right-hand rule
        /// about `axis_dir`): s = 1 lands at the far end.
        angle: T,
    },
}

impl<T: Real> MappedCurve<T> {
    /// The sub-curve covering `[s0, s1]`, reparameterized to `[0, 1]`
    /// (M3 PR 1, for `split_edge`): the restricted description is
    /// again a pushforward of the same shape — the authoritative
    /// source is restricted ([`SketchSegment::restrict`]) or the map's
    /// start is advanced by composing the `s0` motion into `place`
    /// (translation by `vec·s0` / rotation by `s0·angle`) with the
    /// remaining sweep scaled to `s1 − s0`. In exact arithmetic
    /// `restrict(s0, s1).eval(s) = eval(s0 + (s1 − s0)·s)`; the float
    /// discrepancy is metered by the caller's re-certification of the
    /// restricted spec (D4 ¶2 — nothing is trusted untested). Fixed
    /// evaluation orders as written (D9).
    pub fn restrict(&self, s0: T, s1: T) -> Self {
        match *self {
            MappedCurve::PlacedSegment { segment, place } => MappedCurve::PlacedSegment {
                segment: segment.restrict(s0, s1),
                place,
            },
            MappedCurve::ExtrudedPoint { point, place, vec } => MappedCurve::ExtrudedPoint {
                point,
                place: Affine3::translation(vec * s0) * place,
                vec: vec * (s1 - s0),
            },
            MappedCurve::RevolvedPoint {
                point,
                place,
                axis_origin,
                axis_dir,
                angle,
            } => MappedCurve::RevolvedPoint {
                point,
                place: Affine3::rotation_about_axis(axis_origin, axis_dir, s0 * angle) * place,
                axis_origin,
                axis_dir,
                angle: (s1 - s0) * angle,
            },
        }
    }

    /// The described point at normalized parameter `s ∈ [0, 1]` — the
    /// authoritative locus the cached carrier is certified against
    /// (module docs). Total; fixed evaluation orders as documented per
    /// variant.
    pub fn eval(&self, s: T) -> Point3<T> {
        match *self {
            MappedCurve::PlacedSegment { segment, place } => place_point(place, segment.eval(s)),
            MappedCurve::ExtrudedPoint { point, place, vec } => place_point(place, point) + vec * s,
            MappedCurve::RevolvedPoint {
                point,
                place,
                axis_origin,
                axis_dir,
                angle,
            } => {
                let p = place_point(place, point);
                Affine3::rotation_about_axis(axis_origin, axis_dir, s * angle).transform_point(p)
            }
        }
    }
}

/// A sketch-plane point under a placement: `place · (x, y, 0)`.
fn place_point<T: Real>(place: Affine3<T>, p: Point2<T>) -> Point3<T> {
    place.transform_point(Point3::new(p.x, p.y, T::zero()))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use core::f64::consts::{FRAC_PI_2, PI};

    use super::*;

    #[test]
    fn line_segment_sweeps_the_chord() {
        let seg = SketchSegment::Line {
            a: Point2::new(1.0, 2.0),
            b: Point2::new(3.0, -2.0),
        };
        let p0 = seg.eval(0.0);
        let p1 = seg.eval(1.0);
        assert_eq!((p0.x, p0.y), (1.0, 2.0));
        assert_eq!((p1.x, p1.y), (3.0, -2.0));
        let pm = seg.eval(0.5);
        assert_eq!((pm.x, pm.y), (2.0, 0.0));
    }

    #[test]
    fn arc_segment_matches_bulge_closed_forms() {
        // Quarter circle from (1,0) to (0,1) on the unit circle,
        // counterclockwise: θ = π/2, bulge = tan(π/8).
        let bulge = (PI / 8.0).tan();
        let seg = SketchSegment::Arc {
            a: Point2::new(1.0, 0.0),
            b: Point2::new(0.0, 1.0),
            bulge,
        };
        // Endpoints reproduce exactly-ish.
        let p0 = seg.eval(0.0);
        assert!((p0.x - 1.0).abs() < 1e-15 && p0.y.abs() < 1e-15);
        let p1 = seg.eval(1.0);
        assert!(p1.x.abs() < 1e-12 && (p1.y - 1.0).abs() < 1e-12);
        // Midpoint is the arc apex at 45°.
        let pm = seg.eval(0.5);
        let r = (FRAC_PI_2 / 2.0).cos(); // cos(π/4)
        assert!((pm.x - r).abs() < 1e-12 && (pm.y - r).abs() < 1e-12);
        // Every sample lies on the unit carrier circle.
        for i in 0..=8 {
            let p = seg.eval(f64::from(i) / 8.0);
            assert!((p.x * p.x + p.y * p.y - 1.0).abs() < 1e-12);
        }
        // Negative bulge mirrors: the clockwise arc's carrier is the
        // unit circle centered at (1, 1), and its apex bows toward the
        // origin (an arc bows away from its center — the profile
        // crate's ratified sign semantics).
        let neg = SketchSegment::Arc {
            a: Point2::new(1.0, 0.0),
            b: Point2::new(0.0, 1.0),
            bulge: -bulge,
        };
        let pm_neg = neg.eval(0.5);
        let d_center = pm_neg.distance(Point2::new(1.0, 1.0));
        assert!((d_center - 1.0).abs() < 1e-12);
        assert!(pm_neg.x * pm_neg.x + pm_neg.y * pm_neg.y < 1.0);
        // Endpoints unchanged under the mirror.
        let p1 = neg.eval(1.0);
        assert!(p1.x.abs() < 1e-12 && (p1.y - 1.0).abs() < 1e-12);
    }

    #[test]
    fn extruded_point_is_the_translation_trajectory() {
        let mc = MappedCurve::ExtrudedPoint {
            point: Point2::new(0.5, 0.25),
            place: Affine3::identity(),
            vec: Vec3::new(0.0, 0.0, 2.0),
        };
        let p = mc.eval(0.75);
        assert_eq!((p.x, p.y, p.z), (0.5, 0.25, 1.5));
    }

    #[test]
    fn revolved_point_is_the_rotation_trajectory() {
        let mc = MappedCurve::RevolvedPoint {
            point: Point2::new(2.0, 0.0),
            place: Affine3::identity(),
            axis_origin: Point3::origin(),
            axis_dir: Vec3::unit_y(),
            angle: PI,
        };
        // s = 0: the placed point itself.
        let p0 = mc.eval(0.0);
        assert_eq!((p0.x, p0.y, p0.z), (2.0, 0.0, 0.0));
        // s = 1/2: quarter turn about +y takes +x to −z.
        let ph = mc.eval(0.5);
        assert!((ph.x).abs() < 1e-12 && (ph.z + 2.0).abs() < 1e-12);
        // s = 1: half turn lands at −x.
        let p1 = mc.eval(1.0);
        assert!((p1.x + 2.0).abs() < 1e-12 && p1.z.abs() < 1e-12);
    }

    #[test]
    fn placed_segment_composes_placement() {
        // Place the sketch xy-plane at z = 3 rotated 90° about x:
        // sketch (x, y) ↦ world (x, −0·…): use a simple translation to
        // keep the check exact.
        let place = Affine3::translation(Vec3::new(0.0, 0.0, 3.0));
        let mc = MappedCurve::PlacedSegment {
            segment: SketchSegment::Line {
                a: Point2::new(0.0, 0.0),
                b: Point2::new(1.0, 0.0),
            },
            place,
        };
        let p = mc.eval(0.5);
        assert_eq!((p.x, p.y, p.z), (0.5, 0.0, 3.0));
    }
}
