//! **A spiric cut far off and turned reads as it does at home.** The
//! torus `R = 2, r = 0.5` cut by the half-space `x ≥ 0.3` meets the
//! plane in a spiric: its caps are planes bounded by spirics and its
//! walls tori, each measured by quadrature. A rigid motion preserves
//! volume, and moving a body moves no face's width: every face's flux
//! is read about a vertex of its shell's first quadrature face
//! (`topo::props`' `shell_reference`), or of its quadrature region's first
//! face where the shell has more than one region (`topo::props`'
//! `References`), so the lever a width scales with is the extent of a
//! region's quadrature faces wherever the body sits.
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
            topo::mass_properties(
                &topo::test_support::finished("the measured body", topo::Body::clone(&b), tol),
                tol,
            )
            .unwrap()
            .volume_pad
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

/// **A shell's pad reads alike whichever quadrature face it stores
/// first.** The torus at home and a small torus 900 m down one bar are one
/// shell with two quadrature regions, each torus's walls and the bar's
/// end cap they cut, joined only through the bar's closed-form sides.
/// Read about one point per shell, a rotation of the stored face list
/// moved that point from one torus to the other and read the other's
/// faces across the bar: pads 2.58× apart across the 12 rotations at 1e-9,
/// 2.71× at 1e-12 and 90.2× at 1e-6, from 2.28e-5, 2.14e-8 and 4.14e-5 at
/// best (`work/flux/a-shells-pad-still-depends-on-which-quadrature-face-it-stores-first.md`).
/// Each region read about its own point and carried to the shell's by its
/// boundary's vector area (`topo::props`' `References`), the pads are
/// 4.10e-6, 3.69e-9 – 3.74e-9 and 4.10e-6, within 1.02× across the
/// rotations at every ε: pinned within 1.1×, and each below 1.1× its
/// measure, so a carry that brought the width back would turn it red. The
/// cut far off and turned (`P ∖ T`, one shell of one region, 10 rotations)
/// moves within its quadrature faces' extent, as before: 1.36×, 1.51× and
/// 1.88×, each pinned with headroom.
#[test]
fn a_shells_pad_reads_alike_whichever_quadrature_face_it_stores_first() {
    let tol = Tol::witness();
    let (pin, turned) = match tol.eps() {
        1e-6 => (4.52e-6, 2.15),
        1e-9 => (4.52e-6, 1.6),
        1e-12 => (4.12e-9, 1.75),
        _ => {
            test_utils::vacuity::stood_down(
                "non-matrix eps",
                "the pads are measured at the matrix rows",
            );
            return;
        }
    };
    let ratio = |pads: &[f64]| {
        pads.iter().copied().fold(0.0, f64::max)
            / pads.iter().copied().fold(f64::INFINITY, f64::min)
    };
    let pads = pads_over_rotations(&two_tori_on_a_bar());
    let widest = pads.iter().copied().fold(0.0, f64::max);
    assert!(
        ratio(&pads) <= 1.1 && widest <= pin,
        "two tori a kilometre apart: pads {pads:?}, {}× apart, past {pin}",
        ratio(&pads)
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

/// The two tori on a bar's one shell, its two quadrature regions, and the
/// far one's carry.
fn far_region(body: &topo::Body<f64>) -> (Vec<topo::FaceKey>, geom_core::Interval) {
    let regions = topo::test_support::region_carries_of(body);
    let [(_, regions)] = &regions[..] else {
        panic!("the two tori on a bar are one shell: {regions:?}");
    };
    let [(_, None), (far, Some((_, carry)))] = &regions[..] else {
        panic!("one shell of two regions, the second carried: {regions:?}");
    };
    (far.clone(), *carry)
}

/// **A region's carry moves the volume by what it says, with its sign.**
/// The far torus's region is read about its own point and carried to the
/// shell's by `(c_g − c)·A⃗(∂g)`, `∂g` the far end cap's square: about
/// `−900.6 × −144 ≈ 1.3e5`, the bar's whole volume three times over. Read
/// with every face about the shell's one point instead
/// (`mass_properties_one_point`), the same body's volume agrees within the
/// two readings' pads, so a carry of the wrong sign, which would move the
/// volume by two thirds of it, cannot hold.
#[test]
fn a_regions_carry_moves_the_volume_by_what_it_says_with_its_sign() {
    use geom_core::Bounds as _;
    let tol = Tol::witness();
    let body = two_tori_on_a_bar();
    let (_, carry) = far_region(&body);
    let carried = topo::mass_properties(&finished("the tori", body.clone(), tol), tol).unwrap();
    let one = topo::test_support::mass_properties_one_point(&body, tol).unwrap();
    let slack = carried.volume_pad + one.volume_pad;
    assert!(
        (carried.volume - one.volume).abs() <= slack,
        "carried {} ± {} against one point's {} ± {}",
        carried.volume,
        carried.volume_pad,
        one.volume,
        one.volume_pad
    );
    let moved = 2.0 * carry.lo().abs().min(carry.hi().abs()) / 3.0;
    assert!(
        moved > 1e6 * slack,
        "a carry of the wrong sign moves the volume by {moved}, against {slack} of pads"
    );
}

/// **Every door reads a carried region about its own point.** Each walk a
/// door enters on the two tori on a bar — the reporting walk, the target
/// walk and its continuation, the sign walk refined past the target and
/// its interval re-derivation, each shell's role walk
/// (`topo::test_support::every_door`), and the doors themselves
/// (`mass_properties`, the raw measurement door, tier 3) — reads every
/// face of the far region about the region's point and every other face
/// about the shell's, and their enclosures overlap.
#[test]
fn every_door_reads_a_carried_region_about_its_own_point() {
    let tol = Tol::witness();
    let body = two_tori_on_a_bar();
    let (far, _) = far_region(&body);
    let at_rest = finished("the tori", body.clone(), tol);
    let check = |door: &str, reads: &[(topo::FaceKey, bool)]| {
        assert!(!reads.is_empty(), "{door} read no face");
        for &(face, regional) in reads {
            assert_eq!(
                regional,
                far.contains(&face),
                "{door}: {face:?} read about the wrong point"
            );
        }
        for face in &far {
            assert!(
                reads.iter().any(|(f, _)| f == face),
                "{door} never read the far region's {face:?}"
            );
        }
    };
    let (_, reads) = topo::test_support::read_points::recorded(&at_rest, || {
        topo::mass_properties(&at_rest, tol).unwrap()
    });
    check("mass_properties", &reads);
    let (_, reads) = topo::test_support::read_points::recorded(&body, || {
        topo::test_support::measure_unvalidated(&body, tol).unwrap()
    });
    check("the raw measurement door", &reads);
    let (_, reads) = topo::test_support::read_points::recorded(&body, || {
        topo::validate_geometric(&body, tol).unwrap()
    });
    check("tier 3", &reads);
    let (doors, reads) = topo::test_support::read_points::recorded(&body, || {
        topo::test_support::every_door(&body, tol).unwrap()
    });
    check("every walk", &reads);
    let spans: Vec<(&str, (f64, f64))> = doors
        .into_iter()
        .filter_map(|(door, span)| span.map(|s| (door, s)))
        .collect();
    for (a, (alo, ahi)) in &spans {
        for (b, (blo, bhi)) in &spans {
            assert!(
                alo <= bhi && blo <= ahi,
                "{a} [{alo}, {ahi}] and {b} [{blo}, {bhi}] do not overlap"
            );
        }
    }
}

/// **A shell of one quadrature region reads as about one point, bit for
/// bit**: its region's point is the shell's, so it carries nothing. Each op
/// on the torus and the half-space far off and turned, and their union at
/// home, measures to the bits the one-point reading gives.
#[test]
fn a_shell_of_one_quadrature_region_reads_as_about_one_point_bit_for_bit() {
    let tol = Tol::witness();
    let home = topo::union(
        &torus(2.0, 0.5),
        &half(Point3::new(0.3, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)).body,
        tol,
    )
    .unwrap()
    .body()
    .unwrap()
    .body
    .clone();
    for (label, body) in built(100.0).into_iter().chain([("T ∪ P at home", home)]) {
        for (_, regions) in topo::test_support::region_carries_of(&body) {
            assert!(
                regions.len() <= 1,
                "{label}: a shell of {} regions",
                regions.len()
            );
        }
        let carried = topo::mass_properties(&body, tol).unwrap();
        let one = topo::test_support::mass_properties_one_point(&body, tol).unwrap();
        assert_eq!(
            [carried.volume, carried.volume_pad].map(f64::to_bits),
            [one.volume, one.volume_pad].map(f64::to_bits),
            "{label}"
        );
    }
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
