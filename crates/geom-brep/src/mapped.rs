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
//! Sketch segments use the profile's canonical segment form: the
//! endpoints stored verbatim, and an arc's carrier and signed sweep as
//! the one arc value the `profile` crate's segments also hold
//! ([`geom_core::Arc2`]), so the sweep hands a validated arc across
//! whole. The
//! **line/arc split is structural** ([`SketchSegment`]), mirroring the
//! upstream trilean classification: by the time a description exists,
//! straightness was already *decided* (profile validation), so
//! evaluation here never re-decides it (no value branch).
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

use geom_core::{Affine3, Arc2, Point2, Point3, Real, Vec3};

/// A 2-D sketch-plane segment in the canonical form (module docs):
/// verbatim endpoints, and for an arc its carrier and signed sweep. The
/// line/arc split is structural — decided upstream, never re-decided
/// here.
///
/// An arc's fields are redundant by design — the endpoints lie on the
/// carrier, and the sweep turns `a` into `b` about the centre — and
/// nothing re-decides that redundancy at this type's door: every
/// reader of the locus reads the same fields, so no reader can see a
/// different circle from the one certification meters.
///
/// - [`SketchSegment::eval`] (and so certification, which meters the
///   description against its carrier through it) reads `a`, the centre
///   and the sweep only, through [`Arc2::point_from`]: the locus is `a`
///   turned about the centre.
/// - [`SketchSegment::restrict`] reads what `eval` reads and carries
///   the radius through.
/// - `sweep::skin::segment_curve`, public through `sweep` and `pncad`,
///   converts that same locus: its on-arc control points are `eval`'s
///   own points and its others the spoke `a − centre` turned about the
///   centre. It never reads `b` or the radius.
///
/// The radius is read by the carrier a sweep builds beside the
/// description, and certification meters the one against the other. A
/// segment the sweep mints from a validated profile carries the
/// profile's arc, whose consistency validation holds (checked for a
/// table, by construction for a constructed loop:
/// `crates/profile/README.md`, "Where an arc's consistency is decided").
#[derive(Clone, Copy, Debug)]
pub enum SketchSegment<T: Real> {
    /// The straight chord from `a` to `b`; `s` sweeps it affinely.
    Line {
        /// Start point (s = 0), sketch-plane meters.
        a: Point2<T>,
        /// End point (s = 1).
        b: Point2<T>,
    },
    /// The circular arc from `a` to `b` on `arc`'s carrier, turning
    /// through its signed sweep (positive counterclockwise), with
    /// 0 < |Δθ| ≤ 2π — a full turn closes on its start, `b = a` (D1).
    Arc {
        /// Start point (s = 0), stored verbatim.
        a: Point2<T>,
        /// End point (s = 1), stored verbatim.
        b: Point2<T>,
        /// The carrier and the signed sweep from `a` to `b`.
        arc: Arc2<T>,
    },
}

impl<T: Real> SketchSegment<T> {
    /// The sub-segment covering `[s0, s1]` of this segment,
    /// reparameterized to `[0, 1]` (for `split_edge`): endpoints by
    /// [`SketchSegment::eval`]; an arc keeps its carrier (centre and
    /// radius pass through unchanged) and its sweep becomes
    /// `sweep·(s1 − s0)`. Fixed evaluation order (D9); total —
    /// degenerate inputs yield degenerate data, caught by the caller's
    /// certification.
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
    /// successive splits compound it — see [`Arc2::point_from`]'s
    /// anchoring note for why the evaluation is written to keep that
    /// width at the endpoints' own scale.
    pub fn restrict(&self, s0: T, s1: T) -> Self {
        match *self {
            SketchSegment::Line { .. } => SketchSegment::Line {
                a: self.eval(s0),
                b: self.eval(s1),
            },
            SketchSegment::Arc { arc, .. } => SketchSegment::Arc {
                a: self.eval(s0),
                b: self.eval(s1),
                arc: Arc2 {
                    sweep: arc.sweep * (s1 - s0),
                    ..arc
                },
            },
        }
    }

    /// The point at normalized parameter `s ∈ [0, 1]` (module docs).
    ///
    /// Line: `lerp(a, b, s)`. Arc: `a` rotated about the centre by
    /// `s·sweep`, [`Arc2::point_from`] — exact at `s = 0`, within the
    /// rotation's rounding of `b` at `s = 1`. Endpoint authority is
    /// held elsewhere: the topology's endpoints are the vertices, never
    /// this evaluation, and certification meters the evaluation
    /// against the carrier at every sample, `s = 1` included.
    pub fn eval(&self, s: T) -> Point2<T> {
        match *self {
            SketchSegment::Line { a, b } => a.lerp(b, s),
            SketchSegment::Arc { a, arc, .. } => arc.point_from(a, s),
        }
    }
}

/// The sketch pushforward (module docs): one authoritative sketch source plus the map that carries it
/// into 3-space, combined per variant so incoherent pairings are
/// unrepresentable.
///
/// Placements map sketch coordinates `(x, y)` to `place · (x, y, 0)`.
/// Rigidity of the placement (orthonormal linear part) is not decided
/// here and not re-examined here: what arrives is whatever built the
/// map, and a placement minted from a frame witness
/// (`profile::SketchPlane::from_frame`, over `geom_core::OrthoFrame`)
/// carries that decision with it while one assigned by hand carries
/// only its own source's.
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
    /// side struts: `s ↦ place(point) + vec·range.at(s)`.
    ExtrudedPoint {
        /// The authoritative sketch point.
        point: Point2<T>,
        /// The rigid placement of the sketch plane in 3-space.
        place: Affine3<T>,
        /// The **full** extrusion vector (meters) of the whole strut:
        /// its normalized parameter 1 lands at the far end.
        vec: Vec3<T>,
        /// The part of the whole strut's normalized parameter this
        /// trajectory covers: [`SweepRange::whole`], or a sub-range of
        /// it after [`MappedCurve::restrict`].
        range: SweepRange<T>,
    },
    /// A sketch point's trajectory under a rotation family — revolve
    /// latitude arcs: `s ↦ rotate(place(point))` about the axis by
    /// `range.at(s)·angle`.
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
        /// The **full** signed revolve angle of the whole sweep
        /// (radians, right-hand rule about `axis_dir`): its normalized
        /// parameter 1 lands at the far end.
        angle: T,
        /// The part of the whole sweep's normalized parameter this
        /// trajectory covers: [`SweepRange::whole`], or a sub-range of
        /// it after [`MappedCurve::restrict`].
        range: SweepRange<T>,
    },
}

/// The sub-range of a whole sweep's normalized parameter `u ∈ [0, 1]`
/// a trajectory covers, as `u` at s = 0 and the signed span from there
/// to s = 1. The sweep's own angle or vector is stored once beside it
/// and applied at evaluation, as `angle·u` or `vec·u`.
///
/// Restriction lives here, in the parameter, and never in the
/// placement: [`MappedCurve::restrict`] narrows the range and leaves
/// the placement as built, so however often a trajectory is split its
/// evaluation applies ONE motion to the placed point. In `u` a dyadic
/// split is exact, so a chain of them from either end stores no
/// rounding at all.
///
/// The whole sweep's start `0` and span `1` are held as absent rather
/// than as values, so [`SweepRange::at`] reads `s` itself on a whole
/// range: an unrestricted trajectory evaluates in exactly the
/// unrestricted form, `angle·s` or `vec·s`, at every scalar, and a
/// caller can tell an exact start from a computed one.
#[derive(Clone, Copy, Debug)]
pub struct SweepRange<T: Real> {
    start: Option<T>,
    span: Option<T>,
}

impl<T: Real> SweepRange<T> {
    /// The whole sweep, `u ∈ [0, 1]`.
    pub fn whole() -> Self {
        SweepRange {
            start: None,
            span: None,
        }
    }

    /// `u` at s = 0, or `None` at the whole sweep's exact start `0`.
    pub fn start(self) -> Option<T> {
        self.start
    }

    /// The signed span of `u` from s = 0 to s = 1, or `None` for the
    /// whole sweep's `1`.
    pub fn span(self) -> Option<T> {
        self.span
    }

    /// `u` at normalized parameter `s`: `start + span·s`, each absent
    /// term dropped, so the whole range reads `s` itself.
    pub fn at(self, s: T) -> T {
        let along = match self.span {
            None => s,
            Some(span) => span * s,
        };
        match self.start {
            None => along,
            Some(start) => start + along,
        }
    }

    /// The sub-range covering `[s0, s1]` of this one: the start moves
    /// to [`SweepRange::at`]`(s0)` and the span scales by `s1 − s0`.
    /// The difference is formed once, from the split parameters, and
    /// never from two stored ends.
    pub fn restrict(self, s0: T, s1: T) -> Self {
        let width = s1 - s0;
        SweepRange {
            start: Some(self.at(s0)),
            span: Some(match self.span {
                None => width,
                Some(span) => span * width,
            }),
        }
    }

    /// The same range with its start moved by `d0` and its end by `d1`,
    /// both in `u`. An end given `None` stays where it was in exact
    /// arithmetic; the start then keeps its stored value bit for bit,
    /// while the end is re-read as the moved start plus the moved span,
    /// so it can differ from before in the last bits.
    pub fn moved(self, d0: Option<T>, d1: Option<T>) -> Self {
        let span = self.span.unwrap_or_else(T::one);
        SweepRange {
            start: match (self.start, d0) {
                (start, None) => start,
                (None, Some(d0)) => Some(d0),
                (Some(start), Some(d0)) => Some(start + d0),
            },
            span: match (d0, d1) {
                (None, None) => self.span,
                (Some(d0), None) => Some(span - d0),
                (None, Some(d1)) => Some(span + d1),
                (Some(d0), Some(d1)) => Some(span + (d1 - d0)),
            },
        }
    }
}

impl<T: Real> MappedCurve<T> {
    /// The sub-curve covering `[s0, s1]`, reparameterized to `[0, 1]`
    /// (for `split_edge`): the restricted description is again a
    /// pushforward of the same shape — the authoritative source is
    /// restricted ([`SketchSegment::restrict`]) or the motion's
    /// [`SweepRange`] is, with the placement kept as built. In exact
    /// arithmetic `restrict(s0, s1).eval(s) = eval(s0 + (s1 − s0)·s)`;
    /// the float discrepancy is metered by the caller's re-certification
    /// of the restricted spec (D4 ¶2 — nothing is trusted untested).
    /// Fixed evaluation orders as written (D9).
    pub fn restrict(&self, s0: T, s1: T) -> Self {
        match *self {
            MappedCurve::PlacedSegment { segment, place } => MappedCurve::PlacedSegment {
                segment: segment.restrict(s0, s1),
                place,
            },
            MappedCurve::ExtrudedPoint {
                point,
                place,
                vec,
                range,
            } => MappedCurve::ExtrudedPoint {
                point,
                place,
                vec,
                range: range.restrict(s0, s1),
            },
            MappedCurve::RevolvedPoint {
                point,
                place,
                axis_origin,
                axis_dir,
                angle,
                range,
            } => MappedCurve::RevolvedPoint {
                point,
                place,
                axis_origin,
                axis_dir,
                angle,
                range: range.restrict(s0, s1),
            },
        }
    }

    /// The described point at normalized parameter `s ∈ [0, 1]` — the
    /// authoritative locus the cached carrier is certified against
    /// (module docs). Total; fixed evaluation orders as written (D9).
    pub fn eval(&self, s: T) -> Point3<T> {
        match *self {
            MappedCurve::PlacedSegment { segment, place } => place_point(place, segment.eval(s)),
            MappedCurve::ExtrudedPoint {
                point,
                place,
                vec,
                range,
            } => place_point(place, point) + vec * range.at(s),
            MappedCurve::RevolvedPoint {
                point,
                place,
                axis_origin,
                axis_dir,
                angle,
                range,
            } => {
                let p = place_point(place, point);
                Affine3::rotation_about_axis(axis_origin, axis_dir, range.at(s) * angle)
                    .transform_point(p)
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

    /// The quarter circle from (1, 0) to (0, 1) on the unit circle,
    /// counterclockwise (`sweep = π/2`), or its clockwise mirror about
    /// the unit circle centred at (1, 1) (`sweep = −π/2`).
    fn quarter(ccw: bool) -> SketchSegment<f64> {
        SketchSegment::Arc {
            a: Point2::new(1.0, 0.0),
            b: Point2::new(0.0, 1.0),
            arc: Arc2 {
                centre: if ccw {
                    Point2::new(0.0, 0.0)
                } else {
                    Point2::new(1.0, 1.0)
                },
                radius: 1.0,
                sweep: if ccw { FRAC_PI_2 } else { -FRAC_PI_2 },
            },
        }
    }

    #[test]
    fn arc_segment_turns_the_start_about_the_carrier() {
        let seg = quarter(true);
        // The start comes back as stored, bit for bit; the end is the
        // start turned by the whole sweep.
        let p0 = seg.eval(0.0);
        assert_eq!((p0.x, p0.y), (1.0, 0.0));
        let p1 = seg.eval(1.0);
        assert!(p1.x.abs() < 1e-15 && (p1.y - 1.0).abs() < 1e-15);
        // Midpoint is the arc apex at 45°.
        let pm = seg.eval(0.5);
        let r = (FRAC_PI_2 / 2.0).cos(); // cos(π/4)
        assert!((pm.x - r).abs() < 1e-15 && (pm.y - r).abs() < 1e-15);
        // Every sample lies on the unit carrier circle.
        for i in 0..=8 {
            let p = seg.eval(f64::from(i) / 8.0);
            assert!((p.x * p.x + p.y * p.y - 1.0).abs() < 1e-15);
        }
        // The clockwise mirror: its carrier is the unit circle centred
        // at (1, 1), and its apex bows toward the origin.
        let neg = quarter(false);
        let pm_neg = neg.eval(0.5);
        let d_center = pm_neg.distance(Point2::new(1.0, 1.0));
        assert!((d_center - 1.0).abs() < 1e-15);
        assert!(pm_neg.x * pm_neg.x + pm_neg.y * pm_neg.y < 1.0);
        let p1 = neg.eval(1.0);
        assert!(p1.x.abs() < 1e-15 && (p1.y - 1.0).abs() < 1e-15);
    }

    /// A restriction keeps the carrier bit for bit, scales the sweep,
    /// and evaluates as the parent does at the mapped parameter.
    #[test]
    fn restricted_arc_keeps_its_carrier_and_scales_its_sweep() {
        for ccw in [true, false] {
            let seg = quarter(ccw);
            let sub = seg.restrict(0.25, 0.75);
            let (
                SketchSegment::Arc {
                    arc:
                        Arc2 {
                            centre,
                            radius,
                            sweep,
                        },
                    ..
                },
                SketchSegment::Arc {
                    arc:
                        Arc2 {
                            centre: c2,
                            radius: r2,
                            sweep: w2,
                        },
                    a,
                    ..
                },
            ) = (seg, sub)
            else {
                panic!("an arc restricts to an arc");
            };
            assert_eq!((c2.x, c2.y, r2), (centre.x, centre.y, radius));
            assert_eq!(w2, sweep * 0.5);
            let p = seg.eval(0.25);
            assert_eq!(
                (a.x, a.y),
                (p.x, p.y),
                "the sub-arc starts where it was cut"
            );
            for i in 0..=4 {
                let s = f64::from(i) / 4.0;
                let (q, r) = (sub.eval(s), seg.eval(0.25 + 0.5 * s));
                assert!(q.distance(r) < 1e-15, "ccw {ccw}, s = {s}");
            }
        }
    }

    #[test]
    fn extruded_point_is_the_translation_trajectory() {
        let mc = MappedCurve::ExtrudedPoint {
            point: Point2::new(0.5, 0.25),
            place: Affine3::identity(),
            vec: Vec3::new(0.0, 0.0, 2.0),
            range: SweepRange::whole(),
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
            range: SweepRange::whole(),
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
