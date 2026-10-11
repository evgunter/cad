//! Reviewer probes for PR 4696 (`cone_face_closed_form` by vector area).
//!
//! The oracle is independent of the PR's: closed forms of the chart
//! integrals `∫∫ p·(p_u × p_v)` and `∫∫ |p_u × p_v|` over chart
//! rectangles (no quadrature), and for plane-section boundaries a
//! one-dimensional integral over the azimuth of the closed-form
//! `s(u)` (composite Simpson, or the periodic trapezoid for a whole
//! turn).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use crate::shared::tol::band;
use crate::shared::topo;
use geom::{Curve3, Surface};
use geom_brep::intersect::{PlaneConeSection, plane_cone_section};
use geom_brep::props::{LoopEdge, PropsError, curved_face_loops};
use geom_core::{Bounds, Decide, Interval, Point3, Real, Vec3};

const PI: f64 = core::f64::consts::PI;
const TAU: f64 = core::f64::consts::TAU;

type V = [f64; 3];
fn dot(a: V, b: V) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V, b: V) -> V {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn norm(a: V) -> V {
    let n = dot(a, a).sqrt();
    [a[0] / n, a[1] / n, a[2] / n]
}
fn add(a: V, b: V, s: f64) -> V {
    [a[0] + s * b[0], a[1] + s * b[1], a[2] + s * b[2]]
}
fn pt<T: Real>(a: V) -> Point3<T> {
    Point3::new(T::from_f64(a[0]), T::from_f64(a[1]), T::from_f64(a[2]))
}
fn vt<T: Real>(a: V) -> Vec3<T> {
    Vec3::new(T::from_f64(a[0]), T::from_f64(a[1]), T::from_f64(a[2]))
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Cone {
    apex: V,
    a: V,
    e1: V,
    e2: V,
    alpha: f64,
}

impl Cone {
    fn new(apex: V, axis: V, alpha: f64) -> Self {
        let a = norm(axis);
        let seed = if a[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
        let e1 = norm(add(seed, a, -dot(seed, a)));
        let e2 = cross(a, e1);
        Cone { apex, a, e1, e2, alpha }
    }
    fn surf<T: Real>(&self) -> Surface<T> {
        Surface::Cone {
            apex: pt(self.apex),
            axis: vt(self.a),
            half_angle: T::from_f64(self.alpha),
            u_ref: vt(self.e1),
        }
    }
    fn g(&self, u: f64) -> V {
        let (s, c) = self.alpha.sin_cos();
        let r = add([0.0; 3], self.e1, s * u.cos());
        let r = add(r, self.e2, s * u.sin());
        add(r, self.a, c)
    }
    fn rim<T: Real>(&self, v: f64, u0: f64, u1: f64, i: u32, j: u32) -> LoopEdge<T> {
        let (s, c) = self.alpha.sin_cos();
        let ur = if v < 0.0 { add([0.0; 3], self.e1, -1.0) } else { self.e1 };
        topo::edge(
            Curve3::Circle {
                center: pt(add(self.apex, self.a, v * c)),
                axis: vt(self.a),
                radius: T::from_f64(v.abs() * s),
                u_ref: vt(ur),
            },
            u0,
            u1,
            i,
            j,
        )
    }
    fn gen_<T: Real>(&self, u: f64, t0: f64, t1: f64, i: u32, j: u32) -> LoopEdge<T> {
        topo::edge(
            Curve3::Line {
                origin: pt(self.apex),
                dir: vt(self.g(u)),
            },
            t0,
            t1,
            i,
            j,
        )
    }
    fn chart_loop<T: Real>(&self, verts: &[(f64, f64)]) -> Vec<LoopEdge<T>> {
        let n = verts.len();
        let id = |i: usize| {
            if verts[i % n].1 == 0.0 {
                u32::try_from(n).unwrap()
            } else {
                u32::try_from(i % n).unwrap()
            }
        };
        let mut edges = Vec::new();
        for i in 0..n {
            let ((ua, va), (ub, vb)) = (verts[i], verts[(i + 1) % n]);
            if va == vb && va == 0.0 {
                continue;
            }
            edges.push(if va == vb {
                self.rim(va, ua, ub, id(i), id(i + 1))
            } else {
                assert_eq!(ua, ub);
                self.gen_(ua, va, vb, id(i), id(i + 1))
            });
        }
        edges
    }
    /// `(∫∫ p·(p_u×p_v), ∫∫ |p_u×p_v|)` over a chart rectangle, closed form.
    fn rect(&self, (u0, u1): (f64, f64), (v0, v1): (f64, f64)) -> (f64, f64) {
        let (s, c) = self.alpha.sin_cos();
        let half = (v1 * v1 - v0 * v0) / 2.0;
        let ae1 = dot(self.apex, self.e1);
        let ae2 = dot(self.apex, self.e2);
        let aa = dot(self.apex, self.a);
        let flux = s * half
            * (c * (ae1 * (u1.sin() - u0.sin()) + ae2 * (u0.cos() - u1.cos())) - s * aa * (u1 - u0));
        let area = s * (u1 - u0).abs() * half.abs();
        (flux, area)
    }
    /// The per-azimuth flux and area densities of the region
    /// `0 ≤ v ≤ s` at azimuth `u`, integrated in `v`.
    fn column(&self, u: f64, sv: f64) -> (f64, f64) {
        let (s, c) = self.alpha.sin_cos();
        let ae1 = dot(self.apex, self.e1);
        let ae2 = dot(self.apex, self.e2);
        let aa = dot(self.apex, self.a);
        let half = sv * sv / 2.0;
        (
            s * half * (c * (ae1 * u.cos() + ae2 * u.sin()) - s * aa),
            s * half,
        )
    }
}

fn orientation(verts: &[(f64, f64)]) -> f64 {
    let n = verts.len();
    if verts.iter().all(|p| p.1 == verts[0].1) {
        // A rim-only cap: the rim is the far side of the region from
        // the apex; on `v > 0` it is the top, so `+u` is clockwise.
        let du = verts[n - 1].0 - verts[0].0;
        return if verts[0].1 > 0.0 { -du.signum() } else { du.signum() };
    }
    let twice: f64 = (0..n)
        .map(|i| {
            let ((ua, va), (ub, vb)) = (verts[i], verts[(i + 1) % n]);
            ua * vb - ub * va
        })
        .sum();
    twice.signum()
}

/// A chart face: outer polygon then holes (already wound against it),
/// and the chart rectangles tiling it, weighted ±1.
#[derive(Clone)]
pub(crate) struct ChartFace {
    name: &'static str,
    loops: Vec<Vec<(f64, f64)>>,
    rects: Vec<((f64, f64), (f64, f64), f64)>,
}

fn rev(v: &[(f64, f64)]) -> Vec<(f64, f64)> {
    v.iter().rev().copied().collect()
}

impl ChartFace {
    fn scaled(&self, k: f64) -> Self {
        ChartFace {
            name: self.name,
            loops: self
                .loops
                .iter()
                .map(|l| l.iter().map(|&(u, v)| (u, v * k)).collect())
                .collect(),
            rects: self
                .rects
                .iter()
                .map(|&(u, (v0, v1), w)| (u, (v0 * k, v1 * k), w))
                .collect(),
        }
    }
    fn mirrored(&self) -> Self {
        ChartFace {
            name: self.name,
            loops: self
                .loops
                .iter()
                .map(|l| l.iter().map(|&(u, v)| (u, -v)).collect())
                .collect(),
            rects: self
                .rects
                .iter()
                .map(|&(u, (v0, v1), w)| (u, (-v1, -v0), w))
                .collect(),
        }
    }
    fn oracle(&self, c: &Cone) -> (f64, f64) {
        let (mut f, mut a) = (0.0, 0.0);
        for &(u, v, w) in &self.rects {
            let (fr, ar) = c.rect(u, v);
            f += w * fr;
            a += w * ar;
        }
        (f, a)
    }
}

fn faces() -> Vec<ChartFace> {
    let notched = vec![
        (0.0, 1.0),
        (PI, 1.0),
        (PI, 2.0),
        (2.0, 2.0),
        (2.0, 1.6),
        (1.0, 1.6),
        (1.0, 2.0),
        (0.0, 2.0),
    ];
    let notched_rects = vec![
        ((0.0, PI), (1.0, 1.6), 1.0),
        ((0.0, 1.0), (1.6, 2.0), 1.0),
        ((2.0, PI), (1.6, 2.0), 1.0),
    ];
    let mut out = vec![
        ChartFace {
            name: "notched",
            loops: vec![notched.clone()],
            rects: notched_rects.clone(),
        },
        ChartFace {
            name: "stepped",
            loops: vec![vec![
                (0.5, 1.0),
                (3.5, 1.0),
                (3.5, 1.4),
                (2.5, 1.4),
                (2.5, 1.9),
                (1.5, 1.9),
                (1.5, 2.6),
                (0.5, 2.6),
            ]],
            rects: vec![
                ((0.5, 3.5), (1.0, 1.4), 1.0),
                ((0.5, 2.5), (1.4, 1.9), 1.0),
                ((0.5, 1.5), (1.9, 2.6), 1.0),
            ],
        },
        ChartFace {
            name: "notched + 2 holes, one straddling u=pi",
            loops: vec![
                notched.clone(),
                rev(&[(2.3, 1.1), (2.8, 1.1), (2.8, 1.4), (2.3, 1.4)]),
                rev(&[(0.2, 1.05), (0.7, 1.05), (0.7, 1.9), (0.2, 1.9)]),
            ],
            rects: {
                let mut r = notched_rects.clone();
                r.push(((2.3, 2.8), (1.1, 1.4), -1.0));
                r.push(((0.2, 0.7), (1.05, 1.9), -1.0));
                r
            },
        },
        ChartFace {
            name: "wall across the atan2 cut, vertices at pi",
            loops: vec![
                vec![(2.0, 1.0), (4.5, 1.0), (4.5, 2.0), (2.0, 2.0)],
                rev(&[(PI - 0.3, 1.2), (PI, 1.2), (PI, 1.4), (PI + 0.4, 1.4), (PI + 0.4, 1.8), (PI - 0.3, 1.8)]),
            ],
            rects: vec![
                ((2.0, 4.5), (1.0, 2.0), 1.0),
                ((PI - 0.3, PI), (1.2, 1.8), -1.0),
                ((PI, PI + 0.4), (1.4, 1.8), -1.0),
            ],
        },
        ChartFace {
            name: "apex cap, one full rim",
            loops: vec![vec![(0.0, 1.5), (TAU, 1.5)]],
            rects: vec![((0.0, TAU), (0.0, 1.5), 1.0)],
        },
        ChartFace {
            name: "apex cap with a hole across pi",
            loops: vec![
                vec![(0.0, 1.5), (TAU, 1.5)],
                vec![(2.9, 0.4), (3.6, 0.4), (3.6, 1.1), (2.9, 1.1)],
            ],
            rects: vec![
                ((0.0, TAU), (0.0, 1.5), 1.0),
                ((2.9, 3.6), (0.4, 1.1), -1.0),
            ],
        },
        ChartFace {
            name: "sector",
            loops: vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.5), (0.0, 1.5)]],
            rects: vec![((0.0, 2.0), (0.0, 1.5), 1.0)],
        },
        ChartFace {
            name: "wide sector 5.9 rad",
            loops: vec![vec![(0.2, 0.0), (6.1, 0.0), (6.1, 1.5), (0.2, 1.5)]],
            rects: vec![((0.2, 6.1), (0.0, 1.5), 1.0)],
        },
        ChartFace {
            name: "sector across pi",
            loops: vec![vec![(2.5, 0.0), (PI, 0.0), (PI, 1.5), (2.5, 1.5)]],
            rects: vec![((2.5, PI), (0.0, 1.5), 1.0)],
        },
        ChartFace {
            name: "slit apex cap (seam generator both ways)",
            loops: vec![vec![(0.0, 0.0), (TAU, 0.0), (TAU, 1.5), (0.0, 1.5)]],
            rects: vec![((0.0, TAU), (0.0, 1.5), 1.0)],
        },
        ChartFace {
            name: "full band with seam",
            loops: vec![vec![(0.0, 1.0), (TAU, 1.0), (TAU, 2.0), (0.0, 2.0)]],
            rects: vec![((0.0, TAU), (1.0, 2.0), 1.0)],
        },
        ChartFace {
            name: "wall 6.2 rad",
            loops: vec![vec![(0.0, 1.0), (6.2, 1.0), (6.2, 2.0), (0.0, 2.0)]],
            rects: vec![((0.0, 6.2), (1.0, 2.0), 1.0)],
        },
    ];
    // An apex cap whose rim arrives in three pieces.
    out.push(ChartFace {
        name: "apex cap, three rim pieces",
        loops: vec![vec![(0.0, 1.5), (2.0, 1.5), (4.0, 1.5), (TAU, 1.5)]],
        rects: vec![((0.0, TAU), (0.0, 1.5), 1.0)],
    });
    out
}

/// Loops of a chart face where a `TAU`-closing single rim is one edge.
fn build<T: Real>(c: &Cone, f: &ChartFace, forward: bool) -> Vec<Vec<LoopEdge<T>>> {
    f.loops
        .iter()
        .map(|l| {
            let l = if forward { l.clone() } else { rev(l) };
            // A polygon whose every vertex is at one slant is a rim
            // chain: chart_loop closes it with a rim back to start,
            // which for the cap is the full turn (TAU,v)->(0,v); drop
            // that closing rim and instead let the last vertex be TAU.
            if l.iter().all(|p| p.1 == l[0].1) {
                let v = l[0].1;
                let mut us: Vec<f64> = l.iter().map(|p| p.0).collect();
                // us runs 0..TAU or TAU..0; edges between consecutive.
                let mut e = Vec::new();
                let n = us.len();
                for i in 0..n - 1 {
                    e.push(c.rim(v, us[i], us[i + 1], u32::try_from(i).unwrap(), u32::try_from((i + 1) % (n - 1)).unwrap()));
                }
                us.clear();
                e
            } else {
                c.chart_loop(&l)
            }
        })
        .collect()
}

fn run<T: Decide + Bounds>(c: &Cone, loops: &[Vec<LoopEdge<T>>]) -> Result<(T, T), PropsError> {
    let ls: Vec<&[LoopEdge<T>]> = loops.iter().map(Vec::as_slice).collect();
    curved_face_loops(&c.surf::<T>(), &ls, true, band()).map(|fc| (fc.flux, fc.area))
}

fn cones() -> Vec<Cone> {
    let mut v = Vec::new();
    for &alpha in &[1e-6, 1e-3, 0.5, 1.2, PI / 2.0 - 1e-3, PI / 2.0 - 1e-6] {
        for &apex in &[[0.2, -0.1, -1.0], [31.0, -17.0, 9.0]] {
            for &axis in &[[0.0, 0.0, 1.0], [0.3, -0.4, 1.0], [1.0, 1.0, 0.1]] {
                v.push(Cone::new(apex, axis, alpha));
            }
        }
    }
    v
}

/// Every chart face × cone × nappe × traversal × scale: f64 within
/// relative 1e-9 of the oracle (reporting the worst), and the Interval
/// enclosure containing the oracle up to a slack of 64 ulps of the
/// face's magnitude scale (the inputs are themselves f64-rounded).
#[test]
fn probe_chart_faces_against_closed_form_oracle() {
    let mut fails = Vec::new();
    let mut worst_rel: f64 = 0.0;
    let mut worst_miss_ulps: f64 = 0.0;
    let mut n = 0;
    for c in cones() {
        for f0 in faces() {
            for &k in &[1e-3, 1.0, 1e3] {
                for (nap, f) in [("+", f0.scaled(k)), ("-", f0.scaled(k).mirrored())] {
                    let (flux_ccw, area) = f.oracle(&c);
                    // flux scale: |apex|·area plus slant·area
                    let fscale = (dot(c.apex, c.apex).sqrt() + 3.0 * k) * area.max(1e-300);
                    for forward in [true, false] {
                        let o = orientation(&if forward { f.loops[0].clone() } else { rev(&f.loops[0]) });
                        let want_flux = o * flux_ccw;
                        let tag = format!(
                            "{} k={k} nappe {nap} fwd {forward} alpha {} apex {:?} axis {:?}",
                            f.name, c.alpha, c.apex, c.a
                        );
                        n += 1;
                        match run::<f64>(&c, &build::<f64>(&c, &f, forward)) {
                            Ok((fl, ar)) => {
                                let ra = (ar - area).abs() / area;
                                let rf = (fl - want_flux).abs() / fscale;
                                worst_rel = worst_rel.max(ra).max(rf);
                                if ra > 1e-9 || rf > 1e-9 {
                                    fails.push(format!("F64 WRONG {tag}: area {ar:e} vs {area:e} (rel {ra:e}); flux {fl:e} vs {want_flux:e} (rel {rf:e})"));
                                }
                            }
                            Err(e) => fails.push(format!("F64 REFUSED {tag}: {e:?}")),
                        }
                        match run::<Interval>(&c, &build::<Interval>(&c, &f, forward)) {
                            Ok((fl, ar)) => {
                                let slack_a = 64.0 * f64::EPSILON * area;
                                let slack_f = 64.0 * f64::EPSILON * fscale;
                                let miss_a = (ar.lo() - area).max(area - ar.hi()).max(0.0);
                                let miss_f = (fl.lo() - want_flux).max(want_flux - fl.hi()).max(0.0);
                                worst_miss_ulps = worst_miss_ulps
                                    .max(miss_a / (f64::EPSILON * area))
                                    .max(miss_f / (f64::EPSILON * fscale));
                                if miss_a > slack_a || miss_f > slack_f {
                                    fails.push(format!(
                                        "IV MISS {tag}: area [{:e},{:e}] vs {area:e}; flux [{:e},{:e}] vs {want_flux:e}",
                                        ar.lo(), ar.hi(), fl.lo(), fl.hi()
                                    ));
                                }
                            }
                            Err(e) => fails.push(format!("IV REFUSED {tag}: {e:?}")),
                        }
                    }
                }
            }
        }
    }
    eprintln!("cases {n}; worst f64 rel {worst_rel:e}; worst interval miss {worst_miss_ulps:e} ulps-of-scale; {} failures", fails.len());
    let mut kinds: std::collections::BTreeMap<String, usize> = Default::default();
    for f in &fails {
        let key: String = f.split(" k=").next().unwrap().to_string();
        *kinds.entry(key).or_default() += 1;
    }
    for (k, c) in &kinds {
        eprintln!("  {c:5} × {k}");
    }
    if let Ok(path) = std::env::var("R2_DUMP") {
        std::fs::write(path, fails.join("\n")).unwrap();
    }
    for f in fails.iter().filter(|f| f.contains("IV REFUSED") && f.contains("alpha 0.5 ")).take(400) {
        eprintln!("{f}");
    }
    assert!(fails.is_empty());
}

// ---------------------------------------------------------------------
// Plane-section (ellipse) boundaries
// ---------------------------------------------------------------------

/// A cone off the origin about a skew axis (either sign) and its section
/// by a plane tilted `tilt` off the axis `h` along it.
fn section(c: &Cone, tilt: f64, h: f64) -> (Curve3<f64>, V, V) {
    let side = norm(cross(c.a, [1.0, 0.0, 0.0]));
    let normal = norm(add(add([0.0; 3], c.a, tilt.cos()), side, tilt.sin()));
    let origin = add(c.apex, c.a, h);
    let plane = Surface::Plane {
        origin: pt(origin),
        normal: vt(normal),
        u_ref: vt(cross(side, normal)),
    };
    let Ok(PlaneConeSection::TiltedEllipse(e)) = plane_cone_section(&plane, &c.surf::<f64>(), 3.0, band())
    else {
        panic!("not an ellipse");
    };
    (e, origin, normal)
}

fn lift<T: Real>(e: &Curve3<f64>) -> Curve3<T> {
    let &Curve3::Ellipse { center, axis, major, minor, u_ref } = e else { unreachable!() };
    Curve3::Ellipse {
        center: pt([center.x, center.y, center.z]),
        axis: vt([axis.x, axis.y, axis.z]),
        major: T::from_f64(major),
        minor: T::from_f64(minor),
        u_ref: vt([u_ref.x, u_ref.y, u_ref.z]),
    }
}

fn eval(e: &Curve3<f64>, t: f64) -> V {
    let p = e.eval(t);
    [p.x, p.y, p.z]
}

/// The chart azimuth of a point on the cone.
fn azimuth(c: &Cone, p: V) -> f64 {
    let mut w = add(p, c.apex, -1.0);
    if dot(w, c.a) < 0.0 {
        w = add([0.0; 3], w, -1.0);
    }
    dot(w, c.e2).atan2(dot(w, c.e1))
}

/// The plane's slant `s(u)` at azimuth `u`: `n·(P0 − A) / (n·g(u))`.
fn s_of(c: &Cone, o: V, n: V, u: f64) -> f64 {
    dot(n, add(o, c.apex, -1.0)) / dot(n, c.g(u))
}

/// Region `0 ≤ v ≤ s(u)` over azimuth `u ∈ [ua, ub]` — composite Simpson.
fn section_oracle(c: &Cone, o: V, n: V, ua: f64, ub: f64) -> (f64, f64) {
    let m = 20000;
    let h = (ub - ua) / f64::from(m);
    let (mut f, mut a) = (0.0, 0.0);
    for i in 0..=m {
        let u = ua + h * f64::from(i);
        let w = if i == 0 || i == m { 1.0 } else if i % 2 == 1 { 4.0 } else { 2.0 };
        let (cf, ca) = c.column(u, s_of(c, o, n, u));
        f += w * cf;
        a += w * ca;
    }
    (f * h / 3.0, a * h / 3.0)
}

fn check<T: Decide + Bounds>(label: &str, c: &Cone, loops: &[Vec<LoopEdge<T>>], flux: f64, area: f64, fails: &mut Vec<String>) {
    match run::<T>(c, loops) {
        Ok((fl, ar)) => {
            let fs = (dot(c.apex, c.apex).sqrt() + 5.0) * area;
            let ok_a = ar.lo() - 1e-9 * area <= area && area <= ar.hi() + 1e-9 * area;
            let ok_f = fl.lo() - 1e-9 * fs <= flux && flux <= fl.hi() + 1e-9 * fs;
            eprintln!(
                "{label}: area [{:.12e},{:.12e}] oracle {area:.12e}; flux [{:.12e},{:.12e}] oracle {flux:.12e}",
                ar.lo(), ar.hi(), fl.lo(), fl.hi()
            );
            if !(ok_a && ok_f) {
                fails.push(label.to_string());
            }
        }
        Err(e) => {
            eprintln!("{label}: REFUSED {e:?}");
            fails.push(format!("{label} refused {e:?}"));
        }
    }
}

#[test]
fn probe_ellipse_cut_faces() {
    let mut fails = Vec::new();
    for &(alpha, tilt) in &[(0.45, 0.5), (0.05, 1.5), (1.4, 0.1), (0.9, 0.6)] {
        for &sgn in &[1.0, -1.0] {
            let base = Cone::new([0.4, -0.7, 0.2], [0.2, -0.3, 1.0], alpha);
            let (e, o, n) = section(&base, tilt, 2.5);
            // The same locus about the opposite axis sign: the face is on
            // the other nappe of that cone.
            let c = Cone::new(base.apex, add([0.0; 3], base.a, sgn), alpha);
            // Whole tilted cap: one full ellipse edge.
            let (f_ccw, area) = section_oracle(&c, o, n, 0.0, TAU);
            let p0 = eval(&e, 0.0);
            let p1 = eval(&e, 1e-3);
            let du = (azimuth(&c, p1) - azimuth(&c, p0) + 3.0 * PI).rem_euclid(TAU) - PI;
            for forward in [true, false] {
                // In the chart the curve is the TOP of `0 ≤ v ≤ s`; a
                // top traversed in +u is clockwise.
                let o_sign = -du.signum() * if forward { 1.0 } else { -1.0 };
                let mk = |fwd: bool| -> Vec<Vec<LoopEdge<f64>>> {
                    vec![vec![topo::edge(e.clone(), if fwd { 0.0 } else { TAU }, if fwd { TAU } else { 0.0 }, 0, 0)]]
                };
                let mki = |fwd: bool| -> Vec<Vec<LoopEdge<Interval>>> {
                    vec![vec![topo::edge(lift::<Interval>(&e), if fwd { 0.0 } else { TAU }, if fwd { TAU } else { 0.0 }, 0, 0)]]
                };
                let label = format!("ellipse cap alpha {alpha} tilt {tilt} axis {sgn} fwd {forward}");
                check(&format!("{label} f64"), &c, &mk(forward), o_sign * f_ccw, area, &mut fails);
                check(&format!("{label} iv"), &c, &mki(forward), o_sign * f_ccw, area, &mut fails);
            }
            // Ellipse sector: apex → E(ta) → arc → E(tb) → apex.
            for &(ta, tb) in &[(0.3, 2.1), (2.0, 4.6), (5.0, 7.5)] {
                let (pa, pb) = (eval(&e, ta), eval(&e, tb));
                let (ua, ub0) = (azimuth(&c, pa), azimuth(&c, pb));
                // Unwrap ub to follow the arc's own turn.
                let mut ub = ua;
                let steps = 400;
                let mut prev = ua;
                for i in 1..=steps {
                    let t = ta + (tb - ta) * f64::from(i) / f64::from(steps);
                    let z = azimuth(&c, eval(&e, t));
                    let d = (z - prev + 3.0 * PI).rem_euclid(TAU) - PI;
                    ub += d;
                    prev = z;
                }
                let _ = ub0;
                let (f_ccw, area) = section_oracle(&c, o, n, ua.min(ub), ua.max(ub));
                let o_sign = -(ub - ua).signum();
                let la = dot(add(pa, c.apex, -1.0), add(pa, c.apex, -1.0)).sqrt();
                let lb = dot(add(pb, c.apex, -1.0), add(pb, c.apex, -1.0)).sqrt();
                let gdir = |p: V, l: f64, t0: f64, t1: f64, i: u32, j: u32| {
                    let d = norm(add(p, c.apex, -1.0));
                    let _ = l;
                    (d, t0, t1, i, j)
                };
                let (da, ..) = gdir(pa, la, 0.0, la, 9, 0);
                let (db, ..) = gdir(pb, lb, lb, 0.0, 1, 9);
                fn mk<T: Real>(c: &Cone, e: &Curve3<T>, da: V, db: V, la: f64, lb: f64, ta: f64, tb: f64) -> Vec<Vec<LoopEdge<T>>> {
                    vec![vec![
                        topo::edge(Curve3::Line { origin: pt(c.apex), dir: vt(da) }, 0.0, la, 9, 0),
                        topo::edge(e.clone(), ta, tb, 0, 1),
                        topo::edge(Curve3::Line { origin: pt(c.apex), dir: vt(db) }, lb, 0.0, 1, 9),
                    ]]
                }
                let label = format!("ellipse sector alpha {alpha} tilt {tilt} axis {sgn} t {ta}..{tb}");
                check(&format!("{label} f64"), &c, &mk(&c, &e, da, db, la, lb, ta, tb), o_sign * f_ccw, area, &mut fails);
                check(&format!("{label} iv"), &c, &mk(&c, &lift::<Interval>(&e), da, db, la, lb, ta, tb), o_sign * f_ccw, area, &mut fails);
            }
        }
    }
    eprintln!("failures: {fails:#?}");
    assert!(fails.is_empty());
}

// ---------------------------------------------------------------------
// Premise probes
// ---------------------------------------------------------------------

fn c0() -> Cone {
    Cone::new([0.2, -0.1, -1.0], [0.3, -0.4, 1.0], 0.5)
}

fn report<T: Decide + Bounds>(label: &str, c: &Cone, loops: &[Vec<LoopEdge<T>>]) -> Option<(f64, f64)> {
    match run::<T>(c, loops) {
        Ok((f, a)) => {
            eprintln!("{label}: MEASURES area [{:.15e},{:.15e}] flux [{:.6e},{:.6e}]", a.lo(), a.hi(), f.lo(), f.hi());
            Some((a.lo(), a.hi()))
        }
        Err(e) => {
            eprintln!("{label}: refuses {e:?}");
            None
        }
    }
}

/// A quad whose generators cross the apex by `δ` onto the other nappe,
/// at δ spanning the band: what does the nappe decide answer, and what
/// does the face measure against its two-nappe area?
#[test]
fn probe_nappe_band_ladder() {
    let c = c0();
    let b = band();
    let s = 0.5_f64.sin();
    for &(k, which) in &[(0.25, "zero"), (0.9, "zero"), (1.5, "zero"), (0.5, "esc"), (1.0, "esc"), (2.0, "esc"), (10.0, "esc")] {
        let d = k * if which == "zero" { b.zero() } else { b.escalate() };
        let quad = [(0.0, -d), (1.0, -d), (1.0, 2.0), (0.0, 2.0)];
        let true_area = s * 1.0 * (4.0 + d * d) / 2.0;
        let label = format!("cross by {k}·{which} = {d:e} (true two-nappe area {true_area:.15e})");
        report::<f64>(&format!("{label} f64"), &c, &[c.chart_loop(&quad)]);
        report::<Interval>(&format!("{label} iv"), &c, &[c.chart_loop(&quad)]);
    }
    // An apex cap whose rim is at slant d (a face of tiny size near apex).
    for &k in &[0.5, 2.0, 20.0] {
        let d = k * b.escalate();
        let l = vec![c.rim::<f64>(d, 0.0, TAU, 0, 0)];
        report::<f64>(&format!("tiny apex cap at slant {d:e}"), &c, &[l]);
    }
}

/// Loops that bound their region more than once, or with a lobe
/// against: a sector stated twice, a rectangle stated twice, and a
/// figure-eight.
#[test]
fn probe_loops_stated_twice_and_pinched() {
    let c = c0();
    let s = 0.5_f64.sin();
    let sector = [(0.0, 0.0), (2.0, 0.0), (2.0, 1.5), (0.0, 1.5)];
    let one = s * 2.0 * 1.5 * 1.5 / 2.0;
    let mut twice = c.chart_loop::<f64>(&sector);
    twice.extend(c.chart_loop::<f64>(&sector));
    eprintln!("sector area {one:e}");
    report::<f64>("sector stated twice", &c, &[twice]);
    let mut twice_i = c.chart_loop::<Interval>(&sector);
    twice_i.extend(c.chart_loop::<Interval>(&sector));
    report::<Interval>("sector stated twice iv", &c, &[twice_i]);

    let rect = [(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)];
    let r1 = s * 1.0 * 3.0 / 2.0;
    let mut t = c.chart_loop::<f64>(&rect);
    t.extend(c.chart_loop::<f64>(&rect));
    eprintln!("rect area {r1:e}");
    report::<f64>("rect stated twice", &c, &[t]);

    // Figure-eight: lobe A = [0,1]x[1,2] CCW, lobe B = [1,2]x[2,3] CW.
    let eight = [(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (1.0, 3.0), (2.0, 3.0), (2.0, 2.0), (1.0, 2.0), (0.0, 2.0)];
    let la = s * 1.0 * (4.0 - 1.0) / 2.0;
    let lb = s * 1.0 * (9.0 - 4.0) / 2.0;
    eprintln!("figure-eight lobes {la:e} and {lb:e}: true region {:e}", la + lb);
    // chart_loop's ids are positional; the pinch vertex appears twice.
    report::<f64>("figure-eight", &c, &[c.chart_loop(&eight)]);

    // A sector traversed out along u=0, round the rim 4 rad past the
    // start, and back: a rim overlapping itself in a non-closed winding.
    let over = [(0.0, 0.0), (TAU + 2.0, 0.0), (TAU + 2.0, 1.5), (0.0, 1.5)];
    report::<f64>("sector of 2pi+2 (rim overlaps itself)", &c, &[c.chart_loop(&over)]);
    let over = [(0.0, 0.0), (TAU + 2.9, 0.0), (TAU + 2.9, 1.5), (0.0, 1.5)];
    report::<f64>("sector of 2pi+2.9", &c, &[c.chart_loop(&over)]);
    // A wall with a rim wrapping 2pi+1 on top and 1 on the bottom
    // cannot close; a wall wrapping 2pi+1 both rims: winding 0.
    let w = [(0.0, 1.0), (TAU + 1.0, 1.0), (TAU + 1.0, 2.0), (0.0, 2.0)];
    report::<f64>("wall of 2pi+1 (overlaps itself)", &c, &[c.chart_loop(&w)]);
}

/// Edges that are not on the cone at all.
#[test]
fn probe_off_surface_edges() {
    let c = c0();
    let (s, co) = 0.5_f64.sin_cos();
    let rect = [(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)];
    let good = c.chart_loop::<f64>(&rect);
    report::<f64>("rect on the cone", &c, &[good.clone()]);
    // Rim of the wrong radius (10% too large) at v=2, endpoints joined
    // by lines from the true lower corners.
    let mut bad = good.clone();
    // edges: rim v=1 (0->1), gen u=1 (1->2), rim v=2 (1->0), gen u=0 (2->1)
    bad[2] = topo::edge(
        Curve3::Circle {
            center: pt(add(c.apex, c.a, 2.0 * co)),
            axis: vt(c.a),
            radius: 1.1 * 2.0 * s,
            u_ref: vt(c.e1),
        },
        1.0,
        0.0,
        2,
        3,
    );
    report::<f64>("rim radius 10% off (loop does not close)", &c, &[bad]);
    // A loop of four straight chords between the corners: closes, but
    // only the generators lie on the cone.
    let corner = |u: f64, v: f64| add(c.apex, c.g(u), v);
    let chord = |p: V, q: V, i: u32, j: u32| {
        let d = add(q, p, -1.0);
        let l = dot(d, d).sqrt();
        topo::edge(Curve3::Line { origin: pt(p), dir: vt(norm(d)) }, 0.0, l, i, j)
    };
    let chords = vec![
        chord(corner(0.0, 1.0), corner(1.0, 1.0), 0, 1),
        good[1].clone(),
        chord(corner(1.0, 2.0), corner(0.0, 2.0), 2, 3),
        good[3].clone(),
    ];
    report::<f64>("rims replaced by straight chords (planar-ish quad, off the cone)", &c, &[chords]);
    // A generator replaced by a line at the generator angle that misses
    // the apex: the axis-offset copy of generator u=1 by 0.3 in e2.
    let mut off = good.clone();
    off[1] = topo::edge(
        Curve3::Line { origin: pt(add(c.apex, c.e2, 0.3)), dir: vt(c.g(1.0)) },
        1.0,
        2.0,
        1,
        2,
    );
    report::<f64>("generator offset off the apex (open)", &c, &[off]);
}

/// Ring premises: with the face, non-contractible, overlapping, and
/// outside the region.
#[test]
fn probe_rings() {
    let c = c0();
    let s = 0.5_f64.sin();
    let outer = [(0.0, 1.0), (3.0, 1.0), (3.0, 2.0), (0.0, 2.0)];
    let ra = |u0: f64, u1: f64, v0: f64, v1: f64| s * (u1 - u0) * (v1 * v1 - v0 * v0) / 2.0;
    let oa = ra(0.0, 3.0, 1.0, 2.0);
    let h1 = [(0.2, 1.1), (2.0, 1.1), (2.0, 1.9), (0.2, 1.9)];
    let h2 = [(1.0, 1.1), (2.8, 1.1), (2.8, 1.9), (1.0, 1.9)];
    eprintln!("outer {oa:e}; h1 {:e}; h2 {:e}; union {:e}", ra(0.2, 2.0, 1.1, 1.9), ra(1.0, 2.8, 1.1, 1.9), ra(0.2, 2.8, 1.1, 1.9));
    let o = c.chart_loop::<f64>(&outer);
    report::<f64>("ring wound WITH the face", &c, &[o.clone(), c.chart_loop(&h1)]);
    report::<f64>("overlapping rings, sum > region", &c, &[o.clone(), c.chart_loop(&rev(&h1)), c.chart_loop(&rev(&h2))]);
    let h3 = [(0.2, 1.3), (1.4, 1.3), (1.4, 1.7), (0.2, 1.7)];
    let h4 = [(1.0, 1.3), (2.2, 1.3), (2.2, 1.7), (1.0, 1.7)];
    eprintln!("h3 {:e} h4 {:e} union {:e} → true face {:e}", ra(0.2, 1.4, 1.3, 1.7), ra(1.0, 2.2, 1.3, 1.7), ra(0.2, 2.2, 1.3, 1.7), oa - ra(0.2, 2.2, 1.3, 1.7));
    report::<f64>("overlapping rings, sum < region", &c, &[o.clone(), c.chart_loop(&rev(&h3)), c.chart_loop(&rev(&h4))]);
    let h5 = [(4.0, 1.3), (5.0, 1.3), (5.0, 1.7), (4.0, 1.7)];
    eprintln!("ring outside the region: h5 {:e} → true face (outer alone) {oa:e}", ra(4.0, 5.0, 1.3, 1.7));
    report::<f64>("ring outside the outer loop", &c, &[o.clone(), c.chart_loop(&rev(&h5))]);
    let h6 = [(2.5, 1.3), (3.5, 1.3), (3.5, 1.7), (2.5, 1.7)];
    let h7 = [(0.1, 1.05), (2.0, 1.05), (2.0, 1.95), (0.1, 1.95)];
    let h8 = [(1.0, 1.05), (2.9, 1.05), (2.9, 1.95), (1.0, 1.95)];
    eprintln!("h7 {:e} h8 {:e} sum {:e} > outer {oa:e}", ra(0.1, 2.0, 1.05, 1.95), ra(1.0, 2.9, 1.05, 1.95), ra(0.1, 2.0, 1.05, 1.95) + ra(1.0, 2.9, 1.05, 1.95));
    report::<f64>("overlapping rings, sum TRULY > region", &c, &[o.clone(), c.chart_loop(&rev(&h7)), c.chart_loop(&rev(&h8))]);
    report::<f64>("ring straddling the outer loop", &c, &[o.clone(), c.chart_loop(&rev(&h6))]);
    // Non-contractible ring: an apex cap with a full rim ring inside it.
    let cap = vec![c.rim::<f64>(1.5, 0.0, TAU, 0, 0)];
    let ring = vec![c.rim::<f64>(0.5, TAU, 0.0, 1, 1)];
    report::<f64>("apex cap with an annular (winding) ring", &c, &[cap.clone(), ring]);
    let ring_with = vec![c.rim::<f64>(0.5, 0.0, TAU, 1, 1)];
    report::<f64>("apex cap with an annular ring wound with it", &c, &[cap, ring_with]);
    // A ring around the apex but through the apex region: a sector-ring
    // hole touching the apex inside an apex cap.
    let cap = vec![c.rim::<f64>(1.5, 0.0, TAU, 0, 0)];
    report::<f64>("apex cap with a ring touching the apex", &c, &[cap, c.chart_loop(&rev(&[(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)]))]);
}
