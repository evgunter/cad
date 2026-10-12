//! **A boolean's body against a truth that reads no kernel code**: the
//! operands' point membership in closed form, and the check every body
//! an op returns is held to — its volume, tier 3, and
//! `point_in_solid` at named points and over a grid, each against the
//! op's truth.
//!
//! A [`Solid`]'s [`Solid::depth`] is positive inside and negative
//! outside, and its magnitude is at most the distance to the boundary
//! (a minimum of signed distances to the bounding surfaces). A point
//! whose depth is within [`MARGIN`] of zero is too near the boundary
//! for a two-valued truth, and the grid skips it.
//!
//! **Deliberately not absorbed**, and the whole of it:
//! [`super::differential`]'s `outcome` line, which prints tiers 2 and
//! 3′ and the operand question against a polygon oracle's volume and
//! reads no point; [`super::pinch_cones`], which reads a body at one
//! point's link; and [`super::oracles`]' closed-form volumes, which a
//! row here passes in as its `Want`.

use core::f64::consts::{PI, TAU};

use geom_core::{Affine3, Band, Point3, Tol, Vec3};
use topo::{BooleanError, BooleanResult, PointInSolidError, SolidContainment};

/// How deep a point must sit, inside or out, for its truth to count.
pub const MARGIN: f64 = 1e-4;

/// A solid with closed-form membership.
pub enum Solid {
    /// About the `y` axis over `y ∈ [y0, y1]`, radius
    /// `r0 + k·(y − y0)` (a cone wall, or a cylinder at `k = 0`),
    /// within the azimuth window `[0, w]`, the azimuth measured as a
    /// revolve sweeps it, from `+x` toward `−z`; `None` is the full
    /// turn.
    Revolved {
        y0: f64,
        y1: f64,
        r0: f64,
        k: f64,
        window: Option<f64>,
    },
    /// The axis-aligned box `[x] × [y] × [z]`.
    Brick([(f64, f64); 3]),
    /// A solid moved by a rigid map.
    Posed(Box<Solid>, Affine3<f64>),
    /// Disjoint lumps.
    Lumps(Vec<Solid>),
}

impl Solid {
    /// The solid moved by the rigid `map`.
    pub fn posed(self, map: Affine3<f64>) -> Self {
        Self::Posed(Box::new(self), map)
    }

    /// Positive inside, negative outside, no larger in magnitude than
    /// the distance to the boundary.
    pub fn depth(&self, p: Point3<f64>) -> f64 {
        match self {
            Self::Revolved {
                y0,
                y1,
                r0,
                k,
                window,
            } => {
                let rho = p.x.hypot(p.z);
                let wall = (r0 + k * (p.y - y0) - rho) / k.hypot(1.0);
                let caps = (p.y - y0).min(y1 - p.y);
                let mut d = wall.min(caps);
                if let Some(w) = window {
                    let phi = (-p.z).atan2(p.x).rem_euclid(TAU);
                    // The distance to a half-plane at angular offset `a`.
                    let off = |a: f64| if a >= PI / 2.0 { rho } else { rho * a.sin() };
                    let side = if phi <= *w {
                        off(phi.min(w - phi))
                    } else {
                        -off((phi - w).min(TAU - phi))
                    };
                    d = d.min(side);
                }
                d
            }
            Self::Brick(b) => {
                let c = [p.x, p.y, p.z];
                (0..3)
                    .map(|i| (c[i] - b[i].0).min(b[i].1 - c[i]))
                    .fold(f64::INFINITY, f64::min)
            }
            Self::Posed(s, map) => s.depth(map.inverse().transform_point(p)),
            Self::Lumps(ls) => ls
                .iter()
                .map(|s| s.depth(p))
                .fold(f64::NEG_INFINITY, f64::max),
        }
    }
}

/// An op's truth from its operands' depths.
#[derive(Clone, Copy, Debug)]
pub enum Op {
    Union,
    Intersect,
    /// `A ∖ B`.
    Subtract,
}

impl Op {
    pub fn depth(self, a: &Solid, b: &Solid, p: Point3<f64>) -> f64 {
        let (da, db) = (a.depth(p), b.depth(p));
        match self {
            Self::Union => da.max(db),
            Self::Intersect => da.min(db),
            Self::Subtract => da.min(-db),
        }
    }
}

/// `n³` cell midpoints over the box `lo..hi`.
pub fn grid(lo: Point3<f64>, hi: Point3<f64>, n: usize) -> Vec<Point3<f64>> {
    let d = (hi - lo) / n as f64;
    let mut out = Vec::with_capacity(n * n * n);
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                out.push(
                    lo + Vec3::new(
                        (i as f64 + 0.5) * d.x,
                        (j as f64 + 0.5) * d.y,
                        (k as f64 + 0.5) * d.z,
                    ),
                );
            }
        }
    }
    out
}

/// The midpoint-rule volume of `{depth > 0}` over the box `lo..hi` at
/// `n³` cells: a truth for a volume with no closed form, good to the
/// boundary's area times the cell's width.
pub fn grid_volume(
    depth: impl Fn(Point3<f64>) -> f64,
    lo: Point3<f64>,
    hi: Point3<f64>,
    n: usize,
) -> f64 {
    let d = (hi - lo) / n as f64;
    let inside = grid(lo, hi, n)
        .into_iter()
        .filter(|&p| depth(p) > 0.0)
        .count();
    inside as f64 * d.x * d.y * d.z
}

/// What an op must return.
#[derive(Clone, Copy, Debug)]
pub enum Want {
    /// A body of this volume, within the absolute tolerance.
    Body(f64, f64),
    /// No material.
    Empty,
}

/// **The check every returned body is held to.** `got` is `want`: a
/// body whose `mass_properties` volume is the one wanted, that passes
/// tier 3 (`validate_geometric`), and whose `point_in_solid` agrees
/// with `truth` at every point of `points` deeper than [`MARGIN`]
/// (each of `named` must be, and is checked whatever the margin says);
/// or `Empty`, with no point of `points` inside the truth. Returns the
/// body's volume, or 0 for `Empty`.
///
/// # Panics
///
/// On a refusal, the wrong outcome, or any check failing.
pub fn assert_is(
    label: &str,
    got: &Result<BooleanResult<f64>, BooleanError>,
    want: Want,
    truth: &dyn Fn(Point3<f64>) -> f64,
    named: &[Point3<f64>],
    points: &[Point3<f64>],
) -> f64 {
    assert_is_but(label, got, want, truth, named, points, 0)
}

/// [`assert_is`] on a body with a cone face bounded by a tilted section,
/// whose `point_in_solid` refuses `PartialConeFace` at the grid points
/// whose rays it cannot clear
/// (`work/inside/cone-chart-trim-reads-a-tilted-section-as-its-vertex-window.md`):
/// at most `partial_cone_refusals` of `points` may refuse so, each row's
/// measured count, and never a named point.
///
/// # Panics
///
/// As [`assert_is`], and on more refusals than allowed.
pub fn assert_is_but(
    label: &str,
    got: &Result<BooleanResult<f64>, BooleanError>,
    want: Want,
    truth: &dyn Fn(Point3<f64>) -> f64,
    named: &[Point3<f64>],
    points: &[Point3<f64>],
    partial_cone_refusals: usize,
) -> f64 {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let r = got
        .as_ref()
        .unwrap_or_else(|e| panic!("{label}: wanted {want:?}, refused {e:?}"));
    match (want, r) {
        (Want::Empty, BooleanResult::Empty) => {
            let inside = points.iter().find(|&&p| truth(p) > MARGIN);
            assert!(
                inside.is_none(),
                "{label}: Empty, but the truth holds {inside:?}"
            );
            0.0
        }
        (Want::Body(v, tolerance), BooleanResult::Body(bb)) => {
            let body = &bb.body;
            let got_v = topo::mass_properties(body, tol)
                .unwrap_or_else(|e| panic!("{label}: the volume refused {e:?}"))
                .volume;
            assert!(
                (got_v - v).abs() <= tolerance,
                "{label}: volume {got_v} against {v} (± {tolerance})"
            );
            topo::validate_geometric(body, tol)
                .unwrap_or_else(|e| panic!("{label}: tier 3: {e:?}"));
            for &p in named {
                assert!(
                    truth(p).abs() > MARGIN,
                    "{label}: the named point {p:?} sits on the truth's boundary"
                );
            }
            let (mut checked, mut refused) = (0, 0);
            for (i, &p) in named.iter().chain(points).enumerate() {
                let d = truth(p);
                if d.abs() <= MARGIN {
                    continue;
                }
                let want_in = d > 0.0;
                let got_c = match topo::point_in_solid(body, p, band, tol) {
                    Ok(c) => c,
                    Err(PointInSolidError::PartialConeFace { .. }) if i >= named.len() => {
                        refused += 1;
                        continue;
                    }
                    Err(e) => panic!("{label}: point_in_solid at {p:?} refused {e:?}"),
                };
                let ok = match got_c {
                    SolidContainment::In => want_in,
                    SolidContainment::Out => !want_in,
                    SolidContainment::OnBoundary => false,
                };
                assert!(
                    ok,
                    "{label}: at {p:?} (depth {d:e}) point_in_solid says {got_c:?}"
                );
                checked += 1;
            }
            assert!(
                checked > named.len() && refused <= partial_cone_refusals,
                "{label}: {checked} points answered, {refused} refused `PartialConeFace` \
                 (at most {partial_cone_refusals})"
            );
            got_v
        }
        (want, r) => panic!(
            "{label}: wanted {want:?}, got {}",
            match r {
                BooleanResult::Empty => "Empty".to_owned(),
                BooleanResult::Body(bb) => format!("a {:?} body", bb.kind),
            }
        ),
    }
}
