//! **Reviewer probes for the `On` deferral** (`splitdoor/two-rims-coplanar`
//! review): curved bodies whose faces lie on one another's carriers and
//! whose every vertex and edge lies on the other's boundary, without the
//! two bodies being one. The ladder offers no interior witness on a
//! curved face (`topo::stands`), so the shells' `On` verdict rests on the
//! settled carrier pairs alone; these rows ask whether the deferred
//! section pair is then sided right.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::revert_ops::subtract_both_orders_and_intersect;
use geom_core::{Point2, Tol};
use sweep::test_support::{finished, revolved_about_y};
use topo::{AtRestBody, Body, BooleanError, BooleanResult};

fn tol() -> Tol {
    Tol::witness()
}

fn vol(body: &AtRestBody<f64>) -> (f64, f64) {
    let m = topo::test_support::pinned_volume("probe", body, tol());
    (m.volume, m.volume_pad)
}

/// What each op returned, as `Some(volume)` / `None` (empty) / the error.
fn read(r: &Result<BooleanResult<f64>, BooleanError>) -> Result<Option<f64>, String> {
    read_voided(r, 0)
}

/// [`read`] of a body holding `voids` cavities, each a shell beside its
/// solid's outer one.
fn read_voided(
    r: &Result<BooleanResult<f64>, BooleanError>,
    voids: usize,
) -> Result<Option<f64>, String> {
    match r {
        Ok(BooleanResult::Body(bb)) => {
            let t3 = topo::validate_geometric(&bb.body, tol());
            let t3p = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol());
            let (shells, solids) = (bb.body.shells().count(), bb.body.solids().count());
            eprintln!("  tier 3 {t3:?}, tier 3' {t3p:?}, shells {shells}, solids {solids}");
            if t3.is_err() || t3p.is_err() || shells != solids + voids {
                return Err(format!(
                    "unsound body: {t3:?} {t3p:?} shells {shells} solids {solids}"
                ));
            }
            Ok(Some(vol(&bb.body).0))
        }
        Ok(BooleanResult::Empty) => Ok(None),
        Err(e) => Err(format!("{e:?}")),
    }
}

/// Every op on `(a, b)` against the closed form `want` (`None`: empty).
/// A typed refusal passes; a wrong body or a wrong `Empty` fails.
fn check(label: &str, a: &Body<f64>, b: &Body<f64>, want: [(&str, Option<f64>); 4]) {
    let mut wrong = Vec::new();
    for (op, r) in &ops(a, b) {
        let w = want.iter().find(|(o, _)| o == op).unwrap().1;
        let g = read(r);
        eprintln!("{label} {op}: want {w:?}, got {g:?}");
        match g {
            Err(e) if e.starts_with("unsound") => wrong.push(format!("{op}: {e}")),
            Err(_) => {}
            g if closed_form(&g, w) => {}
            g => wrong.push(format!("{op}: want {w:?}, got {g:?}")),
        }
    }
    assert!(wrong.is_empty(), "{label}: wrong answers: {wrong:#?}");
}

/// `g` is the closed form `w` (`None`: empty).
fn closed_form(g: &Result<Option<f64>, String>, w: Option<f64>) -> bool {
    match (g, w) {
        (Ok(None), None) => true,
        (Ok(Some(v)), Some(w)) => (v - w).abs() <= 1e-6 * w.max(1.0),
        _ => false,
    }
}

/// Every op on `(a, b)`, `∪` first.
fn ops(
    a: &Body<f64>,
    b: &Body<f64>,
) -> Vec<(&'static str, Result<BooleanResult<f64>, BooleanError>)> {
    let (fa, fb) = (
        finished("A", a.clone(), tol()),
        finished("B", b.clone(), tol()),
    );
    let mut got = vec![("A ∪ B", topo::union(&fa, &fb, tol()))];
    got.extend(subtract_both_orders_and_intersect(a, b));
    got
}

/// Every op on `(a, b)` builds its closed form `want` (`None`: empty),
/// each body sound at tier 3 and 3′. A refusal fails.
fn builds(label: &str, a: &Body<f64>, b: &Body<f64>, want: [(&str, Option<f64>); 4]) {
    builds_voided(label, a, b, want, [0; 4]);
}

/// [`builds`], each op's body holding the cavities `voids` counts (in
/// `want`'s order).
fn builds_voided(
    label: &str,
    a: &Body<f64>,
    b: &Body<f64>,
    want: [(&str, Option<f64>); 4],
    voids: [usize; 4],
) {
    let wrong: Vec<String> = ops(a, b)
        .iter()
        .filter_map(|(op, r)| {
            let k = want.iter().position(|(o, _)| o == op).unwrap();
            let w = want[k].1;
            let g = read_voided(r, voids[k]);
            eprintln!("{label} {op}: want {w:?}, got {g:?}");
            (!closed_form(&g, w)).then(|| format!("{op}: want {w:?}, got {g:?}"))
        })
        .collect();
    assert!(
        wrong.is_empty(),
        "{label}: not built at the closed form: {wrong:#?}"
    );
}

/// The bulge of the counter-clockwise arc about `c` from `p` to `q`.
fn ccw_bulge(c: (f64, f64), p: (f64, f64), q: (f64, f64)) -> f64 {
    let a0 = (p.1 - c.1).atan2(p.0 - c.0);
    let a1 = (q.1 - c.1).atan2(q.0 - c.0);
    let mut sweep = a1 - a0;
    while sweep <= 0.0 {
        sweep += 2.0 * core::f64::consts::PI;
    }
    (sweep / 4.0).tan()
}

fn pt(p: (f64, f64)) -> Point2<f64> {
    Point2::new(p.0, p.1)
}

/// Two coaxial tori of tube radius 1 on rings `2` and `2.8`: the solid
/// union of the two tubes, and their lens. Both are bounded by one face
/// on each torus and meet along the same two circles; the lens lies
/// inside the union, and each face of the lens lies on the carrier of a
/// face of the union at the same orientation.
fn union_and_lens() -> (Body<f64>, Body<f64>) {
    let (c1, c2) = ((2.0, 0.0), (2.8, 0.0));
    let h = (1.0f64 - 0.16).sqrt();
    let (up, dn) = ((2.4, h), (2.4, -h));
    let union = revolved_about_y(
        vec![
            (pt(up), ccw_bulge(c1, up, dn)),
            (pt(dn), ccw_bulge(c2, dn, up)),
        ],
        sweep::Revolution::Full,
        tol(),
    );
    let lens = revolved_about_y(
        vec![
            (pt(dn), ccw_bulge(c1, dn, up)),
            (pt(up), ccw_bulge(c2, up, dn)),
        ],
        sweep::Revolution::Full,
        tol(),
    );
    (union, lens)
}

/// **A lens inside the union of its tori is never read `On` it, and is
/// sided where one operand keeps.** Each face of the lens lies on the
/// carrier of a face of the union at the same orientation, and the two
/// bodies meet along the same two circles, so every face pair is settled
/// one carrier and the section pass defers the wall pairs to the shells'
/// verdicts. The patches differ: the lens's seam meridians run off the
/// union's boundary, and their midpoints side the shells (the union
/// `Out` of the lens, the lens `In` the union), so neither reads `On`.
/// The sides stand where the op keeps one operand: `B ∖ A` builds,
/// nothing. `∪`
/// and `∩` pass the deferral and escalate at the coplanar merge: its
/// torus witness reads only the faces' vertices, which all lie on the
/// circles both tori share, so it cannot tell the two rings apart
/// (`work/contacthold/the-torus-witness-reads-only-vertices-two-tori-share.md`).
/// `A ∖ B` keeps both shells, touching along the circles, and keeps the
/// deferred pair's refusal. Red if the `On`
/// verdict ever weakens on a curved face far enough to read the lens
/// `On` the union, on a wrong body, or if the refusal moves to another
/// door.
#[test]
fn a_lens_inside_the_union_of_its_tori_is_never_read_on_it() {
    let (u, l) = union_and_lens();
    let (vu, vl) = (
        vol(&finished("u", u.clone(), tol())).0,
        vol(&finished("l", l.clone(), tol())).0,
    );
    assert!(
        vl > 0.1 && vu > vl + 1.0,
        "the lens is a proper part: {vl}, {vu}"
    );
    check(
        "union vs lens",
        &u,
        &l,
        [
            ("A ∪ B", Some(vu)),
            ("A ∩ B", Some(vl)),
            ("A ∖ B", Some(vu - vl)),
            ("B ∖ A", None),
        ],
    );
    let (fu, fl) = (
        finished("A", u.clone(), tol()),
        finished("B", l.clone(), tol()),
    );
    let mut got = vec![("A ∪ B", topo::union(&fu, &fl, tol()))];
    got.extend(subtract_both_orders_and_intersect(&u, &l));
    for (op, r) in got {
        match op {
            "B ∖ A" => assert!(r.is_ok(), "union vs lens, {op}: builds: {r:?}"),
            "A ∖ B" => assert!(
                matches!(r, Err(BooleanError::FallbackExtentUnsupported { .. })),
                "union vs lens, {op}: the deferred wall pair's refusal: {r:?}"
            ),
            _ => assert!(
                matches!(&r, Err(BooleanError::Merge(_)))
                    && format!("{r:?}").contains("carrier_torus_witness"),
                "union vs lens, {op}: the merge's torus witness escalates: {r:?}"
            ),
        }
    }
}

fn rod(r: f64, h: f64) -> Body<f64> {
    rod_span(r, 0.0, h)
}

/// **Rods sharing a wall at two lengths are sided by their extent.** The
/// short rod's wall lies on the long one's carrier and its top cap
/// stands inside the long rod, so neither shell reads `On`. `∩` and the
/// short rod's difference reach the no-crossings fallback with the wall
/// pairs deferred, the short shell's `In` stands, and every op builds at
/// its closed form, in both orders, and with the short rod clear of both
/// of the long rod's caps. Red on a wrong body or a refusal.
#[test]
fn rods_sharing_a_wall_at_different_lengths_are_sided_by_their_extent() {
    let q = core::f64::consts::PI / 4.0;
    for (label, short, long) in [
        ("short vs long rod", rod(0.5, 1.0), rod(0.5, 2.0)),
        (
            "inner rod vs long rod",
            rod_span(0.5, 0.5, 1.5),
            rod(0.5, 2.0),
        ),
    ] {
        let (a, b) = (q, 2.0 * q);
        builds(
            label,
            &short,
            &long,
            [
                ("A ∪ B", Some(b)),
                ("A ∩ B", Some(a)),
                ("A ∖ B", None),
                ("B ∖ A", Some(b - a)),
            ],
        );
        builds(
            &format!("{label}, swapped"),
            &long,
            &short,
            [
                ("A ∪ B", Some(b)),
                ("A ∩ B", Some(a)),
                ("A ∖ B", Some(b - a)),
                ("B ∖ A", None),
            ],
        );
    }
}

/// **Rods overlapping over part of each one's length build.** A runs
/// from `y = 0.5` to `2.5` and B from `0` to `2`: each wall has a rim of
/// the other inside it, the two rims read sides apart, so whatever the op
/// one of them is a crossing and the join cuts both walls at the other's
/// rim. Red on a wrong body or a refusal.
#[test]
fn rods_overlapping_over_part_of_their_length_build() {
    let q = core::f64::consts::PI / 4.0;
    let (a, b) = (rod_span(0.5, 0.5, 2.5), rod(0.5, 2.0));
    let want = [
        ("A ∪ B", Some(2.5 * q)),
        ("A ∩ B", Some(1.5 * q)),
        ("A ∖ B", Some(0.5 * q)),
        ("B ∖ A", Some(0.5 * q)),
    ];
    builds("partial overlap", &a, &b, want);
    builds("partial overlap, swapped", &b, &a, want);
}

/// **Bodies sharing a wall whose sides change across it build at their
/// closed forms**: a rod in a wall with a conical dimple, a rod through
/// a stepped body whose narrow wall is a band of the rod's, and a
/// flanged rod whose flange the long rod's wall runs into. Each turns
/// one operand's boundary across the shared wall, the shape a side
/// certificate must not answer. Red on a wrong body or a refusal.
#[test]
fn walls_whose_sides_turn_across_the_shared_band_build() {
    use core::f64::consts::PI;
    let q = PI / 4.0;
    let dimple = PI * 0.25 * 0.5 / 3.0;
    builds(
        "rod vs dimpled wall",
        &rod(0.5, 1.0),
        &prof(&[(0.0, 0.5), (0.5, 0.0), (0.5, 2.0), (0.0, 2.0)]),
        [
            ("A ∪ B", Some(2.0 * q)),
            ("A ∩ B", Some(q - dimple)),
            ("A ∖ B", Some(dimple)),
            ("B ∖ A", Some(q)),
        ],
    );
    builds(
        "rod vs stepped body",
        &rod(0.5, 2.0),
        &prof(&[
            (0.0, -1.0),
            (1.0, -1.0),
            (1.0, 0.5),
            (0.5, 0.5),
            (0.5, 1.5),
            (0.0, 1.5),
        ]),
        [
            ("A ∪ B", Some(1.5 * PI + 1.5 * q)),
            ("A ∩ B", Some(1.5 * q)),
            ("A ∖ B", Some(0.5 * q)),
            ("B ∖ A", Some(1.5 * PI - 0.5 * q)),
        ],
    );
    builds(
        "flanged rod vs long rod",
        &prof(&[
            (0.0, 0.0),
            (0.5, 0.0),
            (0.5, 1.0),
            (1.0, 1.0),
            (1.0, 1.5),
            (0.0, 1.5),
        ]),
        &rod(0.5, 2.0),
        [
            ("A ∪ B", Some(1.5 * q + 0.5 * PI)),
            ("A ∩ B", Some(1.5 * q)),
            ("A ∖ B", Some(0.5 * PI - 0.5 * q)),
            ("B ∖ A", Some(0.5 * q)),
        ],
    );
}

/// A Boolean door.
type Op = fn(&AtRestBody<f64>, &AtRestBody<f64>, Tol) -> Result<BooleanResult<f64>, BooleanError>;

/// `(label, A, B, [A ∪ B, A ∩ B, A ∖ B, B ∖ A], each one's cavities)`;
/// `None` is empty.
type Pose = (
    &'static str,
    Body<f64>,
    Body<f64>,
    [Option<f64>; 4],
    [usize; 4],
);

/// **A rod standing on a step of a wider body builds every op.** Each
/// wall has an edge of the other inside it: the rod's top rim lies in
/// the body's narrow wall, and the step's inner rim in the rod's wall.
/// The rod's cap turns into the body and the step's annulus away from
/// the rod, the sides Eq. 15.3 lumps each one's wall to for `∩` and the
/// rod's difference (for `∪` and the body's difference in the swapped
/// order), so neither rim is a crossing there: those ops reach the
/// no-crossings fallback with the wall pairs deferred, and the rod's `In`
/// and the body's `Out` answer them. The same holds with a cavity in the
/// step (inside the rod, or below it), with the rod as three shells, and
/// on a conical step. Where the rod surrounds the cavity, `∩` keeps a
/// shell of each operand, the cavity's and the rod's, which share no
/// patch. Red on a wrong body or a refusal, in either order: in
/// particular if the sides are refused wherever both operands have an
/// edge inside the other's face, or wherever the op keeps a shell of
/// each.
#[test]
fn a_rod_standing_on_a_step_builds_every_op() {
    use core::f64::consts::PI;
    let q = PI / 4.0;
    let step = prof(&[
        (0.0, -1.0),
        (1.0, -1.0),
        (1.0, 0.5),
        (0.5, 0.5),
        (0.5, 2.0),
        (0.0, 2.0),
    ]);
    let s = 1.5 * PI + 1.5 * q;
    let op = |f: Op, a: &Body<f64>, b: &Body<f64>| {
        let (fa, fb) = (
            finished("A", a.clone(), tol()),
            finished("B", b.clone(), tol()),
        );
        match f(&fa, &fb, tol()) {
            Ok(BooleanResult::Body(r)) => r.body.into_body(),
            other => panic!("the fixture builds: {other:?}"),
        }
    };
    let (c, cavity) = (PI * 0.09 * 0.5, rod_span(0.3, -0.8, -0.3));
    let cavitied = op(topo::subtract, &step, &cavity);
    let three = op(
        topo::union,
        &op(topo::union, &rod(0.5, 1.0), &rod_span(0.5, 1.2, 1.8)),
        &rod_span(0.5, 3.0, 3.5),
    );
    let cone_step = prof(&[
        (0.0, -1.0),
        (1.5, -1.0),
        (1.5, 0.5),
        (0.75, 0.5),
        (0.0, 2.0),
    ]);
    let (cs, frustum) = (3.375 * PI + 0.28125 * PI, 1.75 * PI / 3.0);
    let poses: [Pose; 6] = [
        (
            "rod on the step",
            rod(0.5, 1.0),
            step.clone(),
            [Some(s), Some(q), None, Some(s - q)],
            [0; 4],
        ),
        (
            "rod from inside the step",
            rod_span(0.5, 0.25, 1.0),
            step.clone(),
            [Some(s), Some(0.75 * q), None, Some(s - 0.75 * q)],
            [0; 4],
        ),
        (
            "rod on the cavitied step",
            rod(0.5, 1.0),
            cavitied.clone(),
            [Some(s - c), Some(q), None, Some(s - c - q)],
            [1, 0, 0, 1],
        ),
        (
            "rod around the step's cavity",
            rod_span(0.5, -0.9, 1.0),
            cavitied,
            [Some(s), Some(1.9 * q - c), Some(c), Some(s - 1.9 * q)],
            [0, 1, 0, 0],
        ),
        (
            "three-shell rod on the step",
            three,
            step,
            [
                Some(s + 0.5 * q),
                Some(1.6 * q),
                Some(0.5 * q),
                Some(s - 1.6 * q),
            ],
            [0; 4],
        ),
        (
            "frustum on the conical step",
            prof(&[(0.0, 0.0), (1.0, 0.0), (0.5, 1.0), (0.0, 1.0)]),
            cone_step,
            [Some(cs), Some(frustum), None, Some(cs - frustum)],
            [0; 4],
        ),
    ];
    for (label, a, b, [u, i, ab, ba], [vu, vi, vab, vba]) in poses {
        let want = [("A ∪ B", u), ("A ∩ B", i), ("A ∖ B", ab), ("B ∖ A", ba)];
        builds_voided(label, &a, &b, want, [vu, vi, vab, vba]);
        builds_voided(
            &format!("{label}, swapped"),
            &b,
            &a,
            [want[0], want[1], ("A ∖ B", ba), ("B ∖ A", ab)],
            [vu, vi, vba, vab],
        );
    }
}

/// **Cones sharing a carrier at different extents build**: frusta of the
/// cone of apex `(0, 2)` and base radius 1, inside it, clear of both its
/// ends, and overlapping over part of each one's height, in both orders
/// where they differ. Red on a wrong body or a refusal.
#[test]
fn cones_sharing_a_carrier_at_different_extents_build() {
    use core::f64::consts::PI;
    let r = |y: f64| 1.0 - y / 2.0;
    let v =
        |y0: f64, y1: f64| PI / 3.0 * (y1 - y0) * (r(y0).powi(2) + r(y0) * r(y1) + r(y1).powi(2));
    let span = |y0: f64, y1: f64| prof(&[(0.0, y0), (r(y0), y0), (r(y1), y1), (0.0, y1)]);
    let cone = prof(&[(0.0, 0.0), (1.0, 0.0), (0.0, 2.0)]);
    for (label, y0, y1) in [
        ("frustum in cone", 0.0, 1.0),
        ("inner frustum in cone", 0.5, 1.0),
    ] {
        let (a, b) = (v(y0, y1), v(0.0, 2.0));
        builds(
            label,
            &span(y0, y1),
            &cone,
            [
                ("A ∪ B", Some(b)),
                ("A ∩ B", Some(a)),
                ("A ∖ B", None),
                ("B ∖ A", Some(b - a)),
            ],
        );
        builds(
            &format!("{label}, swapped"),
            &cone,
            &span(y0, y1),
            [
                ("A ∪ B", Some(b)),
                ("A ∩ B", Some(a)),
                ("A ∖ B", Some(b - a)),
                ("B ∖ A", None),
            ],
        );
    }
    let (a, b, i) = (v(0.5, 1.5), v(0.0, 1.0), v(0.5, 1.0));
    builds(
        "overlapping frusta",
        &span(0.5, 1.5),
        &span(0.0, 1.0),
        [
            ("A ∪ B", Some(a + b - i)),
            ("A ∩ B", Some(i)),
            ("A ∖ B", Some(a - i)),
            ("B ∖ A", Some(b - i)),
        ],
    );
}

/// **A ball segment inside the ball refuses at the sphere scan where the
/// segment's shell decides.** The segment's cap stands inside the ball
/// with its rim on the ball's sphere, so the sphere's section circle on
/// the cap's plane is the cap's own boundary, and the extent scan refuses
/// it as a circle near the plane face's boundary before the section pass
/// runs (`work/splitdoor/a-ball-segment-inside-its-ball-refuses-at-the-sphere-scan.md`).
/// The ops the ball decides build at their closed forms, in both
/// orders. Red on a wrong body, or where a refusal moves.
#[test]
fn a_ball_segment_inside_its_ball_refuses_at_the_sphere_scan_where_the_segment_decides() {
    use core::f64::consts::PI;
    let seg = |h: f64| PI * (1.0 + h).powi(2) * (2.0 - h) / 3.0;
    let s = (1.0f64 - 0.25).sqrt();
    let ball = revolved_about_y(
        vec![
            (
                pt((0.0, -1.0)),
                ccw_bulge((0.0, 0.0), (0.0, -1.0), (0.0, 1.0)),
            ),
            (pt((0.0, 1.0)), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    );
    let below = revolved_about_y(
        vec![
            (
                pt((0.0, -1.0)),
                ccw_bulge((0.0, 0.0), (0.0, -1.0), (s, 0.5)),
            ),
            (pt((s, 0.5)), 0.0),
            (pt((0.0, 0.5)), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    );
    let (a, b) = (seg(0.5), 4.0 * PI / 3.0);
    for (label, x, y, want) in [
        (
            "segment vs ball",
            &below,
            &ball,
            [
                ("A ∪ B", Some(b)),
                ("A ∩ B", Some(a)),
                ("A ∖ B", None),
                ("B ∖ A", Some(b - a)),
            ],
        ),
        (
            "ball vs segment",
            &ball,
            &below,
            [
                ("A ∪ B", Some(b)),
                ("A ∩ B", Some(a)),
                ("A ∖ B", Some(b - a)),
                ("B ∖ A", None),
            ],
        ),
    ] {
        let segment_decides: &[&str] = if label == "segment vs ball" {
            &["A ∩ B", "A ∖ B"]
        } else {
            &["A ∪ B", "B ∖ A"]
        };
        for (op, r) in ops(x, y) {
            let w = want.iter().find(|(o, _)| *o == op).unwrap().1;
            if segment_decides.contains(&op) {
                assert!(
                    matches!(
                        r,
                        Err(BooleanError::FallbackExtentUnsupported { what, .. })
                            if what.contains("near the plane face's boundary")
                    ),
                    "{label}, {op}: the sphere scan's refusal: {r:?}"
                );
            } else {
                let g = read(&r);
                assert!(closed_form(&g, w), "{label}, {op}: want {w:?}, got {g:?}");
            }
        }
    }
}

/// **A third operand against a sided result**: the short rod's
/// intersection with the long one, against the long rod again and
/// against a longer rod clear of both its caps, every op at its closed
/// form. Red on a wrong body or a refusal.
#[test]
fn a_sided_result_is_sided_again_against_a_third_rod() {
    let q = core::f64::consts::PI / 4.0;
    let (short, long) = (
        finished("A", rod(0.5, 1.0), tol()),
        finished("B", rod(0.5, 2.0), tol()),
    );
    let Ok(BooleanResult::Body(x)) = topo::intersect(&short, &long, tol()) else {
        panic!("the short rod's intersection with the long one builds");
    };
    for (label, c, vc) in [
        ("(A ∩ B) vs B", rod(0.5, 2.0), 2.0 * q),
        ("(A ∩ B) vs C", rod_span(0.5, -1.0, 3.0), 4.0 * q),
    ] {
        builds(
            label,
            &x.body,
            &c,
            [
                ("A ∪ B", Some(vc)),
                ("A ∩ B", Some(q)),
                ("A ∖ B", None),
                ("B ∖ A", Some(vc - q)),
            ],
        );
    }
}

#[test]
fn a_rod_against_itself_builds_at_every_eps_and_again_against_its_result() {
    let a = rod(0.5, 1.0);
    let va = vol(&finished("a", a.clone(), tol())).0;
    let want = [
        ("A ∪ B", Some(va)),
        ("A ∩ B", Some(va)),
        ("A ∖ B", None),
        ("B ∖ A", None),
    ];
    check("rod vs itself", &a, &a, want);
    // A third operand: the union's result against the rod again.
    let fa = finished("A", a.clone(), tol());
    let Ok(BooleanResult::Body(u)) = topo::union(&fa, &fa, tol()) else {
        return;
    };
    check("(rod ∪ rod) vs rod", &u.body, &a, want);
}

fn rod_span(r: f64, y0: f64, y1: f64) -> Body<f64> {
    prof(&[(0.0, y0), (r, y0), (r, y1), (0.0, y1)])
}

fn prof(pts: &[(f64, f64)]) -> Body<f64> {
    revolved_about_y(
        pts.iter().map(|&p| (pt(p), 0.0)).collect(),
        sweep::Revolution::Full,
        tol(),
    )
}
