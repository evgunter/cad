//! **A cone face is measured by its boundary's vector area, whatever
//! the boundary's shape** (`cone_face_closed_form`).
//!
//! The flux `apex·A⃗` and the area `|axis·A⃗|/sin α` read nothing but
//! `A⃗`, so they are exact for any region on one nappe: a notched wall,
//! an L-shaped wall, a wall with a hole, and a sector whose closure holds
//! the apex. These rows measure such faces against Gauss–Legendre over
//! the chart, `∫∫ p·(p_u × p_v)` and `∫∫ |p_u × p_v|`, which never forms
//! `A⃗`; and they pin what the form does need, each refused by name: one
//! nappe (`props_cone_nappe`), loops that close (`props_loop_closed`)
//! and bound their region once (`props_cone_loop_winding`), and a side
//! (`props_cone_area_side`).
//!
//! The cone has an off-origin apex and a half-angle that is not 45°, so
//! a dropped or mis-signed component of `A⃗` moves the flux. This file
//! is on CI's `eps ∈ {default, 1e-6, 1e-12}` matrix.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::point::{p3, v3};
use crate::shared::tol::band;
use crate::shared::topo;
use geom::{Curve3, Surface};
use geom_brep::props::{
    LoopEdge, MaterialSign, PropsError, boundary_material_sign_loops, curved_face_loops,
};
use geom_core::Sign;

const ALPHA: f64 = 0.5;
const APEX: (f64, f64, f64) = (0.2, -0.1, -1.0);
const PI: f64 = core::f64::consts::PI;
const TAU: f64 = core::f64::consts::TAU;

fn cone() -> Surface<f64> {
    Surface::Cone {
        apex: p3(APEX.0, APEX.1, APEX.2),
        axis: v3(0.0, 0.0, 1.0),
        half_angle: ALPHA,
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// The chart `p(u, v) = apex + v·(sin α cos u, sin α sin u, cos α)`, `v`
/// the signed slant, on both nappes.
fn chart(u: f64, v: f64) -> [f64; 3] {
    let (s, c) = ALPHA.sin_cos();
    [
        APEX.0 + v * s * u.cos(),
        APEX.1 + v * s * u.sin(),
        APEX.2 + v * c,
    ]
}

/// The rim at slant `v`, traversed in azimuth from `u0` to `u1`; its
/// circle parameter is the chart's `u` on either nappe.
fn rim(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    let (s, c) = ALPHA.sin_cos();
    topo::edge(
        Curve3::Circle {
            center: p3(APEX.0, APEX.1, APEX.2 + v * c),
            axis: v3(0.0, 0.0, 1.0),
            radius: v.abs() * s,
            u_ref: v3(v.signum(), 0.0, 0.0),
        },
        u0,
        u1,
        a,
        b,
    )
}

/// The generator at azimuth `u`, from slant `t0` to `t1`.
fn generator(u: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    let (s, c) = ALPHA.sin_cos();
    topo::edge(
        Curve3::Line {
            origin: p3(APEX.0, APEX.1, APEX.2),
            dir: v3(s * u.cos(), s * u.sin(), c),
        },
        t0,
        t1,
        a,
        b,
    )
}

/// The loop through the chart polygon `verts` (rectilinear in `(u, v)`):
/// a step in `u` is a rim, a step in `v` a generator. A step along
/// `v = 0` is the apex, a point, and carries no edge.
fn chart_loop(verts: &[(f64, f64)]) -> Vec<LoopEdge<f64>> {
    let n = verts.len();
    // Every chart vertex on `v = 0` is the one apex vertex.
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
            rim(va, ua, ub, id(i), id(i + 1))
        } else {
            assert_eq!(ua, ub, "a chart polygon step is in u or in v");
            generator(ua, va, vb, id(i), id(i + 1))
        });
    }
    edges
}

/// The polygon traversed the other way.
fn reversed(verts: &[(f64, f64)]) -> Vec<(f64, f64)> {
    verts.iter().rev().copied().collect()
}

/// The polygon reflected onto the other nappe, `v ↦ −v`.
fn mirrored(verts: &[(f64, f64)]) -> Vec<(f64, f64)> {
    verts.iter().map(|&(u, v)| (u, -v)).collect()
}

/// The polygon's signed chart area: positive when `verts` run
/// counter-clockwise in `(u, v)`.
fn chart_orientation(verts: &[(f64, f64)]) -> f64 {
    let n = verts.len();
    let twice: f64 = (0..n)
        .map(|i| {
            let ((ua, va), (ub, vb)) = (verts[i], verts[(i + 1) % n]);
            ua * vb - ub * va
        })
        .sum();
    twice.signum()
}

/// Gauss–Legendre nodes and weights on `[−1, 1]`, by Newton on `P_n`.
fn gauss_legendre(n: usize) -> Vec<(f64, f64)> {
    (0..n)
        .map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let (nf, k) = (n as f64, i as f64);
            let mut x = (PI * (k + 0.75) / (nf + 0.5)).cos();
            loop {
                let (mut p0, mut p1) = (1.0, x);
                for j in 2..=n {
                    #[allow(clippy::cast_precision_loss)]
                    let jf = j as f64;
                    (p0, p1) = (p1, ((2.0 * jf - 1.0) * x * p1 - (jf - 1.0) * p0) / jf);
                }
                let dp = nf * (x * p1 - p0) / (x * x - 1.0);
                let dx = p1 / dp;
                x -= dx;
                if dx.abs() < 1e-16 {
                    let (mut q0, mut q1) = (1.0, x);
                    for j in 2..=n {
                        #[allow(clippy::cast_precision_loss)]
                        let jf = j as f64;
                        (q0, q1) = (q1, ((2.0 * jf - 1.0) * x * q1 - (jf - 1.0) * q0) / jf);
                    }
                    let dq = nf * (x * q1 - q0) / (x * x - 1.0);
                    return (x, 2.0 / ((1.0 - x * x) * dq * dq));
                }
            }
        })
        .collect()
}

/// A chart rectangle `([u0, u1], [v0, v1], weight)`.
type Rect = ((f64, f64), (f64, f64), f64);

/// The oracle over the chart rectangles `[u0, u1] × [v0, v1]`, each
/// weighted `±1`: `(∫∫ p·(p_u × p_v), ∫∫ |p_u × p_v|)` by 24-point
/// Gauss–Legendre in each direction, with `p_u × p_v` the chart's own
/// derivatives, differenced from [`chart`] in closed form.
fn oracle(rects: &[Rect]) -> (f64, f64) {
    let gl = gauss_legendre(24);
    let (s, c) = ALPHA.sin_cos();
    let (mut flux, mut area) = (0.0, 0.0);
    for &((u0, u1), (v0, v1), w) in rects {
        let (hu, hv) = ((u1 - u0) / 2.0, (v1 - v0) / 2.0);
        for &(xu, wu) in &gl {
            for &(xv, wv) in &gl {
                let (u, v) = (u0 + hu * (xu + 1.0), v0 + hv * (xv + 1.0));
                let pu = [-v * s * u.sin(), v * s * u.cos(), 0.0];
                let pv = [s * u.cos(), s * u.sin(), c];
                let n = [
                    pu[1] * pv[2] - pu[2] * pv[1],
                    pu[2] * pv[0] - pu[0] * pv[2],
                    pu[0] * pv[1] - pu[1] * pv[0],
                ];
                let p = chart(u, v);
                let jw = w * wu * wv * hu * hv;
                flux += jw * (p[0] * n[0] + p[1] * n[1] + p[2] * n[2]);
                area += jw * (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            }
        }
    }
    (flux, area)
}

fn measure(loops: &[Vec<LoopEdge<f64>>], sense: bool) -> Result<(f64, f64), PropsError> {
    let loops: Vec<&[LoopEdge<f64>]> = loops.iter().map(Vec::as_slice).collect();
    curved_face_loops(&cone(), &loops, sense, band()).map(|fc| (fc.flux, fc.area))
}

fn side(loops: &[Vec<LoopEdge<f64>>]) -> Result<MaterialSign, PropsError> {
    let loops: Vec<&[LoopEdge<f64>]> = loops.iter().map(Vec::as_slice).collect();
    boundary_material_sign_loops(&cone(), &loops, band())
}

/// A face as chart polygons (outer first, then holes wound against it)
/// and the chart rectangles that tile its region.
#[derive(Clone)]
struct Face {
    loops: Vec<Vec<(f64, f64)>>,
    rects: Vec<Rect>,
}

impl Face {
    /// The same face on the other nappe, its region mirrored with it.
    fn mirrored(&self) -> Face {
        Face {
            loops: self.loops.iter().map(|l| mirrored(l)).collect(),
            rects: self
                .rects
                .iter()
                .map(|&((u0, u1), (v0, v1), w)| ((u0, u1), (-v1, -v0), w))
                .collect(),
        }
    }
}

/// **`name` measures its oracle on both nappes, both traversals and
/// both senses**, its flux signed by the traversal and its area not;
/// and the gate reads the side the traversal encodes. The flux and
/// area agree with the oracle to `1e-12` relative to the area's scale.
fn measures_exactly(name: &str, face: &Face) {
    for (nappe, face) in [("v > 0", face.clone()), ("v < 0", face.mirrored())] {
        let (flux_ccw, area) = oracle(&face.rects);
        let scale = area.max(1.0);
        for forward in [true, false] {
            let loops: Vec<Vec<LoopEdge<f64>>> = face
                .loops
                .iter()
                .map(|l| chart_loop(&if forward { l.clone() } else { reversed(l) }))
                .collect();
            let orient = chart_orientation(&if forward {
                face.loops[0].clone()
            } else {
                reversed(&face.loops[0])
            });
            let want_flux = orient * flux_ccw;
            for sense in [true, false] {
                let (flux, got) = measure(&loops, sense).unwrap_or_else(|e| {
                    panic!("{name} ({nappe}, forward {forward}, sense {sense}): refused {e:?}")
                });
                assert!(
                    (got - area).abs() <= 1e-12 * scale,
                    "{name} ({nappe}, forward {forward}, sense {sense}): area {got:.15e} \
                     against the oracle's {area:.15e}"
                );
                assert!(
                    (flux - want_flux).abs() <= 1e-12 * scale,
                    "{name} ({nappe}, forward {forward}, sense {sense}): flux {flux:.15e} \
                     against the oracle's {want_flux:.15e}"
                );
            }
            let want_side = if orient > 0.0 {
                Sign::Positive
            } else {
                Sign::Negative
            };
            assert_eq!(
                side(&loops).ok(),
                Some(MaterialSign::Encoded(want_side)),
                "{name} ({nappe}, forward {forward}): the side the traversal encodes"
            );
        }
    }
}

/// `[0, π] × [1, 2]` with the notch `[1, 2] × [1.6, 2]` cut from its
/// top rim: two rims at `v = 2`, one at `v = 1.6` interior to the
/// extent.
fn notched() -> Face {
    Face {
        loops: vec![vec![
            (0.0, 1.0),
            (PI, 1.0),
            (PI, 2.0),
            (2.0, 2.0),
            (2.0, 1.6),
            (1.0, 1.6),
            (1.0, 2.0),
            (0.0, 2.0),
        ]],
        rects: vec![
            ((0.0, PI), (1.0, 1.6), 1.0),
            ((0.0, 1.0), (1.6, 2.0), 1.0),
            ((2.0, PI), (1.6, 2.0), 1.0),
        ],
    }
}

/// `[0, 2] × [1, 1.5]` stepped up to `[0, 1] × [1.5, 2]`.
fn l_shaped() -> Face {
    Face {
        loops: vec![vec![
            (0.0, 1.0),
            (2.0, 1.0),
            (2.0, 1.5),
            (1.0, 1.5),
            (1.0, 2.0),
            (0.0, 2.0),
        ]],
        rects: vec![((0.0, 2.0), (1.0, 1.5), 1.0), ((0.0, 1.0), (1.5, 2.0), 1.0)],
    }
}

/// [`notched`] with the window `[2.3, 2.8] × [1.1, 1.4]` cut from it,
/// wound against it.
fn notched_with_a_hole() -> Face {
    let mut face = notched();
    face.loops
        .push(reversed(&[(2.3, 1.1), (2.8, 1.1), (2.8, 1.4), (2.3, 1.4)]));
    face.rects.push(((2.3, 2.8), (1.1, 1.4), -1.0));
    face
}

/// The sector `[0, 2] × [0, 1.5]`: two generators meeting at the apex
/// and the rim between them. Its closure holds the apex, which the form
/// does not mind — `n·axis` is constant on the nappe and `(p − apex)·n`
/// vanishes on it, the apex included.
fn apex_sector() -> Face {
    Face {
        loops: vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.5), (0.0, 1.5)]],
        rects: vec![((0.0, 2.0), (0.0, 1.5), 1.0)],
    }
}

/// **A notched cone wall measures exactly.** Red on the iso premise,
/// which refused the rim at `v = 1.6` as not at an extreme.
#[test]
fn a_notched_cone_wall_measures_exactly() {
    measures_exactly("notched wall", &notched());
}

/// **An L-shaped cone wall measures exactly.** Red on the iso premise:
/// the rim at `v = 1.5` is at neither extreme.
#[test]
fn an_l_shaped_cone_wall_measures_exactly() {
    measures_exactly("L-shaped wall", &l_shaped());
}

/// **A ringed notched wall measures its outer loop less its ring.**
/// Red against the mutant "ring dropped", which answers the notched
/// wall's whole area.
#[test]
fn a_notched_cone_wall_with_a_hole_measures_exactly() {
    measures_exactly("notched wall with a hole", &notched_with_a_hole());
}

/// **A sector reaching the apex measures exactly.** The form's one-nappe
/// premise is a boundary that does not CROSS the apex; one that touches
/// it bounds a region on one nappe, and the vector area is that
/// region's.
#[test]
fn a_sector_reaching_the_apex_measures_exactly() {
    measures_exactly("apex sector", &apex_sector());
}

/// The `(predicate, sign)` verdicts one `curved_face_loops` call records.
fn verdicts(loops: &[Vec<LoopEdge<f64>>]) -> Vec<(String, Sign)> {
    let bracket = geom_core::k_stats::Bracket::open();
    let _ = measure(loops, true);
    bracket
        .finish()
        .verdicts
        .iter()
        .map(|v| (v.predicate.to_string(), v.sign))
        .collect()
}

/// **A face crossing the apex onto the other nappe refuses**, named
/// `props_cone_nappe`: `n·axis` changes sign at the apex, so
/// `|axis·A⃗|/sin α` is no area there. The quad `[0, 1] × [−1, 2]` runs
/// its generators through the apex.
#[test]
fn a_face_crossing_the_apex_refuses_by_name() {
    let quad = [(0.0, -1.0), (1.0, -1.0), (1.0, 2.0), (0.0, 2.0)];
    for loops in [vec![chart_loop(&quad)], vec![chart_loop(&reversed(&quad))]] {
        let got = measure(&loops, true);
        assert!(
            matches!(got, Err(PropsError::NappeSpanning)),
            "a face on both nappes must refuse NappeSpanning; got {got:?}"
        );
        let v = verdicts(&loops);
        assert!(
            v.contains(&("props_cone_nappe".to_string(), Sign::Negative))
                && v.contains(&("props_cone_nappe".to_string(), Sign::Positive)),
            "the refusal is props_cone_nappe's, both ends decided: {v:?}"
        );
    }
}

/// **A loop that does not close refuses, named `props_loop_closed`**:
/// the notched wall with its last generator dropped bounds no region, and
/// its vector area is not one.
#[test]
fn an_open_loop_refuses_by_name() {
    let mut edges = chart_loop(&notched().loops[0]);
    edges.pop();
    let got = measure(&[edges], true);
    assert!(
        matches!(
            got,
            Err(PropsError::NotIsoRectangle {
                what: "props_loop_closed"
            })
        ),
        "an open loop must refuse at props_loop_closed; got {got:?}"
    );
}

/// **A loop that winds the axis twice refuses, named
/// `props_cone_loop_winding`**: a full rim stated twice closes at every
/// joint, and its vector area counts the apex cap twice.
#[test]
fn a_loop_winding_the_axis_twice_refuses_by_name() {
    let twice = vec![rim(1.5, 0.0, TAU, 0, 0), rim(1.5, 0.0, TAU, 0, 0)];
    let got = measure(&[twice], true);
    assert!(
        matches!(
            got,
            Err(PropsError::NotIsoRectangle {
                what: "props_cone_loop_winding"
            })
        ),
        "a loop winding twice must refuse at props_cone_loop_winding; got {got:?}"
    );
}
