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
//! and `T ∩ P`, `P ∩ T` and `T ∖ P` stopped short of the resolution at every
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
            topo::mass_properties(b, tol, topo::test_support::pinned_resolution(tol))
                .unwrap_or_else(|e| panic!("s = {s}, {label}: the volume: {e:?}"))
        };
        let (home, far) = (at(&home, 0.0), at(&far, 100.0));
        assert!(
            (far.enclosure.volume - home.enclosure.volume).abs()
                <= far.enclosure.volume_pad + home.enclosure.volume_pad,
            "{label}: far off {} ± {} against at home {} ± {}",
            far.enclosure.volume,
            far.enclosure.volume_pad,
            home.enclosure.volume,
            home.enclosure.volume_pad,
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
            topo::test_support::quadrature_rounds(b, tol, topo::test_support::pinned_len(tol))
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
        let pad = |b: &AtRestBody<f64>| {
            topo::mass_properties(b, tol, topo::test_support::pinned_resolution(tol))
                .expect("the measured body")
                .met()
                .expect("the measured body")
                .volume_pad
        };
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
        topo::mass_properties(
            &got.body().unwrap().body,
            tol,
            topo::test_support::pinned_resolution(tol),
        )
        .expect("the measured body")
        .met()
        .expect("the measured body")
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

/// Pads of `body` with every shell's face list rotated by each `k`: each
/// rotation's volume enclosure's half-width, read at the pinned resolution
/// whatever the measurement's stop. These rows compare brackets, so a
/// rotation whose face stopped at its floor short of the resolution
/// (`Stop::Floor`, at 1e-12 on the two tori's bar) is read as the bracket
/// it states, not refused as a number unmet.
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
                topo::test_support::pinned_resolution(tol),
            )
            .expect("the measured body")
            .enclosure
            .volume_pad
        })
        .collect()
}

/// The torus at home and a small torus 900 m down one bar: one shell
/// whose quadrature faces lie a kilometre apart.
pub(crate) fn two_tori_on_a_bar() -> topo::Body<f64> {
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
/// best (`a-shells-pad-still-depends-on-which-quadrature-face-it-stores-first`, closed by PR 5699).
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

/// Tori `(R, r, centre)`, each about `y`, united in turn and then with the
/// 900.6 m bar of 12 × 12 from `x = 0.3` along `−x`: the two tori on a
/// bar's bar.
fn tori_on_the_bar(tori: &[(f64, f64, Vec3<f64>)]) -> topo::Body<f64> {
    let tol = Tol::witness();
    let placed = |&(big, small, at): &(f64, f64, Vec3<f64>)| {
        finished(
            "a torus on the bar",
            topo::transform_rigid(
                &torus(big, small),
                &geom_core::test_support::rigid(&Affine3::translation(at), tol),
                tol,
            )
            .unwrap(),
            tol,
        )
    };
    let union = |a: &AtRestBody<f64>, b: &AtRestBody<f64>| {
        topo::union(a, b, tol).unwrap().body().unwrap().body.clone()
    };
    let tori = tori
        .iter()
        .map(placed)
        .reduce(|a, b| union(&a, &b))
        .unwrap();
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
    union(&tori, &bar).into_body()
}

/// R1: the home torus and one on the bar's long top face near its far end,
/// `torus(2, 0.5)` at `(−895, 0, 5.7)`.
fn a_torus_on_the_bars_top_face() -> topo::Body<f64> {
    tori_on_the_bar(&[
        (2.0, 0.5, Vec3::new(0.0, 0.0, 0.0)),
        (2.0, 0.5, Vec3::new(-895.0, 0.0, 5.7)),
    ])
}

/// R2: the two tori on a bar and a third on the top face midway,
/// `torus(2, 0.5)` at `(−450, 0, 6.3)`.
fn a_torus_midway_on_the_top_face() -> topo::Body<f64> {
    tori_on_the_bar(&[
        (2.0, 0.5, Vec3::new(0.0, 0.0, 0.0)),
        (0.2, 0.05, Vec3::new(-900.27, 0.0, 0.0)),
        (2.0, 0.5, Vec3::new(-450.0, 0.0, 6.3)),
    ])
}

/// **A shell joined along a long face reads alike whichever face it
/// stores first.** A torus on the bar's long top face cuts it, so the top
/// face takes the quadrature and shares its straight edges with the end
/// caps the end tori cut, which take it too. Joined across those edges,
/// each body was one region spanning the bar, read about one point: R1,
/// the home torus and one on the top face near the far end, read pads
/// 456× apart across rotations at 1e-6, and R2, a torus midway with both
/// ends' tori, 83×
/// (`a-shells-pad-still-depends-on-face-order-where-one-quadrature-region-spans-the-lever`, closed by PR 5801).
/// Joined only across rims (`topo::props`' `References`), each torus is a
/// region with the faces its rims bound, read about its first curved
/// face's start. Across the 12 and 16 rotations the pads are within 1.006×
/// and 1.008× at every ε: R1 8.20e-6 at 1e-6 and 1e-9 and 7.29e-9 –
/// 7.33e-9 at 1e-12, R2 8.20e-6, 6.21e-6 and 5.60e-9 – 5.65e-9. Pinned
/// within 1.1×, and each below 1.1× its measure.
#[test]
fn a_shell_joined_along_a_long_face_reads_alike_whichever_face_it_stores_first() {
    let tol = Tol::witness();
    let (r1, r2) = match tol.eps() {
        1e-6 => (8.20e-6, 8.21e-6),
        1e-9 => (8.20e-6, 6.22e-6),
        1e-12 => (7.33e-9, 5.65e-9),
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
    for (what, body, measured) in [
        ("R1", a_torus_on_the_bars_top_face(), r1),
        ("R2", a_torus_midway_on_the_top_face(), r2),
    ] {
        let pads = pads_over_rotations(&body);
        let widest = pads.iter().copied().fold(0.0, f64::max);
        assert!(
            ratio(&pads) <= 1.1 && widest <= 1.1 * measured,
            "{what}: pads {pads:?}, {}× apart, widest past 1.1 × {measured:e}",
            ratio(&pads)
        );
    }
}

/// B5: the home torus, one astride the bar's end edge at `(−0.7, 0, 5)`,
/// cutting the end cap and the top face, and one on the top face near the
/// far end, `torus(2, 0.5)` each.
fn a_torus_astride_the_end_edge_and_one_far_on_top() -> topo::Body<f64> {
    tori_on_the_bar(&[
        (2.0, 0.5, Vec3::new(0.0, 0.0, 0.0)),
        (2.0, 0.5, Vec3::new(-0.7, 0.0, 5.0)),
        (2.0, 0.5, Vec3::new(-895.0, 0.0, 5.7)),
    ])
}

/// **The faces a circle joins stay regions of their own** (review of PR
/// 5801's B5). A torus astride the bar's end edge cuts the end cap and the
/// top face, and the home torus's walls meet each other along circles. A
/// region joined across those circles too spans the bar with the far
/// torus's: pads 186× apart across the 16 rotations at 1e-6. Joined only
/// across what `loop_vector_area` does not read, the pads are 2.52×,
/// 2.83× and 80.8× apart at 1e-6, 1e-9 and 1e-12, the widest 8.55e-5,
/// 3.35e-5 and 8.53e-7. What is left is the end-edge torus's long top
/// face read about a point off its plane
/// (`work/spiricarm/a-long-plane-face-cut-by-a-torus-reads-a-wide-pad-about-a-point-off-its-plane.md`).
/// Pinned within 1.1× of both.
#[test]
fn a_shell_whose_tori_meet_along_circles_keeps_each_torus_a_region() {
    let tol = Tol::witness();
    let (apart, widest) = match tol.eps() {
        1e-6 => (2.52, 8.55e-5),
        1e-9 => (2.83, 3.35e-5),
        1e-12 => (80.8, 8.53e-7),
        _ => {
            test_utils::vacuity::stood_down(
                "non-matrix eps",
                "the pads are measured at the matrix rows",
            );
            return;
        }
    };
    let pads = pads_over_rotations(&a_torus_astride_the_end_edge_and_one_far_on_top());
    let max = pads.iter().copied().fold(0.0, f64::max);
    let ratio = max / pads.iter().copied().fold(f64::INFINITY, f64::min);
    assert!(
        ratio <= 1.1 * apart && max <= 1.1 * widest,
        "B5: pads {pads:?}, {ratio}× apart (pinned {apart}×), widest {max:e} (pinned {widest:e})"
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

/// The two tori on a bar's exact volume: the bar's `900.6 × 12 × 12`, the
/// home torus's part past `x = 0.3`, and the far torus's part past the
/// bar's far end, each torus `R, r` about `y` cut by a plane `x = a` off
/// its axis in `∫ 2√(r² − (s − R)²) · s · 2 acos(a/s) ds` over its tube,
/// read to 40 digits (mpmath) over the stored `f64` ends, `0.3 − 900.6`
/// and `−900.27`.
const TWO_TORI_ON_A_BAR: f64 = 129_690.866_149_077_57;

/// **A region's carry moves the volume by what it says, with its sign.**
/// The far torus's region is read about its own point and carried to the
/// shell's by `(c_g − c)·A⃗(∂g)`, `∂g` the far end cap's square: about
/// `−900.6 × −144 ≈ 1.3e5`, the bar's whole volume. The carried reading
/// holds the body's exact volume within its own pad, 4.1e-6 at 1e-9 and
/// 1e-6 and 3.7e-9 at 1e-12, off it by 8.7e-8 and 2.9e-11: a carry one
/// part in 2e9 off (6.5e-5) cannot hold it at any ε, and one of the wrong
/// sign moves the volume by two thirds of it.
#[test]
fn a_regions_carry_moves_the_volume_by_what_it_says_with_its_sign() {
    use geom_core::Bounds as _;
    let tol = Tol::witness();
    let body = two_tori_on_a_bar();
    let (_, carry) = far_region(&body);
    let carried = topo::mass_properties(
        &finished("the tori", body, tol),
        tol,
        topo::test_support::pinned_resolution(tol),
    )
    .unwrap();
    assert!(
        (carried.enclosure.volume - TWO_TORI_ON_A_BAR).abs() <= carried.enclosure.volume_pad,
        "carried {} ± {} against the exact {TWO_TORI_ON_A_BAR}",
        carried.enclosure.volume,
        carried.enclosure.volume_pad,
    );
    let moved = 2.0 * carry.lo().abs().min(carry.hi().abs()) / 3.0;
    assert!(
        moved > 1e6 * carried.enclosure.volume_pad,
        "a carry of the wrong sign moves the volume by {moved}, against {} of pad",
        carried.enclosure.volume_pad
    );
}

/// A bar whose far end is a roof, two planes meeting in a ridge along
/// `y`, and a small torus about `y` whose hole the ridge runs through, so
/// it crosses both roof planes and touches no edge: one region of the
/// torus's walls and both roof faces, whose boundary `∂g` lies in two
/// planes.
fn a_torus_astride_a_roof() -> topo::Body<f64> {
    use geom_core::{Point2, RigidMotion};
    let tol = Tol::witness();
    let ridge = -900.3;
    let bar = finished(
        "a bar with a roof at its far end",
        sweep::test_support::prism_on(
            profile::SketchPlane::from_frame(RigidMotion::axes_zx(Point3::new(0.0, -6.0, 0.0))),
            vec![
                (Point2::new(-6.0, 0.3), 0.0),
                (Point2::new(-6.0, ridge + 6.0), 0.0),
                (Point2::new(0.0, ridge), 0.0),
                (Point2::new(6.0, ridge + 6.0), 0.0),
                (Point2::new(6.0, 0.3), 0.0),
            ],
            12.0,
            tol,
        ),
        tol,
    );
    let astride = finished(
        "the torus astride the ridge",
        topo::transform_rigid(
            &torus(0.2, 0.05),
            &geom_core::test_support::rigid(
                &Affine3::translation(Vec3::new(ridge - 0.05, 0.0, 0.0)),
                tol,
            ),
            tol,
        )
        .unwrap(),
        tol,
    );
    let tt = topo::union(&torus(2.0, 0.5), &astride, tol)
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone();
    topo::union(&tt, &bar, tol)
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone()
        .into_body()
}

/// **A region whose boundary leaves one plane is carried about its own
/// first face's start.** The torus astride the roof's ridge joins both
/// roof faces into one far region, whose faces' starts lie on two planes:
/// a carry read about any other point of the region than the one its
/// faces are read about moves the volume by that point's offset along
/// `A⃗(∂g)`, which no point of one plane escapes on a region of one cap.
/// Read so, the body agrees with every face read about the shell's one
/// point within the two readings' pads.
#[test]
fn a_region_bounded_off_one_plane_is_carried_about_its_own_point() {
    let tol = Tol::witness();
    let body = a_torus_astride_a_roof();
    let regions = topo::test_support::region_carries_of(&body);
    let [(_, regions)] = &regions[..] else {
        panic!("the roofed bar is one shell: {regions:?}");
    };
    let [(_, None), (far, Some(_))] = &regions[..] else {
        panic!("one shell of two regions, the second carried: {regions:?}");
    };
    let planes: Vec<topo::FaceKey> = far
        .iter()
        .copied()
        .filter(|&f| {
            let face = body.get_face(f).unwrap();
            matches!(
                body.get_surface(face.surface).unwrap(),
                geom::Surface::Plane { .. }
            )
        })
        .collect();
    assert_eq!(planes.len(), 2, "both roof faces are in the far region");
    let carried = topo::mass_properties(
        &finished("the roofed bar", body.clone(), tol),
        tol,
        topo::test_support::pinned_resolution(tol),
    )
    .unwrap();
    let one = topo::test_support::mass_properties_one_point(
        &body,
        tol,
        topo::test_support::pinned_len(tol),
    )
    .unwrap();
    assert!(
        (carried.enclosure.volume - one.volume).abs()
            <= carried.enclosure.volume_pad + one.volume_pad,
        "carried {} ± {} against one point's {} ± {}",
        carried.enclosure.volume,
        carried.enclosure.volume_pad,
        one.volume,
        one.volume_pad
    );
}

/// **A carried region at the certified scalar.** Two tori `R = 1,
/// r = 0.25` about `y`, built at `Interval` (dyadic, so every datum is
/// exact), on a `101 × 1 × 3` bar whose ends cut each: the far torus's
/// region is carried, its carry an enclosure rather than a point, and the
/// door's volume holds the exact `303 + π²/8` (the bar, plus one torus's
/// part past its end and the other's all but that part, which is one
/// whole torus, `2π²Rr²`).
#[test]
fn a_carried_region_at_the_certified_scalar_holds_its_volume() {
    use geom_core::{Bounds as _, Interval, Real as _};
    let tol = Tol::witness();
    let at = |b: topo::Body<Interval>| sweep::test_support::at_rest(&b, tol);
    let iv = Interval::from_f64;
    let ring = {
        let lp = profile::test_support::bulge_loop(vec![
            (geom_core::Point2::new(iv(1.0), iv(-0.25)), iv(1.0)),
            (geom_core::Point2::new(iv(1.0), iv(0.25)), iv(1.0)),
        ]);
        let vp = profile::Profile::new(profile::SketchPlane::<Interval>::xy(), vec![lp])
            .validate(tol)
            .unwrap();
        let axis = sweep::RevolveAxis {
            origin: geom_core::Point2::new(iv(0.0), iv(0.0)),
            dir: geom_core::Vec2::new(iv(0.0), iv(1.0)),
        };
        at(sweep::revolve(&vp, axis, sweep::Revolution::Full, tol)
            .unwrap()
            .body)
    };
    let far = at(topo::transform_rigid(
        &ring,
        &geom_core::test_support::rigid(
            &Affine3::translation(Vec3::new(iv(-101.0), iv(0.0), iv(0.0))),
            tol,
        ),
        tol,
    )
    .unwrap());
    let bar = at(sweep::test_support::brick(
        (-100.5, 0.5),
        (-0.5, 0.5),
        (-1.5, 1.5),
        tol,
    ));
    let tt = topo::union(&ring, &far, tol)
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone();
    let body = topo::union(&tt, &bar, tol)
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone();
    let regions = topo::test_support::region_carries_of(&body.clone().into_body());
    let [(_, regions)] = &regions[..] else {
        panic!("the bar is one shell: {regions:?}");
    };
    let [(_, None), (_, Some((_, carry)))] = &regions[..] else {
        panic!("one shell of two regions, the second carried: {regions:?}");
    };
    assert!(
        carry.lo() < carry.hi(),
        "the carry is an enclosure, not a point: {carry:?}"
    );
    let truth = 303.0 + std::f64::consts::PI * std::f64::consts::PI / 8.0;
    let volume = topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol))
        .expect("the measured body")
        .met()
        .expect("the measured body")
        .volume;
    assert!(
        volume.lo() <= truth && truth <= volume.hi(),
        "{volume:?} holds {truth}"
    );
}

/// **Every door reads a carried region about its own point.** Each walk a
/// door enters on the two tori on a bar (`topo::test_support::every_door`:
/// the reporting walk, the target walk and its continuation, a sign walk
/// settled at round 0 and its continuation, the sign walk refined past the
/// target and its interval re-derivation, each shell's role walk) and the
/// doors themselves (`mass_properties`, the raw measurement door, tier 3)
/// read every face of the far region about the region's point and every
/// other face about the shell's. A walk reads every face, and its
/// continuation every face it resumes: the round-0 sign walk's
/// continuation resumes some of the far region's faces and not others, so
/// a continuation that read only the faces it resumed as the door's
/// whole would hold part of the region and carry none. Each walk's
/// enclosure overlaps every other's, and each shell's role walk reads the
/// shell as an outer one.
#[test]
fn every_door_reads_a_carried_region_about_its_own_point() {
    let tol = Tol::witness();
    let body = two_tori_on_a_bar();
    let (far, _) = far_region(&body);
    let faces = body.faces().count();
    let at_rest = finished("the tori", body.clone(), tol);
    let about = |door: &str, reads: &[(topo::FaceKey, bool)]| {
        assert!(!reads.is_empty(), "{door} read no face");
        for &(face, regional) in reads {
            assert_eq!(
                regional,
                far.contains(&face),
                "{door}: {face:?} read about the wrong point"
            );
        }
    };
    let whole = |door: &str, reads: &[(topo::FaceKey, bool)]| {
        about(door, reads);
        for face in &far {
            assert!(
                reads.iter().any(|(f, _)| f == face),
                "{door} never read the far region's {face:?}"
            );
        }
    };
    let (_, reads) = topo::test_support::read_points::recorded(&at_rest, || {
        topo::mass_properties(&at_rest, tol, topo::test_support::pinned_resolution(tol)).unwrap()
    });
    whole("mass_properties", &reads);
    let (_, reads) = topo::test_support::read_points::recorded(&body, || {
        // Raw door: the door itself is the subject, on a body tier 3 passes: it reads one point per region as the typed door does.
        topo::test_support::measure_unvalidated(
            &body,
            tol,
            topo::test_support::pinned_resolution(tol),
        )
        .unwrap()
    });
    whole("the raw measurement door", &reads);
    let (_, reads) = topo::test_support::read_points::recorded(&body, || {
        topo::validate_geometric(&body, tol).unwrap()
    });
    whole("tier 3", &reads);
    let doors =
        topo::test_support::every_door(&body, tol, topo::test_support::pinned_resolution(tol))
            .unwrap();
    let mut spans = Vec::new();
    for door in &doors {
        let name = door.door;
        match name {
            "the target walk's continuation" | "its continuation to the target" => {
                if !door.reads.is_empty() {
                    about(name, &door.reads);
                }
            }
            _ => whole(name, &door.reads),
        }
        match &door.outcome {
            topo::test_support::DoorOutcome::Span(Some(span)) => spans.push((name, *span)),
            topo::test_support::DoorOutcome::Span(None) => {}
            topo::test_support::DoorOutcome::Role(role) => assert_eq!(
                role,
                &Ok(topo::ShellRole::Outer),
                "{name}: the bar's one shell is outer"
            ),
        }
    }
    assert!(
        doors.iter().filter(|d| d.reads.len() >= faces).count() >= 6,
        "the walks read every face: {doors:?}"
    );
    // At 1e-12 round 0 leaves two of the far region's three faces open, and
    // the continuation resumes those alone; at 1e-9 and 1e-6 it meets the
    // target on every face.
    let resumed = doors
        .iter()
        .find(|d| d.door == "its continuation to the target")
        .map(|d| {
            far.iter()
                .filter(|f| d.reads.iter().any(|(r, _)| r == *f))
                .count()
        });
    if tol.eps() == 1e-12 {
        assert_eq!(
            resumed,
            Some(2),
            "the continuation resumes part of the far region"
        );
    }
    for (a, (alo, ahi)) in &spans {
        for (b, (blo, bhi)) in &spans {
            assert!(
                alo <= bhi && blo <= ahi,
                "{a} [{alo}, {ahi}] and {b} [{blo}, {bhi}] do not overlap"
            );
        }
    }
}

/// **A shell whose quadrature regions all lie near its point reads as
/// about one point, bit for bit**: no region is carried. Each op on the
/// torus and the half-space far off and turned, and their union at home,
/// measures to the bits the one-point reading gives: one torus cut by the
/// plane, whose walls (joined to each other only across their seam
/// circles) are regions of their own, and of their caps. Both readings
/// are this code's (`mass_properties` and
/// `topo::test_support::mass_properties_one_point`), not the code before
/// regions: that the one-point reading is that code's bits is what the
/// differential over the sweep and step-import suites against the merge
/// base shows, not this row.
#[test]
fn a_shell_whose_regions_all_lie_near_its_point_reads_as_about_one_point_bit_for_bit() {
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
                regions.iter().all(|(_, carry)| carry.is_none()),
                "{label}: a region is carried: {regions:?}"
            );
        }
        let carried =
            topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol)).unwrap();
        let one = topo::test_support::mass_properties_one_point(
            &body,
            tol,
            topo::test_support::pinned_len(tol),
        )
        .unwrap();
        assert_eq!(
            [carried.enclosure.volume, carried.enclosure.volume_pad].map(f64::to_bits),
            [one.volume, one.volume_pad].map(f64::to_bits),
            "{label}"
        );
    }
}

/// **A region near its shell's point is not carried.** The two tori on a
/// bar 3.6 m long rather than 900.6 m: the far torus's region lies about
/// 3.3 m from the home torus's point, 13 of its own extents, short of the
/// 16 a carry asks (`topo::props`' `far_from`), where the carry's own
/// rounding outweighs what the shorter lever saves. Its cap counts by the
/// point's distance off its plane, which is nothing: at 10.6 m the region
/// is carried, and narrows the pad from the one-point reading's 4.32e-6,
/// 1.04e-8 and 4.54e-6 to 4.10e-6, 3.68e-9 and 4.10e-6 at 1e-9, 1e-12
/// and 1e-6. So here it is read about the shell's point, and the body
/// measures to the bits the one-point reading gives.
#[test]
fn a_region_near_its_shells_point_is_read_about_it_bit_for_bit() {
    let tol = Tol::witness();
    let near = finished(
        "the small torus 3 m down the bar",
        topo::transform_rigid(
            &torus(0.2, 0.05),
            &geom_core::test_support::rigid(&Affine3::translation(Vec3::new(-3.27, 0.0, 0.0)), tol),
            tol,
        )
        .unwrap(),
        tol,
    );
    let tt = topo::union(&torus(2.0, 0.5), &near, tol)
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone();
    let bar = finished(
        "a 3.6 m bar",
        framed_bar(
            Point3::new(0.3, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            0.0,
            3.6,
            12.0,
        ),
        tol,
    );
    let body = topo::union(&tt, &bar, tol)
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone();
    let regions = topo::test_support::region_carries_of(&body.clone().into_body());
    let [(_, regions)] = &regions[..] else {
        panic!("the short bar is one shell: {regions:?}");
    };
    let [(_, None), (_, None)] = &regions[..] else {
        panic!("one shell of two regions, neither carried: {regions:?}");
    };
    let carried =
        topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol)).unwrap();
    let one = topo::test_support::mass_properties_one_point(
        &body.into_body(),
        tol,
        topo::test_support::pinned_len(tol),
    )
    .unwrap();
    assert_eq!(
        [carried.enclosure.volume, carried.enclosure.volume_pad].map(f64::to_bits),
        [one.volume, one.volume_pad].map(f64::to_bits),
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
        let quadrature: Vec<topo::FaceKey> =
            topo::test_support::quadrature_rounds(&body, tol, topo::test_support::pinned_len(tol))
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
