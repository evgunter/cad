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
use geom_brep::props::{
    LoopEdge, PropsError, boundary_material_sign_loops, curved_face, curved_face_loops,
};
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
/// none, each misses its ends by `0.9ε` at the radius, `360ε` summed,
/// past the loop's slack, and the face still refuses rather than
/// measures.
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

/// **A torus loop's turns count every meridian's own azimuth turn**, at
/// `f64` and at the interval scalar. On a torus of radii `100ε` and
/// `50ε`, a zig-zag of 400 meridian arcs between the outer equator and
/// the top, each tipped about its own radial direction by `0.9ε` (under
/// the meridians' `ε`), turns the axis by `≈ 3.6` rad between them, and
/// a parallel on the outer equator swept back closes it: the loop turns
/// zero times. Read with its meridians as no azimuth turn, it turned a
/// whole turn back, and beside a whole parallel turning the other way
/// the face measured. Counted whole, the zig-zag alone measures and with
/// a whole parallel either way refuses ([`PropsError::LoopsWindChart`]).
#[test]
fn a_torus_loop_of_many_meridians_turns_by_their_own_turns() {
    fn parallel<T: geom_core::Decide>(
        rr: f64,
        r0: f64,
        v: f64,
        (u0, u1): (f64, f64),
    ) -> LoopEdge<T> {
        edge(
            Curve3::Circle {
                center: p3(0.0, 0.0, r0 * v.sin()),
                axis: v3(0.0, 0.0, 1.0),
                radius: T::from_f64(rr + r0 * v.cos()),
                u_ref: v3(1.0, 0.0, 0.0),
            },
            u0,
            u1,
            0,
            0,
        )
    }
    /// The meridian at azimuth `phi` tipped `alpha` about its radial
    /// direction: `c + r·cos t·ρ̂ + r·sin t·(−cos α·ẑ + sin α·t̂)`.
    fn meridian<T: geom_core::Decide>(
        rr: f64,
        r0: f64,
        phi: f64,
        alpha: f64,
        (t0, t1): (f64, f64),
    ) -> LoopEdge<T> {
        let (s, c) = phi.sin_cos();
        let (sa, ca) = alpha.sin_cos();
        edge(
            Curve3::Circle {
                center: p3(rr * c, rr * s, 0.0),
                axis: v3(-s * ca, c * ca, sa),
                radius: T::from_f64(r0),
                u_ref: v3(c, s, 0.0),
            },
            t0,
            t1,
            0,
            0,
        )
    }
    fn measure<T: geom_core::Decide>(with_parallel: Option<bool>) -> Result<(), PropsError> {
        let (rr, r0) = (100.0 * eps(), 50.0 * eps());
        let lateral = 0.9 * eps();
        let alpha = (lateral / r0).asin();
        let step = 2.0 * (lateral / rr).atan();
        let mut zig: Vec<LoopEdge<T>> = Vec::new();
        let mut phi = 0.0;
        for _ in 0..200 {
            // Up from the outer equator to the top, ending `lateral` on;
            // then down from `lateral` back, ending on the next azimuth.
            zig.push(meridian::<T>(
                rr,
                r0,
                phi,
                -alpha,
                (0.0, -core::f64::consts::FRAC_PI_2),
            ));
            phi += step;
            zig.push(meridian::<T>(
                rr,
                r0,
                phi,
                alpha,
                (-core::f64::consts::FRAC_PI_2, 0.0),
            ));
        }
        assert!(phi > 3.5, "the meridians turn {phi} rad between them");
        zig.push(parallel::<T>(rr, r0, 0.0, (phi, 0.0)));
        let s = Surface::Torus {
            center: p3(0.0, 0.0, 0.0),
            axis: v3(0.0, 0.0, 1.0),
            major_radius: T::from_f64(rr),
            minor_radius: T::from_f64(r0),
            u_ref: v3(1.0, 0.0, 0.0),
        };
        let ends = if with_parallel == Some(true) {
            (0.0, TAU)
        } else {
            (TAU, 0.0)
        };
        let whole = vec![parallel::<T>(rr, r0, -1.0, ends)];
        let loops: Vec<&[LoopEdge<T>]> = if with_parallel.is_some() {
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
            "with a whole parallel (up: {up}) at f64"
        );
        assert_eq!(
            measure::<geom_core::Interval>(Some(up)),
            Err(PropsError::LoopsWindChart),
            "with a whole parallel (up: {up}) at the interval scalar"
        );
    }
    assert_eq!(measure::<f64>(None), Ok(()), "the zig-zag alone at f64");
    assert_eq!(
        measure::<geom_core::Interval>(None),
        Ok(()),
        "the zig-zag alone at the interval scalar"
    );
}

/// **A torus face whose parallels and meridians are long chains of arcs
/// off centre measures**, at `f64` and at the interval scalar. A torus
/// rectangle `u ∈ [0, U]`, `v ∈ [v_c − V, v_c + V]` about the inner or
/// the outer equator: its lower parallel in `n` arcs, each centred `0.9ε`
/// off the axis toward the arc's middle azimuth, and its far meridian in
/// `n` arcs, each centred `0.9ε` off the core circle toward the arc's
/// middle tube angle, so the stored turns overshoot their ends' turns
/// about the axis (the core circle) the same way along each chain. A
/// loop's turns carry each arc's stored turn to the axis (the core
/// circle) at its ends; read off the stored turns alone, these faces
/// left their loop past a whole turn's slack.
#[test]
fn torus_chains_of_arcs_off_centre_measure() {
    fn face<T: geom_core::Decide>(
        (big, r): (f64, f64),
        vc: f64,
        n: usize,
        off: f64,
        (u_span, v_half): (f64, f64),
    ) -> Result<(), PropsError> {
        let (v0, v1) = (vc - v_half, vc + v_half);
        let parallel = |v: f64, c: [f64; 2], u0: f64, u1: f64| {
            edge::<T>(
                Curve3::Circle {
                    center: p3(c[0], c[1], r * v.sin()),
                    axis: v3(0.0, 0.0, 1.0),
                    radius: T::from_f64(big + r * v.cos()),
                    u_ref: v3(1.0, 0.0, 0.0),
                },
                u0,
                u1,
                0,
                0,
            )
        };
        let meridian = |u: f64, grow: f64, a: f64, b: f64| {
            let m = 0.5 * (a + b);
            let (gx, gz) = (grow * m.cos(), grow * m.sin());
            edge::<T>(
                Curve3::Circle {
                    center: p3((big + gx) * u.cos(), (big + gx) * u.sin(), gz),
                    axis: v3(u.sin(), -u.cos(), 0.0),
                    radius: T::from_f64(r),
                    u_ref: v3(u.cos(), u.sin(), 0.0),
                },
                a,
                b,
                0,
                0,
            )
        };
        let mut l = Vec::new();
        let du = u_span / n as f64;
        for i in 0..n {
            let (a, b) = (i as f64 * du, (i + 1) as f64 * du);
            let m = 0.5 * (a + b);
            l.push(parallel(v0, [off * m.cos(), off * m.sin()], a, b));
        }
        let dv = (v1 - v0) / n as f64;
        for i in 0..n {
            let (a, b) = (v0 + i as f64 * dv, v0 + (i + 1) as f64 * dv);
            l.push(meridian(u_span, off, a, b));
        }
        l.push(parallel(v1, [0.0, 0.0], u_span, 0.0));
        l.push(meridian(0.0, 0.0, v1, v0));
        let s = Surface::Torus {
            center: p3(0.0, 0.0, 0.0),
            axis: v3(0.0, 0.0, 1.0),
            major_radius: T::from_f64(big),
            minor_radius: T::from_f64(r),
            u_ref: v3(1.0, 0.0, 0.0),
        };
        curved_face_loops(&s, &[&l], true, band()).map(|_| ())
    }
    let e = eps();
    for radii in [(0.02, 0.005), (0.01, 0.008)] {
        for vc in [core::f64::consts::PI, 0.0] {
            for spans in [(6.0, 0.3), (1.0, 3.0), (6.0, 3.0)] {
                for n in [20usize, 50] {
                    for off in [0.0, 0.9 * e] {
                        let what = format!(
                            "{radii:?}, v_c {vc:.2}, spans {spans:?}, {n} arcs, off {off:e}"
                        );
                        assert_eq!(face::<f64>(radii, vc, n, off, spans), Ok(()), "{what}, f64");
                        assert_eq!(
                            face::<geom_core::Interval>(radii, vc, n, off, spans),
                            Ok(()),
                            "{what}, interval"
                        );
                    }
                }
            }
        }
    }
}

/// **Circles just past the axis class's threshold measure** at `f64`
/// and at the interval scalar: the parse admits a rim on a periodic
/// chart once its radius about the axis is definite (`Kε`,
/// `props_circle_axis_class`), and the closed forms read every circle it
/// admits. A cylinder rectangle and band of radius `10.5ε` and `11.5ε`,
/// a thin torus of that minor radius, and spindle and horn tori whose
/// parallel near the axis has that radius all measure, each its closed
/// form's area (`R·Δu·Δz`; `r·Δu·(R·Δv + r·Δ sin v)`); demanding
/// `2ε` more of the radius than the class does refused them. At the
/// interval scalar the near-axis parallel's azimuths round by about
/// `u·|p|/ρ`, which a loop's whole-turn residual charges at that
/// parallel's own radius; charged at the loop's largest it escalated
/// the spindle and horn faces at `ε = 1e-12`. They measure out to
/// `10⁴ε` from the axis, or `1e-8` m where that is nearer.
#[test]
fn circles_just_past_the_axis_class_threshold_measure() {
    /// The face's area, its bracket checked to enclose `want` within
    /// `10⁻⁷` of it: a measured face answers its closed form, to the
    /// rounding of circles a few `ε` across.
    fn area_is<T: geom_core::Decide + geom_core::CertifiedEnclosure>(
        got: Result<geom_brep::props::FaceContribution<T>, PropsError>,
        want: f64,
    ) -> Result<(), PropsError> {
        got.map(|fc| {
            let (lo, hi) = fc.area.certified_bracket().expect("a certified area");
            let slack = 1e-7 * want;
            assert!(
                lo <= want + slack && want - slack <= hi && hi - lo <= slack,
                "area [{lo:e}, {hi:e}], not {want:e}"
            );
        })
    }
    fn rim<T: geom_core::Decide>(radius: f64, z: f64, ends: (f64, f64)) -> LoopEdge<T> {
        off_arc::<T>(radius, [0.0, 0.0], 0.0, z, ends)
    }
    fn cylinder_faces<T: geom_core::Decide + geom_core::CertifiedEnclosure>(
        radius: f64,
    ) -> [Result<(), PropsError>; 2] {
        let at = |u: f64, z: f64| [radius * u.cos(), radius * u.sin(), z];
        let rect = vec![
            rim::<T>(radius, 0.0, (0.0, 2.0)),
            segment::<T>(at(2.0, 0.0), at(2.0, 1.0)),
            rim::<T>(radius, 1.0, (2.0, 0.0)),
            segment::<T>(at(0.0, 1.0), at(0.0, 0.0)),
        ];
        let (lower, upper) = (
            vec![rim::<T>(radius, 0.0, (0.0, TAU))],
            vec![rim::<T>(radius, 1.0, (TAU, 0.0))],
        );
        let s = cylinder_of::<T>(radius);
        [
            area_is(curved_face_loops(&s, &[&rect], true, band()), 2.0 * radius),
            area_is(
                curved_face_loops(&s, &[&lower, &upper], true, band()),
                TAU * radius,
            ),
        ]
    }
    fn torus_rect<T: geom_core::Decide + geom_core::CertifiedEnclosure>(
        (big, r): (f64, f64),
        (v0, v1): (f64, f64),
    ) -> Result<(), PropsError> {
        let parallel = |v: f64, u0: f64, u1: f64| {
            edge::<T>(
                Curve3::Circle {
                    center: p3(0.0, 0.0, r * v.sin()),
                    axis: v3(0.0, 0.0, 1.0),
                    radius: T::from_f64(big + r * v.cos()),
                    u_ref: v3(1.0, 0.0, 0.0),
                },
                u0,
                u1,
                0,
                0,
            )
        };
        let meridian = |u: f64, a: f64, b: f64| {
            edge::<T>(
                Curve3::Circle {
                    center: p3(big * u.cos(), big * u.sin(), 0.0),
                    axis: v3(u.sin(), -u.cos(), 0.0),
                    radius: T::from_f64(r),
                    u_ref: v3(u.cos(), u.sin(), 0.0),
                },
                a,
                b,
                0,
                0,
            )
        };
        let l = vec![
            parallel(v0, 0.0, 1.0),
            meridian(1.0, v0, v1),
            parallel(v1, 1.0, 0.0),
            meridian(0.0, v1, v0),
        ];
        let s = Surface::Torus {
            center: p3(0.0, 0.0, 0.0),
            axis: v3(0.0, 0.0, 1.0),
            major_radius: T::from_f64(big),
            minor_radius: T::from_f64(r),
            u_ref: v3(1.0, 0.0, 0.0),
        };
        // `∫∫ r·(R + r·cos v) du dv` over `u ∈ [0, 1]`.
        let want = (r * (big * (v1 - v0) + r * (v1.sin() - v0.sin()))).abs();
        area_is(curved_face_loops(&s, &[&l], true, band()), want)
    }
    fn faces<T: geom_core::Decide + geom_core::CertifiedEnclosure>(
        rho: f64,
    ) -> Vec<(String, Result<(), PropsError>)> {
        let [rect, whole] = cylinder_faces::<T>(rho);
        let mut out = vec![
            ("cylinder rectangle".to_string(), rect),
            ("cylinder band".to_string(), whole),
            (
                "thin torus".to_string(),
                torus_rect::<T>((1e-3, rho), (0.2, 1.2)),
            ),
        ];
        // The parallel at `v1` has radius `rho` about the axis.
        for (name, big, r) in [
            ("spindle torus", 0.004, 0.005),
            ("horn torus", 0.005, 0.005),
        ] {
            let v1 = ((rho - big) / r).acos();
            out.push((name.to_string(), torus_rect::<T>((big, r), (1.0, v1))));
        }
        out
    }
    let e = eps();
    for k in [10.5, 11.5, 1e2, 1e3, 1e4] {
        if k * e > 1e-8 {
            continue;
        }
        for (what, got) in faces::<f64>(k * e) {
            assert_eq!(got, Ok(()), "{what} at {k}ε, f64");
        }
        for (what, got) in faces::<geom_core::Interval>(k * e) {
            assert_eq!(got, Ok(()), "{what} at {k}ε, interval");
        }
    }
}

/// The cone of half-angle ½ about `+z` through the origin.
fn cone<T: geom_core::Real>() -> Surface<T> {
    Surface::Cone {
        apex: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        half_angle: T::from_f64(0.5),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// The cone's rim at slant `v`, from azimuth `u0` to `u1`.
fn kim<T: geom_core::Real>(v: f64, u0: f64, u1: f64) -> LoopEdge<T> {
    let (s, c) = 0.5_f64.sin_cos();
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, v * c),
            axis: v3(0.0, 0.0, 1.0),
            radius: T::from_f64(v * s),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        0,
        0,
    )
}

/// The cone's generator at azimuth `u`, from slant `v0` to `v1`.
fn kgen<T: geom_core::Real>(u: f64, v0: f64, v1: f64) -> LoopEdge<T> {
    let (s, c) = 0.5_f64.sin_cos();
    edge(
        Curve3::Line {
            origin: p3(0.0, 0.0, 0.0),
            dir: v3(s * u.cos(), s * u.sin(), c),
        },
        v0,
        v1,
        0,
        0,
    )
}

/// **The cone decides closure as the periodic charts do**, at `f64` and
/// at the interval scalar. The rectangle `u ∈ [τ − 1, τ]`, slant
/// `[1, 2]`, its generator on the seam starting `2ε` up from the lower
/// rim's end, measures `sin α·(2² − 1²)/2`; started `0.1` m up it is
/// [`PropsError::ChartLoopOpen`]. The band between whole rims at slants
/// 1 and 2 measures `π·sin α·(2² − 1²)`; the pair of loops each turning
/// twice, and four whole rims wound `+ + − −`, are
/// [`PropsError::LoopsWindChart`].
#[test]
fn the_cone_decides_closure_as_the_periodic_charts_do() {
    /// The face's area, its bracket's midpoint after checking the
    /// bracket is within `10⁻⁹` of it.
    fn face<T: geom_core::Decide + geom_core::CertifiedEnclosure>(
        loops: &[Vec<LoopEdge<T>>],
    ) -> Result<f64, PropsError> {
        let refs: Vec<&[LoopEdge<T>]> = loops.iter().map(Vec::as_slice).collect();
        curved_face_loops(&cone::<T>(), &refs, true, band()).map(|fc| {
            let (lo, hi) = fc.area.certified_bracket().expect("a certified area");
            assert!(hi - lo <= 1e-9 * hi.abs(), "area bracket [{lo}, {hi}]");
            0.5 * (lo + hi)
        })
    }
    fn rows<T: geom_core::Decide + geom_core::CertifiedEnclosure>(scalar: &str) {
        let sa = 0.5_f64.sin();
        let rect = |gap: f64| {
            vec![vec![
                kim::<T>(1.0, TAU - 1.0, TAU),
                kgen::<T>(TAU, 1.0 + gap, 2.0),
                kim::<T>(2.0, TAU, TAU - 1.0),
                kgen::<T>(TAU - 1.0, 2.0, 1.0),
            ]]
        };
        let whole = |v: f64, up: bool, times: usize| {
            let (a, b) = if up { (0.0, TAU) } else { (TAU, 0.0) };
            vec![kim::<T>(v, a, b); times]
        };
        let area =
            face(&rect(2.0 * eps())).unwrap_or_else(|e| panic!("rectangle at {scalar}: {e:?}"));
        let want = sa * 1.5;
        assert!(
            (area - want).abs() < 1e-9 * want,
            "rectangle at {scalar}: {area} vs {want}"
        );
        assert_eq!(
            face(&rect(0.1)).err(),
            Some(PropsError::ChartLoopOpen),
            "open rectangle at {scalar}"
        );
        let area = face(&[whole(1.0, true, 1), whole(2.0, false, 1)])
            .unwrap_or_else(|e| panic!("band at {scalar}: {e:?}"));
        let want = core::f64::consts::PI * sa * 3.0;
        assert!(
            (area - want).abs() < 1e-9 * want,
            "band at {scalar}: {area} vs {want}"
        );
        assert_eq!(
            face(&[whole(1.0, true, 2), whole(2.0, false, 2)]).err(),
            Some(PropsError::LoopsWindChart),
            "two-turn pair at {scalar}"
        );
        assert_eq!(
            face(&[
                whole(1.0, true, 1),
                whole(1.5, true, 1),
                whole(2.0, false, 1),
                whole(2.5, false, 1)
            ])
            .err(),
            Some(PropsError::LoopsWindChart),
            "+ + − − at {scalar}"
        );
    }
    rows::<f64>("f64");
    rows::<geom_core::Interval>("the interval scalar");
}

/// **A cone band whose lower rim is in many arcs off the axis
/// measures**, at `f64` and at the interval scalar, as the cylinder's
/// does ([`rims_in_many_arcs_off_the_axis_measure`]): each arc's stored
/// turn about its own centre is carried to the axis at its ends.
#[test]
fn a_cone_rim_in_many_arcs_off_the_axis_measures() {
    fn band_face<T: geom_core::Decide>(n: usize, gap: f64, off: f64) -> Result<(), PropsError> {
        let (s, c) = 0.5_f64.sin_cos();
        let lower = arc_chain::<T>(s, n, TAU / n as f64, c, (gap, off, 1.0), true);
        let upper = vec![kim::<T>(2.0, TAU, 0.0)];
        curved_face_loops(&cone::<T>(), &[&lower, &upper], true, band()).map(|_| ())
    }
    let e = eps();
    for n in [4usize, 50, 200] {
        for (gap, off) in [(0.0, 0.0), (1.0, 0.9), (2.0, 0.9), (3.5, 0.0)] {
            let (gap, off) = (gap * e, off * e);
            let what = format!("{n} arcs, gap {gap:e}, off {off:e}");
            assert_eq!(band_face::<f64>(n, gap, off), Ok(()), "{what}, f64");
            assert_eq!(
                band_face::<geom_core::Interval>(n, gap, off),
                Ok(()),
                "{what}, interval"
            );
        }
    }
}

/// The cone of half-angle ½ about the skew axis `(0.3, −0.4, 1)` through
/// the apex `(0.2, −0.1, −1)`, its rims and generators: a frame off the
/// origin and the coordinate axes.
struct SkewCone {
    alpha: f64,
    apex: [f64; 3],
    a: [f64; 3],
    e1: [f64; 3],
    e2: [f64; 3],
}

impl SkewCone {
    fn new() -> Self {
        Self::posed(0.5, [0.2, -0.1, -1.0])
    }
    /// The skew axis's cone of half-angle `alpha` through `apex`.
    fn posed(alpha: f64, apex: [f64; 3]) -> Self {
        let n = |v: [f64; 3]| {
            let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            [v[0] / l, v[1] / l, v[2] / l]
        };
        let a = n([0.3, -0.4, 1.0]);
        let e1 = n([a[2], 0.0, -a[0]]);
        let e2 = [
            a[1] * e1[2] - a[2] * e1[1],
            a[2] * e1[0] - a[0] * e1[2],
            a[0] * e1[1] - a[1] * e1[0],
        ];
        SkewCone {
            alpha,
            apex,
            a,
            e1,
            e2,
        }
    }
    fn at(&self, s: [f64; 3]) -> [f64; 3] {
        let k = |i: usize| self.apex[i] + s[0] * self.e1[i] + s[1] * self.e2[i] + s[2] * self.a[i];
        [k(0), k(1), k(2)]
    }
    fn surface<T: geom_core::Real>(&self) -> Surface<T> {
        Surface::Cone {
            apex: p3(self.apex[0], self.apex[1], self.apex[2]),
            axis: v3(self.a[0], self.a[1], self.a[2]),
            half_angle: T::from_f64(self.alpha),
            u_ref: v3(self.e1[0], self.e1[1], self.e1[2]),
        }
    }
    fn rim<T: geom_core::Real>(&self, v: f64, u0: f64, u1: f64) -> LoopEdge<T> {
        let (s, c) = self.alpha.sin_cos();
        let o = self.at([0.0, 0.0, v * c]);
        edge(
            Curve3::Circle {
                center: p3(o[0], o[1], o[2]),
                axis: v3(self.a[0], self.a[1], self.a[2]),
                radius: T::from_f64(v * s),
                u_ref: v3(self.e1[0], self.e1[1], self.e1[2]),
            },
            u0,
            u1,
            0,
            0,
        )
    }
    /// The window `[v0, v1] × [u0, u1]`, counter-clockwise on the chart
    /// when `with`, else clockwise.
    fn window<T: geom_core::Real>(
        &self,
        (v0, v1): (f64, f64),
        (u0, u1): (f64, f64),
        with: bool,
    ) -> Vec<LoopEdge<T>> {
        let l = vec![
            self.rim::<T>(v0, u0, u1),
            self.generator::<T>(u1, v0, v1),
            self.rim::<T>(v1, u1, u0),
            self.generator::<T>(u0, v1, v0),
        ];
        if with { l } else { reversed(l) }
    }
    fn generator<T: geom_core::Real>(&self, u: f64, v0: f64, v1: f64) -> LoopEdge<T> {
        let (s, c) = self.alpha.sin_cos();
        // The direction off the frame itself, not as a point less the
        // apex: an apex far out would round it by its own distance.
        let k = |i: usize| s * u.cos() * self.e1[i] + s * u.sin() * self.e2[i] + c * self.a[i];
        let d = [k(0), k(1), k(2)];
        edge(
            Curve3::Line {
                origin: p3(self.apex[0], self.apex[1], self.apex[2]),
                dir: v3(d[0], d[1], d[2]),
            },
            v0,
            v1,
            0,
            0,
        )
    }
}

/// A cone face's area and flux, their brackets' midpoints after checking
/// each bracket is within `10⁻⁹` of it.
fn skew_face<T: geom_core::Decide + geom_core::CertifiedEnclosure>(
    loops: &[Vec<LoopEdge<T>>],
) -> Result<(f64, f64), PropsError> {
    face_on(&SkewCone::new(), loops)
}

/// [`skew_face`] on the cone `c`.
fn face_on<T: geom_core::Decide + geom_core::CertifiedEnclosure>(
    c: &SkewCone,
    loops: &[Vec<LoopEdge<T>>],
) -> Result<(f64, f64), PropsError> {
    let refs: Vec<&[LoopEdge<T>]> = loops.iter().map(Vec::as_slice).collect();
    curved_face_loops(&c.surface::<T>(), &refs, true, band()).map(|fc| {
        let mid = |x: T| {
            let (lo, hi) = x.certified_bracket().expect("a certified enclosure");
            assert!(
                hi - lo <= 1e-9 * hi.abs().max(lo.abs()),
                "bracket [{lo}, {hi}]"
            );
            0.5 * (lo + hi)
        };
        (mid(fc.area), mid(fc.flux))
    })
}

/// **A band between two whole rims measures its frustum** — a face the
/// cone's flux lane refused before it read windings as the periodic
/// charts do — at `f64` and at the interval scalar, on a cone about a
/// skew axis off the origin: area `π·sin α·(v₁² − v₀²)`, flux the
/// apex's height along the axis times `sin α` times that area, both
/// equal to the band stated as one loop cut along a seam generator,
/// which the iso parse admits (`require_iso_rectangle`). Its rims stored
/// a period off, over `[τ, 2τ]`, measure the same.
#[test]
fn a_cone_band_measures_its_frustum_as_its_seamed_rectangle_does() {
    fn rows<T: geom_core::Decide + geom_core::CertifiedEnclosure>(scalar: &str) {
        let c = SkewCone::new();
        let (v0, v1) = (1.0, 2.0);
        let sa = 0.5_f64.sin();
        let area = core::f64::consts::PI * sa * (v1 * v1 - v0 * v0);
        let height: f64 = (0..3).map(|k| c.apex[k] * c.a[k]).sum();
        let seamed = vec![
            c.rim::<T>(v0, 0.0, TAU),
            c.generator::<T>(TAU, v0, v1),
            c.rim::<T>(v1, TAU, 0.0),
            c.generator::<T>(0.0, v1, v0),
        ];
        assert_eq!(
            geom_brep::props::require_iso_rectangle(&c.surface::<T>(), &seamed, band()),
            Ok(()),
            "the seamed band is an iso rectangle at {scalar}"
        );
        let (sa_, sf) =
            skew_face(&[seamed]).unwrap_or_else(|e| panic!("seamed band at {scalar}: {e:?}"));
        assert!(
            (sa_ - area).abs() <= 1e-9 * area,
            "seamed area {sa_} vs {area} at {scalar}"
        );
        assert!(
            (sf.abs() - height.abs() * sa * area).abs() <= 1e-9 * area,
            "seamed flux {sf} at {scalar}"
        );
        for (name, (u0, u1)) in [
            ("band", (0.0, TAU)),
            ("band a period off", (TAU, 2.0 * TAU)),
        ] {
            let (ba, bf) = skew_face(&[vec![c.rim::<T>(v0, u0, u1)], vec![c.rim::<T>(v1, u1, u0)]])
                .unwrap_or_else(|e| panic!("{name} at {scalar}: {e:?}"));
            assert!(
                (ba - sa_).abs() <= 1e-9 * area,
                "{name} area {ba} vs {sa_} at {scalar}"
            );
            assert!(
                (bf - sf).abs() <= 1e-9 * area,
                "{name} flux {bf} vs {sf} at {scalar}"
            );
        }
    }
    rows::<f64>("f64");
    rows::<geom_core::Interval>("the interval scalar");
}

/// **The apex is the cone chart's puncture**: a face whose loops wind it
/// on balance holds it, and only the apex winding back makes them a
/// boundary. The cap under a whole rim at slant 2, with a hole that
/// winds nothing, measures the cap less the hole, `sin α·(π·2² −
/// ½·(1.5² − 1²))`; read without the apex, its one winding loop is no
/// boundary's. With a second whole rim wound the same way it winds twice
/// on balance and is [`PropsError::LoopsWindChart`]. At `f64` and at the
/// interval scalar, on the skew cone.
#[test]
fn an_apex_cap_with_a_hole_holds_the_apex() {
    fn rows<T: geom_core::Decide + geom_core::CertifiedEnclosure>(scalar: &str) {
        let c = SkewCone::new();
        let sa = 0.5_f64.sin();
        let cap = vec![c.rim::<T>(2.0, 0.0, TAU)];
        let hole = |forward: bool| {
            let l = vec![
                c.rim::<T>(1.0, 1.0, 2.0),
                c.generator::<T>(2.0, 1.0, 1.5),
                c.rim::<T>(1.5, 2.0, 1.0),
                c.generator::<T>(1.0, 1.5, 1.0),
            ];
            if forward {
                l
            } else {
                let mut r: Vec<LoopEdge<T>> = l
                    .into_iter()
                    .map(|mut e| {
                        e.forward = !e.forward;
                        core::mem::swap(&mut e.start, &mut e.end);
                        e
                    })
                    .collect();
                r.reverse();
                r
            }
        };
        let want = sa * (core::f64::consts::PI * 4.0 - 0.5 * (1.5 * 1.5 - 1.0));
        let measured: Vec<f64> = [true, false]
            .into_iter()
            .filter_map(|f| skew_face(&[cap.clone(), hole(f)]).ok().map(|(a, _)| a))
            .collect();
        assert_eq!(
            measured.len(),
            1,
            "one traversal of the hole measures at {scalar}"
        );
        assert!(
            (measured[0] - want).abs() <= 1e-9 * want,
            "cap less hole {} vs {want} at {scalar}",
            measured[0]
        );
        assert_eq!(
            skew_face(&[cap.clone(), vec![c.rim::<T>(1.0, 0.0, TAU)]]).err(),
            Some(PropsError::LoopsWindChart),
            "a cap and a rim wound alike at {scalar}"
        );
    }
    rows::<f64>("f64");
    rows::<geom_core::Interval>("the interval scalar");
}

/// A loop traversed the other way.
fn reversed<T: geom_core::Real>(l: Vec<LoopEdge<T>>) -> Vec<LoopEdge<T>> {
    let mut r: Vec<LoopEdge<T>> = l
        .into_iter()
        .map(|mut e| {
            e.forward = !e.forward;
            core::mem::swap(&mut e.start, &mut e.end);
            e
        })
        .collect();
    r.reverse();
    r
}

/// A face of the skew cone at both doors, the flux lane and the material
/// sign: its area, or the refusal both doors give alike.
fn skew_doors<T: geom_core::Decide + geom_core::CertifiedEnclosure>(
    loops: &[Vec<LoopEdge<T>>],
) -> Result<f64, PropsError> {
    let refs: Vec<&[LoopEdge<T>]> = loops.iter().map(Vec::as_slice).collect();
    let side = boundary_material_sign_loops(&SkewCone::new().surface::<T>(), &refs, band());
    let flux = skew_face(loops);
    match (&flux, &side) {
        (Ok(_), Ok(_)) => {}
        (Err(a), Err(b)) => assert_eq!(a, b, "the doors refuse alike"),
        _ => panic!("the doors disagree: flux {flux:?}, side {side:?}"),
    }
    flux.map(|(area, _)| area)
}

/// **A band's holes are read as a cap's are** (`require_cone_holes`):
/// against the band's two rims together, and inside them in sum. On the
/// skew cone, band `[1, 2]`, at `f64` and at the interval scalar, at the
/// flux lane and the material sign alike:
///
/// - a window `[1.3, 1.6] × [1, 2]` wound against the band (clockwise)
///   measures the band less the window; wound with it, it is
///   [`PropsError::RingWoundWithFace`];
/// - a window `[3, 6] × [0.1, τ − 0.1]` wound against the band is a hole
///   that covers more than the band, [`PropsError::HolesExceedFace`];
///   wound with it, [`PropsError::RingWoundWithFace`].
#[test]
fn a_bands_holes_are_wound_against_it_and_inside_it() {
    fn rows<T: geom_core::Decide + geom_core::CertifiedEnclosure>(scalar: &str) {
        let c = SkewCone::new();
        let sa = 0.5_f64.sin();
        let band = || {
            vec![
                vec![c.rim::<T>(1.0, 0.0, TAU)],
                vec![c.rim::<T>(2.0, TAU, 0.0)],
            ]
        };
        let with_hole = |v: (f64, f64), u: (f64, f64), with: bool| {
            let mut l = band();
            l.push(c.window::<T>(v, u, with));
            skew_doors(&l)
        };
        // The band runs up the inner rim and back down the outer, so a
        // hole against it runs clockwise on the chart.
        let a = false;
        let want = sa * (core::f64::consts::PI * 3.0 - 0.5 * (1.6 * 1.6 - 1.3 * 1.3));
        let got = with_hole((1.3, 1.6), (1.0, 2.0), a)
            .unwrap_or_else(|e| panic!("band less a window at {scalar}: {e:?}"));
        assert!(
            (got - want).abs() <= 1e-9 * want,
            "band less window {got} vs {want} at {scalar}"
        );
        assert_eq!(
            with_hole((1.3, 1.6), (1.0, 2.0), !a),
            Err(PropsError::RingWoundWithFace),
            "a window wound with the band at {scalar}"
        );
        let wide = (0.1, TAU - 0.1);
        assert_eq!(
            with_hole((3.0, 6.0), wide, a),
            Err(PropsError::HolesExceedFace),
            "a hole past the band at {scalar}"
        );
        assert_eq!(
            with_hole((3.0, 6.0), wide, !a),
            Err(PropsError::RingWoundWithFace),
            "a hole past the band wound with it at {scalar}"
        );
    }
    rows::<f64>("f64");
    rows::<geom_core::Interval>("the interval scalar");
}

/// **A rim that winds the axis beside a loop through the apex refuses**
/// ([`PropsError::LoopsWindChart`]): the slit cap of slant 1 with the rim
/// at 2 wound against it, and the sector `[0, 1] × [0, 1]` with that rim,
/// on the skew cone, at `f64` and at the interval scalar, at both doors.
/// Each does bound one region — the band `[1, 2]` and the cap less the
/// sector — which the vector area would measure; the closed form reads
/// the windings of a face that reaches the apex only when no other loop
/// winds, and refuses the rest rather than read them.
#[test]
fn a_winding_rim_beside_a_loop_through_the_apex_refuses() {
    fn rows<T: geom_core::Decide + geom_core::CertifiedEnclosure>(scalar: &str) {
        let c = SkewCone::new();
        let slit = vec![
            c.generator::<T>(0.0, 0.0, 1.0),
            c.rim::<T>(1.0, 0.0, TAU),
            c.generator::<T>(TAU, 1.0, 0.0),
        ];
        let sector = vec![
            c.generator::<T>(0.0, 0.0, 1.0),
            c.rim::<T>(1.0, 0.0, 1.0),
            c.generator::<T>(1.0, 1.0, 0.0),
        ];
        for (name, inner) in [("slit cap", slit), ("sector", sector)] {
            for rim in [c.rim::<T>(2.0, 0.0, TAU), c.rim::<T>(2.0, TAU, 0.0)] {
                assert_eq!(
                    skew_doors(&[inner.clone(), vec![rim]]),
                    Err(PropsError::LoopsWindChart),
                    "a {name} beside a winding rim at {scalar}"
                );
            }
        }
    }
    rows::<f64>("f64");
    rows::<geom_core::Interval>("the interval scalar");
}

/// **A window that runs from near the apex measures its closed form**,
/// `½·sin α·(v₁² − v₀²)·Δu`, at `f64` and at the interval scalar: its
/// rim nearer the apex `kε` from the axis, its generators running out to
/// slant 2 on the skew cone, whose apex is off the origin. A generator's
/// azimuth read so near the axis rounds by `≈ u·|apex|/ρ`, and each edge
/// is charged its miss at its own nearest radius; charged at the loop's
/// widest, the interval scalar would escalate.
#[test]
fn a_window_from_near_the_apex_measures_its_closed_form() {
    fn rows<T: geom_core::Decide + geom_core::CertifiedEnclosure>(scalar: &str) {
        let c = SkewCone::new();
        let sa = 0.5_f64.sin();
        for k in [10.5, 30.0, 100.0] {
            let v0 = k * eps() / sa;
            let want = 0.5 * sa * (4.0 - v0 * v0);
            let got = skew_doors(&[c.window::<T>((v0, 2.0), (1.0, 2.0), true)])
                .unwrap_or_else(|e| panic!("window from {k}ε at {scalar}: {e:?}"));
            assert!(
                (got - want).abs() <= 1e-9 * want,
                "window from {k}ε: {got} vs {want} at {scalar}"
            );
        }
    }
    rows::<f64>("f64");
    rows::<geom_core::Interval>("the interval scalar");
}

/// **A metre spindle torus with a parallel near its axis measures its
/// closed form**, `r·Δu·(R·Δv + r·Δ sin v)`, at `f64` and at the
/// interval scalar: `R = 0.8`, `r = 1`, centred off the origin, the
/// parallel at `v₁` `kε` from the axis. Its meridians' stored turns are
/// carried to the core circle in the meridian's own plane; read in the
/// plane of a point so near the axis, that plane rounds by `u·|w|/ρ`,
/// which the core circle carries `R` times over, past the slack at the
/// interval scalar.
#[test]
fn a_metre_spindle_torus_near_its_axis_measures_its_closed_form() {
    fn face<T: geom_core::Decide + geom_core::CertifiedEnclosure>(
        o: [f64; 3],
        (big, r): (f64, f64),
        (v0, v1): (f64, f64),
    ) -> Result<f64, PropsError> {
        let parallel = |v: f64, u0: f64, u1: f64| {
            edge::<T>(
                Curve3::Circle {
                    center: p3(o[0], o[1], o[2] + r * v.sin()),
                    axis: v3(0.0, 0.0, 1.0),
                    radius: T::from_f64(big + r * v.cos()),
                    u_ref: v3(1.0, 0.0, 0.0),
                },
                u0,
                u1,
                0,
                0,
            )
        };
        let meridian = |u: f64, a: f64, b: f64| {
            edge::<T>(
                Curve3::Circle {
                    center: p3(o[0] + big * u.cos(), o[1] + big * u.sin(), o[2]),
                    axis: v3(u.sin(), -u.cos(), 0.0),
                    radius: T::from_f64(r),
                    u_ref: v3(u.cos(), u.sin(), 0.0),
                },
                a,
                b,
                0,
                0,
            )
        };
        let l = vec![
            parallel(v0, 0.0, 1.0),
            meridian(1.0, v0, v1),
            parallel(v1, 1.0, 0.0),
            meridian(0.0, v1, v0),
        ];
        let s = Surface::Torus {
            center: p3(o[0], o[1], o[2]),
            axis: v3(0.0, 0.0, 1.0),
            major_radius: T::from_f64(big),
            minor_radius: T::from_f64(r),
            u_ref: v3(1.0, 0.0, 0.0),
        };
        curved_face_loops(&s, &[&l], true, band()).map(|fc| {
            let (lo, hi) = fc.area.certified_bracket().expect("a certified area");
            assert!(hi - lo <= 1e-9 * hi, "bracket [{lo}, {hi}]");
            0.5 * (lo + hi)
        })
    }
    fn rows<T: geom_core::Decide + geom_core::CertifiedEnclosure>(scalar: &str) {
        let (o, big, r) = ([0.3, -0.2, 0.5], 0.8, 1.0);
        for k in [10.5, 11.5, 100.0] {
            let v1 = ((k * eps() - big) / r).acos();
            let want = r * (big * (v1 - 1.0) + r * (v1.sin() - 1.0_f64.sin()));
            let got = face::<T>(o, (big, r), (1.0, v1))
                .unwrap_or_else(|e| panic!("spindle at {k}ε at {scalar}: {e:?}"));
            assert!(
                (got - want).abs() <= 1e-9 * want,
                "spindle at {k}ε: {got} vs {want} at {scalar}"
            );
        }
    }
    rows::<f64>("f64");
    rows::<geom_core::Interval>("the interval scalar");
}

/// **A cylinder narrower than `Kε` whose rims the axis class admits
/// measures**: the band between two whole rims of radius `r_c` at
/// heights 0 and 1 on the cylinder of radius `R`, `(R, r_c) ∈ {(9.6ε,
/// 10.4ε), (9.95ε, 10.5ε)}`, at `f64` and at the interval scalar. Each
/// rim is within `ε` of the surface and its radius is past `Kε`, so the
/// axis passes inside it and its stored turn is carried to the axis
/// clear of the branch cut: the premise is the rim's, read at the rim's
/// radius, not the surface's. The face's area is the cylinder's,
/// `2π·R`. A rim nearer the axis is still refused, typed, by the axis
/// class (`props_circle_axis_class`): escalated in its band at `5ε`,
/// and within `ε` no rim ([`PropsError::NotIsoRectangle`]).
#[test]
fn a_cylinder_under_k_eps_with_admitted_rims_measures() {
    fn band_of<T: geom_core::Decide + geom_core::CertifiedEnclosure>(
        radius: f64,
        rim: f64,
    ) -> Result<f64, PropsError> {
        let (lower, upper) = (
            vec![off_arc::<T>(rim, [0.0, 0.0], 0.0, 0.0, (0.0, TAU))],
            vec![off_arc::<T>(rim, [0.0, 0.0], 0.0, 1.0, (TAU, 0.0))],
        );
        curved_face_loops(&cylinder_of::<T>(radius), &[&lower, &upper], true, band()).map(|fc| {
            let (lo, hi) = fc.area.certified_bracket().expect("a certified area");
            0.5 * (lo + hi)
        })
    }
    fn rows<T: geom_core::Decide + geom_core::CertifiedEnclosure>(scalar: &str) {
        let e = eps();
        for (r, rc) in [(9.6, 10.4), (9.95, 10.5)] {
            let got = band_of::<T>(r * e, rc * e)
                .unwrap_or_else(|err| panic!("R = {r}ε, r_c = {rc}ε at {scalar}: {err:?}"));
            let want = TAU * r * e;
            assert!(
                (got - want).abs() <= 1e-9 * want,
                "R = {r}ε, r_c = {rc}ε: {got:e} vs {want:e} at {scalar}"
            );
        }
        assert!(
            matches!(
                band_of::<T>(5.0 * e, 5.0 * e),
                Err(PropsError::Escalated { ref cause, .. })
                    if cause.predicate == Some("props_circle_axis_class")
            ),
            "a rim in the axis class's band at {scalar}"
        );
        assert_eq!(
            band_of::<T>(0.5 * e, 0.5 * e),
            Err(PropsError::NotIsoRectangle {
                what: "cylinder boundary circle is not a rim"
            }),
            "a rim within ε of the axis at {scalar}"
        );
    }
    rows::<f64>("f64");
    rows::<geom_core::Interval>("the interval scalar");
}

/// **A steep, wide frustum wall off the origin measures at every ε**:
/// the cone of half-angle 1.3 about the skew axis through
/// `(480, −320, 1200)`, its half wall `[v₀, v₁] × [0, π]` between rims of
/// radius 50 and 410 (the dimensions of `reach_cone_split`'s steep
/// frustum), measures `½·sin α·(v₁² − v₀²)·π` at `f64` and at the
/// interval scalar. At ε = 1e-12 the interval scalar re-derives each
/// point to `≈ 1e-12` here, so two readings that subtracted such points
/// escalated a legal face:
///
/// - the generators' incidence (`props_meridian_on_surface`), whose
///   radial sag formed `ℓ − δ` as a difference of two norms, though a
///   generator's ends are radially parallel and its sag is rounding
///   squared;
/// - the loop's whole turns (`props_chart_loop_whole`), which read each
///   edge's miss about the axis — a generator's short turn and a rim's
///   carried turn read back against the very ends they were read off.
#[test]
fn a_steep_wide_frustum_wall_off_the_origin_measures_at_every_eps() {
    fn rows<T: geom_core::Decide + geom_core::CertifiedEnclosure>(scalar: &str) {
        let alpha = 1.3_f64;
        let c = SkewCone::posed(alpha, [480.0, -320.0, 1200.0]);
        let sa = alpha.sin();
        let (v0, v1) = (50.0 / sa, 410.0 / sa);
        let want = 0.5 * sa * (v1 * v1 - v0 * v0) * core::f64::consts::PI;
        let wall = c.window::<T>((v0, v1), (0.0, core::f64::consts::PI), true);
        let (got, _) =
            face_on(&c, &[wall]).unwrap_or_else(|e| panic!("the wall at {scalar}: {e:?}"));
        assert!(
            (got - want).abs() <= 1e-9 * want,
            "the wall: {got} vs {want} at {scalar}"
        );
    }
    rows::<f64>("f64");
    rows::<geom_core::Interval>("the interval scalar");
}
