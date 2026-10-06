//! **The REST zip's admission, off the pins' poses** (review of
//! `work/zip/a-flush-declared-reflex-union-ships-the-wrong-volume`).
//!
//! `rest_zip_admission.rs` pins the reflex lever at eight poses of three
//! profiles, all with `sx < 0`, and the tangent lever with one pure
//! contact. These rows take each lever further: overlapping reflex poses
//! the admission check alone keeps from shipping `vol a + vol b′` (a new
//! profile, `sx > 0`, the steepest shear), pure reflex contacts it must
//! still let the zip build, and the tangent lever under pure contacts
//! of other shapes and under dips, a mirrored boss and a second foot.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{OrthoFrame, Point3, Tol};
use sweep::test_support::{extruded, finished};
use topo::AtRestBody;
use topo::test_support::flush_declarations;

use crate::rest_zip_admission::{box_of, join_refuses, never_twice, reflex_beside_a_post, vol};

fn tol() -> Tol {
    Tol::witness()
}

type Span = (f64, f64);

/// Both orders of `p ∪ q` behind a lever: each refuses or builds sound
/// at `want`. Answers how many built.
fn both_orders(
    what: &str,
    p: &AtRestBody<f64>,
    q: &AtRestBody<f64>,
    want: f64,
    overlap: f64,
) -> usize {
    let mut built = 0;
    for (order, x, y) in [("p ∪ q", p, q), ("q ∪ p", q, p)] {
        let what = format!("{what}, {order}");
        let d = flush_declarations(x, y, tol());
        assert!(
            join_refuses(x, y, &d),
            "{what}: the lever no longer defeats the join"
        );
        if never_twice(&what, topo::union_with(x, y, &d, tol()), want, overlap) {
            built += 1;
        }
    }
    built
}

/// **The reflex lever off the pin's poses.** Six overlapping poses at
/// which the zip, without its admission check, ships `vol a + vol b′`
/// in both orders (measured with the check disabled), and six pure
/// contacts (`v∩ = 0`) that the zip builds in both orders.
#[test]
fn the_reflex_lever_off_the_pins_poses_never_ships_the_overlap_twice() {
    for (profile, sx, sy) in [
        ("dRight", -1.0, -1.0),
        ("eBot", -0.5, -0.5),
        ("eLeft", -1.0, -1.0),
        ("eLeft", 0.25, -0.5),
        ("sqQ1", -1.0, -1.0),
        ("sqQ1", 0.25, -0.5),
    ] {
        let (p, b, want) = reflex_beside_a_post(profile, sx, sy, -2.0);
        assert!(
            p.want[0] > 1e-3,
            "{profile} ({sx}, {sy}): the pose overlaps"
        );
        both_orders(
            &format!("{profile} ({sx}, {sy})"),
            &p.a,
            &b,
            want,
            p.want[0],
        );
    }
    for (profile, sx, sy) in [
        ("dLeft", -0.75, 0.1),
        ("sqQ2", -0.5, 0.25),
        ("sqQ4", 0.25, -0.5),
        ("dRight", 0.5, 0.5),
        ("sqQ3", -1.0, -1.0),
        ("dDown", 0.25, -0.5),
    ] {
        let (p, b, want) = reflex_beside_a_post(profile, sx, sy, -2.0);
        let what = format!("pure {profile} ({sx}, {sy})");
        assert_eq!(
            both_orders(&what, &p.a, &b, want, p.want[0]),
            2,
            "{what}: refused"
        );
    }
}

fn rounded_at(at: (f64, f64, f64), w: f64, h: f64, r: f64, height: f64) -> AtRestBody<f64> {
    let t = tol();
    finished(
        "a rounded block",
        extruded(
            profile::SketchPlane::from_frame(OrthoFrame::axes_xy(Point3::new(at.0, at.1, at.2))),
            vec![crate::join2_r1_probes::rounded(w, h, r)],
            height,
            t,
        ),
        t,
    )
}

fn declared_union(p: &AtRestBody<f64>, q: &AtRestBody<f64>) -> AtRestBody<f64> {
    match topo::union_with(p, q, &flush_declarations(p, q, tol()), tol()) {
        Ok(topo::BooleanResult::Body(bb)) => bb.body,
        other => panic!("an operand's own union: {other:?}"),
    }
}

/// **The tangent lever under other contacts.** The rounded 6 × 4 plate
/// (`r = 1`) and an upper solid whose west wall is flush where the
/// plate's south-west fillet starts. Eight pure contacts (two tangent
/// sites, an east wall too, an overhang, a thin slab, a rounded slab,
/// slabs carrying a post) build through the zip in both orders. Dips at
/// other depths and places, a rounded peg, two dips, the plate's own
/// boss rising into the slab and a second foot through the plate's
/// north wall each refuse or build at box arithmetic; the dips inside
/// the contact are
/// `work/zip/a-dip-inside-a-rest-contact-is-refused-by-the-result-gate.md`.
/// No overlap here reaches the admission check: each is refused before
/// it, or by the result gate, so this row stays green without the check
/// and pins the pure contacts it must keep building.
#[test]
fn the_tangent_lever_under_other_contacts_never_ships_the_overlap_twice() {
    let m1 = 1.0 - core::f64::consts::FRAC_1_SQRT_2;
    let plate = rounded_at((0.0, 0.0, 0.0), 6.0, 4.0, 1.0, 1.0);
    let pv = vol(&plate);
    let slab = box_of((0.0, 3.0), (m1, 3.0), (1.0, 2.0));
    for (name, u) in [
        ("slab", slab.clone()),
        (
            "both west fillets",
            box_of((0.0, 3.0), (m1, 4.0 - m1), (1.0, 2.0)),
        ),
        ("west and east", box_of((0.0, 6.0), (m1, 3.0), (1.0, 2.0))),
        (
            "overhanging north",
            box_of((0.0, 3.0), (m1, 5.0), (1.0, 2.0)),
        ),
        ("thin slab", box_of((0.0, 3.0), (m1, 3.0), (1.0, 1.25))),
        (
            "rounded slab",
            rounded_at((0.0, m1, 1.0), 3.0, 2.7, 0.5, 1.0),
        ),
        (
            "a post on the slab",
            declared_union(&slab, &box_of((1.0, 2.0), (1.0, 2.0), (1.9, 3.0))),
        ),
        (
            "a rounded post on the slab",
            declared_union(&slab, &rounded_at((1.0, 1.0, 1.9), 1.0, 1.0, 0.4, 1.0)),
        ),
    ] {
        let what = format!("pure: {name}");
        assert_eq!(
            both_orders(&what, &u, &plate, vol(&u) + pv, 0.0),
            2,
            "{what}: refused"
        );
    }
    let ov = |x: Span, y: Span, z: Span| {
        let f = |s: Span, lo: f64, hi: f64| (s.1.min(hi) - s.0.max(lo)).max(0.0);
        f(x, 0.0, 6.0) * f(y, 0.0, 4.0) * f(z, 0.0, 1.0)
    };
    // Every box lies over the plate's straight edges or inside it, clear
    // of the fillets.
    for (name, x, y, z) in [
        ("0.5 deep", (1.5, 2.5), (1.5, 2.5), (0.5, 1.5)),
        ("0.99 deep", (1.5, 2.5), (1.5, 2.5), (0.01, 1.5)),
        ("to the floor", (1.5, 2.5), (1.5, 2.5), (0.0, 1.5)),
        ("through the plate", (1.5, 2.5), (1.5, 2.5), (-0.5, 1.5)),
        ("near the west edge", (0.1, 1.0), (2.0, 2.9), (0.8, 1.5)),
        (
            "flush with the plate's west wall",
            (0.0, 1.0),
            (1.5, 2.5),
            (0.8, 1.5),
        ),
        (
            "flush with the slab's north wall",
            (1.5, 2.5),
            (2.0, 3.0),
            (0.8, 1.5),
        ),
    ] {
        let u = declared_union(&slab, &box_of(x, y, z));
        let o = ov(x, y, z);
        both_orders(&format!("dip {name}"), &u, &plate, vol(&u) + pv - o, o);
    }
    let foot = 1.0 - (4.0 - core::f64::consts::PI) * 0.16;
    let u = declared_union(&slab, &rounded_at((1.5, 1.5, 0.8), 1.0, 1.0, 0.4, 0.7));
    both_orders(
        "dip a rounded peg",
        &u,
        &plate,
        vol(&u) + pv - 0.2 * foot,
        0.2 * foot,
    );
    let u = declared_union(
        &declared_union(&slab, &box_of((0.5, 1.0), (1.5, 2.0), (0.8, 1.5))),
        &box_of((2.0, 2.5), (2.0, 2.5), (0.8, 1.5)),
    );
    both_orders("two dips", &u, &plate, vol(&u) + pv - 0.1, 0.1);
    let lower = declared_union(&plate, &box_of((1.5, 2.5), (1.5, 2.5), (0.5, 1.2)));
    both_orders(
        "the plate's boss into the slab",
        &slab,
        &lower,
        vol(&slab) + vol(&lower) - 0.2,
        0.2,
    );
    let u = declared_union(
        &box_of((0.0, 5.0), (m1, 3.0), (1.0, 2.0)),
        &box_of((4.0, 4.5), (2.5, 4.5), (0.5, 1.5)),
    );
    both_orders(
        "a foot through the north wall",
        &u,
        &plate,
        vol(&u) + pv - 0.375,
        0.375,
    );
}
