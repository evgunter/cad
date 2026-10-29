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
    match r {
        Ok(BooleanResult::Body(bb)) => {
            let t3 = topo::validate_geometric(&bb.body, tol());
            let t3p = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol());
            let shells = bb.body.shells().count();
            eprintln!("  tier 3 {t3:?}, tier 3' {t3p:?}, shells {shells}");
            if t3.is_err() || t3p.is_err() || shells != 1 {
                return Err(format!("unsound body: {t3:?} {t3p:?} shells {shells}"));
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
    let (fa, fb) = (
        finished("A", a.clone(), tol()),
        finished("B", b.clone(), tol()),
    );
    let mut got = vec![("A ∪ B", topo::union(&fa, &fb, tol()))];
    got.extend(subtract_both_orders_and_intersect(a, b));
    let mut wrong = Vec::new();
    for (op, r) in &got {
        let w = want.iter().find(|(o, _)| o == op).unwrap().1;
        let g = read(r);
        eprintln!("{label} {op}: want {w:?}, got {g:?}");
        match (g, w) {
            (Err(e), _) if e.starts_with("unsound") => wrong.push(format!("{op}: {e}")),
            (Err(_), _) => {}
            (Ok(None), None) => {}
            (Ok(Some(v)), Some(w)) if (v - w).abs() <= 1e-6 * w.max(1.0) => {}
            (g, w) => wrong.push(format!("{op}: want {w:?}, got {g:?}")),
        }
    }
    assert!(wrong.is_empty(), "{label}: wrong answers: {wrong:#?}");
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

/// **A lens inside the union of its tori is never read `On` it.** Each
/// face of the lens lies on the carrier of a face of the union at the
/// same orientation, and the two bodies meet along the same two
/// circles, so every face pair is settled one carrier and the section
/// pass defers the wall pairs to the shells' verdicts. The patches
/// differ: the lens's seam meridians run off the union's boundary, and
/// their midpoints side the shells (the union `Out` of the lens, the
/// lens `In` the union), so neither reads `On` and every op keeps the
/// deferred pair's refusal. Red if the `On` verdict ever weakens on a
/// curved face far enough to read the lens `On` the union, or if the
/// refusal moves to another door; a body built at its closed form would
/// be the pose's next answer.
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
        assert!(
            matches!(r, Err(BooleanError::FallbackExtentUnsupported { .. })),
            "union vs lens, {op}: the deferred wall pair's refusal: {r:?}"
        );
    }
}

fn rod(r: f64, h: f64) -> Body<f64> {
    revolved_about_y(
        vec![
            (pt((0.0, 0.0)), 0.0),
            (pt((r, 0.0)), 0.0),
            (pt((r, h)), 0.0),
            (pt((0.0, h)), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    )
}

/// **Rods sharing a wall at two lengths are sided by their extent, or
/// refuse typed.** The short rod's wall lies on the long one's carrier,
/// and the short rod's top cap stands inside the long rod, so neither
/// shell reads `On` and the wall pair keeps its refusal where the
/// short rod's shell decides the op. `∪` and `B ∖ A` build at their
/// closed forms; `∩` and `A ∖ B` refuse `FallbackExtentUnsupported`
/// (`work/splitdoor/rods-sharing-a-wall-at-two-lengths-refuse-intersect-and-the-short-difference.md`).
/// Red on a wrong body, or where a refusal moves.
#[test]
fn rods_sharing_a_wall_at_different_lengths_are_sided_by_their_extent() {
    let (a, b) = (rod(0.5, 1.0), rod(0.5, 2.0));
    let (va, vb) = (
        vol(&finished("a", a.clone(), tol())).0,
        vol(&finished("b", b.clone(), tol())).0,
    );
    check(
        "short vs long rod",
        &a,
        &b,
        [
            ("A ∪ B", Some(vb)),
            ("A ∩ B", Some(va)),
            ("A ∖ B", None),
            ("B ∖ A", Some(vb - va)),
        ],
    );
    let mut got = vec![(
        "A ∪ B",
        topo::union(
            &finished("A", a.clone(), tol()),
            &finished("B", b.clone(), tol()),
            tol(),
        ),
    )];
    got.extend(subtract_both_orders_and_intersect(&a, &b));
    for (op, r) in got {
        let builds = op == "A ∪ B" || op == "B ∖ A";
        assert_eq!(
            matches!(r, Ok(BooleanResult::Body(_))),
            builds,
            "short vs long rod, {op}: builds only where the long rod decides: {r:?}"
        );
        if !builds {
            assert!(
                matches!(r, Err(BooleanError::FallbackExtentUnsupported { .. })),
                "short vs long rod, {op}: the wall pair's refusal: {r:?}"
            );
        }
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
