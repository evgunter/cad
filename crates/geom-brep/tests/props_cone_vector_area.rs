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

/// **A loop that does not close refuses as the periodic charts' do**
/// ([`PropsError::ChartLoopOpen`]): the notched wall with its last
/// generator dropped bounds no region, and its vector area is not one.
#[test]
fn an_open_loop_refuses_by_name() {
    let mut edges = chart_loop(&notched().loops[0]);
    edges.pop();
    let got = measure(&[edges], true);
    assert_eq!(got.err(), Some(PropsError::ChartLoopOpen), "an open loop");
}

/// **A loop that winds the axis twice refuses as the periodic charts'
/// do** ([`PropsError::LoopsWindChart`]): a full rim stated twice closes
/// at every joint, and its vector area counts the apex cap twice.
#[test]
fn a_loop_winding_the_axis_twice_refuses_by_name() {
    let twice = vec![rim(1.5, 0.0, TAU, 0, 0), rim(1.5, 0.0, TAU, 0, 0)];
    let got = measure(&[twice], true);
    assert_eq!(
        got.err(),
        Some(PropsError::LoopsWindChart),
        "a loop winding twice"
    );
}

/// **`loops` refuses with `want` under both senses.**
fn refuses(name: &str, loops: &[Vec<LoopEdge<f64>>], want: &PropsError) {
    for sense in [true, false] {
        let got = measure(loops, sense);
        assert_eq!(
            got.as_ref().err(),
            Some(want),
            "{name} (sense {sense}): must refuse {want:?}; got {got:?}"
        );
    }
}

/// The chart rectangle `[0, 1] × [1, 2]`, chart-CCW: its low rim, the
/// generator at `u = 1`, its high rim, the generator at `u = 0`.
fn unit_wall() -> Vec<LoopEdge<f64>> {
    chart_loop(&[(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)])
}

/// The straight segment from `p` to `q`.
fn chord(p: [f64; 3], q: [f64; 3], a: u32, b: u32) -> LoopEdge<f64> {
    let d = [q[0] - p[0], q[1] - p[1], q[2] - p[2]];
    let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    topo::edge(
        Curve3::Line {
            origin: p3(p[0], p[1], p[2]),
            dir: v3(d[0] / l, d[1] / l, d[2] / l),
        },
        0.0,
        l,
        a,
        b,
    )
}

/// **An edge off the cone refuses as incidence, by the name the iso
/// parse decides it under.** The form reads only the boundary, so a
/// boundary that is not on the cone answers for a region that is not
/// either: the wall with its rims replaced by straight chords closes
/// and would measure the chords' vector area. A rim of the wrong radius
/// and a line at the generator angle that misses the apex are refused
/// as what they are, not as the gap they leave at a joint.
#[test]
fn an_edge_off_the_cone_refuses_as_incidence() {
    let good = unit_wall();
    let mut chords = good.clone();
    chords[0] = chord(chart(0.0, 1.0), chart(1.0, 1.0), 0, 1);
    chords[2] = chord(chart(1.0, 2.0), chart(0.0, 2.0), 2, 3);
    refuses(
        "rims replaced by chords",
        &[chords],
        &PropsError::OffSurface {
            what: "props_meridian_on_surface",
        },
    );
    let (s, c) = ALPHA.sin_cos();
    let mut wide = good.clone();
    wide[2] = topo::edge(
        Curve3::Circle {
            center: p3(APEX.0, APEX.1, APEX.2 + 2.0 * c),
            axis: v3(0.0, 0.0, 1.0),
            radius: 1.1 * 2.0 * s,
            u_ref: v3(1.0, 0.0, 0.0),
        },
        1.0,
        0.0,
        2,
        3,
    );
    refuses(
        "a rim 10% too wide",
        &[wide],
        &PropsError::OffSurface {
            what: "props_rim_fit",
        },
    );
    let mut missing = good;
    missing[1] = topo::edge(
        Curve3::Line {
            origin: p3(APEX.0, APEX.1 + 0.3, APEX.2),
            dir: v3(s * 1.0_f64.cos(), s * 1.0_f64.sin(), c),
        },
        1.0,
        2.0,
        1,
        2,
    );
    refuses(
        "a generator that misses the apex",
        &[missing],
        &PropsError::OffSurface {
            what: "props_meridian_on_surface",
        },
    );
}

/// The sector `[0, du] × [0, 1.5]` through the apex.
fn sector(du: f64) -> Face {
    Face {
        loops: vec![vec![(0.0, 0.0), (du, 0.0), (du, 1.5), (0.0, 1.5)]],
        rects: vec![((0.0, du), (0.0, 1.5), 1.0)],
    }
}

/// **The slit cap — a loop through the apex that goes round to its own
/// generator — sweeps exactly a turn and measures**: the cap
/// `[0, 2π] × [0, 1.5]`, its seam generator traversed out and back.
#[test]
fn the_slit_cap_sweeping_exactly_a_turn_measures_exactly() {
    measures_exactly("slit cap", &sector(TAU));
}

/// **A loop through the apex that sweeps past a turn refuses**, named
/// `props_cone_apex_loop_sweep`: past a turn it covers part of the
/// nappe twice, and the vector area counts that part twice. A sector of
/// `2π + 0.1` pins the bound a hair past the slit cap, below every
/// off-apex bound; the spiral steps outward as it goes round. Each
/// sector's rim is stated as two arcs, each within the turn a single
/// rim's stored span may take (`props_rim_span_winding`), so the sweep
/// is what refuses it.
#[test]
fn a_loop_through_the_apex_sweeping_past_a_turn_refuses_by_name() {
    let want = PropsError::ConeLoop {
        what: "props_cone_apex_loop_sweep",
    };
    for du in [TAU + 0.1, TAU + 2.0, TAU + 2.9] {
        let verts = vec![
            (0.0, 0.0),
            (du, 0.0),
            (du, 1.5),
            (du / 2.0, 1.5),
            (0.0, 1.5),
        ];
        for verts in [verts.clone(), reversed(&verts)] {
            refuses(&format!("sector of {du}"), &[chart_loop(&verts)], &want);
        }
    }
    let spiral = [
        (0.0, 0.0),
        (0.0, 1.0),
        (4.0, 1.0),
        (4.0, 1.5),
        (TAU + 1.0, 1.5),
        (TAU + 1.0, 0.0),
    ];
    refuses("the spiral", &[chart_loop(&spiral)], &want);
}

/// **A loop that meets itself at a vertex refuses**, named
/// `props_cone_loop_vertex`: the pinched figure-eight — lobes
/// `[0, 1] × [1, 2]` and `[1, 2] × [2, 3]` meeting at `(1, 2)`, the
/// second wound against the first — measures the difference of its
/// lobes, and a loop stated twice measures double, off the apex and
/// through it.
#[test]
fn a_loop_meeting_itself_at_a_vertex_refuses_by_name() {
    let want = PropsError::ConeLoop {
        what: "props_cone_loop_vertex",
    };
    let eight = [
        (0.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (1.0, 3.0),
        (2.0, 3.0),
        (2.0, 2.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ];
    refuses("the figure-eight", &[chart_loop(&eight)], &want);
    for (name, verts) in [
        (
            "a wall stated twice",
            vec![(0.0, 1.0), (2.0, 1.0), (2.0, 2.0), (0.0, 2.0)],
        ),
        ("a sector stated twice", sector(1.2).loops[0].clone()),
    ] {
        let mut twice = chart_loop(&verts);
        twice.extend(chart_loop(&verts));
        refuses(name, &[twice], &want);
    }
}

/// **A seam's two copies of one vertex are a period apart, not a
/// pinch**: the full wall `[0, 2π] × [1, 2]` with its low rim split at
/// `u = 2` passes its seam vertex `(0, 1) ≡ (2π, 1)` twice,
/// non-adjacently, and measures — `props_cone_loop_vertex_sheet` reads
/// the turn the loop makes between the two.
#[test]
fn a_seamed_wall_with_a_split_rim_measures_exactly() {
    measures_exactly(
        "seamed wall, low rim split",
        &Face {
            loops: vec![vec![
                (0.0, 1.0),
                (2.0, 1.0),
                (TAU, 1.0),
                (TAU, 2.0),
                (0.0, 2.0),
            ]],
            rects: vec![((0.0, TAU), (1.0, 2.0), 1.0)],
        },
    );
}

/// **Holes that cover more than the outer loop refuse as such**, not
/// as a ring wound with the face: two holes each wound against the wall
/// `[0, 3] × [1, 2]` but overlapping, their areas summing past it, turn
/// the whole's sign. One ring wound WITH the wall is the other refusal.
#[test]
fn holes_covering_more_than_the_face_refuse_by_name() {
    let outer = [(0.0, 1.0), (3.0, 1.0), (3.0, 2.0), (0.0, 2.0)];
    let h1 = [(0.1, 1.05), (2.0, 1.05), (2.0, 1.95), (0.1, 1.95)];
    let h2 = [(1.0, 1.05), (2.9, 1.05), (2.9, 1.95), (1.0, 1.95)];
    let o = chart_loop(&outer);
    refuses(
        "overlapping holes summing past the face",
        &[
            o.clone(),
            chart_loop(&reversed(&h1)),
            chart_loop(&reversed(&h2)),
        ],
        &PropsError::HolesExceedFace,
    );
    refuses(
        "a ring wound with the face",
        &[o, chart_loop(&h1)],
        &PropsError::RingWoundWithFace,
    );
}

/// **A wall one and a half escalation bands wide measures**: its low
/// rim is an arc `1.5·K·ε` long, short enough that a turn decided on a
/// third of the arc's length would escalate, and its mean width still
/// clears the band. A rim turns through its stored span, so no decide
/// reads the arc's length.
///
/// The area is held to what the rims' chords carry: each is a difference
/// of points of magnitude up to 2, placed to half an ulp there, and the
/// arc's vector area reads it at a lever of the wall's own size, about
/// a metre, so the reading stands within `4·f64::EPSILON` m²: 4·10⁻⁵ of
/// this wall, and as a mean displacement over its metre of length four
/// orders inside the tightest band the matrix runs (`K·ε` = 1e-11 m).
#[test]
fn a_wall_one_and_a_half_bands_wide_measures() {
    let du = 1.5 * band().escalate() / ALPHA.sin();
    let face = Face {
        loops: vec![vec![(0.0, 1.0), (du, 1.0), (du, 2.0), (0.0, 2.0)]],
        rects: vec![((0.0, du), (1.0, 2.0), 1.0)],
    };
    let (_, area) = oracle(&face.rects);
    for verts in [face.loops[0].clone(), reversed(&face.loops[0])] {
        let (_, got) = measure(&[chart_loop(&verts)], true)
            .unwrap_or_else(|e| panic!("a wall {du:e} rad wide: refused {e:?}"));
        assert!(
            (got - area).abs() <= 4.0 * f64::EPSILON,
            "a wall {du:e} rad wide: area {got:e} against {area:e}"
        );
    }
}

/// The wall `[0, 1] × [v0, v1]` (chart-CCW) on the cone of half-angle
/// `alpha` with apex `a` and axis along `k`, at scalar `T`.
fn wall_on<T: geom_core::Real>(
    alpha: f64,
    a: [f64; 3],
    k: [f64; 3],
    v0: f64,
    v1: f64,
) -> (Surface<T>, Vec<LoopEdge<T>>) {
    let unit = |x: [f64; 3]| {
        let n = (x[0] * x[0] + x[1] * x[1] + x[2] * x[2]).sqrt();
        [x[0] / n, x[1] / n, x[2] / n]
    };
    let k = unit(k);
    let t = if k[0].abs() < 0.9 {
        [1.0, 0.0, 0.0]
    } else {
        [0.0, 1.0, 0.0]
    };
    let tk = t[0] * k[0] + t[1] * k[1] + t[2] * k[2];
    let e1 = unit([t[0] - tk * k[0], t[1] - tk * k[1], t[2] - tk * k[2]]);
    let e2 = [
        k[1] * e1[2] - k[2] * e1[1],
        k[2] * e1[0] - k[0] * e1[2],
        k[0] * e1[1] - k[1] * e1[0],
    ];
    let (s, c) = alpha.sin_cos();
    let at = |x: [f64; 3], m: f64, d: [f64; 3]| [x[0] + m * d[0], x[1] + m * d[1], x[2] + m * d[2]];
    let rim = |v: f64, u0: f64, u1: f64, i: u32, j: u32| {
        let o = at(a, v * c, k);
        topo::edge(
            Curve3::Circle {
                center: p3(o[0], o[1], o[2]),
                axis: v3(k[0], k[1], k[2]),
                radius: T::from_f64(v.abs() * s),
                u_ref: v3(e1[0], e1[1], e1[2]),
            },
            u0,
            u1,
            i,
            j,
        )
    };
    let generator = |u: f64, t0: f64, t1: f64, i: u32, j: u32| {
        let (su, cu) = u.sin_cos();
        let d = at(at([0.0; 3], s * cu, e1), s * su, e2);
        let d = at(d, c, k);
        topo::edge(
            Curve3::Line {
                origin: p3(a[0], a[1], a[2]),
                dir: v3(d[0], d[1], d[2]),
            },
            t0,
            t1,
            i,
            j,
        )
    };
    let surface = Surface::Cone {
        apex: p3(a[0], a[1], a[2]),
        axis: v3(k[0], k[1], k[2]),
        half_angle: T::from_f64(alpha),
        u_ref: v3(e1[0], e1[1], e1[2]),
    };
    let edges = vec![
        rim(v0, 0.0, 1.0, 0, 1),
        generator(1.0, v0, v1, 1, 2),
        rim(v1, 1.0, 0.0, 2, 3),
        generator(0.0, v1, v0, 3, 0),
    ];
    (surface, edges)
}

/// **A wall on a near-flat or near-needle cone measures**, at `f64` and
/// at the interval scalar. The incidence decides meter a point's
/// distance from the cone: a rim's is `r·cos α − |h|·sin α` in its
/// meridian plane, and a line's angle off the generator is
/// `|dir × axis|·cos α − |dir·axis|·sin α`. The slant-length spelling
/// `r − |v|·sin α`, with `v = h/cos α`, overstated a rim's deviation
/// by `1/cos α`. At `α = π/2 − 10⁻⁶` that read a rim stored to the
/// last bit as 10⁻⁹ m off the cone and escalated every such face; the
/// cosine-difference spelling of the line's angle understates it by
/// `sin α` as `α → 0`.
#[test]
fn a_wall_on_a_near_flat_or_needle_cone_measures() {
    use geom_core::{Bounds, Interval};
    for (alpha, a, k, ax) in [
        core::f64::consts::FRAC_PI_2 - 1e-6,
        core::f64::consts::FRAC_PI_2 - 1e-3,
        1e-3,
        1e-6,
    ]
    .into_iter()
    .flat_map(|alpha| {
        [[0.2, -0.1, -1.0], [31.0, -17.0, 9.0]]
            .into_iter()
            .flat_map(move |a| [1e-3, 1.0, 1e3].map(move |k| (alpha, a, k)))
    })
    .flat_map(|(alpha, a, k)| {
        [[0.0, 0.0, 1.0], [0.3, -0.4, 1.0], [1.0, 1.0, 0.1]].map(move |ax| (alpha, a, k, ax))
    }) {
        // A rim narrower than a thousand escalation bands sits within
        // a few of the axis everywhere: that sliver refuses as it
        // always has, and is not this row's subject.
        if k * alpha.sin() < 1e3 * band().escalate() {
            continue;
        }
        let (v0, v1) = (k, 2.0 * k);
        let area = alpha.sin() * (v1 * v1 - v0 * v0) / 2.0;
        let (surface, edges) = wall_on::<f64>(alpha, a, ax, v0, v1);
        let got = curved_face_loops(&surface, &[&edges], true, band()).unwrap_or_else(|e| {
            panic!("alpha {alpha} apex {a:?} axis {ax:?} scale {k}: f64 refused {e:?}")
        });
        // `axis·A⃗` rounds at the ulps of the points' reach times `k`,
        // and the area divides it by `sin α`.
        let reach = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt() + 2.0 * k;
        let tol = 1e-9 * area + 256.0 * f64::EPSILON * k * reach / alpha.sin();
        assert!(
            (got.area - area).abs() <= tol,
            "alpha {alpha} apex {a:?} axis {ax:?} scale {k}: area {:e} against {area:e}",
            got.area
        );
        // The interval reading of coordinates `|a| + 2k` across is no
        // finer than their f64 rounding: past the run's zero, every
        // incidence decide escalates on input rounding alone.
        if 64.0 * f64::EPSILON * reach > band().zero() {
            continue;
        }
        let (surface, edges) = wall_on::<Interval>(alpha, a, ax, v0, v1);
        let got = curved_face_loops(&surface, &[&edges], true, band()).unwrap_or_else(|e| {
            panic!("alpha {alpha} apex {a:?} axis {ax:?} scale {k}: interval refused {e:?}")
        });
        assert!(
            got.area.lo() <= area * (1.0 + 1e-12) && area * (1.0 - 1e-12) <= got.area.hi(),
            "alpha {alpha}: interval area {:?} misses {area:e}",
            got.area
        );
    }
}

/// **The material sign refuses what the flux lane refuses**: the gate
/// reads the same loops by the same decides ([`cone_face_read`]), so it
/// answers no side for a loop the flux lane will not measure — past a
/// turn through the apex, meeting itself at a vertex, or stated twice.
#[test]
fn the_material_sign_refuses_what_the_flux_lane_refuses() {
    let eight = [
        (0.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (1.0, 3.0),
        (2.0, 3.0),
        (2.0, 2.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ];
    let du = TAU + 0.1;
    let mut twice = unit_wall();
    twice.extend(unit_wall());
    for (name, loops) in [
        (
            "a sector past a turn",
            vec![chart_loop(&[
                (0.0, 0.0),
                (du, 0.0),
                (du, 1.5),
                (du / 2.0, 1.5),
                (0.0, 1.5),
            ])],
        ),
        ("the figure-eight", vec![chart_loop(&eight)]),
        ("a wall stated twice", vec![twice]),
        (
            "a rim stated twice",
            vec![vec![rim(1.5, 0.0, TAU, 0, 0), rim(1.5, 0.0, TAU, 0, 0)]],
        ),
    ] {
        let flux = measure(&loops, true).err();
        let gate = side(&loops).err();
        assert!(
            matches!(
                flux,
                Some(PropsError::ConeLoop { .. } | PropsError::LoopsWindChart)
            ),
            "{name}: the flux lane refuses it: {flux:?}"
        );
        assert_eq!(gate, flux, "{name}: the gate refuses it alike");
    }
}

/// **A rim off the axis, or tilted against it, is off the cone**, by the
/// circle incidence's own names: the rim at slant 2 of [`unit_wall`]
/// moved 0.05 m off the axis (`props_cone_circle_off_axis`), and tipped
/// 0.05 rad off it (`props_cone_circle_tilt`), each at the cone's radius
/// for its height.
#[test]
fn a_rim_off_or_tilted_against_the_axis_refuses_as_incidence() {
    let (s, c) = ALPHA.sin_cos();
    let rim2 = |center: [f64; 3], axis: [f64; 3]| {
        topo::edge(
            Curve3::Circle {
                center: p3(center[0], center[1], center[2]),
                axis: v3(axis[0], axis[1], axis[2]),
                radius: 2.0 * s,
                u_ref: v3(1.0, 0.0, 0.0),
            },
            1.0,
            0.0,
            2,
            3,
        )
    };
    let h = APEX.2 + 2.0 * c;
    let mut off = unit_wall();
    off[2] = rim2([APEX.0 + 0.05, APEX.1, h], [0.0, 0.0, 1.0]);
    refuses(
        "a rim off the axis",
        &[off],
        &PropsError::OffSurface {
            what: "props_cone_circle_off_axis",
        },
    );
    let mut tilted = unit_wall();
    tilted[2] = rim2([APEX.0, APEX.1, h], [0.05_f64.sin(), 0.0, 0.05_f64.cos()]);
    refuses(
        "a rim tilted against the axis",
        &[tilted],
        &PropsError::OffSurface {
            what: "props_cone_circle_tilt",
        },
    );
}
