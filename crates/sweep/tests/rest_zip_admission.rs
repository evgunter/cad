//! **A union at a tangent site never ships the overlap twice**
//! (`work/zip/a-flush-declared-reflex-union-ships-the-wrong-volume`).
//!
//! Each row pairs a tangent site (a flat wall flush where the other
//! solid's fillet starts, the tangent plane×cylinder pair) with an
//! overlap a pure REST contact cannot have. The join reads the germ
//! tangent to the wall in the face its fillet's rim turns into, so it
//! connects the site; a union that grafted one solid whole and dropped
//! only the contact patches would come back at `vol a + vol b`. Each
//! such union either refuses or builds sound at its closed form; the
//! controls build each site with a pure contact, and each overlap
//! without its site. The last two rows take each site off the pins'
//! poses: more overlaps and pure contacts for the reflex site, and the
//! pure contacts the tangent site must keep building. The last pins a
//! line kiss beside the site, which builds with its contact undeclared.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{OrthoFrame, Point3, Tol};
use sweep::test_support::{brick, extruded, finished, sketch_at};
use topo::test_support::flush_declarations;
use topo::{AtRestBody, BooleanDeclarations, BooleanError, BooleanResult};

use crate::common::differential::{ReflexPose, reflex_pose};
use crate::common::rounded;

fn tol() -> Tol {
    Tol::witness()
}

type Span = (f64, f64);

fn box_of(x: Span, y: Span, z: Span) -> AtRestBody<f64> {
    finished("a brick", brick(x, y, z, tol()), tol())
}

fn vol(b: &AtRestBody<f64>) -> f64 {
    topo::mass_properties(b, tol()).unwrap().volume
}

fn union(p: &AtRestBody<f64>, q: &AtRestBody<f64>) -> AtRestBody<f64> {
    match topo::union(p, q, tol()) {
        Ok(BooleanResult::Body(bb)) => bb.body,
        other => panic!("an operand's own union: {other:?}"),
    }
}

/// Whether the join alone refuses `p ∪ q` under `d`.
fn join_refuses(p: &AtRestBody<f64>, q: &AtRestBody<f64>, d: &BooleanDeclarations) -> bool {
    topo::test_support::boolean_join_refusal(topo::BooleanOp::Union, p, q, d, tol())
        .expect("the reduction runs")
        .is_some()
}

/// `r` refused, or is a body that has volume `want`, passes tiers 2
/// and 3′ and the at-rest certificate, and is a legal operand.
/// `overlap` is the interiors' common volume, which a union that kept
/// both solids whole would add back. Answers whether it built.
fn never_twice(
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
/// `a`'s where its fillets start: the tangent site. Answers the pose,
/// `b′` and `vol a + vol b′ − v∩`.
fn reflex_beside_a_post(
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
            vec![rounded(1.0, 1.0, 0.25)],
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

/// **A reflex union at a tangent site never ships the overlap twice.**
/// `b`'s wall rests flush on `a`'s notch wall (a `Rest`) and its sheared
/// cap crosses `a`'s 45° wall and top into `a`'s material, with the
/// post's tangent site beside it: each union refuses `Join(..)` or
/// builds at `vol a + vol b′ − v∩`, in both operand orders.
#[test]
fn a_reflex_union_at_a_tangent_site_never_ships_the_overlap_twice() {
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
            match topo::union_with(x, y, &d, tol()) {
                Err(BooleanError::Join(_)) => {}
                Err(e) => panic!("{what}: refused, but not by the join: {e:?}"),
                built => {
                    never_twice(&what, built, want, p.want[0]);
                }
            }
        }
    }
}

/// **The tangent site's controls build sound.** The same `b′` with the
/// post off `a`'s west wall (`x0 = −1.9`): no tangent site. And the site
/// with a pure contact (`sqQ1` at `(0.25, 0.25)`, whose cap clears `a`).
/// The join connects each, and builds the union at the closed form.
#[test]
fn a_tangent_sites_controls_build_sound() {
    for (profile, sx, sy, x0) in [
        ("sqQ1", -0.5, 0.25, -1.9),
        ("eBot", -0.25, -0.5, -1.9),
        ("sqQ1", 0.25, 0.25, -2.0),
    ] {
        let (p, b, want) = reflex_beside_a_post(profile, sx, sy, x0);
        for (order, x, y) in [("a ∪ b′", &p.a, &b), ("b′ ∪ a", &b, &p.a)] {
            let what = format!("{profile} ({sx}, {sy}), post at {x0}, {order}");
            let d = flush_declarations(x, y, tol());
            assert!(!join_refuses(x, y, &d), "{what}: the join connects");
            assert!(
                never_twice(&what, topo::union_with(x, y, &d, tol()), want, p.want[0]),
                "{what}: refused"
            );
        }
    }
}

/// **A box dipping into a plate at a tangent site never ships the
/// overlap twice.** A slab rests on the rounded 6 × 4 plate (`r = 1`)
/// with its west wall flush where the plate's south-west fillet starts
/// (the tangent plane×cylinder pair), and a box joined to the slab
/// dips into the plate: across its straight south wall, or inside the
/// contact, where the dip's boundary on the plate's top is the slab's
/// own edges and no section segment marks it
/// (`work/zip/a-dip-inside-a-rest-contact-is-refused-by-the-result-gate.md`).
/// Each builds sound at box arithmetic, in both orders. The control, a
/// box above the plate, joins too.
#[test]
fn a_box_dipping_into_a_plate_at_a_tangent_site_never_ships_the_overlap_twice() {
    let t = tol();
    let m1 = 1.0 - core::f64::consts::FRAC_1_SQRT_2;
    let plate = finished(
        "the plate",
        extruded(sketch_at(0.0), vec![rounded(6.0, 4.0, 1.0)], 1.0, t),
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
            assert!(
                !control || !join_refuses(p, q, &d),
                "{what}: the control's join connects"
            );
            let built = never_twice(&what, topo::union_with(p, q, &d, t), want, overlap);
            assert!(built, "{what}: refused");
        }
    }
}

/// Both orders of `p ∪ q` at a tangent site: each refuses or builds
/// sound at `want`. Answers how many built.
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
        if never_twice(&what, topo::union_with(x, y, &d, tol()), want, overlap) {
            built += 1;
        }
    }
    built
}

/// **The reflex site off the pin's poses.** Six overlapping poses (a
/// further profile, `sx > 0`, the steepest shear), each refused or built
/// at `vol a + vol b′ − v∩`, and six pure contacts (`v∩ = 0`) that build
/// in both orders.
#[test]
fn the_reflex_site_off_the_pins_poses_never_ships_the_overlap_twice() {
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
            vec![rounded(w, h, r)],
            height,
            t,
        ),
        t,
    )
}

fn declared_union(p: &AtRestBody<f64>, q: &AtRestBody<f64>) -> AtRestBody<f64> {
    match topo::union_with(p, q, &flush_declarations(p, q, tol()), tol()) {
        Ok(BooleanResult::Body(bb)) => bb.body,
        other => panic!("an operand's own union: {other:?}"),
    }
}

/// **The tangent site keeps building pure contacts.** The rounded
/// 6 × 4 plate (`r = 1`) and an upper solid whose west wall is flush
/// where the plate's south-west fillet starts. Eight pure contacts (two
/// tangent sites, an east wall too, an overhang, a thin slab, a rounded
/// slab, slabs carrying a post) build in both orders. Dips at other
/// depths and places, a rounded peg, two dips, the plate's own boss
/// rising into the slab and a second foot through the plate's north wall
/// each refuse or build at box arithmetic.
#[test]
fn the_tangent_site_keeps_building_pure_contacts() {
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

/// Each record's cells, with the rows it cites, read through its op's
/// list: a declaration's own rows come first in a declared op's list,
/// so two ops recording one decision cite it at different positions.
fn cited_rows(r: &topo::BooleanBody<f64>) -> Vec<String> {
    r.contacts
        .rows()
        .map(|(cells, cites)| {
            let rows: Vec<String> = cites
                .iter()
                .map(|b| match b {
                    topo::Backing::Decided(k) => format!("{:?}", r.coincidences[k as usize]),
                    carried @ topo::Backing::Carried { .. } => format!("{carried:?}"),
                })
                .collect();
            format!("{cells:?} {rows:?}")
        })
        .collect()
}

/// **A line kiss beside the tangent site builds at its closed form and
/// ships its contact undeclared** (shape 2 of
/// `work/zip/a-dip-inside-a-rest-contact-is-refused-by-the-result-gate`).
/// At these poses one of `b`'s cap edges from the reflex corner lies in
/// `a`'s top, with the cap above it elsewhere: `v∩ = 0`, and `b` kisses
/// `a` along that edge and at its far vertex. Each union, in both
/// orders, builds at `vol a + vol b′` and passes tier 2; tier 3′ finds
/// the kiss's vertex on `a`'s top and its edge along it, and no record
/// backs either. The glue records face pairs, and no face pair meets
/// here, so the union undeclared is the declared one, records and all
/// (D10); the dropped records are
/// `work/fuse/a-kissing-convex-corner-result-ships-an-undeclared-vertex-on-face.md`.
#[test]
fn a_line_kiss_beside_a_tangent_site_ships_its_contact_undeclared() {
    use topo::{CensusContact, ValidationError};
    for (profile, sx, sy) in [
        ("dUp", 0.25, 0.25),
        ("dUp", 0.5, 0.5),
        ("dLeft", -0.5, -0.5),
        ("dLeft", -0.25, 0.25),
        ("dLeft", -0.1, -0.1),
        ("dLeft", -1.0, -1.0),
        ("dDown", -0.5, -0.5),
        ("dDown", -0.1, -0.1),
        ("dDown", -1.0, -1.0),
        ("dRight", 0.25, 0.25),
    ] {
        let (p, b, want) = reflex_beside_a_post(profile, sx, sy, -2.0);
        assert_eq!(
            p.want[0], 0.0,
            "{profile} ({sx}, {sy}): the kiss overlaps nothing"
        );
        for (order, x, y) in [("a ∪ b′", &p.a, &b), ("b′ ∪ a", &b, &p.a)] {
            let what = format!("{profile} ({sx}, {sy}) {order}");
            let d = flush_declarations(x, y, tol());
            let bb = match topo::union_with(x, y, &d, tol()) {
                Ok(BooleanResult::Body(bb)) => bb,
                other => panic!("{what}: the kiss does not build: {other:?}"),
            };
            let v = vol(&bb.body);
            assert!((v - want).abs() < 1e-9, "{what}: volume {v} against {want}");
            assert_eq!(topo::validate_closed(&bb.body), Ok(()), "{what}: tier 2");
            let Ok(BooleanResult::Body(bare)) = topo::union(x, y, tol()) else {
                panic!("{what}: the undeclared union builds");
            };
            assert_eq!(
                (format!("{:?}", bare.body), cited_rows(&bare)),
                (format!("{:?}", bb.body), cited_rows(&bb)),
                "{what}: undeclared, the declared body and records"
            );
            let findings = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
                .expect_err("tier 3′ finds the kiss undeclared");
            let kind = |want: fn(&CensusContact) -> bool| {
                findings
                    .iter()
                    .filter(|e| {
                        matches!(e, ValidationError::UndeclaredContact { contact, .. } if want(contact))
                    })
                    .count()
            };
            let on_face = kind(|c| matches!(c, CensusContact::VertexOnFace { .. }));
            let along = kind(|c| matches!(c, CensusContact::EdgeFaceOverlap { .. }));
            assert!(
                on_face > 0 && along > 0 && on_face + along == findings.len(),
                "{what}: the kiss's vertex on a's top and its edge along it, nothing else: \
                 {findings:?}"
            );
        }
    }
}
