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
//! the face (both its half-edges in the face's loops) is no boundary of
//! it, and its two crossings, which coincide, are dropped together. A
//! pole is no point of interest, because nothing is read in the chart.
//!
//! # Which rays
//!
//! First the rays aimed at points of the boundary — three points of every
//! arc, in loop order — each of which meets the boundary somewhere; then
//! the fixed schedule's directions projected onto the tangent plane at
//! `p` (`splitting::containment::SCHEDULE`), for the points where every
//! aimed ray runs along an arc: a point a hair off a vertex sees every
//! arc through that vertex nearly edge-on.
//!
//! # Grazing
//!
//! A ray is never allowed to decide borderline geometry: a closest
//! crossing at a vertex or along its arc, a tie between two crossings
//! that are not one edge's pair, a tangency of the ray with an arc (the
//! root door's `Uncertain`), an arc lying in the ray's plane, or a ray
//! that meets no arc abandons the ray, and the next is tried. A crossing
//! at `p` itself is the point on the boundary, which is an answer
//! ([`None`]) whatever ray found it.
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

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use super::BooleanDecision;
use super::circle_roots::{
    CircleRoots, FirstHarmonic, FirstHarmonicRows, first_harmonic_roots, rounding_charge,
};
use super::solid_contain::PointInSolidError;
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, LoopBoundary, LoopKey};
use crate::splitting::containment::{PointInLoopError, SCHEDULE};
use crate::validate::decide;

const ROOT_ROWS: FirstHarmonicRows = FirstHarmonicRows {
    noise: "bool_sphere_region_roots_noise",
    coaxial: "bool_sphere_region_roots_coaxial",
    extreme: "bool_sphere_region_roots_extreme",
    root_slack: "bool_sphere_region_roots_slack",
    decision: BooleanDecision::Containment,
};

/// Where along each arc an aimed ray's target lies, as shares of its
/// span.
const TARGET_SHARES: [f64; 3] = [0.5, 0.25, 0.75];

/// A trimmed sphere face's boundary: every arc of every loop, each with
/// the direction its loop traverses it.
#[derive(Clone, Debug)]
pub(crate) struct SphereFaceRegion<T: geom_core::Real> {
    center: Point3<T>,
    radius: T,
    /// The face's sense bit: `false` points its outward normal at the
    /// centre.
    sense: bool,
    /// The face's outer loop, which an exhausted reading names.
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

/// What one ray says about `p`.
enum Ray {
    Inside(bool),
    OnBoundary,
    Abandoned,
}

/// One root of one arc on a ray: the ray parameter, the arc, the root.
struct Hit<T> {
    s: T,
    arc: usize,
    theta: T,
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
            let Some(curve) = body
                .get_curve_geom(edge.curve)
                .and_then(crate::null::CurveGeom::certified)
            else {
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
    /// When no ray decides: the first in-band reading as
    /// [`PointInSolidError::Escalated`], else
    /// [`PointInLoopError::RayExhausted`] on the outer loop. Both are
    /// inconclusive about `p` alone ([`PointInSolidError::inconclusive`]).
    pub(crate) fn contains(
        &self,
        face: FaceKey,
        p: Point3<T>,
        band: Band,
    ) -> Result<Option<bool>, PointInSolidError> {
        let w = p - self.center;
        let a = w / w.norm();
        let aimed = self.arcs.iter().flat_map(|arc| {
            TARGET_SHARES.map(|share| {
                arc.at(arc.t0 + (arc.t1 - arc.t0) * T::from_f64(share)) - self.center
            })
        });
        let scheduled = SCHEDULE.iter().map(|r| r.map(T::from_f64));
        let mut first_diag = None;
        for toward in aimed.chain(scheduled) {
            let raw = toward - a * a.dot(toward);
            let arm = Margin::levered(raw.norm() / toward.norm(), self.radius);
            let read = match decide("bool_sphere_region_arm", arm, band) {
                Ok(Sign::Positive) => self.ray(a, tangent(a, raw), band),
                Ok(Sign::Zero | Sign::Negative) => continue,
                Err(diag) => Err(diag),
            };
            match read {
                Ok(Ray::Inside(inside)) => return Ok(Some(inside)),
                Ok(Ray::OnBoundary) => return Ok(None),
                Ok(Ray::Abandoned) => {}
                Err(diag) => {
                    first_diag.get_or_insert(diag);
                }
            }
        }
        Err(match first_diag {
            Some(diag) => PointInSolidError::Escalated { face, diag },
            None => PointInSolidError::Loop(PointInLoopError::RayExhausted { r#loop: self.outer }),
        })
    }

    /// The ray from `a` along the unit tangent `t` (module docs).
    fn ray(&self, a: Vec3<T>, t: Vec3<T>, band: Band) -> Result<Ray, Indeterminate> {
        let r = self.radius;
        let row = |name, m: T| decide(name, Margin::of(m), band);
        let g = a.cross(t);
        let mut hits: Vec<Hit<T>> = Vec::new();
        for (j, arc) in self.arcs.iter().enumerate() {
            let thetas = match self.ray_roots(arc, g, band)? {
                CircleRoots::Miss => continue,
                CircleRoots::Certified { count, thetas } => thetas[..count].to_vec(),
                CircleRoots::OnSurface | CircleRoots::Uncertain | CircleRoots::CountDisagrees => {
                    return Ok(Ray::Abandoned);
                }
            };
            for theta in thetas {
                let start = row("bool_sphere_region_span", arc.radius * (theta - arc.t0))?;
                let end = row("bool_sphere_region_span", arc.radius * (arc.t1 - theta))?;
                if start == Sign::Negative || end == Sign::Negative {
                    continue;
                }
                let y = arc.at(theta) - self.center;
                let s = y.dot(t).atan2(y.dot(a));
                if row("bool_sphere_region_at", r * s)? == Sign::Zero {
                    return Ok(Ray::OnBoundary);
                }
                if start == Sign::Zero || end == Sign::Zero {
                    return Ok(Ray::Abandoned);
                }
                let s = (T::zero() - s).select_le_zero(s, s + T::tau());
                hits.push(Hit { s, arc: j, theta });
            }
        }
        let tied = |x: T, y: T| {
            matches!(
                row("bool_sphere_region_order", r * (x - y)),
                Ok(Sign::Zero) | Err(_)
            )
        };
        loop {
            let Some(first) = self.closest(&hits, band) else {
                return Ok(Ray::Abandoned);
            };
            let Hit { s, arc, theta } = hits[first];
            // One edge's two half-edges both in the face: no boundary.
            let pair = hits.iter().position(|h| {
                h.arc != arc
                    && self.arcs[h.arc].edge == self.arcs[arc].edge
                    && self.arcs[h.arc].forward != self.arcs[arc].forward
                    && tied(h.s, s)
            });
            if let Some(other) = pair {
                hits.remove(first.max(other));
                hits.remove(first.min(other));
                continue;
            }
            if hits
                .iter()
                .enumerate()
                .any(|(k, h)| k != first && tied(h.s, s))
            {
                return Ok(Ray::Abandoned);
            }
            let y = self.arcs[arc].at(theta) - self.center;
            let y = y / y.norm();
            let outward = geom_brep::OutwardNormal::from_chart(y, self.sense).vec();
            let face_side = outward.cross(self.arcs[arc].traversal(theta));
            let heading = g.cross(y);
            return Ok(
                match row("bool_sphere_region_cross", r * heading.dot(face_side))? {
                    Sign::Negative => Ray::Inside(true),
                    Sign::Positive => Ray::Inside(false),
                    Sign::Zero => Ray::Abandoned,
                },
            );
        }
    }

    /// The index of the hit with the least ray parameter, `None` for no
    /// hit. A comparison that does not decide keeps the earlier hit; the
    /// caller refuses any tie with the one returned.
    fn closest(&self, hits: &[Hit<T>], band: Band) -> Option<usize> {
        let mut best: Option<usize> = None;
        for (k, h) in hits.iter().enumerate() {
            best = match best {
                Some(b) => {
                    let ahead = Margin::of(self.radius * (h.s - hits[b].s));
                    match decide("bool_sphere_region_order", ahead, band) {
                        Ok(Sign::Negative) => Some(k),
                        Ok(Sign::Zero | Sign::Positive) | Err(_) => Some(b),
                    }
                }
                None => Some(k),
            };
        }
        best
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
        first_harmonic_roots(
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
        .map_err(|e| match e {
            super::BooleanError::Escalated { diag, .. } => diag,
            other => unreachable!("the first-harmonic door raises only escalations: {other:?}"),
        })
    }
}
