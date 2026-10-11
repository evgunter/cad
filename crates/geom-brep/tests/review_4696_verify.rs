//! Verifier probes for PR 4696's fix pass (350b7d67, 96a8df44): every
//! row is generic over `f64` and `Interval` and runs on a tilted axis
//! with an off-origin apex.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use crate::shared::tol::band;
use crate::shared::topo;
use geom::{Curve3, Surface};
use geom_brep::props::{
    LoopEdge, MaterialSign, PropsError, boundary_material_sign_loops, curved_face_loops,
};
use geom_core::{Bounds, Decide, Interval, Point3, Real, Vec3};

const PI: f64 = core::f64::consts::PI;
const TAU: f64 = core::f64::consts::TAU;
type V = [f64; 3];
fn dot(a: V, b: V) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V, b: V) -> V {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn unit(a: V) -> V {
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

#[derive(Clone, Copy)]
struct Cone {
    apex: V,
    k: V,
    e1: V,
    e2: V,
    alpha: f64,
}
impl Cone {
    fn new(apex: V, axis: V, alpha: f64) -> Self {
        let k = unit(axis);
        let t = if k[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
        let e1 = unit(add(t, k, -dot(t, k)));
        Cone { apex, k, e1, e2: cross(k, e1), alpha }
    }
    fn surf<T: Real>(&self) -> Surface<T> {
        Surface::Cone {
            apex: pt(self.apex),
            axis: vt(self.k),
            half_angle: T::from_f64(self.alpha),
            u_ref: vt(self.e1),
        }
    }
    fn g(&self, u: f64) -> V {
        let (s, c) = self.alpha.sin_cos();
        add(add(add([0.0; 3], self.e1, s * u.cos()), self.e2, s * u.sin()), self.k, c)
    }
    fn at(&self, u: f64, v: f64) -> V {
        add(self.apex, self.g(u), v)
    }
    fn rim<T: Real>(&self, v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<T> {
        let (s, c) = self.alpha.sin_cos();
        topo::edge(
            Curve3::Circle {
                center: pt(add(self.apex, self.k, v * c)),
                axis: vt(self.k),
                radius: T::from_f64(v.abs() * s),
                u_ref: vt(add([0.0; 3], self.e1, v.signum())),
            },
            u0,
            u1,
            a,
            b,
        )
    }
    fn gen_<T: Real>(&self, u: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<T> {
        topo::edge(Curve3::Line { origin: pt(self.apex), dir: vt(self.g(u)) }, t0, t1, a, b)
    }
    fn chart_loop<T: Real>(&self, verts: &[(f64, f64)]) -> Vec<LoopEdge<T>> {
        let n = verts.len();
        let id = |i: usize| if verts[i % n].1 == 0.0 { n as u32 } else { (i % n) as u32 };
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
    /// Unsigned area of chart rectangles (one nappe, v ≥ 0).
    fn area(&self, rects: &[((f64, f64), (f64, f64))]) -> f64 {
        let s = self.alpha.sin();
        rects.iter().map(|&((u0, u1), (v0, v1))| s * (u1 - u0) * (v1 * v1 - v0 * v0) / 2.0).sum()
    }
}

fn c0() -> Cone {
    Cone::new([0.2, -0.1, -1.0], [0.3, -0.4, 1.0], 0.5)
}

fn run<T: Decide>(c: &Cone, loops: &[Vec<LoopEdge<T>>], sense: bool) -> Result<(T, T), PropsError> {
    let l: Vec<&[LoopEdge<T>]> = loops.iter().map(Vec::as_slice).collect();
    curved_face_loops(&c.surf::<T>(), &l, sense, band()).map(|f| (f.flux, f.area))
}
fn side<T: Decide>(c: &Cone, loops: &[Vec<LoopEdge<T>>]) -> Result<MaterialSign, PropsError> {
    let l: Vec<&[LoopEdge<T>]> = loops.iter().map(Vec::as_slice).collect();
    boundary_material_sign_loops(&c.surf::<T>(), &l, band())
}

/// Both scalars, both senses, the flux lane: the same refusal. The
/// material-sign lane is reported alongside.
fn refuses_both(name: &str, c: &Cone, f: &dyn Fn() -> (Vec<Vec<LoopEdge<f64>>>, Vec<Vec<LoopEdge<Interval>>>), want: &PropsError) {
    let (lf, li) = f();
    for sense in [true, false] {
        let a = run(c, &lf, sense).map(|_| ());
        let b = run(c, &li, sense).map(|_| ());
        eprintln!("{name} sense {sense}: f64 {a:?} iv {b:?}; side f64 {:?}", side(c, &lf));
        assert_eq!(a.as_ref().err(), Some(want), "{name} f64 sense {sense}: {a:?}");
        assert_eq!(b.as_ref().err(), Some(want), "{name} iv sense {sense}: {b:?}");
    }
}

fn measures_both(name: &str, c: &Cone, f: &dyn Fn() -> (Vec<Vec<LoopEdge<f64>>>, Vec<Vec<LoopEdge<Interval>>>), area: f64) {
    let (lf, li) = f();
    let (_, a) = run(c, &lf, true).unwrap_or_else(|e| panic!("{name} f64 refused {e:?}"));
    assert!((a - area).abs() <= 1e-9 * area, "{name} f64 area {a} vs {area}");
    let (_, ai) = run(c, &li, true).unwrap_or_else(|e| panic!("{name} iv refused {e:?}"));
    assert!(ai.lo() - 1e-9 * area <= area && area <= ai.hi() + 1e-9 * area, "{name} iv area [{},{}] vs {area}", ai.lo(), ai.hi());
    eprintln!("{name}: measures {a} (oracle {area})");
}

fn both(c: Cone, verts: Vec<Vec<(f64, f64)>>) -> impl Fn() -> (Vec<Vec<LoopEdge<f64>>>, Vec<Vec<LoopEdge<Interval>>>) {
    move || {
        (
            verts.iter().map(|q| c.chart_loop::<f64>(q)).collect(),
            verts.iter().map(|q| c.chart_loop::<Interval>(q)).collect(),
        )
    }
}
fn rev(v: &[(f64, f64)]) -> Vec<(f64, f64)> {
    v.iter().rev().copied().collect()
}

const WALL: [(f64, f64); 4] = [(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)];

// ---------------------------------------------------------------- item 1

fn off_surface<T: Real>(c: &Cone, which: &str) -> Vec<Vec<LoopEdge<T>>> {
    let good = c.chart_loop::<T>(&WALL);
    let (s, co) = c.alpha.sin_cos();
    let mut l = good.clone();
    let chord = |p: V, q: V, i: u32, j: u32| {
        let d = add(q, p, -1.0);
        let n = dot(d, d).sqrt();
        topo::edge(Curve3::Line { origin: pt(p), dir: vt(unit(d)) }, 0.0, n, i, j)
    };
    match which {
        "chords" => {
            l[0] = chord(c.at(0.0, 1.0), c.at(1.0, 1.0), 0, 1);
            l[2] = chord(c.at(1.0, 2.0), c.at(0.0, 2.0), 2, 3);
        }
        "wide" => {
            l[2] = topo::edge(
                Curve3::Circle { center: pt(add(c.apex, c.k, 2.0 * co)), axis: vt(c.k), radius: T::from_f64(1.1 * 2.0 * s), u_ref: vt(c.e1) },
                1.0, 0.0, 2, 3,
            );
        }
        "miss_apex" => {
            l[1] = topo::edge(Curve3::Line { origin: pt(add(c.apex, c.e2, 0.3)), dir: vt(c.g(1.0)) }, 1.0, 2.0, 1, 2);
        }
        // A rim of the right radius for its height, coaxial in direction,
        // its centre moved off the axis.
        "off_axis_rim" => {
            l[2] = topo::edge(
                Curve3::Circle { center: pt(add(add(c.apex, c.k, 2.0 * co), c.e1, 0.05)), axis: vt(c.k), radius: T::from_f64(2.0 * s), u_ref: vt(c.e1) },
                1.0, 0.0, 2, 3,
            );
        }
        // A rim whose plane is tilted off the axis (centre on it, right radius).
        "tilted_rim" => {
            l[2] = topo::edge(
                Curve3::Circle { center: pt(add(c.apex, c.k, 2.0 * co)), axis: vt(unit(add(c.k, c.e1, 0.1))), radius: T::from_f64(2.0 * s), u_ref: vt(c.e2) },
                1.0, 0.0, 2, 3,
            );
        }
        _ => unreachable!(),
    }
    vec![l]
}

#[test]
fn v1_off_surface_premises_restored() {
    let c = c0();
    for (which, what) in [
        ("chords", "props_meridian_generator"),
        ("wide", "props_rim_fit"),
        ("miss_apex", "props_meridian_apex"),
        ("off_axis_rim", "props_rim_center_on_axis"),
        ("tilted_rim", "props_rim_axis_parallel"),
    ] {
        let f = move || (off_surface::<f64>(&c, which), off_surface::<Interval>(&c, which));
        refuses_both(which, &c, &f, &PropsError::OffSurface { what });
        let s = side(&c, &off_surface::<f64>(&c, which));
        eprintln!("  material-sign lane {which}: {s:?}");
        assert!(s.is_err(), "{which}: material sign answered {s:?}");
    }
}

// ---------------------------------------------------------------- item 2

/// Sector `[0, du] × [0, 1.5]` with its rim split in two.
fn split_sector(du: f64) -> Vec<(f64, f64)> {
    vec![(0.0, 0.0), (du, 0.0), (du, 1.5), (du / 2.0, 1.5), (0.0, 1.5)]
}

#[test]
fn v2_apex_loops_bounded_at_a_turn() {
    let c = c0();
    let sweep = PropsError::ConeLoop { what: "props_cone_apex_loop_sweep" };
    let spiral = vec![(0.0, 0.0), (0.0, 1.0), (4.0, 1.0), (4.0, 1.5), (TAU + 1.0, 1.5), (TAU + 1.0, 0.0)];
    refuses_both("R1 spiral", &c, &both(c, vec![spiral.clone()]), &sweep);
    refuses_both("R1 spiral reversed", &c, &both(c, vec![rev(&spiral)]), &sweep);
    for du in [TAU + 0.1, TAU + 1.0, TAU + 2.0, TAU + 2.9] {
        refuses_both(&format!("split sector {du}"), &c, &both(c, vec![split_sector(du)]), &sweep);
        refuses_both(&format!("split sector {du} rev"), &c, &both(c, vec![rev(&split_sector(du))]), &sweep);
        // mirrored onto the other nappe
        let m: Vec<_> = split_sector(du).iter().map(|&(u, v)| (u, -v)).collect();
        refuses_both(&format!("split sector {du} other nappe"), &c, &both(c, vec![m]), &sweep);
    }
    // One rim of 2π+1: decided by 4680's per-edge span bound first.
    let one = vec![(0.0, 0.0), (TAU + 1.0, 0.0), (TAU + 1.0, 1.0), (0.0, 1.0)];
    let (lf, li) = both(c, vec![one])();
    eprintln!("R1 single-rim 2π+1: f64 {:?} iv {:?}", run(&c, &lf, true).map(|_| ()), run(&c, &li, true).map(|_| ()));
    assert!(run(&c, &lf, true).is_err() && run(&c, &li, true).is_err());
    // The slit cap at exactly 2π (split rim and one rim).
    let cap = c.area(&[((0.0, TAU), (0.0, 1.5))]);
    measures_both("slit cap, split rim", &c, &both(c, vec![split_sector(TAU)]), cap);
    measures_both("slit cap, one rim", &c, &both(c, vec![vec![(0.0, 0.0), (TAU, 0.0), (TAU, 1.5), (0.0, 1.5)]]), cap);
    // Off-apex: a full band (winding once) measures; wound twice refuses.
    let band_ = c.area(&[((0.0, TAU), (1.0, 2.0))]);
    measures_both("seamed band", &c, &both(c, vec![vec![(0.0, 1.0), (PI, 1.0), (TAU, 1.0), (TAU, 2.0), (PI, 2.0), (0.0, 2.0)]]), band_);
}

/// The sweep threshold at the band edge: a split sector `2π + δ` with
/// `δ·arm` at fractions of ε and of K·ε (arm = the rim's radius).
#[test]
fn v2_sweep_threshold_at_the_band_edge() {
    let c = c0();
    let b = band();
    let arm = 1.5 * c.alpha.sin();
    for (tag, x) in [("0.5ε", 0.5 * b.zero()), ("2ε", 2.0 * b.zero()), ("0.5Kε", 0.5 * b.escalate()), ("2Kε", 2.0 * b.escalate()), ("100Kε", 100.0 * b.escalate())] {
        for sgn in [1.0, -1.0] {
            let du = TAU + sgn * x / arm;
            let (lf, li) = both(c, vec![split_sector(du)])();
            let a = run(&c, &lf, true).map(|r| r.1);
            let ib = run(&c, &li, true).map(|r| (r.1.lo(), r.1.hi()));
            eprintln!("sweep 2π{}{tag}/arm: f64 {a:?} iv {ib:?}", if sgn > 0.0 { "+" } else { "-" });
            if sgn > 0.0 && x > b.escalate() {
                assert!(matches!(a, Err(PropsError::ConeLoop { what: "props_cone_apex_loop_sweep" })), "{tag}: {a:?}");
                assert!(matches!(ib, Err(PropsError::ConeLoop { what: "props_cone_apex_loop_sweep" })), "{tag} iv: {ib:?}");
            }
            if sgn < 0.0 && x > b.escalate() {
                assert!(a.is_ok() && ib.is_ok(), "a sector short of a turn by {tag} refused: {a:?} {ib:?}");
            }
            // Never a silent wrong value: a measured answer past a turn is
            // only ever in-band.
            if sgn > 0.0 {
                if let Ok(v) = a {
                    assert!(x <= b.zero() || x <= b.escalate(), "{tag} measured {v}");
                }
            }
        }
    }
}

// ---------------------------------------------------------------- item 3

#[test]
fn v3_repeated_vertices_refuse_typed() {
    let c = c0();
    let vertex = PropsError::ConeLoop { what: "props_cone_loop_vertex" };
    let eight = vec![(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (1.0, 3.0), (2.0, 3.0), (2.0, 2.0), (1.0, 2.0), (0.0, 2.0)];
    refuses_both("figure-eight", &c, &both(c, vec![eight.clone()]), &vertex);
    refuses_both("figure-eight rev", &c, &both(c, vec![rev(&eight)]), &vertex);
    let twice = |v: Vec<(f64, f64)>| {
        let c = c;
        move || {
            let mut f = c.chart_loop::<f64>(&v);
            f.extend(c.chart_loop::<f64>(&v));
            let mut i = c.chart_loop::<Interval>(&v);
            i.extend(c.chart_loop::<Interval>(&v));
            (vec![f], vec![i])
        }
    };
    refuses_both("rect twice", &c, &twice(vec![(0.0, 1.0), (2.0, 1.0), (2.0, 2.0), (0.0, 2.0)]), &vertex);
    refuses_both("sector twice", &c, &twice(vec![(0.0, 0.0), (1.2, 0.0), (1.2, 1.0), (0.0, 1.0)]), &vertex);
    // A figure-eight whose second lobe is wound WITH the first (both CCW
    // in the chart, touching at a vertex): the area would be the sum.
    let eight_same = vec![(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (2.0, 2.0), (2.0, 3.0), (1.0, 3.0), (1.0, 2.0), (0.0, 2.0)];
    refuses_both("two lobes touching at a vertex, same winding", &c, &both(c, vec![eight_same]), &vertex);
}

/// Legal faces still measure: a loop passing near but not through its
/// own vertex (a neck of chart width δ), notched / stepped / ringed
/// walls, apex caps.
#[test]
fn v3_legal_faces_do_not_refuse() {
    let c = c0();
    let s = c.alpha.sin();
    let b = band();
    for (tag, gap) in [("3Kε", 3.0 * b.escalate()), ("1e-6", 1e-6), ("0.1", 0.1)] {
        // vertices (1, 2) and (1 + δ, 2) are `gap` apart in space.
        let d = gap / (2.0 * s);
        let neck = vec![(0.0, 1.0), (1.0 + d, 1.0), (1.0 + d, 2.0), (2.0, 2.0), (2.0, 3.0), (1.0, 3.0), (1.0, 2.0), (0.0, 2.0)];
        let area = c.area(&[((0.0, 1.0 + d), (1.0, 2.0)), ((1.0, 2.0), (2.0, 3.0))]);
        measures_both(&format!("neck {tag}"), &c, &both(c, vec![neck.clone()]), area);
        measures_both(&format!("neck {tag} rev"), &c, &both(c, vec![rev(&neck)]), area);
    }
    // In-band neck: fail-loud (refuse or escalate), never a wrong value.
    for (tag, gap) in [("0.5ε", 0.5 * b.zero()), ("2ε", 2.0 * b.zero())] {
        let d = gap / (2.0 * s);
        let neck = vec![(0.0, 1.0), (1.0 + d, 1.0), (1.0 + d, 2.0), (2.0, 2.0), (2.0, 3.0), (1.0, 3.0), (1.0, 2.0), (0.0, 2.0)];
        let (lf, _) = both(c, vec![neck])();
        eprintln!("neck {tag}: {:?}", run(&c, &lf, true));
    }
    let notched = vec![(0.0, 1.0), (3.0, 1.0), (3.0, 2.0), (2.0, 2.0), (2.0, 1.5), (1.0, 1.5), (1.0, 2.0), (0.0, 2.0)];
    measures_both("notched", &c, &both(c, vec![notched]), c.area(&[((0.0, 3.0), (1.0, 1.5)), ((0.0, 1.0), (1.5, 2.0)), ((2.0, 3.0), (1.5, 2.0))]));
    let ringed = vec![vec![(0.0, 1.0), (3.0, 1.0), (3.0, 2.0), (0.0, 2.0)], rev(&[(1.0, 1.2), (2.0, 1.2), (2.0, 1.8), (1.0, 1.8)])];
    measures_both("ringed", &c, &both(c, ringed), c.area(&[((0.0, 3.0), (1.0, 2.0))]) - c.area(&[((1.0, 2.0), (1.2, 1.8))]));
    let cap = c.area(&[((0.0, TAU), (0.0, 1.5))]);
    let lf = vec![vec![c.rim::<f64>(1.5, 0.0, TAU, 0, 0)]];
    let li = vec![vec![c.rim::<Interval>(1.5, 0.0, TAU, 0, 0)]];
    measures_both("apex cap", &c, &move || (lf.clone(), li.clone()), cap);
    // Apex cap of three rim pieces: vertices at 0, 2, 4 (all distinct).
    let cap3 = move || {
        (
            vec![vec![c.rim::<f64>(1.5, 0.0, 2.0, 0, 1), c.rim::<f64>(1.5, 2.0, 4.0, 1, 2), c.rim::<f64>(1.5, 4.0, TAU, 2, 0)]],
            vec![vec![c.rim::<Interval>(1.5, 0.0, 2.0, 0, 1), c.rim::<Interval>(1.5, 2.0, 4.0, 1, 2), c.rim::<Interval>(1.5, 4.0, TAU, 2, 0)]],
        )
    };
    measures_both("apex cap, three rims", &c, &cap3, cap);
}

/// The disclosed residual: a loop that crosses itself between vertices
/// (no shared vertex) is not decided — what does it answer?
#[test]
fn v3_self_crossing_residual() {
    let c = c0();
    // Bow-tie in the chart made of rims and generators: two rectangles
    // overlapping at a corner region, traced as one loop crossing itself.
    let cross = vec![(0.0, 1.0), (2.0, 1.0), (2.0, 3.0), (1.0, 3.0), (1.0, 1.5), (3.0, 1.5), (3.0, 2.0), (0.0, 2.0)];
    let (lf, _) = both(c, vec![cross])();
    eprintln!("self-crossing loop: {:?}", run(&c, &lf, true));
}

/// Item 7: a near-flat cone (α = π/2 − 1e-6) off the origin — R2's
/// sweep's new refusals. The rims are on the cone to rounding; how far
/// is each rim point from the surface, against what `props_rim_fit`
/// meters?
#[test]
fn v7_near_flat_cone_rim_fit() {
    let alpha = PI / 2.0 - 1e-6;
    for apex in [[0.2, -0.1, -1.0], [31.0, -17.0, 9.0]] {
        for axis in [[0.0, 0.0, 1.0], [0.3, -0.4, 1.0]] {
            let c = Cone::new(apex, axis, alpha);
            let (lf, li) = both(c, vec![WALL.to_vec()])();
            let got = run(&c, &lf, true);
            let goti = run(&c, &li, true).map(|r| (r.1.lo(), r.1.hi()));
            // Geometric distance of the stored rims' points from the cone.
            let (s, co) = alpha.sin_cos();
            let mut worst_dist = 0.0_f64;
            let mut worst_fit = 0.0_f64;
            for e in &lf[0] {
                if let Curve3::Circle { center, radius, .. } = e.carrier {
                    let h = dot(add([center.x, center.y, center.z], c.apex, -1.0), c.k);
                    worst_fit = worst_fit.max((radius - (h / co).abs() * s).abs());
                    for t in [e.t0, 0.5 * (e.t0 + e.t1), e.t1] {
                        let q = e.carrier.eval(t);
                        let w = add([q.x, q.y, q.z], c.apex, -1.0);
                        let hh = dot(w, c.k);
                        let rho = (dot(w, w) - hh * hh).max(0.0).sqrt();
                        worst_dist = worst_dist.max((rho * co - hh.abs() * s).abs());
                    }
                }
            }
            eprintln!(
                "near-flat apex {apex:?} axis {axis:?}: f64 {:?} iv {:?}; rim_fit residual {worst_fit:e}, rim points' distance to the cone {worst_dist:e}",
                got.map(|r| r.1),
                goti
            );
        }
    }
}
