//! D2's intensional edge descriptions: what an edge's locus **is**.
//!
//! An edge's geometry is stored as an intensional description
//! ([`EdgeGeometry`]); every concrete representation — the 3-D carrier
//! curve now, pcurves at M3 — is a *derived cache* certified against the
//! description (D4 ¶2, [`crate::EdgeCurve`]). The sum type is D2's,
//! verbatim, with **no `Explicit` variant** — the deliberate omission is
//! the design's spine: there is no extensional escape hatch, so it can
//! never be reached for.
//!
//! # The taxonomy (D2's dichotomy)
//!
//! - **Intrinsic**: [`EdgeGeometry::Intersection`] — the locus is
//!   determined by its two surfaces (a transverse surface–surface
//!   intersection; the witness point selects the connected component
//!   and later seeds marching). Its validity precondition is
//!   *transversality* — normals linearly independent along the locus —
//!   checked at certification through the dihedral machinery
//!   ([`crate::classify_dihedral`]). `TangencyLocus` (the second
//!   intrinsic variant) arrives with fillets (M5); at M2 a tangential
//!   contact classifies as sliver and fails loudly.
//! - **Conventional**: [`EdgeGeometry::MappedCurve`] and
//!   [`EdgeGeometry::Seam`] — loci the surfaces *under*-determine, so
//!   the description carries its own defining data. `MappedCurve` is a
//!   pushforward: one authoritative source (a sketch entity) and a map,
//!   never two peer representations. `Seam` is a closed-chart
//!   parameterization seam — pure convention, carried by the surface's
//!   own `u_ref` (the `u_ref`-half-plane meridian is *the* seam; see
//!   [`EdgeGeometry::Seam`] for the spatial definition and the
//!   mirror-nappe cone caveat).
//!
//! # `MappedCurve`'s payload at M2
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

use crate::keys::SurfaceKey;

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

/// The pushforward payload of [`EdgeGeometry::MappedCurve`] (module
/// docs): one authoritative sketch source plus the map that carries it
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

/// D2's intensional edge-geometry sum type (module docs). **No
/// `Explicit` variant, by ratified design.**
///
/// Surface references are body-arena keys ([`SurfaceKey`]) —
/// lineage-scoped per Q1 (see [`crate::keys`]); certification receives
/// a resolver from the owning body rather than resolving keys itself.
#[derive(Clone, Copy, Debug)]
pub enum EdgeGeometry<T: Real> {
    /// Intrinsic: the connected component of the transverse intersection
    /// S₁ ∩ S₂ selected by `witness`. Transversality (normals linearly
    /// independent along the locus) is the validity precondition,
    /// enforced at certification. The witness SELECTS the component
    /// and nothing else: the marching rung mints its own
    /// (`carrier(mid)`, from the fitted cache) and seeds from surviving
    /// cell centres, so it consumes no witness from here —
    /// `geom_brep::ssi::certify` states that contract in its own words.
    Intersection {
        /// The first surface (one of the edge's two adjacent faces'
        /// surfaces — coherence checked by the tier-3 validator).
        s1: SurfaceKey,
        /// The second surface.
        s2: SurfaceKey,
        /// A point on (within ε of) the intended component of the
        /// intersection locus — and, by the certification contract
        /// (M2 PR 3 fix pass), **the edge's mid-parameter point**:
        /// certification pins `carrier((t₀ + t₁)/2)` to this point
        /// within ε, so the witness selects not just the component but
        /// the traversed arc and winding between the endpoints.
        /// Constructors mint it by evaluating the carrier at the
        /// interval midpoint (for straight chords: the chord midpoint).
        witness: Point3<T>,
    },
    /// Intrinsic, one differential order up (D2 as sharpened per
    /// CURVED-DESIGN OQ7; M5 PR 9): the connected component of the
    /// TANGENTIAL contact locus of S₁ and S₂ selected by `witness` —
    /// surfaces coincident and normal-parallel *along* the locus,
    /// separating quadratically *transverse* to it (relative
    /// transverse normal curvature bounded away from zero — the jet
    /// system's IFT denominator, enforced at certification as the
    /// second-order margin). No stored contact-order field: order-k
    /// contact beyond k = 1 is out of scope at M5 (D2's note records
    /// the generalization). Fillet trimlines (M5 PR 12) STORE this
    /// variant; at PR 9 it is minted by classification of the C5
    /// table's tangent arms — never by marching (the SSI σ₂ band
    /// refuses toward this variant instead of desingularizing).
    TangentIntersection {
        /// The first surface (one of the edge's two adjacent faces'
        /// surfaces — coherence checked by the tier-3 validator).
        s1: SurfaceKey,
        /// The second surface.
        s2: SurfaceKey,
        /// A point on the intended component of the tangency locus —
        /// and, by the certification contract, **the edge's
        /// mid-parameter point** (the same S2 pin as
        /// [`EdgeGeometry::Intersection`]'s witness).
        witness: Point3<T>,
    },
    /// Conventional: a pushforward of a sketch entity under a sweep map
    /// — the defining data for loci the surfaces under-determine
    /// (profile-join splits, swept trajectories).
    MappedCurve(MappedCurve<T>),
    /// Conventional: the parameterization seam of a closed (periodic)
    /// surface — the **`u_ref`-half-plane meridian**, defined
    /// spatially: the surface's locus in the closed half-plane spanned
    /// by the axis and `u_ref` (certification meters the wrong-side
    /// excess `max(0, −w·u_ref)`; seam placement is conventional data,
    /// D2). On most walls that locus is also the chart's u = 0
    /// iso-curve, but not always: a mirror-nappe cone (walls on
    /// v < 0 — revolve always aims the chart axis at +a₃, so a
    /// downward-opening cone sweeps its mirror nappe) has its u = 0
    /// iso-curve on the spatial-π meridian, and the seam meridian is
    /// chart u = π. The spatial definition is the one the kernel
    /// certifies.
    /// Both of the edge's faces lie on this one surface. Pcurves (the
    /// classical second payload) are M3 derived caches, absent at M2.
    Seam {
        /// The periodic surface whose seam this edge is.
        surface: SurfaceKey,
    },
    /// Conventional: the `u = const` **iso-parameter curve** of a
    /// parametric surface — the loft/sweep assembly's wall–wall seam
    /// class (M6-3).
    ///
    /// Why this is its own variant and not an
    /// [`EdgeGeometry::Intersection`]: a definitional wall junction's
    /// contact class is the profile's **declared** corner structure
    /// (Q8/C11), not a derived one — and on a NURBS wall the
    /// implicit-form machinery (`implicit_residual`,
    /// `curvature_lever_arm`) is poison anyway, so `classify_dihedral`
    /// cannot run there. The certified statement is instead the
    /// genuinely metric residual
    /// `|C(t) − S(u, v0 + (v1 − v0)·(t − t0)/(t1 − t0))|` at the CERT
    /// schedule (two-tolerance, definite arms included), and tier-3
    /// adjacency reads as `surface ∈ {fs_plus, fs_minus}`.
    ///
    /// The definitional payoff (M5 PR 10 §3): an iso-curve's pcurve on
    /// its own chart is an **exact straight line in UV** — no fit
    /// anywhere.
    IsoCurve {
        /// The surface whose iso-curve this edge is (one of the edge's
        /// two adjacent faces' surfaces — tier-3 coherence).
        surface: SurfaceKey,
        /// The fixed `u` parameter of the iso-curve.
        u: T,
        /// The `v` value at the edge's `param_start`.
        v0: T,
        /// The `v` value at the edge's `param_end`.
        v1: T,
    },
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
