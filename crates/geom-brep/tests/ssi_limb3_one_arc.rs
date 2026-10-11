//! **Limb 3's tube holds the traced arc and nothing else** (C2, C3).
//!
//! A search banks every cell inside a certified branch's tube as
//! accounted, so a second arc of the locus inside the tube is lost
//! without a word unless limb 3 proves there is none. A zero-free
//! transverse derivative proves only one solution per transverse line:
//! a second arc beside the first along the tube, leaving through the
//! domain's sides, passes it. These rows plant that second arc on both
//! lanes; each must be traced, or the op refuse.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::{band, eps};
use geom::{NurbsSurface, Surface};
use geom_brep::ssi::{self, BranchEnd, ChartAxis, ChartEnd, SsiDomain, SsiOutcome};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};
use test_utils::vacuity;

/// A biquadratic Bézier graph `z = g(x) + h(y)` over `[x0, x1] × [0, l]`:
/// `g` and `h` are quadratics, given by their values and their slopes at
/// the low end, and the tensor net of a sum is the sum of the nets.
fn graph_wall(
    (x0, x1): (f64, f64),
    l: f64,
    (g, dg0): (impl Fn(f64) -> f64, f64),
    (h, dh0): (impl Fn(f64) -> f64, f64),
) -> NurbsSurface<f64> {
    let k = || KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let gb = [g(x0), g(x0) + 0.5 * (x1 - x0) * dg0, g(x1)];
    let hb = [h(0.0), h(0.0) + 0.5 * l * dh0, h(l)];
    let (xs, ys) = ([x0, 0.5 * (x0 + x1), x1], [0.0, 0.5 * l, l]);
    let control = (0..9)
        .map(|k| Point3::new(xs[k / 3], ys[k % 3], gb[k / 3] + hb[k % 3]))
        .collect();
    NurbsSurface::new(k(), k(), control, vec![1.0; 9]).unwrap()
}

/// The fold `z = c·x + a·x² + 4β·y(L − y)/L²` over
/// `x ∈ [x0·w, x1·w]`, `y ∈ [0, L]`, every height scaled to ε: β = ε,
/// c = `ck`·ε, a = `ak`·c²/β, w = β/c, L = `lk`·w.
struct Fold {
    wall: NurbsSurface<f64>,
    phi: Box<dyn Fn(f64, f64) -> f64>,
    xr: (f64, f64),
    l: f64,
    w: f64,
}

fn fold(ck: f64, ak: f64, (x0, x1): (f64, f64), lk: f64) -> Fold {
    let beta = eps();
    let c = ck * beta;
    let a = ak * c * c / beta;
    let w = beta / c;
    let l = lk * w;
    let xr = (x0 * w, x1 * w);
    let g = move |x: f64| c * x + a * x * x;
    let h = move |y: f64| 4.0 * beta * y * (l - y) / (l * l);
    Fold {
        wall: graph_wall(xr, l, (g, c + 2.0 * a * xr.0), (h, 4.0 * beta / l)),
        phi: Box::new(move |x, y| g(x) + h(y)),
        xr,
        l,
        w,
    }
}

/// The plane `z = 0` and a window around the fold.
fn ground() -> (Surface<f64>, SsiDomain) {
    (
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
        SsiDomain {
            center: Point3::new(0.0, 0.0, 0.0),
            half_extent: 1.0,
            extent: 1.0,
            floor_scale: 1.0,
        },
    )
}

/// **The fold: two arcs side by side along one transverse direction.**
/// The [`fold`] with c = 80ε, a = 0.28·c²/β over `x ∈ [−1.5w, 1.8w]`,
/// at L = 1.2w and L = 1.5w. `∂φ/∂x > 0` everywhere, and the locus is two
/// arcs: from `(0, 0)` and from `(0, L)` on the `v` sides, each to the low
/// `u` side, with `φ > 0` between them. A cubic from `(0, 0)` to `(0, L)`
/// stays within ε of the plane across that gap, and its one window — the
/// whole wall — is a graph over `x`-lines.
///
/// The Hermite candidate from `(0, 0)` is that cubic, `(0, L)` being the
/// nearest crossing, and it passes limbs 1 and 2; only limb 3's one-arc
/// proof on the chart lane refuses it, after which the march pairs the
/// fold as the locus does. Certified on the graph alone, it answers the
/// wrong pairing and this row goes red. At
/// L = 1.2w the fold answers at every ε; at L = 1.5w it answers or
/// refuses (at ε 1e-6 limb 3's margin is too close to call), and never
/// answers another pairing.
#[test]
fn the_fold_answers_its_two_arcs_paired_as_the_locus_pairs_them() {
    let (plane, domain) = ground();
    for (lk, must_answer) in [(1.2, true), (1.5, false)] {
        let at = format!("the fold at L = {lk}w, ε {:e}", eps());
        let f = fold(80.0, 0.28, (-1.5, 1.8), lk);
        let out = match ssi::plane_nurbs_ssi(&plane, &f.wall, domain, band()) {
            Ok(out) => out,
            Err(e) if !must_answer => {
                vacuity::stood_down(&at, &format!("refused, so no pairing to judge: {e}"));
                continue;
            }
            Err(e) => panic!("{at}: expected its two arcs, got {e}"),
        };

        // Each branch joins a `v` side to the low `u` side.
        let key = |p: &[(ChartAxis, ChartEnd); 2]| format!("{p:?}");
        let sorted = |mut e: [(ChartAxis, ChartEnd); 2]| {
            e.sort_by_key(|s| format!("{s:?}"));
            e
        };
        let mut pairs: Vec<_> = out
            .branches
            .iter()
            .map(|b| match b.end {
                BranchEnd::Crossings { from, to } => sorted([
                    (from.side.fixed, from.side.end),
                    (to.side.fixed, to.side.end),
                ]),
                other => panic!("{at}: an open branch ended {other:?}"),
            })
            .collect();
        pairs.sort_by_key(key);
        let low_u = (ChartAxis::U, ChartEnd::Low);
        let mut want = vec![
            sorted([(ChartAxis::V, ChartEnd::Low), low_u]),
            sorted([(ChartAxis::V, ChartEnd::High), low_u]),
        ];
        want.sort_by_key(key);
        assert_eq!(pairs, want, "{at}: branches paired across the gap");
        if let Err(why) = paired_as_the_locus(&f.phi, f.xr, f.l, &out) {
            panic!("{at}: {why}");
        }

        let far = far_zeros(&f.phi, f.xr, f.l, &out, 0.3 * f.w);
        assert_eq!(far, 0, "{at}: zeros 0.3w from every carrier");
    }
}

/// How many zeros of `φ`, found as sign changes along a 400 × 400 grid
/// of `y` lines over `[x0, x1] × [0, l]`, lie farther than `far` in `xy`
/// from every carrier sampled at 4001 points.
fn far_zeros(
    phi: &impl Fn(f64, f64) -> f64,
    (x0, x1): (f64, f64),
    l: f64,
    out: &SsiOutcome,
    far: f64,
) -> usize {
    let pts: Vec<Point3<f64>> = out
        .branches
        .iter()
        .flat_map(|b| {
            let (t0, t1) = b.params;
            (0..=4000).map(move |k| b.carrier.eval(t0 + (t1 - t0) * f64::from(k) / 4000.0))
        })
        .collect();
    let n = 400;
    let at = |i: u32, lo: f64, hi: f64| lo + (hi - lo) * f64::from(i) / f64::from(n);
    let zeros: Vec<(f64, f64)> = (0..=n)
        .flat_map(|j| {
            let y = at(j, 0.0, l);
            (0..n).filter_map(move |i| {
                let (a, b) = (at(i, x0, x1), at(i + 1, x0, x1));
                (phi(a, y).signum() != phi(b, y).signum()).then_some((0.5 * (a + b), y))
            })
        })
        .collect();
    assert!(zeros.len() > 100, "FIXTURE: the grid finds the locus");
    zeros
        .iter()
        .filter(|(x, y)| {
            pts.iter()
                .all(|p| (p.x - x).powi(2) + (p.y - y).powi(2) > far * far)
        })
        .count()
}

/// **A short arc inside a long arc's end box, on the ℝ³ lane.** A unit
/// cylinder about `z` and a radius-3 sphere about the origin meet in the
/// circle of radius 1 at `z = √8`. The slab's `x` face at `cos 5°` and its
/// `y` face at `sin 11°` cut that circle into a long arc (11° round to
/// 355°) and a short one (5° to 11°) that continues it along the long
/// arc's end tangent. The long arc's end box at the widest rung that is a
/// graph holds the short arc, every slice of it once; the seeds trace the
/// long arc first, and a seed landing in a tube is skipped. The short arc
/// must be a branch.
#[test]
fn a_short_arc_in_a_long_arcs_end_box_is_traced() {
    let cylinder = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let sphere = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 3.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let (alpha, beta) = (5.0f64.to_radians(), 11.0f64.to_radians());
    let half = 2.0;
    let domain = SsiDomain {
        center: Point3::new(alpha.cos() - half, beta.sin() - half, 2.0),
        half_extent: half,
        extent: 4.8,
        floor_scale: 1.0,
    };
    let out = match ssi::cylinder_sphere_ssi(&cylinder, &sphere, domain, band()) {
        Ok(out) => out,
        Err(e) => panic!("the clipped circle: expected two arcs, got {e}"),
    };
    let mid = 0.5 * (alpha + beta);
    let on_short = Point3::new(mid.cos(), mid.sin(), 8.0f64.sqrt());
    let carried = out.branches.iter().any(|b| {
        let (t0, t1) = b.params;
        (0..=2000).any(|i| {
            let q = b.carrier.eval(t0 + (t1 - t0) * f64::from(i) / 2000.0);
            (q - on_short).norm() < 1e-3
        })
    });
    assert!(
        carried && out.branches.len() == 2,
        "the clipped circle: {} branches, the short arc carried: {carried}",
        out.branches.len(),
    );
}

/// Which component of `φ = 0` over `[x0, x1] × [0, l]` each boundary
/// crossing lies on, by marching squares on an `n × n` grid: the
/// crossings as `(x, y, component)`.
fn boundary_components(
    phi: &impl Fn(f64, f64) -> f64,
    (x0, x1): (f64, f64),
    l: f64,
    n: usize,
) -> Vec<(f64, f64, usize)> {
    let xs = |i: usize| x0 + (x1 - x0) * i as f64 / n as f64;
    let ys = |j: usize| l * j as f64 / n as f64;
    let neg = |i: usize, j: usize| phi(xs(i), ys(j)) < 0.0;
    // Edge ids: horizontal (i, j)-(i+1, j) then vertical (i, j)-(i, j+1).
    let h_id = |i: usize, j: usize| j * n + i;
    let v_id = |i: usize, j: usize| n * (n + 1) + j * (n + 1) + i;
    let mut parent: Vec<usize> = (0..2 * n * (n + 1)).collect();
    fn find(p: &mut [usize], mut a: usize) -> usize {
        while p[a] != a {
            p[a] = p[p[a]];
            a = p[a];
        }
        a
    }
    for j in 0..n {
        for i in 0..n {
            let edges = [
                (h_id(i, j), neg(i, j) != neg(i + 1, j)),
                (v_id(i + 1, j), neg(i + 1, j) != neg(i + 1, j + 1)),
                (h_id(i, j + 1), neg(i, j + 1) != neg(i + 1, j + 1)),
                (v_id(i, j), neg(i, j) != neg(i, j + 1)),
            ];
            let cut: Vec<usize> = edges.iter().filter(|e| e.1).map(|e| e.0).collect();
            let pairs: Vec<(usize, usize)> = match cut.len() {
                2 => vec![(cut[0], cut[1])],
                4 => {
                    let centre = phi(0.5 * (xs(i) + xs(i + 1)), 0.5 * (ys(j) + ys(j + 1))) < 0.0;
                    if centre == neg(i, j) {
                        vec![(cut[0], cut[1]), (cut[2], cut[3])]
                    } else {
                        vec![(cut[0], cut[3]), (cut[1], cut[2])]
                    }
                }
                _ => vec![],
            };
            for (a, b) in pairs {
                let (ra, rb) = (find(&mut parent, a), find(&mut parent, b));
                parent[ra] = rb;
            }
        }
    }
    let mut out = Vec::new();
    for i in 0..n {
        for j in [0, n] {
            if neg(i, j) != neg(i + 1, j) {
                out.push((
                    0.5 * (xs(i) + xs(i + 1)),
                    ys(j),
                    find(&mut parent, h_id(i, j)),
                ));
            }
        }
    }
    for j in 0..n {
        for i in [0, n] {
            if neg(i, j) != neg(i, j + 1) {
                out.push((
                    xs(i),
                    0.5 * (ys(j) + ys(j + 1)),
                    find(&mut parent, v_id(i, j)),
                ));
            }
        }
    }
    out
}

/// Whether `out` pairs the crossings as the locus does: each branch's
/// two ends lie nearest crossings of one component, no two branches
/// share a component, and every component the boundary meets is a branch.
fn paired_as_the_locus(
    phi: &impl Fn(f64, f64) -> f64,
    xr: (f64, f64),
    l: f64,
    out: &SsiOutcome,
) -> Result<(), String> {
    let crossings = boundary_components(phi, xr, l, 600);
    let comp = |p: Point3<f64>| {
        crossings
            .iter()
            .min_by(|a, b| {
                let da = (a.0 - p.x).powi(2) + (a.1 - p.y).powi(2);
                let db = (b.0 - p.x).powi(2) + (b.1 - p.y).powi(2);
                da.total_cmp(&db)
            })
            .map(|c| c.2)
    };
    let mut seen = Vec::new();
    for b in &out.branches {
        let (t0, t1) = b.params;
        let (p, q) = (b.carrier.eval(t0), b.carrier.eval(t1));
        let (cp, cq) = (comp(p), comp(q));
        if cp.is_none() || cp != cq {
            return Err(format!(
                "a branch joins ({:.3e}, {:.3e}) to ({:.3e}, {:.3e}) across components",
                p.x, p.y, q.x, q.y
            ));
        }
        if seen.contains(&cp) {
            return Err("two branches on one component".into());
        }
        seen.push(cp);
    }
    let mut comps: Vec<usize> = crossings.iter().map(|c| c.2).collect();
    comps.sort_unstable();
    comps.dedup();
    if comps.len() != seen.len() {
        return Err(format!(
            "{} components meet the boundary, {} branches",
            comps.len(),
            seen.len()
        ));
    }
    Ok(())
}

/// A fold wall at rest: `z = c·x + a·x² + h(y)` over
/// `x ∈ [−1.5w, 1.8w]`, `y ∈ [0, L]`, with β = `scale`·ε, c = 800ε,
/// a = `ak`·c²/β, w = β/c, L = 1.2w, and `h` the gap's height across `y`:
/// `4β·y(L − y)/L²` (two arcs, the fold) or `β(y/L)²` (one arc leaving
/// the low `u` side short of `y = L`).
fn fold_at_rest(scale: f64, ak: f64, two_arcs: bool) -> (NurbsSurface<f64>, f64) {
    let beta = scale * eps();
    let c = 800.0 * eps();
    let a = ak * c * c / beta;
    let w = beta / c;
    let l = 1.2 * w;
    let xr = (-1.5 * w, 1.8 * w);
    let g = move |x: f64| c * x + a * x * x;
    let wall = if two_arcs {
        let h = move |y: f64| 4.0 * beta * y * (l - y) / (l * l);
        graph_wall(xr, l, (g, c + 2.0 * a * xr.0), (h, 4.0 * beta / l))
    } else {
        let h = move |y: f64| beta * (y / l) * (y / l);
        graph_wall(xr, l, (g, c + 2.0 * a * xr.0), (h, 0.0))
    };
    (wall, l)
}

/// The declared carrier `from → to` on the line `x = 0` of a fold wall,
/// against `z = 0`, through the public at-rest door.
fn declared(
    wall: &NurbsSurface<f64>,
    (from, to): (f64, f64),
) -> Result<geom_brep::PlaneNurbsLimbs<f64>, geom_brep::PlaneNurbsRefusal> {
    let (plane, _) = ground();
    let carrier =
        crate::shared::fixture::segment(Point3::new(0.0, from, 0.0), Point3::new(0.0, to, 0.0));
    geom_brep::plane_nurbs_limbs::<f64>(&carrier, &plane, wall, 1.0, band())
}

/// A declared carrier that must not certify: refused by limb 3's one-arc
/// proof, or, below the default ε where β = ε sits at limb 1's rounding,
/// by an earlier limb, which this row then stands down on, loudly.
fn refuses_by_its_tube(
    at: &str,
    got: Result<geom_brep::PlaneNurbsLimbs<f64>, geom_brep::PlaneNurbsRefusal>,
) {
    match got {
        Err(geom_brep::PlaneNurbsRefusal::TubeNotOneArc { .. }) => {}
        Err(e @ geom_brep::PlaneNurbsRefusal::Escalated { .. })
            if eps() < geom_core::tolerance::DEFAULT_EPS =>
        {
            vacuity::stood_down(at, &format!("an earlier limb escalated: {e}"));
        }
        other => panic!("{at}: {other:?}"),
    }
}

/// **At rest, a carrier joining the fold's two arcs refuses** (C2: one
/// arc at every door). The straight carrier `(0, 0, 0) → (0, L, 0)`
/// lies within ε of the plane and of the wall, so limbs 1 and 2 pass,
/// and its midpoint `(0, L/2, 0)` lies where `φ = β > 0` and there is no
/// intersection. On the graph alone it certifies, at β = ε and ¼ε: red
/// under the at-rest door proving the graph alone.
#[test]
fn a_declared_carrier_joining_the_folds_two_arcs_refuses_at_rest() {
    for scale in [1.0, 0.25] {
        let (wall, l) = fold_at_rest(scale, 0.28, true);
        let at = format!("the fold at β = {scale}ε, ε {:e}", eps());
        refuses_by_its_tube(&at, declared(&wall, (0.0, l)));
    }
}

/// **At rest, a carrier overrunning its arc refuses, at either end.**
/// The half fold (a = 0.28c²/β, `h = β(y/L)²`, β = ε) has one arc, from
/// `(0, 0)` to the low `u` side near `y = 0.93L`; the tail fold
/// (a = 0.24c²/β) leaves that side near `y = 0.98L`. The carrier along
/// `x = 0` reaches `y = L`, where `φ = β > 0` and the nearest solution
/// lies millimetres off: no slice through that end holds one, and the
/// count alone certifies it. The tail fold's carrier is declared both
/// ways, so the overrun end is its last and then its first: red under
/// either end's coverage check removed.
#[test]
fn a_declared_carrier_overrunning_its_arc_refuses_at_rest_at_either_end() {
    let half = fold_at_rest(1.0, 0.28, false);
    let tail = fold_at_rest(1.0, 0.24, false);
    for (name, (wall, _), ends) in [
        (
            "the half fold, overrun at its last end",
            &half,
            (0.0, half.1),
        ),
        (
            "the tail fold, overrun at its last end",
            &tail,
            (0.0, tail.1),
        ),
        (
            "the tail fold, overrun at its first end",
            &tail,
            (tail.1, 0.0),
        ),
    ] {
        refuses_by_its_tube(&format!("{name}, ε {:e}", eps()), declared(wall, ends));
    }
}

/// **A branch leaving the domain obliquely through a side its slices
/// run along certifies.** A straight locus from `(0.5, 0)` to `(1, 0.7)`
/// on the unit chart: the search's carrier ends at the crossing on the
/// `u = 1` side, which is the end window's rail, and that end counts
/// because the carrier's end lies within ε of the locus there.
#[test]
fn a_branch_leaving_obliquely_through_a_rail_side_certifies() {
    let k = 0.5;
    let g = move |x: f64| k * (x - 0.5);
    let h = move |y: f64| -k * y / 1.4;
    let wall = graph_wall((0.0, 1.0), 1.0, (g, k), (h, -k / 1.4));
    let (plane, _) = ground();
    let domain = SsiDomain {
        center: Point3::new(0.5, 0.5, 0.0),
        half_extent: 1.0,
        extent: 1.0,
        floor_scale: 1.0,
    };
    let out = ssi::plane_nurbs_ssi(&plane, &wall, domain, band())
        .unwrap_or_else(|e| panic!("the rail-end branch, ε {:e}: {e}", eps()));
    let sides: Vec<_> = out
        .branches
        .iter()
        .map(|b| match b.end {
            BranchEnd::Crossings { from, to } => {
                let mut s = [from.side, to.side].map(|s| (s.fixed, s.end));
                s.sort_by_key(|x| format!("{x:?}"));
                s
            }
            other => panic!("the rail-end branch ended {other:?}"),
        })
        .collect();
    assert_eq!(
        sides,
        vec![[
            (ChartAxis::U, ChartEnd::High),
            (ChartAxis::V, ChartEnd::Low)
        ]],
        "one branch, from the low v side to the high u side"
    );
}

/// `z = g(x) + h(y)` over `[x0, x1] × [y0, y1]`, `g` and `h` quadratics
/// (exact in the biquadratic net).
fn graph_over(
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    (g, dg0): (impl Fn(f64) -> f64, f64),
    (h, dh0): (impl Fn(f64) -> f64, f64),
) -> NurbsSurface<f64> {
    let k = || KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let gb = [g(x0), g(x0) + 0.5 * (x1 - x0) * dg0, g(x1)];
    let hb = [h(y0), h(y0) + 0.5 * (y1 - y0) * dh0, h(y1)];
    let (xs, ys) = ([x0, 0.5 * (x0 + x1), x1], [y0, 0.5 * (y0 + y1), y1]);
    let control = (0..9)
        .map(|k| Point3::new(xs[k / 3], ys[k % 3], gb[k / 3] + hb[k % 3]))
        .collect();
    NurbsSurface::new(k(), k(), control, vec![1.0; 9]).unwrap()
}

/// The three walls a side within the band of the plane holds no arc of,
/// each with the carrier along that side: the phantom, the side fold and
/// the linear overrun.
fn side_without_arc() -> Vec<(&'static str, NurbsSurface<f64>, (f64, f64))> {
    let e = eps();
    let l = 1e-2;
    let (xr, yr) = ((0.0, 2e-3), (-0.1 * l, 1.1 * l));
    let beta = 0.5 * e;
    let phantom = graph_over(xr, yr, (|x: f64| x, 1.0), (move |_| beta, 0.0));
    let h = move |y: f64| 4.0 * beta * y * (l - y) / (l * l);
    let dh = move |y: f64| 4.0 * beta * (l - 2.0 * y) / (l * l);
    let side_fold = graph_over(xr, yr, (|x: f64| x, 1.0), (h, dh(yr.0)));
    let (h0, h1) = (-0.5 * e, 0.6 * e);
    let overrun = graph_over(
        (0.0, 1.0),
        (0.0, 1.0),
        (|x: f64| 10.0 * x, 10.0),
        (move |y: f64| h0 + (h1 - h0) * y, h1 - h0),
    );
    vec![
        ("the phantom", phantom, (0.0, l)),
        ("the side fold", side_fold, (0.0, l)),
        ("the linear overrun", overrun, (0.0, 1.0)),
    ]
}

/// **At rest, a carrier along a side the boundary pass reads clear
/// refuses, as the search answers that wall** (C2, the side arm read by
/// the search's own #3862 rule). Each wall lies within ε of the plane
/// along the side `x = 0`, and the carrier runs along it:
/// - **the phantom** `z = x + ½ε` meets the plane nowhere, and the
///   search answers nothing;
/// - **the side fold** `z = x + 4β·y(L − y)/L²`, β = ½ε, has two arcs,
///   one meeting the side at each end of the carrier, and the search
///   answers those two;
/// - **the linear overrun** `z = 10x + h(y)`, `h` from −½ε to 0.6ε, has
///   one arc, leaving the side near `y = 0.45` of the carrier's 1, and
///   the search answers the side as a region, with no branch.
///
/// The side's cover holds every zero within ε of it, but the plane is
/// clear of the side along the middle of the carrier (the fold), the
/// whole of it (the phantom) or past the arc (the overrun). Red under
/// the side arm reading the cover alone.
#[test]
fn a_carrier_along_a_side_the_boundary_pass_reads_clear_refuses_as_the_search_answers() {
    let (plane, _) = ground();
    for (name, wall, ends) in side_without_arc() {
        let at = format!("{name}, ε {:e}", eps());
        refuses_by_its_tube(&at, declared(&wall, ends));
        let domain = SsiDomain {
            center: Point3::new(0.0, 0.5 * (ends.0 + ends.1), 0.0),
            half_extent: 1.0,
            extent: 1.0,
            floor_scale: 1.0,
        };
        let out = ssi::plane_nurbs_ssi(&plane, &wall, domain, band())
            .unwrap_or_else(|e| panic!("{at}: the search refused: {e}"));
        let low_u = ssi::ChartSide {
            fixed: ChartAxis::U,
            end: ChartEnd::Low,
        };
        let (branches, region) = (
            out.branches.len(),
            out.boundary
                .iter()
                .any(|c| matches!(c, ssi::SsiBoundaryContact::Side { side, .. } if *side == low_u)),
        );
        let expected = match name {
            "the phantom" => (0, false),
            "the side fold" => (2, false),
            _ => (0, true),
        };
        assert_eq!(
            (branches, region),
            expected,
            "{at}: the search's answer (branches, a region along the side)"
        );
    }
}

/// **A flush side on a conical or twisted wall certifies.** The m7_8
/// quarter cylinder with its top arc at radius `r₁`, a quarter frustum
/// whose `u = 0` ruling from `(1, 0, 0)` to `(r₁, 0, 1)` lies in the plane
/// `y = 0`, and the ramp `z = x + x·y`, flush with `z = 0` along `x = 0`:
/// each carrier is the side itself. The side's `|φ|` is read from the
/// side's own Bernstein form, as the boundary pass reads it, not from the
/// wall's boxes over the window, whose derivative across a twisted or
/// conical wall leaves it near 10⁻⁵ and refused the side at every rung.
#[test]
fn a_flush_side_on_a_conical_or_twisted_wall_certifies() {
    use crate::shared::fixture::{segment, transverse_plane};
    let lin = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    for r1 in [1.001, 1.1, 2.0] {
        let w = core::f64::consts::FRAC_1_SQRT_2;
        let arc = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let control = vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(r1, 0.0, 1.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(r1, r1, 1.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.0, r1, 1.0),
        ];
        let wall = NurbsSurface::new(arc, lin(), control, vec![1.0, 1.0, w, w, 1.0, 1.0]).unwrap();
        let carrier = segment(Point3::new(1.0, 0.0, 0.0), Point3::new(r1, 0.0, 1.0));
        let got =
            geom_brep::plane_nurbs_limbs::<f64>(&carrier, &transverse_plane(), &wall, 1.0, band());
        assert!(got.is_ok(), "the frustum r₁ = {r1}, ε {:e}: {got:?}", eps());
    }
    let ramp = NurbsSurface::new(
        lin(),
        lin(),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 1.0),
            Point3::new(1.0, 1.0, 2.0),
        ],
        vec![1.0; 4],
    )
    .unwrap();
    let (plane, _) = ground();
    let carrier = segment(Point3::new(0.0, 0.2, 0.0), Point3::new(0.0, 0.8, 0.0));
    let got = geom_brep::plane_nurbs_limbs::<f64>(&carrier, &plane, &ramp, 1.0, band());
    assert!(got.is_ok(), "the ramp, ε {:e}: {got:?}", eps());
}

/// `z = k·x + h(y)` over `[0, 1]²`, `h` the quadratic spline with a
/// double knot at `y = ½` and control values `hs` at `y = 0, ¼, ½, ¾, 1`.
fn double_dip(k: f64, hs: [f64; 5]) -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let ys = [0.0, 0.25, 0.5, 0.75, 1.0];
    let control = (0..10)
        .map(|i| {
            let x = f64::from(i / 5);
            Point3::new(x, ys[(i % 5) as usize], k * x + hs[(i % 5) as usize])
        })
        .collect();
    NurbsSurface::new(ku, kv, control, vec![1.0; 10]).unwrap()
}

/// **At rest, a carrier along a side whose locus wanders past ε of it
/// refuses.** `z = 10⁻³x + h(y)` with `h ≤ 0`, zero at `y = 0, ½, 1` and
/// down to about `0.45ε` between: no piece of the side `x = 0` reads clear,
/// and `|φ| ≤ ε` along it, but the locus is two arcs, each from the side
/// into the wall about `450ε` and back. The side's cover over a window in
/// either dip reaches past ε, so no window there is the side's, and the
/// walk refuses. Red under the side arm skipping the cover's verdict.
#[test]
fn a_carrier_along_a_side_whose_locus_wanders_past_eps_refuses_at_rest() {
    let a = 0.9 * eps();
    let wall = double_dip(1e-3, [0.0, -a, 0.0, -a, 0.0]);
    refuses_by_its_tube(
        &format!("the double dip, ε {:e}", eps()),
        declared(&wall, (0.0, 1.0)),
    );
}

/// `z = k·x + h(y)` over `[0, 1]²`: degree 1 in `u`, degree 2 in `v`,
/// with `hs.len() = 2m + 1` heights over `m` C0 quadratic spans (each
/// interior knot doubled), `y` linear in `v`.
fn c0_wall(k: f64, hs: &[f64]) -> NurbsSurface<f64> {
    let n = hs.len();
    let m = (n - 1) / 2;
    let mut kv = vec![0.0, 0.0, 0.0];
    for i in 1..m {
        #[allow(clippy::cast_precision_loss)]
        let t = i as f64 / m as f64;
        kv.extend([t, t]);
    }
    kv.extend([1.0, 1.0, 1.0]);
    let kvv = KnotVector::clamped(kv, 2).unwrap();
    let kvu = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let mut control = Vec::new();
    for x in [0.0, 1.0] {
        for (j, h) in hs.iter().enumerate() {
            #[allow(clippy::cast_precision_loss)]
            let y = j as f64 / (n - 1) as f64;
            control.push(Point3::new(x, y, k * x + h));
        }
    }
    NurbsSurface::new(kvu, kvv, control, vec![1.0; 2 * n]).unwrap()
}

/// **A carrier running past a flush stretch into one the boundary pass
/// reads clear refuses as an overrun.** `z = 10x + h(y)`, `h = 0` for
/// `y ≤ ½` and rising to `0.6ε` at `y = 1`: the side `x = 0` is the
/// locus up to `y = ½`, and past it the plane is clear of the side
/// within the band. A carrier along the side to `y = 1` (or to 0.6)
/// overruns its arc: [`OneArcRefusal::Short`], a certain verdict, not an
/// undecided one. The carrier to `y = ½` certifies. Red under an
/// undecided end window refusing as undecided.
#[test]
fn a_carrier_past_a_flush_stretch_into_a_clear_one_refuses_as_an_overrun() {
    let e = eps();
    let m = 64;
    let hs: Vec<f64> = (0..=2 * m)
        .map(|j| {
            #[allow(clippy::cast_precision_loss)]
            let y = j as f64 / (2 * m) as f64;
            if y <= 0.5 {
                0.0
            } else {
                0.6 * e * ((y - 0.5) / 0.5).powi(2)
            }
        })
        .collect();
    let wall = c0_wall(10.0, &hs);
    let got = declared(&wall, (0.0, 0.5));
    assert!(got.is_ok(), "the carrier to y = ½, ε {e:e}: {got:?}");
    for to in [0.6, 1.0] {
        let got = declared(&wall, (0.0, to));
        assert!(
            matches!(
                got,
                Err(geom_brep::PlaneNurbsRefusal::TubeNotOneArc {
                    cause: ssi::OneArcRefusal::Short,
                    ..
                })
            ),
            "the carrier to y = {to}, ε {e:e}: {got:?}"
        );
    }
}

/// **A side whose slope across it dips refuses by its cover.**
/// `z = s(y)·x − ½ε` over `[0, 1]²`, `s` quadratic over each of eight
/// C0 spans of `y`, from `10⁻³` at a span's ends to 1 at its middle, and
/// the carrier along `x = 0`. `|φ| = ½ε` along the side and no piece of
/// it is clear, and the screen ahead of the cover reads `|φ|` over each
/// window's steepest rise, 1, so it passes the side on. But at a span's
/// ends the locus lies `½ε / s`, up to 500ε, from the side, so the side's
/// cover reaches past ε: no window holds the side's piece, and the
/// chain, 500ε from the locus at each knot, refuses. Red under the side
/// arm skipping `side_cover`, where it certifies.
#[test]
fn a_side_whose_slope_across_it_dips_refuses_by_its_cover() {
    let (lo, b, m) = (1e-3, -0.5 * eps(), 8);
    let mut kv = vec![0.0, 0.0, 0.0];
    for i in 1..m {
        #[allow(clippy::cast_precision_loss)]
        let t = f64::from(i) / f64::from(m);
        kv.extend([t, t]);
    }
    kv.extend([1.0, 1.0, 1.0]);
    let n = 2 * m + 1;
    let mut control = Vec::new();
    for x in [0.0, 1.0] {
        for j in 0..n {
            let y = f64::from(j) / f64::from(n - 1);
            let s = if j % 2 == 0 { lo } else { 2.0 - lo };
            control.push(Point3::new(x, y, s * x + b));
        }
    }
    let wall = NurbsSurface::new(
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        KnotVector::clamped(kv, 2).unwrap(),
        control,
        vec![1.0; 2 * n as usize],
    )
    .unwrap();
    refuses_by_its_tube(
        &format!("the dipping slope, ε {:e}", eps()),
        declared(&wall, (0.0, 1.0)),
    );
}

/// `m` C0 quadratic spans with Bernstein coefficients `(P, −N, P)` on
/// each: `P, −N, P, …, P`.
fn loose(m: usize, p: f64, n: f64) -> Vec<f64> {
    (0..=2 * m)
        .map(|j| if j % 2 == 0 { p } else { -n })
        .collect()
}

/// The loose-side rows ([`a_side_whose_hull_is_loose_reads_clear_at_both_doors`]):
/// `z = k·x + h(y)`, `h` over `m` C0 spans each `(P, −N, P)` in units of
/// ε, then the control's `m`s.
fn loose_side_rows(grid: &[((f64, f64), f64, usize)], controls: &[usize]) {
    let e = eps();
    let (plane, _) = ground();
    let domain = SsiDomain {
        center: Point3::new(0.5, 0.5, 0.0),
        half_extent: 1.0,
        extent: 1.0,
        floor_scale: 1.0,
    };
    let low_u = ssi::ChartSide {
        fixed: ChartAxis::U,
        end: ChartEnd::Low,
    };
    let answer = |wall: &NurbsSurface<f64>, at: &str| {
        let out = ssi::plane_nurbs_ssi(&plane, wall, domain, band())
            .unwrap_or_else(|err| panic!("{at}: the search refused: {err}"));
        let region = out
            .boundary
            .iter()
            .any(|c| matches!(c, ssi::SsiBoundaryContact::Side { side, .. } if *side == low_u));
        (out.branches.len(), out.boundary.len(), region)
    };
    for &((pk, nk), k, m) in grid {
        let at = format!("P {pk}ε, N {nk}ε, k {k}, m {m}, ε {e:e}");
        let wall = c0_wall(k, &loose(m, pk * e, nk * e));
        assert_eq!(
            answer(&wall, &at),
            (0, 0, false),
            "{at}: the search's answer (branches, contacts, a region along the side)"
        );
        let got = declared(&wall, (0.0, 1.0));
        if m <= 64 {
            assert!(
                matches!(
                    got,
                    Err(geom_brep::PlaneNurbsRefusal::TubeNotOneArc {
                        cause: ssi::OneArcRefusal::Count { solutions: 0 },
                        ..
                    })
                ),
                "{at}: the carrier along the side passes limbs 1 and 2, and its tube holds \
                 no solution: {got:?}"
            );
        } else {
            assert!(
                matches!(
                    got,
                    Err(geom_brep::PlaneNurbsRefusal::Limb {
                        limb: ssi::SsiLimb::HullSup,
                        ..
                    } | geom_brep::PlaneNurbsRefusal::Escalated {
                        limb: ssi::SsiLimb::HullSup,
                        ..
                    })
                ),
                "{at}: limb 2's subdivision budget is spent, and it refuses on its bound: \
                 {got:?}"
            );
        }
    }
    for &m in controls {
        let at = format!("the control, m {m}, ε {e:e}");
        let wall = c0_wall(10.0, &loose(m, 0.3 * e, 0.5 * e));
        assert_eq!(
            answer(&wall, &at),
            (0, 1, true),
            "{at}: the search's answer (branches, contacts, a region along the side)"
        );
    }
}

/// **A side whose Bernstein hull is loose reads clear, at the search and
/// at rest.** `z = k·x + h(y)`, `h` over `m` C0 spans each `(P, −N, P)`,
/// `P > N`: along the side `x = 0`, `φ = h ≥ (P − N)/2 > 0` and the wall
/// rises inward, so the locus is empty, but every span's hull holds `−N`.
/// The search answers no branch and no region. The carrier along the
/// side lies within `P < ε` of the plane, so limb 2, subdivided, passes
/// it over 64 spans, and its tube refuses with no solution in a box;
/// over 256 or more, each of the composite's spans meets many of the
/// wall's, its hull clears only after more halving than limb 2's budget
/// holds, and it refuses on its bound. Red under the side's sign read on
/// its unrefined hull: the search then reports a `Side` region on the
/// empty locus.
///
/// The control, `(0.3ε, −0.5ε, 0.3ε)` on each span, holds two zeros of
/// `φ` on every span: its hulls straddle because `φ` does, and the side
/// is a `Side` region, as the locus within ε of it is. One row of the
/// grid and one control; the whole grid is
/// [`a_side_whose_hull_is_loose_reads_clear_over_the_grid`].
#[test]
fn a_side_whose_hull_is_loose_reads_clear_at_both_doors() {
    loose_side_rows(&[((0.6, 0.2), 10.0, 64)], &[64]);
}

/// [`a_side_whose_hull_is_loose_reads_clear_at_both_doors`] over the
/// reported grid: `P/N ∈ {0.6/0.2, 0.9/0.5}`, `k ∈ {10, 100}`,
/// `m ∈ {64, 256, 1024}`, and the control at `m = 1024` too.
#[test]
fn a_side_whose_hull_is_loose_reads_clear_over_the_grid() {
    let mut grid = Vec::new();
    for pn in [(0.6, 0.2), (0.9, 0.5)] {
        for k in [10.0, 100.0] {
            for m in [64, 256, 1024] {
                grid.push((pn, k, m));
            }
        }
    }
    loose_side_rows(&grid, &[64, 1024]);
}
