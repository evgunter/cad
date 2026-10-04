//! **A trimmed sphere face's region, read from its boundary arcs** — the
//! one reading both point doors take of a sphere face that is not closed
//! on its own surface: the ray lane's hit test and boundary pre-pass
//! ([`super::solid_contain`]) and the pierce arm's landing test
//! ([`super::contain::curved_face_containment`]).
//!
//! # Method: parity along a geodesic to a boundary target
//!
//! Every boundary edge of such a face is a circle arc on the sphere (a
//! plane's section of it). The face is the side of its loops to their
//! left under its OUTWARD normal `N = σ·(p − c)/R` (`σ` the sense bit) —
//! the convention the flux lane's Gauss–Bonnet area reads
//! (`geom_brep::props`' sphere circle loop) — so a point just to that side
//! of an arc's interior is in the face. That is a reference with no chart
//! in it, and parity carries it to the query:
//!
//! 1. Pick a TARGET `m`, a point inside one arc's span.
//! 2. Walk the minor great-circle arc from `p` to `m`. Every arc of every
//!    loop meets the walk's plane `ĝ·(x − c) = 0` (`ĝ ∝ (p − c) × (m − c)`)
//!    at the roots of a first harmonic,
//!    `ĝ·(C − c) + ρ(ĝ·û) cos θ + ρ(ĝ·v̂) sin θ`, certified by the shared
//!    first-harmonic door ([`super::circle_roots::first_harmonic_roots`]);
//!    a root counts where it lies inside its arc's span and strictly
//!    between `p` and `m` on the walk.
//! 3. The walk arrives at `m` from the face's side or from the other,
//!    which the target arc's traversal tangent `τ` says: the face side at
//!    `m` is `N × τ`, and the walk arrives along `ĝ × m̂`.
//!
//! `p` is in the face exactly when the arrival side and the crossing
//! count's parity disagree in the obvious way: from inside with an even
//! count, or from outside with an odd one.
//!
//! The walk reads every loop of the face at once, rings included, so a
//! ringed face needs no case of its own; an edge both of whose sides are
//! the face (a seam inside one loop) is crossed twice and changes nothing.
//! A pole is no point of interest here, because nothing is read in the
//! chart.
//!
//! # Grazing
//!
//! A walk is never allowed to decide borderline geometry: a root at a
//! vertex, a root at the target other than the target itself, a tangency
//! of the walk with an arc (the door's `Uncertain`), an arc lying in the
//! walk's plane, an antipodal target, or an arrival along the target arc
//! abandons the walk, and the next target is tried — three points of
//! every arc, in loop order. A root at `p` is the point on the boundary,
//! which is an answer ([`None`]) whatever walk found it.
//!
//! # Predicates (meters)
//!
//! - `bool_sphere_region_target`: the chord from `p` to a target, and
//!   from `p` to the target's antipode — `p` AT the target is on the
//!   boundary; at its antipode the walk has no plane.
//! - `bool_sphere_region_span`: a root's parameter against its arc's
//!   span ends, at the arc's radius.
//! - `bool_sphere_region_walk`: a root's place along the walk, each
//!   side's sine at the sphere's radius.
//! - `bool_sphere_region_arrive`: the sine between the walk and the
//!   target arc at the target, at the sphere's radius.
//! - `bool_sphere_region_roots_*`: the first-harmonic door's own meters.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use super::BooleanDecision;
use super::circle_roots::{
    CircleRoots, FirstHarmonic, FirstHarmonicRows, first_harmonic_roots, rounding_charge,
};
use super::solid_contain::PointInSolidError;
use crate::body::Body;
use crate::entity::{FaceKey, LoopBoundary, LoopKey};
use crate::splitting::containment::PointInLoopError;
use crate::validate::decide;

const ROOT_ROWS: FirstHarmonicRows = FirstHarmonicRows {
    noise: "bool_sphere_region_roots_noise",
    coaxial: "bool_sphere_region_roots_coaxial",
    extreme: "bool_sphere_region_roots_extreme",
    root_slack: "bool_sphere_region_roots_slack",
    decision: BooleanDecision::Containment,
};

/// Where along each arc a target is tried, as shares of its span.
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

/// What one walk says about `p`.
enum Walk {
    Inside(bool),
    OnBoundary,
    Abandoned,
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
    /// When no target's walk decides: the first in-band reading as
    /// [`PointInSolidError::Escalated`], else
    /// [`PointInLoopError::RayExhausted`] on the outer loop. Both are
    /// inconclusive about `p` alone ([`PointInSolidError::inconclusive`]).
    pub(crate) fn contains(
        &self,
        face: FaceKey,
        p: Point3<T>,
        band: Band,
    ) -> Result<Option<bool>, PointInSolidError> {
        let unit = |x: Point3<T>| {
            let w = x - self.center;
            w / w.norm()
        };
        let a = unit(p);
        let mut first_diag = None;
        for (i, arc) in self.arcs.iter().enumerate() {
            for share in TARGET_SHARES {
                let t_m = arc.t0 + (arc.t1 - arc.t0) * T::from_f64(share);
                match self.walk(a, unit(arc.at(t_m)), i, t_m, band) {
                    Ok(Walk::Inside(inside)) => return Ok(Some(inside)),
                    Ok(Walk::OnBoundary) => return Ok(None),
                    Ok(Walk::Abandoned) => {}
                    Err(diag) => {
                        first_diag.get_or_insert(diag);
                    }
                }
            }
        }
        Err(match first_diag {
            Some(diag) => PointInSolidError::Escalated { face, diag },
            None => PointInSolidError::Loop(PointInLoopError::RayExhausted { r#loop: self.outer }),
        })
    }

    /// The walk from `a` to the target `b` on arc `target` at `t_m`, both
    /// unit directions from the centre.
    fn walk(
        &self,
        a: Vec3<T>,
        b: Vec3<T>,
        target: usize,
        t_m: T,
        band: Band,
    ) -> Result<Walk, Indeterminate> {
        let r = self.radius;
        let row = |name, m: T| decide(name, Margin::of(m), band);
        if row("bool_sphere_region_target", r * (a - b).norm())? == Sign::Zero {
            return Ok(Walk::OnBoundary);
        }
        if row("bool_sphere_region_target", r * (a + b).norm())? == Sign::Zero {
            return Ok(Walk::Abandoned);
        }
        let g = a.cross(b);
        let g = g / g.norm();
        let mut crossings = 0_usize;
        let mut met_target = false;
        for (j, arc) in self.arcs.iter().enumerate() {
            let thetas = match self.walk_roots(arc, g, band)? {
                CircleRoots::Miss => continue,
                CircleRoots::Certified { count, thetas } => thetas[..count].to_vec(),
                CircleRoots::OnSurface | CircleRoots::Uncertain | CircleRoots::CountDisagrees => {
                    return Ok(Walk::Abandoned);
                }
            };
            for theta in thetas {
                let start = row("bool_sphere_region_span", arc.radius * (theta - arc.t0))?;
                let end = row("bool_sphere_region_span", arc.radius * (arc.t1 - theta))?;
                if start == Sign::Negative || end == Sign::Negative {
                    continue;
                }
                let at_vertex = start == Sign::Zero || end == Sign::Zero;
                let w = arc.at(theta) - self.center;
                let y = w / w.norm();
                let after_a = row("bool_sphere_region_walk", r * a.cross(y).dot(g))?;
                let before_b = row("bool_sphere_region_walk", r * y.cross(b).dot(g))?;
                match (after_a, before_b) {
                    (Sign::Negative, _) | (_, Sign::Negative) => {}
                    (Sign::Zero, _) => return Ok(Walk::OnBoundary),
                    (Sign::Positive, Sign::Zero) => {
                        if j != target || met_target || at_vertex {
                            return Ok(Walk::Abandoned);
                        }
                        met_target = true;
                    }
                    (Sign::Positive, Sign::Positive) => {
                        if at_vertex {
                            return Ok(Walk::Abandoned);
                        }
                        crossings += 1;
                    }
                }
            }
        }
        if !met_target {
            return Ok(Walk::Abandoned);
        }
        let outward = if self.sense { b } else { -b };
        let face_side = outward.cross(self.arcs[target].traversal(t_m));
        let arrival = g.cross(b);
        let from_face = match row(
            "bool_sphere_region_arrive",
            r * (T::zero() - arrival.dot(face_side)),
        )? {
            Sign::Positive => true,
            Sign::Negative => false,
            Sign::Zero => return Ok(Walk::Abandoned),
        };
        Ok(Walk::Inside(from_face != (crossings % 2 == 1)))
    }

    /// `arc`'s crossings with the walk's plane through the centre, normal
    /// `g` (module docs, step 2).
    fn walk_roots(
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
