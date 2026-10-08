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
//!    sits at.
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
//! circle ([`ConicArc::hit`], the boundary reading the planar loop walk
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
//! tangency of the ray with an arc (the root door's `Uncertain`), or an
//! arc lying in the ray's plane grazes; a direction with no tangent
//! share, or a great circle that meets no arc (it lies wholly on one
//! side), gives nothing to read; and the next is tried. Root counts that
//! disagree refuse the query: a broken invariant, not a ray's
//! conditioning.
//!
//! The boundary reading is of the ARC, not its circle: a point in the
//! band of an arc's great circle where the circle runs on past the arc's
//! ends, and clear of both ends, is off the boundary ([`ConicArc::hit`]).
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
//! - `bool_sphere_region_arc_{span,on,end,trim}`: the boundary reading
//!   of an arc ([`ConicArc::hit`]'s rows, on a circle): its gap to a
//!   period, `p`'s distance from its circle and from either end, and
//!   which side of the ends `p` is — or `p`'s foot, where `p` is in the
//!   band of the circle. `bool_sphere_region_arc_straddle` is
//!   the name an ellipse's straddle would carry; a circle never does.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use super::BooleanDecision;
use super::circle_roots::{
    CircleRoots, FirstHarmonic, FirstHarmonicRows, first_harmonic_roots_in_band, rounding_charge,
};
use super::solid_contain::PointInSolidError;
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, LoopBoundary, LoopKey};
use crate::ray_walk::{self, Crossings, RayFault};
use crate::splitting::PointInLoopError;
use crate::splitting::containment::{ConicArc, ConicArcError, ConicHit, ConicRows, SCHEDULE};
use crate::validate::decide;

const ROOT_ROWS: FirstHarmonicRows = FirstHarmonicRows {
    noise: "bool_sphere_region_roots_noise",
    coaxial: "bool_sphere_region_roots_coaxial",
    extreme: "bool_sphere_region_roots_extreme",
    root_slack: "bool_sphere_region_roots_slack",
    decision: BooleanDecision::CONTAINMENT_UNNAMED,
};

/// The boundary reading's rows (module docs).
const BOUNDARY: ConicRows = ConicRows {
    span: "bool_sphere_region_arc_span",
    on: "bool_sphere_region_arc_on",
    end: "bool_sphere_region_arc_end",
    trim: "bool_sphere_region_arc_trim",
    straddle: "bool_sphere_region_arc_straddle",
};

/// Where along each arc an aimed ray's target lies, as shares of its
/// span.
const TARGET_SHARES: [f64; 3] = [0.5, 0.25, 0.75];

/// Why [`SphereFaceRegion::contains`] gave no answer. Each door that
/// reads the region names the face it read.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum RegionRefusal {
    /// The point is in the band of a boundary arc; or no ray decided,
    /// and the first one set aside read in band; or, the walk's refusal
    /// of the whole query, two computations of one root count disagreed
    /// (`bool_sphere_region_roots_count`, an invalid margin).
    Escalated(Indeterminate),
    /// No ray settled ([`ray_walk::NoRaySettled`]): each grazed or gave
    /// nothing to read. `outer` is the face's outer loop, the loop the
    /// solid door names as the region walk's.
    RayExhausted {
        /// The face's outer loop.
        outer: LoopKey,
    },
    /// A boundary arc winds definitely past a whole turn, which bounds
    /// no region.
    WoundPastPeriod,
}

impl RegionRefusal {
    /// The refusal as the point-in-solid door states it, for a point on
    /// `face`'s sphere: a region walk no ray of which settled is that
    /// face's loop walk's, as a planar face's is
    /// ([`PointInSolidError::Loop`]) — not the solid's own sweep, whose
    /// exhaustion says the point is off the solid's boundary.
    pub(crate) fn of_face(self, face: FaceKey) -> PointInSolidError {
        match self {
            Self::Escalated(diag) => PointInSolidError::Escalated { face, diag },
            Self::RayExhausted { outer } => {
                PointInSolidError::Loop(PointInLoopError::RayExhausted { r#loop: outer })
            }
            Self::WoundPastPeriod => PointInSolidError::CorruptFace { face },
        }
    }
}

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
    arcs: Vec<RegionArc<T>>,
}

/// One boundary arc: the circle `center + radius·(û cos θ + v̂ sin θ)`,
/// `v̂ = axis × û`, over its certified span `[t0, t1]`.
#[derive(Clone, Debug)]
struct RegionArc<T: geom_core::Real> {
    edge: EdgeKey,
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    u_ref: Vec3<T>,
    t0: T,
    t1: T,
    /// Whether the loop runs `t0 → t1` (its `he_plus`).
    forward: bool,
}

impl<T: Decide> RegionArc<T> {
    fn at(&self, theta: T) -> Point3<T> {
        let (s, c) = theta.sin_cos();
        let v_ref = self.axis.cross(self.u_ref);
        self.center + self.u_ref * (self.radius * c) + v_ref * (self.radius * s)
    }

    /// The loop's unit traversal direction at `theta`.
    fn traversal(&self, theta: T) -> Vec3<T> {
        let d = self.axis.cross(self.at(theta) - self.center);
        let d = d / d.norm();
        if self.forward { d } else { -d }
    }
}

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
    let corrupt = || PointInSolidError::CorruptFace { face };
    let f = body.get_face(face).ok_or_else(corrupt)?;
    let mut arcs = Vec::new();
    for (i, &lk) in core::iter::once(&f.outer).chain(&f.rings).enumerate() {
        let first = match body.get_loop(lk).ok_or_else(corrupt)?.boundary {
            LoopBoundary::Cycle { first } => first,
            // A lone ring vertex bounds no area; an outer loop must.
            LoopBoundary::Empty { .. } if i > 0 => continue,
            LoopBoundary::Empty { .. } => return Err(corrupt()),
        };
        for he in body.loop_cycle(first).ok_or_else(corrupt)? {
            let edge_key = body.get_half_edge(he).ok_or_else(corrupt)?.edge;
            let edge = body.get_edge(edge_key).ok_or_else(corrupt)?;
            let Some(curve) = body.edge_curve_linked(edge_key, edge).certified() else {
                return Ok(None);
            };
            let geom::Curve3::Circle {
                center: c_c,
                axis,
                radius: r_c,
                u_ref,
            } = *curve.carrier()
            else {
                return Ok(None);
            };
            let (t0, t1) = curve.params();
            arcs.push(RegionArc {
                edge: edge_key,
                center: c_c,
                axis,
                radius: r_c,
                u_ref,
                t0,
                t1,
                forward: he == edge.he_plus,
            });
        }
    }
    let seam = |e: EdgeKey| arcs.iter().filter(|a: &&RegionArc<T>| a.edge == e).count() > 1;
    let seams: Vec<EdgeKey> = arcs.iter().map(|a| a.edge).filter(|&e| seam(e)).collect();
    arcs.retain(|a| !seams.contains(&a.edge));
    Ok(Some(SphereFaceRegion {
        center,
        radius,
        sense: f.sense,
        outer: f.outer,
        arcs,
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
        if self.arcs.is_empty() {
            return Ok(Some(true));
        }
        for arc in &self.arcs {
            let circle = geom::Curve3::Circle {
                center: arc.center,
                axis: arc.axis,
                radius: arc.radius,
                u_ref: arc.u_ref,
            };
            let k = match ConicArc::of(&circle, (arc.t0, arc.t1), BOUNDARY, band) {
                Ok(Some(k)) => k,
                Err(ConicArcError::Escalated(e)) => return Err(RegionRefusal::Escalated(e.diag)),
                Ok(None) | Err(ConicArcError::WoundPastPeriod) => {
                    return Err(RegionRefusal::WoundPastPeriod);
                }
            };
            match k
                .hit(p, BOUNDARY, band)
                .map_err(|e| RegionRefusal::Escalated(e.diag))?
            {
                ConicHit::On | ConicHit::End => return Ok(None),
                ConicHit::Off | ConicHit::Carrier => {}
            }
        }
        let w = p - self.center;
        let a = w / w.norm();
        let aimed: Vec<Vec3<T>> = self
            .arcs
            .iter()
            .flat_map(|arc| {
                TARGET_SHARES.map(|share| {
                    arc.at(arc.t0 + (arc.t1 - arc.t0) * T::from_f64(share)) - self.center
                })
            })
            .collect();
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
                    Ok(Sign::Positive) => self.ray(a, tangent(a, raw), band),
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

    /// The ray from `a` along the unit tangent `t` (module docs): whether
    /// `p` is in the face.
    fn ray(&self, a: Vec3<T>, t: Vec3<T>, band: Band) -> Result<bool, RayFault<RegionRefusal>> {
        let r = self.radius;
        let in_band = |diag| RayFault::InBand(RegionRefusal::Escalated(diag));
        let row = |name, m: T| decide(name, Margin::of(m), band).map_err(in_band);
        let g = a.cross(t);
        let mut crossings = Crossings::new();
        for (j, arc) in self.arcs.iter().enumerate() {
            let thetas = match self.ray_roots(arc, g, band).map_err(in_band)? {
                CircleRoots::Miss => continue,
                CircleRoots::Certified { count, thetas } => thetas[..count].to_vec(),
                // The arc in the ray's plane, or a tangency; a plane has no
                // apex, so `AtApex` is the cone's and never reaches here.
                CircleRoots::OnSurface | CircleRoots::Uncertain | CircleRoots::AtApex => {
                    return Err(RayFault::Graze);
                }
                // Two computations of one count disagreeing is a broken
                // invariant, not a ray's conditioning (D9); every caller of
                // the root doors refuses on it.
                CircleRoots::CountDisagrees => {
                    return Err(RayFault::Fatal(RegionRefusal::Escalated(
                        crate::invalid_margin::invalid(band, "bool_sphere_region_roots_count"),
                    )));
                }
            };
            for theta in thetas {
                let start = row("bool_sphere_region_span", arc.radius * (theta - arc.t0))?;
                let end = row("bool_sphere_region_span", arc.radius * (arc.t1 - theta))?;
                if start == Sign::Negative || end == Sign::Negative {
                    continue;
                }
                let y = arc.at(theta) - self.center;
                // `s ∈ (−π, π]`, read from `p` either way round the great
                // circle, which has no behind: only its distance from `p`.
                let s = y.dot(t).atan2(y.dot(a));
                ray_walk::apart("bool_sphere_region_at", Margin::of((r * s).abs()), band)
                    .map_err(|f| f.map(RegionRefusal::Escalated))?;
                let s = (T::zero() - s).select_le_zero(s, s + T::tau());
                crossings.push(r * s, (j, theta), start == Sign::Zero || end == Sign::Zero);
            }
        }
        let (arc, theta) = crossings
            .closest("bool_sphere_region_order", band, |diag, _| {
                RegionRefusal::Escalated(diag)
            })?
            // A great circle meeting no arc lies wholly on one side.
            .ok_or(RayFault::Unread)?;
        let y = self.arcs[arc].at(theta) - self.center;
        let y = y / y.norm();
        let outward = geom_brep::OutwardNormal::from_chart(y, self.sense).vec();
        let face_side = outward.cross(self.arcs[arc].traversal(theta));
        let heading = g.cross(y);
        match row("bool_sphere_region_cross", r * heading.dot(face_side))? {
            Sign::Negative => Ok(true),
            Sign::Positive => Ok(false),
            Sign::Zero => Err(RayFault::Graze),
        }
    }

    /// `arc`'s crossings with the ray's plane through the centre, normal
    /// `g` (module docs, step 2).
    fn ray_roots(
        &self,
        arc: &RegionArc<T>,
        g: Vec3<T>,
        band: Band,
    ) -> Result<CircleRoots<T>, Indeterminate> {
        let v_ref = arc.axis.cross(arc.u_ref);
        let offset = arc.center - self.center;
        let d = g.dot(offset);
        let (cos_part, sin_part) = (arc.radius * g.dot(arc.u_ref), arc.radius * g.dot(v_ref));
        let amplitude = (cos_part.powi(2) + sin_part.powi(2)).sqrt();
        let noise = rounding_charge(offset.norm() + arc.radius);
        first_harmonic_roots_in_band(
            &FirstHarmonic {
                lo: d - amplitude,
                hi: d + amplitude,
                cos_part,
                sin_part,
                lo_noise: noise,
                hi_noise: noise,
                phase_noise: T::zero(),
            },
            arc.radius,
            arc.t0,
            arc.t1,
            &ROOT_ROWS,
            band,
        )
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
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
            arcs: vec![RegionArc {
                edge: EdgeKey::from(KeyData::from_ffi(1)),
                center: Point3::new(0.0, 0.0, 0.5),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: 0.75f64.sqrt(),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
                t0: 0.0,
                t1: core::f64::consts::TAU,
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
                                if diag.predicate == Some("bool_sphere_region_arc_on")
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
        let meridian = |az: f64, forward: bool, key: u64| RegionArc {
            edge: EdgeKey::from(KeyData::from_ffi(key)),
            center: Point3::origin(),
            axis: Vec3::new(-az.sin(), az.cos(), 0.0),
            radius: 1.0,
            u_ref: Vec3::new(0.0, 0.0, 1.0),
            t0: 0.0,
            t1: core::f64::consts::PI,
            forward,
        };
        SphereFaceRegion {
            center: Point3::origin(),
            radius: 1.0,
            sense: true,
            outer: LoopKey::default(),
            arcs: vec![
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
    /// refused every one of these on `bool_sphere_region_arc_on`.
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
    /// refused on the arc's distance, past it on its side — the lune
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
                            if diag.predicate == Some("bool_sphere_region_arc_on")
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
}
