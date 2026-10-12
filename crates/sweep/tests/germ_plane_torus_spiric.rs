//! **The plane × torus chord for the spiric ovals, through the public
//! ops** (`docs/GERM-TORUS-PLANE-FRAME-SPEC.md`, U-T4): a plane parallel
//! to the torus's axis, short of the inner equator, cuts two spiric
//! ovals, and the join takes an arc of the oval its chord's sites lie
//! on, on the oval's own parameter.
//!
//! Every op of every pose either builds its true body, held to the truth
//! three ways (its volume against a closed-form oracle of the analytic
//! operands, itself held to a Monte Carlo of them; tier 3; and
//! `point_in_solid` at sampled points against the analytic memberships
//! under the op), or stops past the join at one of two doors, each named
//! per op and filed on its owner:
//!
//! - the result's shells are sorted by `point_in_solid`, which refuses a
//!   torus face that is not a chart rectangle
//!   (`work/inside/point-in-solid-refuses-an-l-shaped-torus-face.md`);
//! - the result's volume does not read a torus face holding a ring
//!   bounded by ovals
//!   (`work/flux/a-torus-face-with-a-spiric-ring-has-no-volume-lane.md`).
//!
//! No op returns a wrong body. The poses run on the donut, the one-face
//! torus (its vertex at three phases), the torus seamed on `y = 0`, and,
//! for the corner bar's lens, a fat torus and its one-face and seamed
//! spellings: two loops of one section can lie on one face pair.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::germ_plane_torus_chord::{
    Solid, about_the_tube, boxed, contains, half, kept, meet_is_the_monte_carlo, ring,
    tori_of_one_face_pair, volume_and_points, volume_tol,
};
use crate::germ_plane_torus_frame::{SIX, corner_bar, one_face_torus, seamed_torus, six, torus};
use core::f64::consts::PI;
use geom_core::{Point3, Vec3};
use topo::{AtRestBody, BooleanError};

/// `∫ √(ρ² − x²) dx` from `0` to `x`, `|x| ≤ ρ`.
fn under_circle(rho: f64, x: f64) -> f64 {
    let x = x.clamp(-rho, rho);
    0.5 * (x * (rho * rho - x * x).max(0.0).sqrt() + rho * rho * (x / rho).asin())
}

/// The area of the disc of radius `rho` about the origin of the
/// `(x, z)` plane inside the rectangle `xs × zs`, in closed form: the
/// span of `x` is cut where the circle crosses `z = z0` or `z = z1`, and
/// on each piece the disc's chord in `z`, clipped to `zs`, is a constant
/// or `±√(ρ² − x²)`, each integrated exactly.
fn disc_in_rect(rho: f64, (x0, x1): (f64, f64), (z0, z1): (f64, f64)) -> f64 {
    let (a, b) = (x0.max(-rho), x1.min(rho));
    if a >= b {
        return 0.0;
    }
    let mut cuts = vec![a, b];
    for z in [z0, z1] {
        if z.abs() < rho {
            let x = (rho * rho - z * z).sqrt();
            cuts.extend([-x, x].into_iter().filter(|&x| a < x && x < b));
        }
    }
    cuts.sort_by(f64::total_cmp);
    let arc = |u: f64, v: f64| under_circle(rho, v) - under_circle(rho, u);
    cuts.windows(2)
        .map(|w| {
            let (u, v) = (w[0], w[1]);
            let m = 0.5 * (u + v);
            let h = (rho * rho - m * m).sqrt();
            let (top, bottom) = (h.min(z1), (-h).max(z0));
            if top <= bottom {
                return 0.0;
            }
            let top = if h < z1 { arc(u, v) } else { z1 * (v - u) };
            let bottom = if -h > z0 { -arc(u, v) } else { z0 * (v - u) };
            top - bottom
        })
        .sum()
}

/// Adaptive Simpson on `[a, b]` to `tol`.
fn simpson(f: &dyn Fn(f64) -> f64, a: f64, b: f64, tol: f64) -> f64 {
    #[allow(clippy::too_many_arguments)]
    fn rec(
        f: &dyn Fn(f64) -> f64,
        a: f64,
        b: f64,
        fa: f64,
        fm: f64,
        fb: f64,
        whole: f64,
        tol: f64,
        depth: u32,
    ) -> f64 {
        let m = 0.5 * (a + b);
        let (lm, rm) = (0.5 * (a + m), 0.5 * (m + b));
        let (flm, frm) = (f(lm), f(rm));
        let left = (m - a) / 6.0 * (fa + 4.0 * flm + fm);
        let right = (b - m) / 6.0 * (fm + 4.0 * frm + fb);
        if depth == 0 || (left + right - whole).abs() <= 15.0 * tol {
            return left + right + (left + right - whole) / 15.0;
        }
        rec(f, a, m, fa, flm, fm, left, tol / 2.0, depth - 1)
            + rec(f, m, b, fm, frm, fb, right, tol / 2.0, depth - 1)
    }
    let (fa, fb, fm) = (f(a), f(b), f(0.5 * (a + b)));
    rec(
        f,
        a,
        b,
        fa,
        fm,
        fb,
        (b - a) / 6.0 * (fa + 4.0 * fm + fb),
        tol,
        40,
    )
}

/// The volume of the ring torus `(R, r)` about `y` through the origin
/// inside the box `xs × ys × zs`: at height `y` the tube's section is the
/// annulus `R ± s`, `s = √(r² − y²)`, whose area inside the rectangle is
/// two discs' ([`disc_in_rect`]), integrated over `y = r·sin θ`.
pub(crate) fn torus_in_box(
    (big_r, r): (f64, f64),
    xs: (f64, f64),
    (y0, y1): (f64, f64),
    zs: (f64, f64),
) -> f64 {
    let (lo, hi) = (y0.max(-r), y1.min(r));
    if lo >= hi {
        return 0.0;
    }
    let slice = |theta: f64| {
        let cs = theta.cos();
        let s = r * cs;
        r * cs * (disc_in_rect(big_r + s, xs, zs) - disc_in_rect(big_r - s, xs, zs))
    };
    simpson(&slice, (lo / r).asin(), (hi / r).asin(), 1e-15)
}

/// **The box oracle reads the closed forms**: the torus above a plane
/// square to its axis (Pappus, `germ_plane_torus_chord::cap`), half the
/// torus, and the whole torus, through boxes that hold them.
#[test]
fn the_box_oracle_reads_the_closed_forms() {
    let (big_r, r) = (2.0, 0.5);
    let all = (-9.0, 9.0);
    let whole = 2.0 * PI * PI * big_r * r * r;
    for (what, got, want) in [
        (
            "the whole torus",
            torus_in_box((big_r, r), all, all, all),
            whole,
        ),
        (
            "half of it",
            torus_in_box((big_r, r), (0.0, 9.0), all, all),
            whole / 2.0,
        ),
        (
            "above y = 0.2",
            torus_in_box((big_r, r), all, (0.2, 9.0), all),
            crate::germ_plane_torus_chord::cap(big_r, r, 0.2),
        ),
    ] {
        assert!(
            (got - want).abs() <= 1e-12 * want,
            "{what}: {got} against {want}"
        );
    }
}

/// Where an op stops past the join, short of a body.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stop {
    /// The shells' sort reads `point_in_solid`, which refuses a torus
    /// face that is not a chart rectangle (`PartialTorusFace`).
    Sort,
    /// The result's volume does not read a torus face holding a ring
    /// (`RingOnCurvedFace`).
    Volume,
}

/// The stop an op's refusal names, or `None` for any other refusal.
fn stop(e: &BooleanError) -> Option<Stop> {
    match e {
        BooleanError::Pieces(topo::PieceSortError::Probe {
            source: topo::PointInSolidError::PartialTorusFace { .. },
            ..
        }) => Some(Stop::Sort),
        BooleanError::ResultInvalid { errors } if !errors.is_empty() => errors
            .iter()
            .all(|e| {
                matches!(
                    e,
                    topo::ValidationError::VolumeUncomputable {
                        source: topo::MassPropsError::RingOnCurvedFace { .. },
                        ..
                    }
                )
            })
            .then_some(Stop::Volume),
        _ => None,
    }
}

/// Whether an op's refusal is a [`Stop`] past the join.
pub(crate) fn stops_past_the_join(e: &BooleanError) -> bool {
    stop(e).is_some()
}

/// **Each of the six ops of `t` against `p` answers as `want` says**,
/// one letter per op in [`SIX`]'s order: `B` a body, held to the truth
/// (its volume and tier 3, and `point_in_solid` wherever it answers:
/// [`volume_and_points`]), `s` the sort's stop and `v` the volume's
/// ([`Stop`]); where `tie`, any op may instead escalate the matcher's
/// ranking (`bool_join_nearest`, [`tie_in_band`]). `meet` is first held
/// to the Monte Carlo. Each op's body, and the sampled points
/// `point_in_solid` read over them all.
fn answers(
    pose: &str,
    (t, p): (&Solid, &Solid),
    meet: f64,
    within: (Point3<f64>, Point3<f64>),
    (want, tie): (&str, bool),
) -> (Vec<Option<AtRestBody<f64>>>, usize) {
    meet_is_the_monte_carlo(pose, t, p, meet, within);
    let want: Vec<char> = want.chars().collect();
    assert_eq!(want.len(), 6, "{pose}: one answer per op");
    let (mut bodies, mut read) = (Vec::new(), 0usize);
    for (((label, r), &(_, op, swap)), &w) in six(&t.body, &p.body).into_iter().zip(&SIX).zip(&want)
    {
        let what = format!("{pose}, {label}");
        match (w, r) {
            (_, Err(BooleanError::Escalated { diag, .. }))
                if tie && diag.predicate == Some("bool_join_nearest") =>
            {
                bodies.push(None);
            }
            ('B', Ok(r)) => {
                let b = r
                    .body()
                    .unwrap_or_else(|| panic!("{what}: empty"))
                    .body
                    .clone();
                read += volume_and_points(&what, (op, swap), (t, p), meet, &b, within, true);
                bodies.push(Some(b));
            }
            ('s' | 'v', Err(e)) => {
                let want = if w == 's' { Stop::Sort } else { Stop::Volume };
                assert_eq!(stop(&e), Some(want), "{what}: {e:?}");
                bodies.push(None);
            }
            (w, r) => panic!(
                "{what}: want {w}, got {:?}",
                r.map(|r| r.body().map(|b| b.kind))
            ),
        }
    }
    (bodies, read)
}

/// The poses every torus `(2, 0.5)` about `y` runs, each with its planar
/// solid and the volume it shares with the torus: S1, the half-space
/// `x ≥ 0.3`, whose plane cuts two ovals; M3 and P4, boxes whose faces
/// `z = 1.2` cut two ovals beside their meridian or concentric face (the
/// far faces `x = 1` and `z = 2.8` cut ovals too); a box whose faces
/// `x = 0.3` and `x = 1` both cut ovals; B1, the bar across the hole,
/// whose sides `x = ±0.1` cut ovals and `y = ±0.1` concentric circles;
/// B2, the bar through the tube; and the wide bar across the hole whose
/// sections' ends tie for nearest at a coarse run's band
/// (`germ_plane_torus_frame`'s
/// `a_nearest_tie_in_band_on_a_bar_across_the_hole_escalates_by_name`),
/// held where they do not ([`tie_in_band`]).
fn donut_poses() -> Vec<(&'static str, Solid, f64)> {
    let d = (2.0, 0.5);
    let mut out = vec![(
        "S1",
        half(Point3::new(0.3, 0.0, 0.0), Vec3::unit_x()),
        torus_in_box(d, (0.3, 9.0), (-9.0, 9.0), (-9.0, 9.0)),
    )];
    let (x, w) = (0.0053, 0.277);
    for (name, xs, ys, zs) in [
        ("M3", (0.0, 1.0), (-1.0, 1.0), (1.2, 2.8)),
        ("P4", (-1.0, 1.0), (0.2, 1.0), (1.2, 2.8)),
        ("the spiric box", (0.3, 1.0), (-1.0, 1.0), (-3.0, 3.0)),
        ("B1", (-0.1, 0.1), (-0.1, 0.1), (-2.0, 2.0)),
        ("B2", (-0.05, 0.05), (0.25, 0.35), (1.0, 3.0)),
        ("the tie bar", (x - w, x + w), (-w, w), (-2.0, 2.0)),
    ] {
        out.push((name, boxed(xs, ys, zs), torus_in_box(d, xs, ys, zs)));
    }
    out
}

/// Whether the run's band holds the tie bar's tie, `3.2e-6 m` in the
/// ranking's measure (`germ_plane_torus_frame`'s
/// `a_nearest_tie_in_band_on_a_bar_across_the_hole_escalates_by_name`).
fn tie_in_band() -> bool {
    let band = geom_core::Band::linear(geom_core::Tol::witness()).expect("the run's band");
    band.zero() < 3.2e-6 && 3.2e-6 < band.escalate()
}

/// [`donut_poses`] on `t`, each pose answering as `want` names it.
fn donut_poses_answer(name: &str, t: AtRestBody<f64>, want: &[(&str, &str)]) {
    let t = ring(t, (2.0, 0.5));
    let poses = donut_poses();
    assert_eq!(poses.len(), want.len(), "{name}: an answer per pose");
    for ((pose, p, meet), &(named, answer)) in poses.iter().zip(want) {
        assert_eq!(*pose, named, "{name}: the poses in order");
        let what = format!("{name}, {pose}");
        let tie = *pose == "the tie bar" && tie_in_band();
        let (_, read) = answers(
            &what,
            (&t, p),
            *meet,
            about_the_tube(2.0, 0.5),
            (answer, tie),
        );
        if read == 0 && answer.contains('B') {
            test_utils::vacuity::stood_down(
                &what,
                "point_in_solid: it refuses PartialTorusFace on every built body",
            );
        }
    }
}

/// **The donut's spiric poses build their true bodies, or stop past the
/// join**: S1, M3 and P4 build in every op; the box between two ovals'
/// planes, B1 and B2 stop where a result's torus face is not a chart
/// rectangle or holds a ring.
#[test]
fn the_donuts_spiric_poses_build_their_true_bodies_or_stop_past_the_join() {
    donut_poses_answer(
        "the donut",
        torus(2.0, 0.5),
        &[
            ("S1", "BBBBBB"),
            ("M3", "BBBBBB"),
            ("P4", "BBBBBB"),
            ("the spiric box", "BBsssB"),
            ("B1", "vvssvB"),
            ("B2", "vvBBvs"),
            ("the tie bar", "vvssvB"),
        ],
    );
}

/// **On the one-face tori and the seamed torus the spiric poses build
/// their true bodies, or stop past the join**: an oval of S1 crosses a
/// one-face torus's wrap edges once, so a section's two loops are two
/// one-site records of one face pair, and on the seamed torus both ovals
/// cross both faces. Measured: S1 and M3 build in every op on all four;
/// P4 builds in every op on the one-face torus with its vertex off the
/// equators and keeps a ring the volume does not read on the other three;
/// B1 builds in four ops where the torus has an edge on its inner equator
/// (the one-face torus with its vertex there, and the seamed torus), and
/// so does the tie bar.
#[test]
fn the_one_face_and_seamed_tori_s_spiric_poses_answer_as_the_donuts() {
    let ring_stops = [
        ("S1", "BBBBBB"),
        ("M3", "BBBBBB"),
        ("P4", "vvBBvB"),
        ("the spiric box", "BBsssB"),
        ("B1", "vvssvB"),
        ("B2", "vvBBvs"),
        ("the tie bar", "vvssvB"),
    ];
    let mut off_equator = ring_stops;
    off_equator[2] = ("P4", "BBBBBB");
    let mut across = ring_stops;
    across[4] = ("B1", "BBssBB");
    across[6] = ("the tie bar", "BBssBB");
    for (name, t, _) in tori_of_one_face_pair(2.0, 0.5) {
        let want = if name.ends_with("vertex at 1") {
            &off_equator
        } else if name.ends_with("vertex at 0") {
            &ring_stops
        } else {
            &across
        };
        donut_poses_answer(&name, t, want);
    }
}

/// The edges of `b` on spiric carriers, each with both ends in the
/// corner bar's box `[−w, w]² × [−z, z]`: the oval arcs that bound the
/// lens, on its end squares and on the bar's sides `x = ±w`, each
/// against the torus (the corner edges along `y` dip into the tube),
/// split where a torus's own edge crosses them.
fn oval_arcs(b: &topo::Body<f64>, (w, z): (f64, f64)) -> usize {
    let in_box =
        |p: Point3<f64>| p.x.abs() <= w + 1e-6 && p.y.abs() <= w + 1e-6 && p.z.abs() <= z + 1e-6;
    let mut n = 0;
    for (_, edge) in b.edges() {
        let Some(c) = b.get_curve_geom(edge.curve).and_then(|c| c.certified()) else {
            continue;
        };
        if let geom::Curve3::Spiric { .. } = c.carrier() {
            let (t0, t1) = c.params();
            assert!(
                in_box(c.carrier().eval(t0)) && in_box(c.carrier().eval(t1)),
                "a spiric edge off the corner bar: {:?}",
                c.carrier()
            );
            n += 1;
        }
    }
    n
}

/// The fat torus `(1, 0.6)` and its one-face and seamed spellings, each
/// named.
fn fat_tori() -> Vec<(String, AtRestBody<f64>)> {
    let mut tori = vec![("the fat torus".to_string(), torus(1.0, 0.6))];
    for phase in [0.0, 1.0, PI] {
        tori.push((
            format!("the fat one-face torus, vertex at {phase}"),
            one_face_torus(1.0, 0.6, phase),
        ));
    }
    tori.push(("the fat seamed torus".to_string(), seamed_torus(1.0, 0.6)));
    tori
}

/// The corner bar's half-width and half-length on the torus `(R, r)`:
/// its corners on the inner face.
fn end(big_r: f64, r: f64, w: f64) -> (f64, f64) {
    let rho = big_r - (r * r - w * w).sqrt();
    (w, (rho * rho - w * w).sqrt())
}

/// The corner bar of half-width `w` on the torus `(R, r)` about `y` as a
/// solid, its eight corners on the inner face, and the volume it shares
/// with the torus.
fn corner(big_r: f64, r: f64, w: f64) -> (Solid, f64) {
    let (_, z) = end(big_r, r, w);
    let solid = Solid {
        body: corner_bar(big_r, r, w),
        ..boxed((-w, w), (-w, w), (-z, z))
    };
    (solid, torus_in_box((big_r, r), (-w, w), (-w, w), (-z, z)))
}

/// Points of the corner bar's lens, the bar's part inside the tube: at
/// heights between the tube's inner wall and the end square, near the
/// bar's sides `x = ±w` and its middle `y = 0`, where the lens is
/// deepest, and off every boundary.
fn lens_points(big_r: f64, r: f64, w: f64) -> Vec<Point3<f64>> {
    let (_, end) = end(big_r, r, w);
    let mut out = Vec::new();
    for x in [0.9 * w, -0.9 * w, 0.7 * w, -0.7 * w] {
        for y in [0.0, 0.3 * w, -0.3 * w] {
            let wall = ((big_r - (r * r - y * y).sqrt()).powi(2) - x * x).sqrt();
            for z in [0.5 * (wall + end), -0.5 * (wall + end)] {
                out.push(Point3::new(x, y, z));
            }
        }
    }
    out
}

/// **The corner bar's lens is far above the rows' tolerance**: the bar
/// on the fat torus `(1, 0.6)` at half-width `0.1` (B5) and `0.3` (B5′)
/// dips into the tube between each end square and the tube's inner
/// wall. The lens is the volume the bar shares with the torus, held to
/// a Monte Carlo of the two: `7.08e-5 m³` on B5, `9.40e-3 m³` on B5′.
/// Against the largest volume tolerance the rows read at this run's ε
/// (the union's): five times it and more on B5′ at ε `1e-7` and finer,
/// and on B5 at the default ε and finer. At a coarser ε the volume does
/// not see the lens, and the lens's arcs on every built body carry the
/// rows ([`oval_arcs`]).
#[test]
fn the_corner_bars_lens_is_well_above_the_rows_tolerance() {
    let (big_r, r) = (1.0, 0.6);
    for (pose, w, lens) in [("B5", 0.1, 7.08e-5), ("B5′", 0.3, 9.40e-3)] {
        let (bar, meet) = corner(big_r, r, w);
        let t = ring(torus(big_r, r), (big_r, r));
        let around = (Point3::new(-w, -w, -0.5), Point3::new(w, w, 0.5));
        meet_is_the_monte_carlo(pose, &t, &bar, meet, around);
        assert!((meet - lens).abs() < 1e-3 * lens, "{pose}: the lens {meet}");
        let tol = volume_tol(bar.volume + t.volume);
        let reads = if pose == "B5′" { 1e-7 } else { 1e-9 };
        if geom_core::Tol::witness().eps() <= reads {
            assert!(
                meet > 5.0 * tol,
                "{pose}: the lens {meet} against the tolerance {tol}"
            );
        } else {
            test_utils::vacuity::stood_down(
                pose,
                "the lens against the volume tolerance: it is under five tolerances at this ε, \
                 and the lens's arcs carry the rows",
            );
        }
        let inside = lens_points(big_r, r, w)
            .into_iter()
            .filter(|&q| (t.inside)(q) == Some(true) && (bar.inside)(q) == Some(true))
            .count();
        assert_eq!(
            inside, 24,
            "{pose}: every lens point is in the bar and the tube"
        );
    }
}

/// **The fat corner bar's lens is the true body or a typed refusal, never
/// a wrong body** (the spec's Q1): B5 and B5′ against the fat torus and
/// its one-face and seamed spellings, all six ops. Each end square cuts
/// two ovals, each running corner to corner, and the chord traces the
/// oval's arc between the corners, through the arc rule every curved
/// chord takes. A body is held to the truth, carries the eight oval arcs
/// on its end squares and sides (or more, where a torus's own edge
/// splits them), and is held to `point_in_solid`
/// at the lens's points wherever it answers; every other op stops past
/// the join. Red against the mutant that chords the end square along its
/// own edges, skipping the oval arcs between the corners: its bodies
/// miss the lens.
#[test]
fn the_fat_corner_bars_lens_is_the_true_body_or_a_typed_refusal() {
    let (big_r, r) = (1.0, 0.6);
    for (name, body) in fat_tori() {
        let t = ring(body, (big_r, r));
        let want =
            if name.ends_with("vertex at 3.141592653589793") || name.ends_with("seamed torus") {
                "BBssBB"
            } else {
                "vvssvB"
            };
        for (pose, w) in [("B5", 0.1), ("B5′", 0.3)] {
            let (bar, meet) = corner(big_r, r, w);
            let what = format!("{name}, {pose}");
            let (bodies, _) = answers(
                &what,
                (&t, &bar),
                meet,
                about_the_tube(big_r, r),
                (want, false),
            );
            let mut read = 0usize;
            for (b, &(label, op, swap)) in bodies.iter().zip(&SIX) {
                let Some(b) = b else { continue };
                let arcs = oval_arcs(b, end(big_r, r, w));
                assert!(
                    arcs >= 8,
                    "{what}, {label}: {arcs} oval arcs on the end squares and the sides `x = ±w`"
                );
                for q in lens_points(big_r, r, w) {
                    let (x, y) = if swap { (&bar, &t) } else { (&t, &bar) };
                    let (Some(a), Some(c)) = ((x.inside)(q), (y.inside)(q)) else {
                        panic!("{what}: a lens point on a boundary: {q:?}");
                    };
                    match contains(b, q) {
                        Err(topo::PointInSolidError::PartialTorusFace { .. }) => {}
                        got => {
                            assert_eq!(
                                got.ok(),
                                Some(kept(op, a, c)),
                                "{what}, {label}: point_in_solid in the lens at {q:?}"
                            );
                            read += 1;
                        }
                    }
                }
            }
            if read == 0 {
                test_utils::vacuity::stood_down(
                    &what,
                    "point_in_solid in the lens: it refuses PartialTorusFace on every built body",
                );
            }
        }
    }
}

/// **A plane parallel to the axis just short of the inner equator keeps
/// a typed refusal where its ovals' islands cannot be wound**: the
/// half-space `x ≥ 1.5 − 1e-3` against the donut cuts two ovals pinched
/// near the inner equator, each an island in the plane face. The winding
/// sum's quadrature cannot bound its error under that run's area there,
/// so the ring lane refuses the island typed rather than read a sign it
/// cannot decide
/// (`work/germ/a-planar-island-on-a-pinched-oval-refuses-its-winding.md`).
#[test]
fn a_plane_near_the_inner_equator_refuses_its_islands_typed() {
    let t = torus(2.0, 0.5);
    let p = crate::germ_plane_torus_frame::half_space(
        Point3::new(1.5 - 1e-3, 0.0, 0.0),
        Vec3::unit_x(),
    );
    for (label, r) in six(&t, &p) {
        assert!(
            matches!(
                r,
                Err(BooleanError::Join(topo::SplitJoinError::RingIslandUnread {
                    kind: geom::SurfaceKind::Plane,
                    ..
                }))
            ),
            "{label}: {:?}",
            r.map(|r| r.body().map(|b| b.kind))
        );
    }
}
