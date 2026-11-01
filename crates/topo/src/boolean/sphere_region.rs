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
//!    the shared first-harmonic door, each root as a direction from the
//!    arc's centre ([`super::circle_roots::first_harmonic_root_directions`]);
//!    a root counts where it lies inside its arc's span, at the ray
//!    parameter `s` it sits at. Neither reading is an angle with a cut: a
//!    root is read against its span by its angle from the span's middle,
//!    and a crossing along the ray by the angle swept from `p`, each the
//!    unsigned `2·atan2` of two norms ([`region::between`]), so a root a
//!    half turn from its arc's middle and a crossing at `p`'s antipode —
//!    every meridian ray from a pole meets the other pole there — read as
//!    tightly as any. The ray is a leg of the region skeleton
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
//! arc, in walk order — each of which meets the boundary somewhere; then
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
use super::circle_roots::{FirstHarmonic, FirstHarmonicRows, first_harmonic_root_directions};
use super::region::{
    self, Fault, Landing, Leg, Legs, Region, RegionEdge, RegionRefusal, Round, Scaffolding,
    between, circle_edge_hit, found, span_from_middle, targets,
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
const BOUNDARY: ConicRows = ConicRows::bounded(
    "bool_sphere_region_arc_span",
    "bool_sphere_region_arc_straddle",
);

/// The fraction of each arc's span [`SphereFaceRegion::aim`] steps from:
/// the golden section ([`geom_core::GOLDEN_SECTION`]), off the middle a
/// symmetric pose's planes hold.
const AIM_FRACTION: f64 = 1.0 - geom_core::GOLDEN_SECTION;

/// How many even steps of each arc's span [`SphereFaceRegion::aim`]'s
/// clearance reads.
const CLEARANCE_SAMPLES: usize = 32;

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

    /// A point of the face the side-at-infinity reader may aim at, found
    /// by evaluation alone: the point `m` of the face's first arc (the
    /// outer loop's first, in walk order) at [`AIM_FRACTION`] of its span,
    /// stepped along its great circle toward the face (`N × τ`, module
    /// docs) by an angle `φ` that keeps the step's path off every boundary
    /// arc. `φ` is at most half the arc's circle's angular radius about
    /// the nearer pole, so the path does not meet that circle again, and
    /// `R·φ` at most half a lower bound on `m`'s distance from every other
    /// arc ([`Self::clearance`]), so it meets no other arc: the stepped
    /// point lies in the face, as the face lies along `N × τ` at `m`. One
    /// arc is stepped from, by its place in the walk: choosing among
    /// arcs by their steps would be a choice on values, which two arcs a
    /// symmetric face holds alike tie, and an enclosure of a tie is the
    /// hull of both steps. A face narrower than the rounding there steps
    /// nowhere and aims at `m`; the aim only steers either way. `None`
    /// for a face with no boundary arc, which offers no aim.
    pub(crate) fn aim(&self) -> Option<Point3<T>> {
        let half = T::from_f64(0.5);
        let arc = self.edges.first()?;
        let geom::Curve3::Circle { radius, .. } = arc.carrier else {
            unreachable!("sphere_face_region admits circles only")
        };
        let t = arc.t0 + (arc.t1 - arc.t0) * T::from_f64(AIM_FRACTION);
        let m = arc.at(t);
        let own = (radius / self.radius).abs().min(T::one()).asin() * half;
        let room = self.clearance(0, m) * half / self.radius;
        let phi = own.min(room).max(T::zero());
        let w = m - self.center;
        let into = self.outward(m).cross(arc.traversal(t));
        let (sin, cos) = phi.sin_cos();
        Some(self.center + (w / w.norm() * cos + into / into.norm() * sin) * self.radius)
    }

    /// A lower bound on the distance from `m` to every arc but the `k`th:
    /// per arc, the greater of two, each never above the true distance —
    /// `m`'s distance from the arc's whole circle, and the least distance
    /// from `m` to the arc's points at [`CLEARANCE_SAMPLES`] even steps of
    /// its span less half the arc length between two of them (every point
    /// of the arc lies within that of one), which reads an arc of `m`'s own
    /// circle whose circle passes through `m`.
    fn clearance(&self, k: usize, m: Point3<T>) -> T {
        let n = T::from_f64(CLEARANCE_SAMPLES as f64);
        let mut least: Option<T> = None;
        for (j, arc) in self.edges.iter().enumerate() {
            if j == k {
                continue;
            }
            let geom::Curve3::Circle {
                center,
                axis,
                radius,
                ..
            } = arc.carrier
            else {
                unreachable!("sphere_face_region admits circles only")
            };
            let d = m - center;
            let h = d.dot(axis);
            let circle = (h.powi(2) + ((d - axis * h).norm() - radius.abs()).powi(2)).sqrt();
            let span = arc.t1 - arc.t0;
            let step = span / n;
            let (mut t, mut near) = (arc.t0, (arc.at(arc.t0) - m).norm());
            for _ in 0..CLEARANCE_SAMPLES {
                t = t + step;
                near = near.min((arc.at(t) - m).norm());
            }
            let gap = circle.max(near - (radius * step).abs() * T::from_f64(0.5));
            least = Some(least.map_or(gap, |l| l.min(gap)));
        }
        // A face of one arc: nothing else to keep off.
        least.unwrap_or(self.radius * T::from_f64(4.0))
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

    fn edges_mut(&mut self) -> &mut Vec<Arc<T>> {
        &mut self.edges
    }

    fn outer(&self) -> LoopKey {
        self.outer
    }

    fn on_edge(&self, arc: &Arc<T>, p: Point3<T>, band: Band) -> Result<bool, RegionRefusal> {
        circle_edge_hit(
            &arc.carrier,
            arc.turn.as_ref(),
            BOUNDARY,
            p,
            self.outer,
            band,
        )
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
        let roots = first_harmonic_root_directions(
            &FirstHarmonic::circle_off_plane(
                (center, axis, radius, u_ref),
                (self.center, ray.course.normal),
            ),
            radius,
            &ROOT_ROWS,
            band,
        );
        let v_ref = axis.cross(u_ref);
        let two = T::from_f64(2.0);
        let (mid, half) = ((arc.t0 + arc.t1) / two, (arc.t1 - arc.t0) / two);
        let (sin_mid, cos_mid) = mid.sin_cos();
        let middle = u_ref * cos_mid + v_ref * sin_mid;
        let mut out = Vec::new();
        for (cos, sin) in found(roots)? {
            let toward = u_ref * cos + v_ref * sin;
            let off = between(toward, middle);
            let Some(sideless) =
                span_from_middle("bool_sphere_region_span", off, half, radius, band)?
            else {
                continue;
            };
            // Which way round from the middle: the side of `middle`'s
            // diameter `toward` lies, which only a root a half turn off
            // the middle leaves undecided, and that one lies at a whole
            // turn's ends, sideless.
            let ahead = middle.cross(toward).dot(axis);
            let theta = (T::zero() - ahead).select_le_zero(mid + off, mid - off);
            out.push((center + toward * radius, theta, sideless));
        }
        Ok(out)
    }

    fn outward(&self, y: Point3<T>) -> Vec3<T> {
        let w = y - self.center;
        geom_brep::OutwardNormal::from_chart(w / w.norm(), self.sense).vec()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
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
                cycle: 0,
                carrier: geom::Curve3::Circle {
                    center: Point3::new(0.0, 0.0, 0.5),
                    axis: Vec3::new(0.0, 0.0, 1.0),
                    radius: 0.75f64.sqrt(),
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                },
                t0: 0.0,
                t1: core::f64::consts::TAU,
                turn: crate::boolean::region::fixture_turn(0.0, core::f64::consts::TAU),
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
            cycle: 0,
            carrier: geom::Curve3::Circle {
                center: Point3::origin(),
                axis: Vec3::new(-az.sin(), az.cos(), 0.0),
                radius: 1.0,
                u_ref: Vec3::new(0.0, 0.0, 1.0),
            },
            t0: 0.0,
            t1: core::f64::consts::PI,
            turn: crate::boolean::region::fixture_turn(0.0, core::f64::consts::PI),
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

    /// An arc of the unit sphere's circle at polar angle `theta` about
    /// `pole`, from `t0` to `t1` in the frame `u_ref`, `pole × u_ref`.
    fn circle_arc(
        pole: Vec3<f64>,
        theta: f64,
        u_ref: Vec3<f64>,
        (t0, t1): (f64, f64),
        forward: bool,
        key: u64,
    ) -> RegionEdge<f64> {
        let pole = pole / pole.norm();
        let u = u_ref - pole * pole.dot(u_ref);
        RegionEdge {
            edge: EdgeKey::from(KeyData::from_ffi(key)),
            r#loop: LoopKey::default(),
            cycle: 0,
            carrier: geom::Curve3::Circle {
                center: Point3::origin() + pole * theta.cos(),
                axis: pole,
                radius: theta.sin(),
                u_ref: u / u.norm(),
            },
            t0,
            t1,
            turn: crate::boolean::region::fixture_turn(t0, t1),
            forward,
        }
    }

    /// The circle at polar angle `theta` about `z` in `n` equal arcs from
    /// azimuth `u`, each run `forward`.
    fn rim(theta: f64, u: Vec3<f64>, n: usize, forward: bool, key: u64) -> Vec<RegionEdge<f64>> {
        let z = Vec3::new(0.0, 0.0, 1.0);
        let step = core::f64::consts::TAU / n as f64;
        (0..n)
            .map(|i| {
                let span = (step * i as f64, step * (i + 1) as f64);
                circle_arc(z, theta, u, span, forward, key + i as u64)
            })
            .collect()
    }

    /// A unit-sphere face of `sense` bounded by `edges`.
    fn face(sense: bool, edges: Vec<RegionEdge<f64>>) -> SphereFaceRegion<f64> {
        SphereFaceRegion {
            center: Point3::origin(),
            radius: 1.0,
            sense,
            outer: LoopKey::default(),
            edges,
        }
    }

    /// **The aim lands in the face**, on the sphere, for both senses:
    /// caps either side of a rim, a lune, zones whose rims' seams lie
    /// apart, a zone a thousandth wide, rims in unequal counts of arcs, a
    /// zone symmetric about the equator, and caps with a ring where the
    /// rim's step would land and beside a great arc. A mean of steps, or a
    /// step blind to the face's width, lands outside on most of them.
    #[test]
    fn the_aim_lies_in_the_face() {
        use core::f64::consts::{FRAC_PI_2, PI, TAU};
        let band = Band::new(1e-9, 1e-8).unwrap();
        let (x, mx) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0));
        let z = Vec3::new(0.0, 0.0, 1.0);
        let pole_at = |polar: f64, az: f64| at(polar, az) - Point3::origin();
        let mut shapes: Vec<(String, SphereFaceRegion<f64>)> = vec![
            ("cap".to_owned(), carved(true)),
            ("the rest".to_owned(), carved(false)),
            ("lune".to_owned(), lune()),
        ];
        for sense in [true, false] {
            // With `sense` false the outward normal points in, which turns
            // the face's side: every rim runs the other way to keep it.
            let f = |forward: bool| forward == sense;
            let zone = |hi: f64, lo: f64, u: Vec3<f64>, n: usize| {
                let mut e = rim(hi, x, 1, f(false), 1);
                e.extend(rim(lo, u, n, f(true), 10));
                face(sense, e)
            };
            let ringed = |rim_at: f64, pole: Vec3<f64>, ring: f64| {
                let mut e = rim(rim_at, x, 1, f(true), 1);
                e.push(circle_arc(pole, ring, z, (0.0, TAU), f(false), 20));
                face(sense, e)
            };
            let mut push = |what: &str, r| shapes.push((format!("sense {sense}: {what}"), r));
            push("zone, seams opposite", zone(0.6, 1.2, mx, 1));
            push("sliver zone, seams opposite", zone(1.0, 1.001, mx, 1));
            push("sliver zone, 1 and 3 arcs", zone(1.0, 1.001, x, 3));
            push("wide zone, 1 and 5 arcs", zone(0.3, 2.8, x, 5));
            push(
                "symmetric zone, seams opposite",
                zone(PI / 3.0, 2.0 * PI / 3.0, mx, 1),
            );
            push(
                "cap ringed at its rim's step",
                ringed(1.0, pole_at(0.5, PI), 0.1),
            );
            push(
                "cap ringed near its rim",
                ringed(1.2, pole_at(1.1, 0.0), 0.05),
            );
            push(
                "hemisphere ringed beside its great rim",
                ringed(FRAC_PI_2, pole_at(1.4, 0.3), 0.1),
            );
            push("tiny cap", face(sense, rim(1e-4, x, 1, f(true), 1)));
            push(
                "all but a tiny cap",
                face(sense, rim(1e-4, x, 1, f(false), 1)),
            );
        }
        for (what, region) in shapes {
            let aim = region.aim().expect("an aim");
            assert!(
                ((aim - Point3::origin()).norm() - 1.0).abs() < 1e-12,
                "{what}: {aim:?} is off the sphere"
            );
            assert_eq!(
                region.contains(aim, band),
                Ok(Some(true)),
                "{what}: {aim:?} is not in the face"
            );
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

    /// **A pole whose side the door reads in band refuses typed, with
    /// its margin** (D4 ¶1): the zone of the unit sphere between rims at
    /// heights `0.5` and `0.5 − 3ε·sin 60°`, `3ε` apart along every
    /// meridian, inside the band — the ball split by the upper rim, and
    /// the lower rim planted on the face below it as a ring. Each pole is
    /// far off the zone's boundary, and every great circle through it is
    /// a meridian, which meets the two rims at places the band cannot
    /// order: tier 3 refuses both poles as undecided sides carrying the
    /// order row's margin, never as a pass or a place. (The two caps the
    /// construction leaves each hold a pole, refused as they should be.)
    #[test]
    fn a_pole_read_in_band_refuses_typed_with_its_margin() {
        use crate::construction::{FaceSurface, MefSite, MevSite};
        use crate::entity::LoopBoundary;
        use geom_brep::EdgeCurveSpec;
        use geom_brep::chart_singularity::{SideUnread, Singularity, Undecided};
        let tol = geom_core::Tol::witness();
        let (pi, tau) = (core::f64::consts::PI, core::f64::consts::TAU);
        let (z1, z2) = (0.5, 0.5 - 3.0 * tol.eps() * (pi / 3.0).sin());
        let rim = |z: f64| {
            let r = (1.0f64 - z * z).sqrt();
            let circle = geom::Curve3::Circle {
                center: Point3::new(0.0, 0.0, z),
                axis: Vec3::unit_z(),
                radius: r,
                u_ref: Vec3::unit_x(),
            };
            (r, circle)
        };
        let mut body = Body::<f64>::new();
        let (r1, upper) = rim(z1);
        let seed = body.mvfs(Point3::new(r1, 0.0, z1), true).unwrap();
        body.set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: geom::Surface::Sphere {
                    center: Point3::origin(),
                    radius: 1.0,
                    axis: Vec3::unit_z(),
                    u_ref: Vec3::unit_x(),
                },
                sense: true,
            },
        )
        .unwrap();
        let half = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                Point3::new(-r1, 0.0, z1),
                EdgeCurveSpec::arc_of_circle(upper.clone(), 0.0, pi).unwrap(),
                tol,
            )
            .unwrap();
        let below = body
            .mef(
                MefSite::Chords {
                    he1: half.he_minus,
                    he2: half.he_plus,
                },
                EdgeCurveSpec::arc_of_circle(upper, pi, tau).unwrap(),
                FaceSurface::Inherit,
                tol,
            )
            .unwrap();
        // The lower rim, planted on the face below the upper one from a
        // strut to the far side of it, which the ring's making kills.
        let zone = below.face;
        let LoopBoundary::Cycle { first } = body
            .get_loop(body.get_face(zone).unwrap().outer)
            .unwrap()
            .boundary
        else {
            panic!("the outer loop is a cycle")
        };
        let from = body
            .get_vertex(body.get_half_edge(first).unwrap().start)
            .unwrap()
            .point;
        let side = body.points[from].x.signum();
        let (r2, lower) = rim(z2);
        let strut = body
            .mev_line(
                MevSite::Fan {
                    he1: first,
                    he2: first,
                },
                Point3::new(-side * r2, 0.0, z2),
                tol,
            )
            .unwrap();
        let (t_far, t_near) = if side > 0.0 { (pi, tau) } else { (0.0, pi) };
        let fan = MevSite::Fan {
            he1: strut.he_minus,
            he2: strut.he_minus,
        };
        let across = body
            .mev(
                fan,
                Point3::new(side * r2, 0.0, z2),
                EdgeCurveSpec::arc_of_circle(lower.clone(), t_far, t_near).unwrap(),
                tol,
            )
            .unwrap();
        let (t0, t1) = if side > 0.0 { (0.0, pi) } else { (pi, tau) };
        let close = body
            .mef(
                MefSite::Chords {
                    he1: across.he_minus,
                    he2: across.he_plus,
                },
                EdgeCurveSpec::arc_of_circle(lower, t0, t1).unwrap(),
                FaceSurface::Inherit,
                tol,
            )
            .unwrap();
        body.kemr(strut.he_plus, strut.he_minus).unwrap();
        for edge in [half.edge, below.edge, across.edge, close.edge] {
            body.describe_at_rest(edge, tol).unwrap();
        }
        crate::mint_pcurves(&mut body, tol).unwrap();
        let zone = body
            .faces
            .iter()
            .find(|(_, f)| !f.rings.is_empty())
            .expect("the lower rim is a ring")
            .0;
        let (errors, _) = crate::validate::tier3_local_checks(&body, tol, None);
        for pole in [Singularity::North, Singularity::South] {
            assert!(
                errors.iter().any(|e| matches!(
                    e,
                    crate::ValidationError::ChartSingularityUndecided {
                        face: f,
                        point,
                        reading: Undecided::Side(SideUnread::Banded(diag)),
                    } if *f == zone
                        && *point == pole
                        && diag.predicate == Some("bool_sphere_region_order")
                        && !diag.margin.is_invalid()
                )),
                "{pole:?} is undecided on the order row: {errors:?}"
            );
        }
        assert!(
            !errors.iter().any(|e| matches!(
                e,
                crate::ValidationError::ChartSingularityNotAVertex { face: f, .. } if *f == zone
            )),
            "no place is read off a margin in band: {errors:?}"
        );
    }

    /// The unit sphere's lune between azimuths `π` and `3π/2`, below the
    /// parallel at polar angle `1`, at `T`: its two meridian arcs run down
    /// to the south pole, a vertex of the face, and its parallel arc is
    /// framed about `−z` from `−y`, as a box row's carve frames it.
    fn pole_lune<T: Decide + geom_core::CertifiedBounds + geom_core::spline::SpanLocate>()
    -> SphereFaceRegion<T> {
        let lift = T::from_f64;
        let v = |x: f64, y: f64, z: f64| Vec3::new(lift(x), lift(y), lift(z));
        let pi = core::f64::consts::PI;
        let edge = |carrier, (t0, t1): (f64, f64), forward, key| RegionEdge {
            edge: EdgeKey::from(KeyData::from_ffi(key)),
            r#loop: LoopKey::default(),
            cycle: 0,
            carrier,
            t0: lift(t0),
            t1: lift(t1),
            turn: crate::boolean::region::fixture_turn(t0, t1),
            forward,
        };
        let meridian = |az: f64| geom::Curve3::Circle {
            center: Point3::new(lift(0.0), lift(0.0), lift(0.0)),
            axis: v(-az.sin(), az.cos(), 0.0),
            radius: lift(1.0),
            u_ref: v(0.0, 0.0, 1.0),
        };
        let parallel = geom::Curve3::Circle {
            center: Point3::new(lift(0.0), lift(0.0), lift(1f64.cos())),
            axis: v(0.0, 0.0, -1.0),
            radius: lift(1f64.sin()),
            u_ref: v(0.0, -1.0, 0.0),
        };
        SphereFaceRegion {
            center: Point3::new(lift(0.0), lift(0.0), lift(0.0)),
            radius: lift(1.0),
            sense: true,
            outer: LoopKey::default(),
            edges: vec![
                edge(meridian(pi), (1.0, pi), true, 1),
                edge(parallel, (0.0, pi / 2.0), true, 2),
                edge(meridian(1.5 * pi), (1.0, pi), false, 3),
            ],
        }
    }

    /// **The door settles a pole at the certified scalar.** Every great
    /// circle through the north pole passes the south pole at its
    /// antipode, where it meets the lune's two meridian arcs at their
    /// ends; and the great circle through the lune's middle meets its
    /// parallel arc a half turn from the arc's middle, and the one in the
    /// plane `y = 0` reads the arc's phase on its cut. Read as angles,
    /// each was a whole turn's hull at `Interval`, and the pole refused in
    /// band. The north pole is outside, the south pole on the boundary,
    /// and a point of the lune in it, at `f64` and at `Interval`.
    #[test]
    fn the_door_settles_a_pole_at_the_certified_scalar() {
        fn read<T: Decide + geom_core::CertifiedBounds + geom_core::spline::SpanLocate>(
            scalar: &str,
        ) {
            let band = Band::new(1e-9, 1e-8).unwrap();
            let lift = T::from_f64;
            let region = pole_lune::<T>();
            let at = |x: f64, y: f64, z: f64| Point3::new(lift(x), lift(y), lift(z));
            let pi = core::f64::consts::PI;
            let (polar, az) = (2.0f64, 1.25 * pi);
            let inside = at(polar.sin() * az.cos(), polar.sin() * az.sin(), polar.cos());
            for (what, p, want) in [
                ("the north pole", at(0.0, 0.0, 1.0), Some(false)),
                ("the south pole", at(0.0, 0.0, -1.0), None),
                ("a point of the lune", inside, Some(true)),
            ] {
                assert_eq!(region.contains(p, band), Ok(want), "{what} at {scalar}");
            }
        }
        read::<f64>("f64");
        read::<geom_core::Interval>("Interval");
    }

    /// **A root a half turn from its arc's middle, or on its phase's cut,
    /// reads as tightly as any.** From the north pole, the great circle
    /// toward azimuth `5π/4` meets the lune's parallel arc at its middle
    /// and a half turn from it; the one toward azimuth `π` lies in the
    /// plane `y = 0`, against which the arc's phase (`−y` its reference,
    /// about `−z`) is a half turn. Read as angles, the far root and both
    /// of the second's were a whole turn's hull at `Interval`, which the
    /// span reading refused in band. Read as directions, the first leg
    /// lands once, inside the arc, and the second once, at its end.
    #[test]
    fn a_root_opposite_its_arcs_middle_or_on_its_phase_cut_reads_tight() {
        fn read<T: Decide + geom_core::CertifiedBounds + geom_core::spline::SpanLocate>(
            scalar: &str,
        ) {
            let band = Band::new(1e-9, 1e-8).unwrap();
            let lift = T::from_f64;
            let region = pole_lune::<T>();
            let pole = Vec3::new(lift(0.0), lift(0.0), lift(1.0));
            let pi = core::f64::consts::PI;
            let rim = 1f64.sin();
            for (az, end) in [(1.25 * pi, false), (pi, true)] {
                let toward = Vec3::new(lift(az.cos()), lift(az.sin()), lift(0.0));
                let leg = region.ray(pole, toward);
                let got = region.landings(&region.edges[1], &leg, band);
                let Ok(landings) = got else {
                    panic!("azimuth {az}: the leg lands at {scalar}, got {got:?}")
                };
                assert_eq!(landings.len(), 1, "azimuth {az} at {scalar}");
                let (y, _, sideless) = landings[0];
                let want = Point3::new(rim * az.cos(), rim * az.sin(), 1f64.cos());
                let miss = (y - Point3::new(lift(want.x), lift(want.y), lift(want.z))).norm();
                assert!(
                    geom_core::Bounds::hi(miss) < 1e-12 && sideless == end,
                    "azimuth {az} at {scalar}: {y:?} sideless {sideless}"
                );
            }
        }
        read::<f64>("f64");
        read::<geom_core::Interval>("Interval");
    }
}
