//! **A spiric cut far off and turned reads as it does at home.** The
//! torus `R = 2, r = 0.5` cut by the half-space `x ≥ 0.3` meets the
//! plane in a spiric: its caps are planes bounded by spirics and its
//! walls tori, each measured by quadrature. A rigid motion preserves
//! volume, and moving a body moves no face's width: every face's flux
//! is read about a vertex of its shell's first quadrature face
//! (`topo::props`' `shell_reference`), so the lever a width scales with
//! is the extent of the shell's quadrature faces wherever the body sits.
//!
//! Read about the world origin, the same cut carried `s = 100` along
//! `(8, −3, 5)` and turned by 0.7 rad about a skew axis stood ~990 m
//! off: each spiric face's vector area kept ~7e-11 of width per
//! component at the last round, the lever made the flux ~8.6e-8 wide,
//! and `T ∩ P`, `P ∩ T` and `T ∖ P` refused `QuadratureBudget` at every
//! ε (width 3.6e-8 against 1024ε = 1.024e-9 at 1e-12). Carried without
//! the turn, the faces' widths off their normals stayed zero, so the
//! translation alone built.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::finished;
use topo::AtRestBody;

use crate::common::operands::framed_bar;
use crate::germ_plane_torus_chord::half;
use crate::germ_plane_torus_frame::{six, torus};

/// The torus and the half-space carried `s` along `(8, −3, 5)` after a
/// turn of 0.7 rad about the skew axis `(0.3, −0.5, 0.8)` through the
/// origin.
fn posed(s: f64) -> (AtRestBody<f64>, AtRestBody<f64>) {
    let tol = Tol::witness();
    let axis = Vec3::new(0.3, -0.5, 0.8).normalize();
    let m = Affine3::translation(Vec3::new(8.0, -3.0, 5.0) * s)
        * Affine3::rotation_about_axis(Point3::origin(), axis, 0.7);
    let motion = geom_core::test_support::rigid(&m, tol);
    let mv = |b: &AtRestBody<f64>, who: &str| {
        finished(who, topo::transform_rigid(b, &motion, tol).unwrap(), tol)
    };
    (
        mv(&torus(2.0, 0.5), "the torus"),
        mv(
            &half(Point3::new(0.3, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)).body,
            "the half-space",
        ),
    )
}

/// Each op's result at `s`, built.
fn built(s: f64) -> Vec<(&'static str, AtRestBody<f64>)> {
    let (t, p) = posed(s);
    six(&t, &p)
        .into_iter()
        .map(|(label, got)| {
            let got = got.unwrap_or_else(|e| panic!("s = {s}, {label}: {e:?}"));
            let body = got
                .body()
                .unwrap_or_else(|| panic!("s = {s}, {label}: empty"))
                .body
                .clone();
            (label, body)
        })
        .collect()
}

/// Every op builds far off and turned, and measures the volume it
/// measures at home: the two enclosures overlap.
#[test]
fn every_op_far_off_and_turned_measures_its_volume_at_home() {
    let tol = Tol::witness();
    for ((label, home), (_, far)) in built(0.0).into_iter().zip(built(100.0)) {
        let at = |b: &AtRestBody<f64>, s: f64| {
            topo::mass_properties(b, tol)
                .unwrap_or_else(|e| panic!("s = {s}, {label}: the volume: {e:?}"))
        };
        let (home, far) = (at(&home, 0.0), at(&far, 100.0));
        assert!(
            (far.volume - home.volume).abs() <= far.volume_pad + home.volume_pad,
            "{label}: far off {} ± {} against at home {} ± {}",
            far.volume,
            far.volume_pad,
            home.volume,
            home.volume_pad,
        );
    }
}

/// Every quadrature face, a spiric cap and a torus wall alike, meets
/// the reporting target at the same round far off and turned as at
/// home: its width does not depend on where the body sits.
#[test]
fn every_spiric_face_meets_its_target_at_the_round_it_does_at_home() {
    let tol = Tol::witness();
    let rounds = |b: &AtRestBody<f64>| {
        let mut rounds: Vec<(String, Option<usize>)> =
            topo::test_support::quadrature_rounds(b, tol)
                .unwrap()
                .into_iter()
                .map(|(_, kind, round)| (format!("{kind:?}"), round))
                .collect();
        rounds.sort_unstable();
        rounds
    };
    for ((label, home), (_, far)) in built(0.0).into_iter().zip(built(100.0)) {
        let (home, far) = (rounds(&home), rounds(&far));
        for kind in ["Plane", "Torus"] {
            assert!(
                home.iter().any(|(k, _)| k == kind),
                "{label}: a {kind} face is measured by quadrature: {home:?}"
            );
        }
        assert!(
            home.iter().all(|(_, round)| round.is_some()),
            "{label}: every face meets its target at home: {home:?}"
        );
        assert_eq!(far, home, "{label}: the rounds far off and turned");
    }
}

/// **What a point of the shell buys: a far-off body's pads are its pads
/// at home.** Read about a vertex of the shell's first quadrature face,
/// a quadrature face's lever is at most the extent of the shell's
/// quadrature faces wherever the body sits, so a body carries the width
/// it carries at home. Read about the
/// world origin, this pose's spiric faces' widths grew with the
/// placement's 990 m, to 35 times the target at 1e-12, and three ops
/// refused. Far off and turned, every op's pad is its pad at home to the
/// rounding of the moved coordinates: measured at most 0.21% more at
/// 1e-12, 2e-6 relative at 1e-9 and 1e-6.
///
/// "At home" is `posed(0.0)`, turned and not moved, so the row compares
/// a turned body with a turned body. The same cut left on the axes reads
/// several times less (4.1e-6 against 2.0–2.8e-5 at 1e-9, `P ∪ T`), and
/// that is luck, not a lever: a face whose normal lies on an axis has
/// off-normal width enclosing exactly zero, which any turn ends. So an
/// axis-aligned pad is no baseline for a turned one.
#[test]
fn every_op_far_off_and_turned_keeps_its_pads_at_home() {
    let tol = Tol::witness();
    for ((label, home), (_, far)) in built(0.0).into_iter().zip(built(100.0)) {
        let pad = |b: &AtRestBody<f64>| topo::mass_properties(b, tol).unwrap().volume_pad;
        let (home, far) = (pad(&home), pad(&far));
        assert!(
            home > 0.0 && far <= home * 1.01,
            "{label}: the pad far off and turned {far:e} against at home {home:e}"
        );
    }
}

/// **One solid stored in two face orders carries nearly one pad.**
/// `T ∪ P` and `P ∪ T` are one solid in two face orders. Each face is read
/// about the start of its shell's first quadrature face, so which vertex
/// that is still depends on the stored order, but only among the
/// quadrature faces: the lever is their extent, not the shell's, and that
/// extent is unbounded (the row below). Measured:
/// with a 900 m bar, 1.04× apart at every ε row (4.27e-6 against 4.10e-6
/// at 1e-9); the cut far off and turned (s = 100), 1.11× at 1e-9, 1.10×
/// at 1e-12 and 1.55× at 1e-6. Each ratio is pinned with headroom, so an
/// order the rule starts reading across the shell turns it red. Read
/// about the shell's first face's start, the bar pair sat 14.3× apart at
/// 1e-9, 15.9× at 1e-12 and 911× at 1e-6.
#[test]
fn one_solid_in_two_face_orders_carries_nearly_one_pad() {
    let tol = Tol::witness();
    let (bar_bound, turned_bound) = match tol.eps() {
        1e-6 => (1.1, 1.7),
        1e-9 | 1e-12 => (1.1, 1.2),
        _ => {
            test_utils::vacuity::stood_down(
                "non-matrix eps",
                "the ratios are measured at the matrix rows",
            );
            return;
        }
    };
    let pad = |x: &AtRestBody<f64>, y: &AtRestBody<f64>| {
        let got = topo::union(x, y, tol).unwrap();
        topo::mass_properties(&got.body().unwrap().body, tol)
            .unwrap()
            .volume_pad
    };
    let apart = |x: &AtRestBody<f64>, y: &AtRestBody<f64>| {
        let (xy, yx) = (pad(x, y), pad(y, x));
        (xy.max(yx) / xy.min(yx), xy, yx)
    };
    let bar = finished(
        "a 900 m bar",
        framed_bar(
            Point3::new(0.3, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            0.0,
            900.0,
            12.0,
        ),
        tol,
    );
    let (t, p) = posed(100.0);
    for (what, (ratio, xy, yx), bound) in [
        ("the 900 m bar", apart(&torus(2.0, 0.5), &bar), bar_bound),
        ("the cut far off and turned", apart(&t, &p), turned_bound),
    ] {
        assert!(
            ratio <= bound,
            "{what}: T ∪ P pad {xy:e}, P ∪ T pad {yx:e}, {ratio} apart, past {bound}"
        );
    }
}

/// Pads of `body` with every shell's face list rotated by each `k`.
fn pads_over_rotations(body: &topo::Body<f64>) -> Vec<f64> {
    let tol = Tol::witness();
    let n = *topo::test_support::rotate_shell_faces(&mut body.clone(), 0)
        .iter()
        .max()
        .unwrap();
    (0..n)
        .map(|k| {
            let mut b = body.clone();
            topo::test_support::rotate_shell_faces(&mut b, k);
            topo::mass_properties(&b, tol).unwrap().volume_pad
        })
        .collect()
}

/// The torus at home and a small torus 900 m down one bar: one shell
/// whose quadrature faces lie a kilometre apart.
fn two_tori_on_a_bar() -> topo::Body<f64> {
    let tol = Tol::witness();
    let far = {
        let motion = geom_core::test_support::rigid(
            &Affine3::translation(Vec3::new(-900.27, 0.0, 0.0)),
            tol,
        );
        finished(
            "the far torus",
            topo::transform_rigid(&torus(0.2, 0.05), &motion, tol).unwrap(),
            tol,
        )
    };
    let tt = topo::union(&torus(2.0, 0.5), &far, tol)
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone();
    let bar = finished(
        "a 900.6 m bar",
        framed_bar(
            Point3::new(0.3, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            0.0,
            900.6,
            12.0,
        ),
        tol,
    );
    topo::union(&tt, &bar, tol)
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone()
        .into_body()
}

/// **Known: a shell's pad still depends on which quadrature face it
/// stores first.** The torus at home and a small torus 900 m down one
/// bar are one shell whose quadrature faces lie a kilometre apart: read
/// about the first quadrature face's start, a rotation of the stored face
/// list moves that point from one torus to the other, and the other's
/// walls are read across the bar. Measured across the 12 rotations: pads
/// 2.58× apart at 1e-9, 2.71× at 1e-12 and 90.2× at 1e-6, where about the
/// world origin they do not move. Pinned as known, from a floor the fix
/// clears to twice the measure, so the fix
/// (`work/flux/a-shells-pad-still-depends-on-which-quadrature-face-it-stores-first.md`)
/// turns this red on purpose: then assert the rotations within 2×. The
/// cut far off and turned (`P ∖ T`, one shell, 10 rotations) moves too,
/// within its quadrature faces' extent: 1.36×, 1.51× and 1.88×, each
/// pinned with headroom.
#[test]
fn a_shells_pad_moves_with_which_quadrature_face_it_stores_first_as_known() {
    let tol = Tol::witness();
    let ((floor, tori), turned) = match tol.eps() {
        1e-6 => ((30.0, 90.2), 2.15),
        1e-9 => ((2.0, 2.58), 1.6),
        1e-12 => ((2.0, 2.71), 1.75),
        _ => {
            test_utils::vacuity::stood_down(
                "non-matrix eps",
                "the ratios are measured at the matrix rows",
            );
            return;
        }
    };
    let ratio = |pads: &[f64]| {
        pads.iter().copied().fold(0.0, f64::max)
            / pads.iter().copied().fold(f64::INFINITY, f64::min)
    };
    let got = ratio(&pads_over_rotations(&two_tori_on_a_bar()));
    assert!(
        (floor..2.0 * tori).contains(&got),
        "two tori a kilometre apart: pads {got} apart across rotations, known at {tori}"
    );
    let (t, p) = posed(100.0);
    let cut = topo::subtract(&p, &t, tol)
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone()
        .into_body();
    let got = ratio(&pads_over_rotations(&cut));
    assert!(
        got <= turned,
        "the cut far off and turned: pads {got} apart across rotations, past {turned}"
    );
}

/// **The reference is the first face the dispatch sends to the
/// quadrature.** Each shell's point, read from the walk's side
/// (`topo::test_support::shell_references`), is the stored start of the
/// first face in the shell's order among the faces the walk actually
/// measured by quadrature (`quadrature_rounds`, the dispatch's side), at
/// every rotation of the stored face list, so a scan that skipped a kind
/// the dispatch sends to quadrature reads another point.
#[test]
fn each_shell_is_read_about_the_first_face_its_walk_measures_by_quadrature() {
    let tol = Tol::witness();
    let (t, p) = posed(100.0);
    let bodies = [
        (
            "T ∪ P at home",
            topo::union(
                &torus(2.0, 0.5),
                &half(Point3::new(0.3, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)).body,
                tol,
            ),
        ),
        ("P ∖ T far off and turned", topo::subtract(&p, &t, tol)),
    ];
    for (name, out) in bodies {
        let body = out.unwrap().body().unwrap().body.clone().into_body();
        let quadrature: Vec<topo::FaceKey> = topo::test_support::quadrature_rounds(&body, tol)
            .unwrap()
            .into_iter()
            .map(|(face, _, _)| face)
            .collect();
        let n = *topo::test_support::rotate_shell_faces(&mut body.clone(), 0)
            .iter()
            .max()
            .unwrap();
        for k in 0..n {
            let mut b = body.clone();
            topo::test_support::rotate_shell_faces(&mut b, k);
            for (shell, got) in topo::test_support::shell_references(&b) {
                let faces = &b.get_shell(shell).unwrap().faces;
                let first = faces
                    .iter()
                    .copied()
                    .find(|f| quadrature.contains(f))
                    .unwrap_or(faces[0]);
                let start = match b
                    .get_loop(b.get_face(first).unwrap().outer)
                    .unwrap()
                    .boundary
                {
                    topo::LoopBoundary::Empty { vertex } => vertex,
                    topo::LoopBoundary::Cycle { first } => b.get_half_edge(first).unwrap().start,
                };
                let want = b.vertex_points().find(|(v, _)| *v == start).unwrap().1;
                assert_eq!(
                    [got.x, got.y, got.z].map(f64::to_bits),
                    [want.x, want.y, want.z].map(f64::to_bits),
                    "{name}, rotation {k}: the shell is read about its first quadrature face's start"
                );
            }
        }
    }
}
