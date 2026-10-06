//! **The declared-REST zip admits only a pure REST contact**
//! (`work/zip/a-flush-declared-reflex-union-ships-the-wrong-volume`).
//!
//! The zip opens only where the join refuses, so each row pairs a lever
//! (a contact the join refuses: a flat wall flush where the other
//! solid's fillet starts, the tangent plane×cylinder pair) with an
//! overlap a pure REST contact cannot have. The zip grafts B whole and
//! discards only the contact patches, so a mate it admitted with its
//! interiors overlapping would come back at `vol a + vol b`. Each such
//! union either refuses or builds sound at its closed form; the
//! controls build each lever with a pure contact, and each overlap
//! without its lever.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{OrthoFrame, Point3, Tol};
use sweep::test_support::{brick, extruded, finished, sketch_at};
use topo::test_support::flush_declarations;
use topo::{AtRestBody, BooleanDeclarations, BooleanError, BooleanResult};

use crate::common::differential::{ReflexPose, reflex_pose};

fn tol() -> Tol {
    Tol::witness()
}

type Span = (f64, f64);

pub(crate) fn box_of(x: Span, y: Span, z: Span) -> AtRestBody<f64> {
    finished("a brick", brick(x, y, z, tol()), tol())
}

pub(crate) fn vol(b: &AtRestBody<f64>) -> f64 {
    topo::mass_properties(b, tol()).unwrap().volume
}

pub(crate) fn union(p: &AtRestBody<f64>, q: &AtRestBody<f64>) -> AtRestBody<f64> {
    match topo::union(p, q, tol()) {
        Ok(BooleanResult::Body(bb)) => bb.body,
        other => panic!("an operand's own union: {other:?}"),
    }
}

/// Whether the join alone refuses `p ∪ q` under `d`: the lever that
/// hands the union to the zip.
pub(crate) fn join_refuses(
    p: &AtRestBody<f64>,
    q: &AtRestBody<f64>,
    d: &BooleanDeclarations,
) -> bool {
    topo::test_support::boolean_join_refusal(topo::BooleanOp::Union, p, q, d, tol())
        .expect("the reduction runs")
        .is_some()
}

/// `r` refused, or is a body that has volume `want`, passes tiers 2
/// and 3′ and the at-rest certificate, and is a legal operand.
/// `overlap` is the interiors' common volume, which a zip that kept
/// both solids whole would add back. Answers whether it built.
pub(crate) fn never_twice(
    what: &str,
    r: Result<BooleanResult<f64>, BooleanError>,
    want: f64,
    overlap: f64,
) -> bool {
    let bb = match r {
        Err(_) => return false,
        Ok(BooleanResult::Body(bb)) => bb,
        Ok(BooleanResult::Empty) => panic!("{what}: a union of two solids came back empty"),
    };
    let v = vol(&bb.body);
    assert!(
        (v - want).abs() < 1e-9,
        "{what}: volume {v} against the closed form {want} (excess {}, the overlap {overlap})",
        v - want
    );
    assert_eq!(topo::validate_closed(&bb.body), Ok(()), "{what}: tier 2");
    assert_eq!(
        topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()),
        Ok(()),
        "{what}: tier 3′"
    );
    assert!(
        topo::validate_geometric_certificate(&bb.body, tol()).is_ok(),
        "{what}: the at-rest certificate"
    );
    sweep::test_support::assert_legal_operand(what, &bb.body, tol());
    true
}

/// The reflex pose's `b` joined, through a bridge high over `a`, to a
/// post resting on `a`'s top: its 1 × 1 footprint, corners rounded 0.25,
/// starts at `x0`. At `x0 = −2` the post's west wall is flush with
/// `a`'s where its fillets start, and the join refuses the tangent
/// site. Answers the pose, `b′` and `vol a + vol b′ − v∩`.
pub(crate) fn reflex_beside_a_post(
    profile: &str,
    sx: f64,
    sy: f64,
    x0: f64,
) -> (ReflexPose, AtRestBody<f64>, f64) {
    let t = tol();
    let p = reflex_pose(profile, 0.0, sx, sy, t);
    let post = finished(
        "the post",
        extruded(
            profile::SketchPlane::from_frame(OrthoFrame::axes_xy(Point3::new(x0, -1.5, 1.0))),
            vec![crate::join2_r1_probes::rounded(1.0, 1.0, 0.25)],
            1.8,
            t,
        ),
        t,
    );
    let bridge = box_of((-1.5, 0.7), (-1.2, 0.7), (2.45, 3.45));
    let b = union(&union(&p.b, &bridge), &post);
    let want = vol(&p.a) + vol(&b) - p.want[0];
    (p, b, want)
}

/// **A reflex union behind a join lever never ships the overlap twice.**
/// `b`'s wall rests flush on `a`'s notch wall (a `Rest`) and its sheared
/// cap crosses `a`'s 45° wall and top into `a`'s material; the post
/// makes the join refuse, so the union is the zip's to answer. The cap's
/// crossings are section segments with no contact patch beside them,
/// so the zip declines, and each union refuses or builds at
/// `vol a + vol b′ − v∩`. It used to build `vol a + vol b′` at every
/// pose here, in both operand orders.
#[test]
fn a_reflex_union_behind_a_join_lever_never_ships_the_overlap_twice() {
    for (profile, sx, sy) in [
        ("sqQ1", -0.5, 0.25),
        ("sqQ1", -0.75, 0.1),
        ("sqQ1", -0.3, 0.25),
        ("sqQ1", -0.25, 0.1),
        ("sqQ1", -0.5, -0.25),
        ("eBot", -0.5, -0.25),
        ("eBot", -0.25, -0.5),
        ("dRight", -0.25, -0.5),
    ] {
        let (p, b, want) = reflex_beside_a_post(profile, sx, sy, -2.0);
        for (order, x, y) in [("a ∪ b′", &p.a, &b), ("b′ ∪ a", &b, &p.a)] {
            let what = format!("{profile} ({sx}, {sy}) {order}");
            let d = flush_declarations(x, y, tol());
            assert!(
                join_refuses(x, y, &d),
                "{what}: the post no longer defeats the join, so this row no longer reaches the zip"
            );
            never_twice(&what, topo::union_with(x, y, &d, tol()), want, p.want[0]);
        }
    }
}

/// **The lever's controls build sound.** The same `b′` with the post
/// off `a`'s west wall (`x0 = −1.9`): no lever, and the join builds the
/// union at the closed form. And the lever with a pure contact (`sqQ1`
/// at `(0.25, 0.25)`, whose cap clears `a`): the join refuses, and the
/// zip builds it.
#[test]
fn a_join_levers_controls_build_sound() {
    for (profile, sx, sy, x0, lever) in [
        ("sqQ1", -0.5, 0.25, -1.9, false),
        ("eBot", -0.25, -0.5, -1.9, false),
        ("sqQ1", 0.25, 0.25, -2.0, true),
    ] {
        let (p, b, want) = reflex_beside_a_post(profile, sx, sy, x0);
        for (order, x, y) in [("a ∪ b′", &p.a, &b), ("b′ ∪ a", &b, &p.a)] {
            let what = format!("{profile} ({sx}, {sy}), post at {x0}, {order}");
            let d = flush_declarations(x, y, tol());
            assert_eq!(join_refuses(x, y, &d), lever, "{what}: the lever");
            assert!(
                never_twice(&what, topo::union_with(x, y, &d, tol()), want, p.want[0]),
                "{what}: refused"
            );
        }
    }
}

/// **A box dipping into a plate behind a tangent lever never ships the
/// overlap twice.** A slab rests on the rounded 6 × 4 plate (`r = 1`)
/// with its west wall flush where the plate's south-west fillet starts
/// (the join refuses the tangent pair), and a box joined to the slab
/// dips into the plate: across its straight south wall, or inside the
/// contact, where the dip's boundary on the plate's top is the slab's
/// own edges and no section segment marks it
/// (`work/zip/a-dip-inside-a-rest-contact-is-refused-by-the-result-gate.md`).
/// Each refuses or builds at box arithmetic. The control, a box above
/// the plate, builds through the zip in both orders.
#[test]
fn a_box_dipping_into_a_plate_behind_a_tangent_lever_never_ships_the_overlap_twice() {
    let t = tol();
    let m1 = 1.0 - core::f64::consts::FRAC_1_SQRT_2;
    let plate = finished(
        "the plate",
        extruded(
            sketch_at(0.0),
            vec![crate::join2_r1_probes::rounded(6.0, 4.0, 1.0)],
            1.0,
            t,
        ),
        t,
    );
    let slab = box_of((0.0, 3.0), (m1, 3.0), (1.0, 2.0));
    let ov = |s: Span, lo: f64, hi: f64| (s.1.min(hi) - s.0.max(lo)).max(0.0);
    for (name, y, z, control) in [
        ("above the plate", (0.5, 1.5), (1.5, 2.5), true),
        ("across the south wall", (-0.5, 0.5), (0.8, 1.5), false),
        ("inside the contact", (1.5, 2.5), (0.8, 1.5), false),
        ("deep across the south wall", (-0.5, 0.5), (0.2, 1.5), false),
    ] {
        let x = (1.5, 2.5);
        let upper = union(&slab, &box_of(x, y, z));
        // Every box lies over the plate's straight south edge or inside
        // it, clear of the fillets.
        let overlap = ov(x, 0.0, 6.0) * ov(y, 0.0, 4.0) * ov(z, 0.0, 1.0);
        let want = vol(&upper) + vol(&plate) - overlap;
        for (order, p, q) in [
            ("upper ∪ plate", &upper, &plate),
            ("plate ∪ upper", &plate, &upper),
        ] {
            let what = format!("{name}, {order}");
            let d = flush_declarations(p, q, t);
            assert!(join_refuses(p, q, &d), "{what}: the lever");
            let built = never_twice(&what, topo::union_with(p, q, &d, t), want, overlap);
            assert!(built || !control, "{what}: the control refused");
        }
    }
}
