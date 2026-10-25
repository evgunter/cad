//! **A trimmed sphere face's region, read from its boundary arcs** — the
//! one reading both point doors take of a sphere face that is not closed
//! on its own surface: the ray lane's hit test and boundary pre-pass
//! ([`super::solid_contain`]) and the pierce arm's landing test
//! ([`super::contain::curved_face_containment`]).
//!
//! # Method: the closest crossing along a great circle
//!
//! Every boundary edge of such a face is a circle arc on the sphere (a
//! plane's section of it). The face is the side of its loops to their
//! left under its OUTWARD normal `N = σ·(p − c)/R` (`σ` the sense bit) —
//! the convention the flux lane's Gauss–Bonnet area reads
//! (`geom_brep::props`' sphere circle loop) — so at a point of an arc's
//! interior with traversal tangent `τ`, the face lies along `N × τ`.
//! That is a reference with no chart in it, and the closest crossing
//! carries it to the query, as the solid door's own rays do:
//!
//! 1. Cast a geodesic RAY from `p` along a tangent direction `t`: the
//!    great circle `cos s·p̂ + sin s·t`, `s ∈ (0, 2π)`, in the plane
//!    through the centre with normal `ĝ = p̂ × t`.
//! 2. Every arc of every loop meets that plane at the roots of a first
//!    harmonic, `ĝ·(C − c) + ρ(ĝ·û) cos θ + ρ(ĝ·v̂) sin θ`, certified by
//!    the shared first-harmonic door
//!    ([`super::circle_roots::first_harmonic_roots`]); a root counts
//!    where it lies inside its arc's span, at the ray parameter `s` it
//!    sits at. The ray is a leg of the region skeleton
//!    ([`region::read_leg`]), so each arc's roots are all read against
//!    its span before any is placed along the ray.
//! 3. At the closest crossing, the ray's heading against `N × τ` says
//!    whether it leaves the face there (so `p` is in it) or enters it.
//!
//! The rays read every loop of the face at once, rings included, so a
//! ringed face needs no case of its own. An edge both of whose sides are
//! the face (both its half-edges in the face's loops, a seam) is no
//! boundary of it and is left out of the region, so a face whose every
//! edge is a seam covers the sphere. Nothing is read in the chart, so a
//! pole is a point like any other.
//!
//! # Which rays
//!
//! First the rays aimed at points of the boundary — three points of every
//! arc, in loop order — each of which meets the boundary somewhere; then
//! the fixed schedule's directions projected onto the tangent plane at
//! `p` (`splitting::containment::SCHEDULE`), for the points where every
//! aimed ray runs along an arc: a point a hair off a vertex sees every
//! arc through that vertex nearly edge-on. Each direction is cast both
//! ways. Every great circle through `p` passes `p`'s antipode, so a
//! vertex there is a crossing at `s = π` on every ray, and it is the
//! closest one on every ray whose forward half meets nothing else; the
//! reverse ray reaches the arcs behind `p` first.
//!
//! # The boundary first
//!
//! Before any ray, `p` is read against every arc on the arc's own
//! circle ([`ConicArc::hit`](crate::splitting::containment::ConicArc::hit), the boundary reading the planar loop walk
//! takes of a circle edge): on an arc, or within the band of one of its
//! ends, is the point on the boundary ([`None`]); in the band of one
//! refuses. A crossing's place along a ray is no measure of `p`'s
//! distance from the arc it crosses — on a ray meeting an arc at a
//! shallow angle it is far longer — so the rays never answer it.
//!
//! # Grazing
//!
//! Past that, every reading is about one ray ([`crate::ray_walk`]): a
//! closest crossing at a vertex, a tie with the closest crossing, a
//! tangency of the ray with an arc (the root door's refusal), or an
//! arc lying in the ray's plane grazes; a direction with no tangent
//! share, or a great circle that meets no arc (it lies wholly on one
//! side), gives nothing to read; and the next is tried. Root counts that
//! disagree refuse the query: a broken invariant, not a ray's
//! conditioning.
//!
//! The boundary reading is of the ARC, not its circle: a point in the
//! band of an arc's great circle where the circle runs on past the arc's
//! ends, and clear of both ends, is off the boundary ([`ConicArc::hit`](crate::splitting::containment::ConicArc::hit)).
//!
//! # Predicates (meters)
//!
//! - `bool_sphere_region_arm`: a ray direction's share of the tangent
//!   plane at `p`, at the sphere's radius — a schedule member near the
//!   normal, or a target at `p` or its antipode, casts no ray.
//! - `bool_sphere_region_span`: a root's parameter against its arc's
//!   span ends, at the arc's radius.
//! - `bool_sphere_region_at`: a crossing's place along the ray, from
//!   `p`, at the sphere's radius.
//! - `bool_sphere_region_order`: two crossings' places along the ray.
//! - `bool_sphere_region_cross`: the sine between the ray and the arc's
//!   face side at the closest crossing, at the sphere's radius.
//! - `bool_sphere_region_roots_*`: the first-harmonic door's own meters.
//! - `bool_sphere_region_arc_span`: an arc's gap to a period. Where `p`
//!   sits against the arc is [`ConicArc::hit`](crate::splitting::containment::ConicArc::hit), the on-carrier and span
//!   readings' rows (`curve_carrier_*`, `curve_pair_span_*`). `bool_sphere_region_arc_straddle`
//!   is the name an ellipse's straddle would carry; a circle never does.

use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3};

use super::BooleanDecision;
use super::circle_roots::{FirstHarmonic, FirstHarmonicRows, first_harmonic_roots_in_band};
use super::region::{
    self, Fault, Landing, Leg, Legs, Region, RegionEdge, RegionRefusal, Round, Scaffolding,
    circle_edge_hit, found, span, targets,
};
use super::solid_contain::PointInSolidError;
use crate::body::Body;
use crate::entity::{FaceKey, LoopKey};
use crate::ray_walk::{self, RayFault};
use crate::splitting::containment::{ConicRows, SCHEDULE};
use crate::validate::decide;

const ROOT_ROWS: FirstHarmonicRows = FirstHarmonicRows {
    noise: "bool_sphere_region_roots_noise",
    coaxial: "bool_sphere_region_roots_coaxial",
    extreme: "bool_sphere_region_roots_extreme",
    root_slack: "bool_sphere_region_roots_slack",
    decision: BooleanDecision::CONTAINMENT_UNNAMED,
};

/// The boundary reading's rows (module docs).
const BOUNDARY: ConicRows = ConicRows::bounded("bool_sphere_region_arc_span", "bool_sphere_region_arc_straddle");

/// A trimmed sphere face's boundary: every arc of every loop, each with
/// the direction its loop traverses it.
#[derive(Clone, Debug)]
pub(crate) struct SphereFaceRegion<T: geom_core::Real> {
    center: Point3<T>,
    radius: T,
    /// The face's sense bit: `false` points its outward normal at the
    /// centre.
    sense: bool,
    /// The face's outer loop, named by an exhaustion.
    outer: LoopKey,
    edges: Vec<Arc<T>>,
}

/// One boundary arc.
type Arc<T> = RegionEdge<T>;

/// The unit tangent at the unit `a` along `raw`, a vector already
/// projected off `a` once. A `raw` from a direction near `a` keeps a
/// component along `a` its own size times the rounding, which no ray
/// parameter survives, so the projection is taken again.
fn tangent<T: Decide>(a: Vec3<T>, raw: Vec3<T>) -> Vec3<T> {
    let t = raw / raw.norm();
    let t = t - a * a.dot(t);
    t / t.norm()
}

/// `face`'s region on its sphere `(center, radius)`, or `None` when an
/// edge of it is not a circle arc — a spline or spiric carrier, whose
/// crossings with a great circle have no closed form here, or an edge
/// with no certified carrier at all.
///
/// # Errors
///
/// [`PointInSolidError::CorruptFace`] for a key that does not resolve or
/// an outer loop with no cycle.
///
/// # Panics
///
/// Where a boundary edge's curve does not resolve (D2 row 4): a torn
/// curve is not one with no certified carrier. The links hold at rest
/// and, on the reduction's working copies, by
/// [`crate::live::OPERATORS_KEEP_LINKS`].
pub(crate) fn sphere_face_region<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    center: Point3<T>,
    radius: T,
) -> Result<Option<SphereFaceRegion<T>>, PointInSolidError> {
    let f = body
        .get_face(face)
        .ok_or(PointInSolidError::CorruptFace { face })?;
    let circles = [geom::CurveKind::Circle];
    let Some(edges) = region::boundary(body, face, Scaffolding::Refused, &circles)? else {
        return Ok(None);
    };
    Ok(Some(SphereFaceRegion {
        center,
        radius,
        sense: f.sense,
        outer: f.outer,
        edges,
    }))
}

impl<T: Decide> SphereFaceRegion<T> {
    /// Is `p`, a point on the sphere, in the face: `Some(true)` inside,
    /// `Some(false)` outside, `None` on its boundary (module docs).
    ///
    /// # Errors
    ///
    /// [`RegionRefusal`]: `p` in the band of an arc, or no ray decided
    /// ([`ray_walk::walk`]), or an arc that bounds nothing.
    pub(crate) fn contains(&self, p: Point3<T>, band: Band) -> Result<Option<bool>, RegionRefusal> {
        if self.edges.is_empty() {
            return Ok(Some(true));
        }
        if self.on_boundary(p, band)? {
            return Ok(None);
        }
        let w = p - self.center;
        let a = w / w.norm();
        let aimed: Vec<Vec3<T>> = targets(&self.edges).map(|y| y - self.center).collect();
        let scheduled: Vec<Vec3<T>> = SCHEDULE.iter().map(|r| r.map(T::from_f64)).collect();
        let both_ways = |dirs: Vec<Vec3<T>>| {
            let back: Vec<Vec3<T>> = dirs.iter().map(|&d| -d).collect();
            dirs.into_iter().chain(back)
        };
        ray_walk::walk(
            both_ways(aimed).chain(both_ways(scheduled)),
            |toward| {
                let raw = toward - a * a.dot(toward);
                let arm = Margin::levered(raw.norm() / toward.norm(), self.radius);
                match decide("bool_sphere_region_arm", arm, band) {
                    Ok(Sign::Positive) => {
                        region::read_leg(self, &self.ray(a, tangent(a, raw)), band)
                    }
                    // Along the normal, or aimed at `p` or its antipode:
                    // no tangent direction to cast.
                    Ok(Sign::Zero | Sign::Negative) => Err(RayFault::Unread),
                    Err(diag) => Err(RayFault::InBand(RegionRefusal::Escalated(diag))),
                }
            },
            || RegionRefusal::RayExhausted { outer: self.outer },
        )
        .map(Some)
    }

    /// The ray from `a` along the unit tangent `t` (module docs, step 1):
    /// the great circle round the centre from `a` toward `t`.
    fn ray(&self, a: Vec3<T>, t: Vec3<T>) -> Leg<T, Round<T>> {
        Leg::whole(Round {
            center: self.center,
            normal: a.cross(t),
            radius: self.radius,
            e1: a,
            e2: t,
        })
    }
}

impl<T: Decide> Region<T> for SphereFaceRegion<T> {
    fn edges(&self) -> &[Arc<T>] {
        &self.edges
    }

    fn outer(&self) -> LoopKey {
        self.outer
    }

    fn on_edge(&self, arc: &Arc<T>, p: Point3<T>, band: Band) -> Result<bool, RegionRefusal> {
        let span = (arc.t0, arc.t1);
        circle_edge_hit(&arc.carrier, span, arc.extent, BOUNDARY, p, self.outer, band)
    }
}

impl<T: Decide> Legs<T> for SphereFaceRegion<T> {
    type Course = Round<T>;

    const AT: &'static str = "bool_sphere_region_at";

    const SIDE: (&'static str, &'static str) =
        ("bool_sphere_region_order", "bool_sphere_region_cross");

    fn lever(&self) -> T {
        self.radius
    }

    /// Where `arc` meets the ray's plane through the centre (module docs,
    /// step 2), read in the arc's own parameter, inside its span.
    fn landings(
        &self,
        arc: &Arc<T>,
        ray: &Leg<T, Round<T>>,
        band: Band,
    ) -> Result<Vec<Landing<T>>, Fault> {
        let geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } = arc.carrier
        else {
            unreachable!("sphere_face_region admits circles only")
        };
        let roots = first_harmonic_roots_in_band(
            &FirstHarmonic::circle_off_plane(
                (center, axis, radius, u_ref),
                (self.center, ray.course.normal),
            ),
            radius,
            arc.t0,
            arc.t1,
            &ROOT_ROWS,
            band,
        );
        let mut out = Vec::new();
        for theta in found(roots)? {
            let lands = span(
                "bool_sphere_region_span",
                theta,
                (arc.t0, arc.t1),
                radius,
                band,
            )?;
            if let Some(sideless) = lands {
                out.push((arc.at(theta), theta, sideless));
            }
        }
        Ok(out)
    }

    fn outward(&self, y: Point3<T>) -> Vec3<T> {
        let w = y - self.center;
        geom_brep::OutwardNormal::from_chart(w / w.norm(), self.sense).vec()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::entity::EdgeKey;
    use slotmap::KeyData;

    /// The unit sphere's face on one side of the circle at polar angle
    /// 60°, one arc round the whole turn: the cap above it run
    /// `forward`, the rest of the sphere otherwise.
    fn carved(forward: bool) -> SphereFaceRegion<f64> {
        SphereFaceRegion {
            center: Point3::origin(),
            radius: 1.0,
            sense: true,
            outer: LoopKey::default(),
            edges: vec![RegionEdge {
                edge: EdgeKey::from(KeyData::from_ffi(1)),
                r#loop: LoopKey::default(),
                carrier: geom::Curve3::Circle {
                    center: Point3::new(0.0, 0.0, 0.5),
                    axis: Vec3::new(0.0, 0.0, 1.0),
                    radius: 0.75f64.sqrt(),
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                },
                t0: 0.0,
                t1: core::f64::consts::TAU,
                extent: crate::boolean::region::fixture_extent(0.0, core::f64::consts::TAU),
                forward,
            }],
        }
    }

    /// The point at polar angle `polar` and azimuth `az` on the unit sphere.
    fn at(polar: f64, az: f64) -> Point3<f64> {
        Point3::new(polar.sin() * az.cos(), polar.sin() * az.sin(), polar.cos())
    }

    /// **A point near an arc is read by its distance from the arc, not
    /// along a ray.** Round the whole circle, a point within the band's
    /// zero of it is on the boundary, one in the band refuses, and one
    /// past the band reads its side. A crossing's place along a ray
    /// meeting the arc at a shallow angle is many times the point's
    /// distance from it, which answered `In` or `Out` at a fifth of the
    /// azimuths half a tolerance off the arc.
    #[test]
    fn a_point_near_an_arc_is_read_by_its_distance_from_it() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let rim = core::f64::consts::FRAC_PI_3;
        for forward in [true, false] {
            let region = carved(forward);
            for i in 0..200 {
                let az = f64::from(i) * core::f64::consts::TAU / 200.0 + 1e-4;
                for off in [-0.9e-9, -0.5e-9, 0.5e-9, 0.9e-9] {
                    assert_eq!(
                        region.contains(at(rim + off, az), band),
                        Ok(None),
                        "on the boundary: forward {forward}, azimuth {az}, {off:e} off"
                    );
                }
                for off in [-5e-9, 2e-9] {
                    assert!(
                        matches!(
                            region.contains(at(rim + off, az), band),
                            Err(RegionRefusal::Escalated(diag))
                                if diag.predicate == Some("curve_carrier_gap")
                        ),
                        "in the band: forward {forward}, azimuth {az}, {off:e} off"
                    );
                }
                for (off, above) in [(-1e-6, true), (1e-6, false)] {
                    assert_eq!(
                        region.contains(at(rim + off, az), band),
                        Ok(Some(above == forward)),
                        "past the band: forward {forward}, azimuth {az}, {off:e} off"
                    );
                }
            }
        }
    }

    /// The lune of the unit sphere between the meridians at azimuths `0`
    /// and `π/2`: each a half of a great circle, pole to pole, so each
    /// arc's circle runs on past its ends round the back of the sphere.
    fn lune() -> SphereFaceRegion<f64> {
        let meridian = |az: f64, forward: bool, key: u64| RegionEdge {
            edge: EdgeKey::from(KeyData::from_ffi(key)),
            r#loop: LoopKey::default(),
            carrier: geom::Curve3::Circle {
                center: Point3::origin(),
                axis: Vec3::new(-az.sin(), az.cos(), 0.0),
                radius: 1.0,
                u_ref: Vec3::new(0.0, 0.0, 1.0),
            },
            t0: 0.0,
            t1: core::f64::consts::PI,
            extent: crate::boolean::region::fixture_extent(0.0, core::f64::consts::PI),
            forward,
        };
        SphereFaceRegion {
            center: Point3::origin(),
            radius: 1.0,
            sense: true,
            outer: LoopKey::default(),
            edges: vec![
                meridian(0.0, true, 1),
                meridian(core::f64::consts::FRAC_PI_2, false, 2),
            ],
        }
    }

    /// The point `off` metres off the meridian plane at azimuth 0, at
    /// angle `theta` round that meridian's great circle from the north
    /// pole (`θ ∈ (0, π)` is the arc, `(π, 2π)` its continuation), lifted
    /// back onto the sphere.
    fn off_meridian(theta: f64, off: f64) -> Point3<f64> {
        let v = Vec3::new(theta.sin(), off, theta.cos());
        Point3::origin() + v / v.norm()
    }

    /// **A point in the band of an arc's circle, past the arc, reads by
    /// the rays.** On the meridian's continuation round the back of the
    /// sphere — a unit and more from the face — a point within the band
    /// of the great circle is plainly outside the lune, at every offset
    /// in and below the band. Reading the distance to the whole circle
    /// refused every one of these on its distance row.
    #[test]
    fn a_point_in_band_of_an_arcs_circle_past_the_arc_is_off_it() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let region = lune();
        for theta in [1.2, 1.5, 1.8].map(|f| f * core::f64::consts::PI) {
            for off in [0.0, 0.5e-9, -0.5e-9, 3e-9, -3e-9, 6e-9, 2e-8] {
                assert_eq!(
                    region.contains(off_meridian(theta, off), band),
                    Ok(Some(false)),
                    "theta {theta}, {off:e} off the circle"
                );
            }
        }
    }

    /// **On a partial arc, the boundary reading is the arc's own**: within
    /// the zero band of the arc's interior on the boundary, in the band
    /// refused on the on-carrier reading's gap, past it on its side — the lune
    /// lies on the `+y` side of the meridian at azimuth 0.
    #[test]
    fn a_point_near_a_partial_arc_is_read_by_its_distance_from_it() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let region = lune();
        for theta in [0.2, 0.4, 0.5, 0.7].map(|f| f * core::f64::consts::PI) {
            for off in [0.0, 0.5e-9, -0.5e-9] {
                assert_eq!(
                    region.contains(off_meridian(theta, off), band),
                    Ok(None),
                    "on: theta {theta}, {off:e}"
                );
            }
            for off in [3e-9, -3e-9] {
                assert!(
                    matches!(
                        region.contains(off_meridian(theta, off), band),
                        Err(RegionRefusal::Escalated(diag))
                            if diag.predicate == Some("curve_carrier_gap")
                    ),
                    "in band: theta {theta}, {off:e}"
                );
            }
            for (off, inside) in [(1e-6, true), (-1e-6, false), (0.3, true), (-0.3, false)] {
                assert_eq!(
                    region.contains(off_meridian(theta, off), band),
                    Ok(Some(inside)),
                    "past the band: theta {theta}, {off:e}"
                );
            }
        }
    }

    /// **Null scaffolding leaves a sphere face with no region**, as any
    /// edge with no certified curve does: a zone with a null edge fanned
    /// into its outer loop answers `None`, where the zone without it has
    /// a region.
    #[test]
    fn a_zones_null_edge_leaves_no_region() {
        let sheet = || {
            crate::boolean::boxes::sphere_rect_rows::sphere_zone_sheet(
                (0.2, 1.0),
                (-0.3, 0.4),
                geom_core::Tol::witness(),
            )
        };
        let (body, face) = sheet();
        assert!(
            sphere_face_region(&body, face, Point3::origin(), 1.0)
                .unwrap()
                .is_some(),
            "the zone's region"
        );
        let (mut nulled, nulled_face) = sheet();
        super::region::with_null_edge(&mut nulled, nulled_face);
        assert!(
            sphere_face_region(&nulled, nulled_face, Point3::origin(), 1.0)
                .unwrap()
                .is_none(),
            "a null edge refuses"
        );
    }
}
