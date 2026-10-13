//! **Every lane on a periodic chart decides closure one way**
//! (`geom_brep::props`' closure decision): a loop that does not close is
//! [`PropsError::ChartLoopOpen`], loops that wind as no face's boundary
//! does are [`PropsError::LoopsWindChart`], and a legal face whose
//! junction is open by less than the vertex slack measures.
//!
//! These rows are the cylinder's and the torus's closed forms; the
//! cylinder quadrature's are in `props::quad`'s unit rows
//! (`a_junction_open_by_the_vertex_tolerance_closes` and its
//! neighbours), and the spiric wall's in `props::spiric`'s.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::point::{p3, v3};
use crate::shared::tol::{band, eps};
use crate::shared::topo::edge;
use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, PropsError, curved_face, curved_face_loops};
use std::f64::consts::TAU;

const RR: f64 = 0.020;
const R0: f64 = 0.005;

fn cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

fn torus() -> Surface<f64> {
    Surface::Torus {
        center: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        major_radius: RR,
        minor_radius: R0,
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// The unit cylinder's rim at height `z`, walked in `u` from `u0` to `u1`.
fn crim(z: f64, u0: f64, u1: f64) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, z),
            axis: v3(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        0,
        0,
    )
}

/// The unit cylinder's ruling at azimuth `u`, from height `z0` to `z1`.
fn ruling(u: f64, z0: f64, z1: f64) -> LoopEdge<f64> {
    edge(
        Curve3::Line {
            origin: p3(u.cos(), u.sin(), 0.0),
            dir: v3(0.0, 0.0, 1.0),
        },
        z0,
        z1,
        0,
        0,
    )
}

/// The torus's parallel at tube angle `v`, walked in `u` from `u0` to `u1`.
fn trim(v: f64, u0: f64, u1: f64) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, R0 * v.sin()),
            axis: v3(0.0, 0.0, 1.0),
            radius: RR + R0 * v.cos(),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        0,
        0,
    )
}

/// The torus's meridian at azimuth `u`, walked in `v` from `v0` to `v1`.
fn tmer(u: f64, v0: f64, v1: f64) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: p3(RR * u.cos(), RR * u.sin(), 0.0),
            axis: v3(u.sin(), -u.cos(), 0.0),
            radius: R0,
            u_ref: v3(u.cos(), u.sin(), 0.0),
        },
        v0,
        v1,
        0,
        0,
    )
}

/// The cylinder rectangle `u ∈ [τ − 1, τ]`, `z ∈ [0, 1]`, its ruling on
/// the seam starting `gap` above the bottom rim's end.
fn cylinder_rectangle(gap: f64) -> Vec<LoopEdge<f64>> {
    vec![
        crim(0.0, TAU - 1.0, TAU),
        ruling(0.0, gap, 1.0),
        crim(1.0, TAU, TAU - 1.0),
        ruling(-1.0, 1.0, 0.0),
    ]
}

/// The torus rectangle `u ∈ [τ − 1, τ]`, `v ∈ [0.2, 0.7]`, its meridian
/// on the seam starting `gap` metres round the tube from the lower
/// parallel's end.
fn torus_rectangle(gap: f64) -> Vec<LoopEdge<f64>> {
    let (v0, v1) = (0.2, 0.7);
    vec![
        trim(v0, TAU - 1.0, TAU),
        tmer(TAU, v0 + gap / R0, v1),
        trim(v1, TAU, TAU - 1.0),
        tmer(TAU - 1.0, v1, v0),
    ]
}

/// **A legal face with a junction on the seam, open by `2ε`, measures**
/// on both closed forms: the two carriers' ends about their vertex are
/// within the slack the decision states (`3ε`), whatever whole turn the
/// seam puts between their azimuths. Without that slack the gap sits in
/// the band and escalates.
#[test]
fn a_legal_face_with_a_junction_on_the_seam_measures() {
    let gap = 2.0 * eps();
    let cyl = curved_face(&cylinder(), &cylinder_rectangle(gap), true, band())
        .unwrap_or_else(|e| panic!("cylinder: {e:?}"));
    assert!(
        (cyl.area - 1.0).abs() < 1e-6,
        "cylinder: area {} vs 1",
        cyl.area
    );
    let tor = curved_face(&torus(), &torus_rectangle(gap), true, band())
        .unwrap_or_else(|e| panic!("torus: {e:?}"));
    let want = R0 * (RR * 0.5 + R0 * (0.7_f64.sin() - 0.2_f64.sin()));
    assert!(
        (tor.area - want).abs() < 1e-6 * want,
        "torus: area {} vs {want}",
        tor.area
    );
}

/// **A loop that does not close is [`PropsError::ChartLoopOpen`]** on
/// both closed forms: the seam ruling or meridian starts `0.1` m off the
/// edge before's end.
#[test]
fn an_open_loop_is_chart_loop_open_on_every_closed_form() {
    assert_eq!(
        curved_face(&cylinder(), &cylinder_rectangle(0.1), true, band()).map(|_| ()),
        Err(PropsError::ChartLoopOpen),
        "cylinder"
    );
    assert_eq!(
        curved_face(&torus(), &torus_rectangle(0.1 * R0), true, band()).map(|_| ()),
        Err(PropsError::ChartLoopOpen),
        "torus"
    );
}

/// **Loops winding twice each way are [`PropsError::LoopsWindChart`]**
/// on both closed forms: two whole rims at one level make a loop that
/// turns twice, and its partner at another level turns twice back. The
/// face's loops sum to no turn, so only the one-turn check sees it; read
/// as a band, it measures twice the band's area.
#[test]
fn a_two_turn_pair_winds_as_no_boundary_on_every_closed_form() {
    let twice = |rim: fn(f64, f64, f64) -> LoopEdge<f64>, level: f64, forward: bool| {
        let (a, b) = if forward { (0.0, TAU) } else { (TAU, 0.0) };
        vec![rim(level, a, b), rim(level, a, b)]
    };
    let (lower, upper) = (twice(crim, 0.0, true), twice(crim, 1.0, false));
    assert_eq!(
        curved_face_loops(&cylinder(), &[&lower, &upper], true, band()).map(|_| ()),
        Err(PropsError::LoopsWindChart),
        "cylinder"
    );
    let (lower, upper) = (twice(trim, 0.2, true), twice(trim, 0.7, false));
    assert_eq!(
        curved_face_loops(&torus(), &[&lower, &upper], true, band()).map(|_| ()),
        Err(PropsError::LoopsWindChart),
        "torus"
    );
}

/// **Four whole rims of one cylinder bound no single face**
/// ([`PropsError::LoopsWindChart`]), at `f64` and at the interval
/// scalar. Wound `+ + − −` or `+ − − +` up the axis, each turns once and
/// together they turn zero times, but a face is one connected region
/// and has at most two winding loops on its boundary: read as one face
/// the first measures `8π`, the middle band twice, and the second `0`.
#[test]
fn four_winding_rims_bound_no_single_face() {
    fn rims<T: geom_core::Real>(senses: [bool; 4]) -> Vec<Vec<LoopEdge<T>>> {
        senses
            .iter()
            .enumerate()
            .map(|(i, &up)| {
                let (a, b) = if up { (0.0, TAU) } else { (TAU, 0.0) };
                vec![edge(
                    Curve3::Circle {
                        center: p3(0.0, 0.0, i as f64),
                        axis: v3(0.0, 0.0, 1.0),
                        radius: T::one(),
                        u_ref: v3(1.0, 0.0, 0.0),
                    },
                    a,
                    b,
                    0,
                    0,
                )]
            })
            .collect()
    }
    fn measure<T: geom_core::Decide>(senses: [bool; 4]) -> Result<(), PropsError> {
        let loops = rims::<T>(senses);
        let refs: Vec<&[LoopEdge<T>]> = loops.iter().map(Vec::as_slice).collect();
        let s = Surface::Cylinder {
            origin: p3(0.0, 0.0, 0.0),
            axis: v3(0.0, 0.0, 1.0),
            radius: T::one(),
            u_ref: v3(1.0, 0.0, 0.0),
        };
        curved_face_loops(&s, &refs, true, band()).map(|_| ())
    }
    for senses in [[true, true, false, false], [true, false, false, true]] {
        assert_eq!(
            measure::<f64>(senses),
            Err(PropsError::LoopsWindChart),
            "{senses:?} at f64"
        );
        assert_eq!(
            measure::<geom_core::Interval>(senses),
            Err(PropsError::LoopsWindChart),
            "{senses:?} at the interval scalar"
        );
    }
}

/// **A loop's turns count every edge's own turn** (`props_chart_loop_whole`),
/// at `f64` and at the interval scalar. On a cylinder of radius `100ε`, a
/// zig-zag of 400 rulings, each certified axial (`0.9ε` sideways, under
/// `ε`), turns the axis by `3.6` rad between them, and a rim swept back
/// closes it: the loop turns zero times. Read with its rulings as no
/// turn, it turned `−3.6` rad, a whole turn back by the half-turn
/// thresholds, so with one whole rim turning the other way beside it
/// the face measured. Counted
/// whole, the zig-zag alone measures and with the rim refuses
/// ([`PropsError::LoopsWindChart`]). With the rulings' turns read as
/// none, the loop's residual from a whole turn, `≈ 268ε`, is past its
/// slack and the face still refuses rather than measures.
#[test]
fn a_loop_of_many_rulings_turns_by_their_own_turns() {
    fn zig<T: geom_core::Decide>(radius: f64) -> (Vec<LoopEdge<T>>, f64) {
        let (n, h) = (400usize, 0.01);
        let delta = 2.0 * (0.45 * eps() / radius).asin();
        let at = |u: f64, z: f64| [radius * u.cos(), radius * u.sin(), z];
        let mut loop_ = Vec::with_capacity(n + 1);
        let mut u = 0.0;
        for i in 0..n {
            let (z0, z1) = if i % 2 == 0 { (0.0, h) } else { (h, 0.0) };
            let (a, b) = (at(u, z0), at(u + delta, z1));
            let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
            loop_.push(edge(
                Curve3::Line {
                    origin: p3(a[0], a[1], a[2]),
                    dir: v3(d[0] / len, d[1] / len, d[2] / len),
                },
                0.0,
                len,
                0,
                0,
            ));
            u += delta;
        }
        loop_.push(rim::<T>(radius, 0.0, u, 0.0));
        (loop_, u)
    }
    fn rim<T: geom_core::Decide>(radius: f64, z: f64, u0: f64, u1: f64) -> LoopEdge<T> {
        edge(
            Curve3::Circle {
                center: p3(0.0, 0.0, z),
                axis: v3(0.0, 0.0, 1.0),
                radius: T::from_f64(radius),
                u_ref: v3(1.0, 0.0, 0.0),
            },
            u0,
            u1,
            0,
            0,
        )
    }
    fn measure<T: geom_core::Decide>(with_rim: Option<bool>) -> Result<(), PropsError> {
        let radius = 100.0 * eps();
        let (zig, turned) = zig::<T>(radius);
        assert!(turned > 3.5, "the rulings turn {turned} rad between them");
        let s = Surface::Cylinder {
            origin: p3(0.0, 0.0, 0.0),
            axis: v3(0.0, 0.0, 1.0),
            radius: T::from_f64(radius),
            u_ref: v3(1.0, 0.0, 0.0),
        };
        let (u0, u1) = if with_rim == Some(true) {
            (0.0, TAU)
        } else {
            (TAU, 0.0)
        };
        let whole = vec![rim::<T>(radius, 1.0, u0, u1)];
        let loops: Vec<&[LoopEdge<T>]> = if with_rim.is_some() {
            vec![&zig, &whole]
        } else {
            vec![&zig]
        };
        curved_face_loops(&s, &loops, true, band()).map(|_| ())
    }
    for up in [true, false] {
        assert_eq!(
            measure::<f64>(Some(up)),
            Err(PropsError::LoopsWindChart),
            "with a whole rim (up: {up}) at f64"
        );
        assert_eq!(
            measure::<geom_core::Interval>(Some(up)),
            Err(PropsError::LoopsWindChart),
            "with a whole rim (up: {up}) at the interval scalar"
        );
    }
    assert_eq!(measure::<f64>(None), Ok(()), "the zig-zag alone at f64");
    assert_eq!(
        measure::<geom_core::Interval>(None),
        Ok(()),
        "the zig-zag alone at the interval scalar"
    );
}

/// A rim arc of `radius` at height `z` whose centre sits `off` from the
/// axis and whose axis tips `tilt` toward `+x`, from `u0` to `u1` about
/// its own centre.
fn off_arc<T: geom_core::Decide>(
    radius: f64,
    off: [f64; 2],
    tilt: f64,
    z: f64,
    (u0, u1): (f64, f64),
) -> LoopEdge<T> {
    let (s, c) = tilt.sin_cos();
    edge(
        Curve3::Circle {
            center: p3(off[0], off[1], z),
            axis: v3(s, 0.0, c),
            radius: T::from_f64(radius),
            u_ref: v3(c, 0.0, -s),
        },
        u0,
        u1,
        0,
        0,
    )
}

fn segment<T: geom_core::Decide>(a: [f64; 3], b: [f64; 3]) -> LoopEdge<T> {
    let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    edge(
        Curve3::Line {
            origin: p3(a[0], a[1], a[2]),
            dir: v3(d[0] / len, d[1] / len, d[2] / len),
        },
        0.0,
        len,
        0,
        0,
    )
}

fn cylinder_of<T: geom_core::Decide>(radius: f64) -> Surface<T> {
    Surface::Cylinder {
        origin: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        radius: T::from_f64(radius),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// `n` arcs of one rim over `[t0, t0 + n·step]`, consecutive ones meeting
/// `gap` metres apart in azimuth (alternating sides), each centred `off`
/// metres from the axis toward (`toward = 1`) or away from (`−1`) its
/// middle, traversed forward or back.
fn arc_chain<T: geom_core::Decide>(
    radius: f64,
    n: usize,
    step: f64,
    z: f64,
    (gap, off, toward): (f64, f64, f64),
    forward: bool,
) -> Vec<LoopEdge<T>> {
    let half = 0.5 * gap / radius;
    let mut arcs: Vec<LoopEdge<T>> = (0..n)
        .map(|i| {
            let (t0, t1) = (i as f64 * step, (i + 1) as f64 * step);
            let s = if i % 2 == 0 { 1.0 } else { -1.0 };
            let a = if i == 0 { t0 } else { t0 + s * half };
            let b = if i == n - 1 { t1 } else { t1 + s * half };
            let tm = 0.5 * (t0 + t1);
            let c = [toward * off * tm.cos(), toward * off * tm.sin()];
            let ends = if forward { (a, b) } else { (b, a) };
            off_arc::<T>(radius, c, 0.0, z, ends)
        })
        .collect();
    if !forward {
        arcs.reverse();
    }
    arcs
}

/// **Rims whose carriers sit within the band of the axis, in many arcs,
/// measure** at `f64` and at the interval scalar. Each arc's stored turn
/// is about its own centre, up to `0.9ε` off the axis, which turns its
/// ends about the axis by up to `2·off/R` more or less than the stored
/// turn, arc by arc. A loop's turns read each rim's turn about the axis
/// between its own ends, so a band of two rims, a rectangle and a whole
/// wall cut along a seam ruling, their rims in up to 200 arcs whose
/// junctions sit within the slack (gaps up to `3.5ε` on the axis, less
/// twice the centres' offset off it), close and turn as they bound. Read
/// off the stored turns alone, they left up to `≈ 9.4ε` from a whole
/// turn.
#[test]
fn rims_in_many_arcs_off_the_axis_measure() {
    fn band_face<T: geom_core::Decide>(
        radius: f64,
        n: usize,
        gap: f64,
        off: f64,
    ) -> Result<(), PropsError> {
        let lower = arc_chain::<T>(radius, n, TAU / n as f64, 0.0, (gap, off, 1.0), true);
        let upper = vec![off_arc::<T>(radius, [0.0, 0.0], 0.0, 1.0, (TAU, 0.0))];
        curved_face_loops(&cylinder_of::<T>(radius), &[&lower, &upper], true, band()).map(|_| ())
    }
    fn rect_face<T: geom_core::Decide>(
        radius: f64,
        n: usize,
        gap: f64,
        off: f64,
    ) -> Result<(), PropsError> {
        let span = 5.0;
        let at = |u: f64, z: f64| [radius * u.cos(), radius * u.sin(), z];
        let mut l = arc_chain::<T>(radius, n, span / n as f64, 0.0, (gap, off, 1.0), true);
        l.push(segment::<T>(at(span, 0.0), at(span, 1.0)));
        l.push(off_arc::<T>(radius, [0.0, 0.0], 0.0, 1.0, (span, 0.0)));
        l.push(segment::<T>(at(0.0, 1.0), at(0.0, 0.0)));
        curved_face_loops(&cylinder_of::<T>(radius), &[&l], true, band()).map(|_| ())
    }
    let e = eps();
    for radius in [1.0, 0.01] {
        for n in [4usize, 50, 200] {
            for gap in [0.0, 1.0, 2.0, 3.0, 3.5] {
                for off in [0.0, 0.5, 0.9] {
                    // Two arcs' centres off the axis on either side of a
                    // junction move its ends apart by up to `2·off`: past
                    // the slack and `ε`, the junction is not closed, and
                    // escalating it is right.
                    if gap + 2.0 * off >= 4.0 {
                        continue;
                    }
                    let (gap, off) = (gap * e, off * e);
                    let what = format!("R = {radius}, {n} arcs, gap {gap:e}, off {off:e}");
                    assert_eq!(
                        band_face::<f64>(radius, n, gap, off),
                        Ok(()),
                        "band, {what}, f64"
                    );
                    assert_eq!(
                        band_face::<geom_core::Interval>(radius, n, gap, off),
                        Ok(()),
                        "band, {what}, interval"
                    );
                    assert_eq!(
                        rect_face::<f64>(radius, n, gap, off),
                        Ok(()),
                        "rectangle, {what}, f64"
                    );
                    assert_eq!(
                        rect_face::<geom_core::Interval>(radius, n, gap, off),
                        Ok(()),
                        "rectangle, {what}, interval"
                    );
                }
            }
        }
    }
}

/// **A whole wall cut along a seam ruling, both rims in arcs off the
/// axis, measures** at `f64` and at the interval scalar: the lower rim's
/// arcs centred toward their middles and the upper's away, so both
/// chains' stored turns overshoot their ends' turns about the axis the
/// same way round the loop, and with the arcs' axes tipped `0.9ε/R` as
/// well. Read off the stored turns alone, `n = 50` at `0.99ε` was `0.57ε`
/// from refusing as winding.
#[test]
fn a_seam_cut_wall_with_arcs_off_the_axis_measures() {
    fn wall<T: geom_core::Decide>(n: usize, off: f64, tilt: f64) -> Result<(), PropsError> {
        let radius = 1.0;
        let step = TAU / n as f64;
        let at = |u: f64, z: f64| [radius * u.cos(), radius * u.sin(), z];
        let arc = |i: usize, z: f64, toward: f64, forward: bool| {
            let (t0, t1) = (i as f64 * step, (i + 1) as f64 * step);
            let tm = 0.5 * (t0 + t1);
            let c = [toward * off * tm.cos(), toward * off * tm.sin()];
            off_arc::<T>(
                radius,
                c,
                tilt,
                z,
                if forward { (t0, t1) } else { (t1, t0) },
            )
        };
        let mut l: Vec<LoopEdge<T>> = (0..n).map(|i| arc(i, 0.0, 1.0, true)).collect();
        l.push(segment::<T>(at(0.0, 0.0), at(0.0, 1.0)));
        l.extend((0..n).rev().map(|i| arc(i, 1.0, -1.0, false)));
        l.push(segment::<T>(at(0.0, 1.0), at(0.0, 0.0)));
        curved_face_loops(&cylinder_of::<T>(radius), &[&l], true, band()).map(|_| ())
    }
    let e = eps();
    for n in [2usize, 4, 8, 50] {
        for off in [0.0, 0.5, 0.9, 0.99] {
            for tilt in [0.0, 0.9 * e] {
                let what = format!("{n} arcs, off {off}ε, tilt {tilt:e}");
                assert_eq!(wall::<f64>(n, off * e, tilt), Ok(()), "{what}, f64");
                assert_eq!(
                    wall::<geom_core::Interval>(n, off * e, tilt),
                    Ok(()),
                    "{what}, interval"
                );
            }
        }
    }
}
