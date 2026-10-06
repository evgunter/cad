//! ZIP's probes for the declared-REST zip's admission
//! (`work/zip/a-flush-declared-reflex-union-ships-the-wrong-volume`) and
//! the rabbet row's fold-order shape
//! (`work/zip/a-declared-continuation-across-a-rabbet-step-leaves-six-loose-ends`).
//! `#[ignore]`d measurements, one line per run:
//! `cargo test -p sweep --test all zip_ -- --ignored --nocapture`.
//!
//! The REST zip opens only where the join refuses, so each admission
//! probe pairs a LEVER (a contact the join refuses on main: a coaxial
//! bore mate, or a flat wall flush where the other solid's fillet starts)
//! with a transverse overlap that a pure REST contact cannot have. The
//! controls build each lever alone and each overlap without its lever.
//! `zip_rest_admission_reflex_lever` is the one that ships wrong bodies.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use sweep::test_support::{brick, finished};
use topo::test_support::flush_declarations;
use topo::{AtRestBody, BooleanResult};

use crate::common::differential::outcome;

fn tol() -> Tol {
    Tol::witness()
}

type Span = (f64, f64);

fn box_of(x: Span, y: Span, z: Span) -> AtRestBody<f64> {
    finished("a brick", brick(x, y, z, tol()), tol())
}

/// `a` = (0,1)³, `c` = x∈(0.5,1.5), y∈(−1,0), `b` = x∈(0.5,1.5),
/// y∈(0,1), all z∈(0,1), folded by `union_with` in each of the six
/// orders, `flush_declarations` at every step. The finished union is
/// the L `x∈(0,1.5) × y∈(0,1) ∪ x∈(0.5,1.5) × y∈(−1,0)`, volume 2.5.
#[test]
#[ignore = "measurement: run with --ignored --nocapture"]
fn zip_rabbet_fold_orders() {
    let t = tol();
    let part = |n: char| -> (Span, Span) {
        match n {
            'a' => ((0.0, 1.0), (0.0, 1.0)),
            'b' => ((0.5, 1.5), (0.0, 1.0)),
            _ => ((0.5, 1.5), (-1.0, 0.0)),
        }
    };
    let area = |x: Span, y: Span| (x.1 - x.0) * (y.1 - y.0);
    // The first step's union, by box arithmetic: a ∪ b overlaps in
    // x∈(0.5,1), the other two pairs only touch.
    let first = |p: char, q: char| -> f64 {
        let (px, py) = part(p);
        let (qx, qy) = part(q);
        let ox = (px.1.min(qx.1) - px.0.max(qx.0)).max(0.0);
        let oy = (py.1.min(qy.1) - py.0.max(qy.0)).max(0.0);
        area(px, py) + area(qx, qy) - ox * oy
    };
    for order in ["acb", "cab", "abc", "bac", "bca", "cba"] {
        let n: Vec<char> = order.chars().collect();
        let body = |c: char| {
            let (x, y) = part(c);
            box_of(x, y, (0.0, 1.0))
        };
        let (p, q, r) = (body(n[0]), body(n[1]), body(n[2]));
        let step1 = topo::union_with(&p, &q, &flush_declarations(&p, &q, t), t);
        let want1 = first(n[0], n[1]);
        eprintln!("ZIPFOLD {order} step1 {}{} begins", n[0], n[1]);
        let acc = match &step1 {
            Ok(BooleanResult::Body(bb)) => Some(bb.body.clone()),
            _ => None,
        };
        println!(
            "ZIPFOLD {order} step1 ({}∪{}) => {}",
            n[0],
            n[1],
            outcome(step1, want1, t)
        );
        let Some(acc) = acc else { continue };
        eprintln!("ZIPFOLD {order} step2 begins");
        let step2 = topo::union_with(&acc, &r, &flush_declarations(&acc, &r, t), t);
        println!(
            "ZIPFOLD {order} step2 (({}∪{})∪{}) => {}",
            n[0],
            n[1],
            n[2],
            outcome(step2, 2.5, t)
        );
    }
}

/// The area of `{x ∈ (x0, x1), |y| < w, r_in < r < r_out}` (composite
/// Simpson over `y`, the chord lengths in closed form).
fn ring_slab_area(x: Span, w: f64, r_in: f64, r_out: f64) -> f64 {
    let len = |y: f64| {
        let lo = (r_in * r_in - y * y).max(0.0).sqrt();
        let hi = (r_out * r_out - y * y).max(0.0).sqrt();
        // x ∈ (x0, x1) ∩ ((−hi, −lo) ∪ (lo, hi)).
        let piece = |a: f64, b: f64| (x.1.min(b) - x.0.max(a)).max(0.0);
        piece(lo, hi) + piece(-hi, -lo)
    };
    let n = 200_000;
    let h = 2.0 * w / n as f64;
    let mut s = len(-w) + len(w);
    for i in 1..n {
        let k = if i % 2 == 1 { 4.0 } else { 2.0 };
        s += k * len(-w + i as f64 * h);
    }
    s * h / 3.0
}

/// **The coaxial lever.** A shaft in a bore declared `Rest` wall to
/// wall always defeats the join (no join arm for a coaxial cylinder
/// pair: `CurvedBooleanUnsupported`), so every such union is the REST
/// zip's. Each pose adds a box to the peg (`peg ∪ box`, undeclared,
/// through the join) that ALSO enters the collar's material, so the
/// mate is not a pure REST contact: the interiors overlap in
/// `box ∩ collar`. The union's closed form is
/// `vol collar + vol (peg ∪ box) − vol (box ∩ collar)`.
#[test]
#[ignore = "measurement: run with --ignored --nocapture"]
fn zip_rest_admission_coaxial_lever() {
    use crate::mate2_common::{collar, peg_at, volume, wall_decls};
    let t = tol();
    // (name, box x, box z, peg (z0, h)); every box spans |y| < 0.2.
    let poses: [(&str, Span, Span, Span); 4] = [
        // Through the bore wall inside the contact, below the top.
        ("through-wall", (0.0, 1.0), (1.2, 1.4), (0.5, 2.0)),
        // Across the collar's top annulus, from a proud peg end.
        ("over-cap", (0.3, 1.0), (1.8, 2.6), (0.5, 2.0)),
        // Out through the outer wall as well.
        ("through-both", (0.0, 2.0), (1.2, 1.4), (0.5, 2.0)),
        // The control: the box above the collar, a pure REST mate.
        ("control-above", (0.0, 1.0), (2.2, 2.4), (0.5, 2.0)),
    ];
    let w = 0.2;
    let c = finished("the collar", collar(), t);
    for ((name, bx, bz, (z0, h)), deg0) in poses.into_iter().flat_map(|p| [(p, 0.0), (p, 60.0)]) {
        let p = finished("the peg", peg_at(deg0, z0, h), t);
        let bx_body = box_of(bx, (-w, w), bz);
        let shaft = match topo::union(&p, &bx_body, t) {
            Ok(BooleanResult::Body(bb)) => bb.body,
            other => {
                println!(
                    "ZIPLEVER {name} peg@{deg0}: peg ∪ box => {}",
                    outcome(other, 0.0, t)
                );
                continue;
            }
        };
        let zc = (bz.0.max(1.0), bz.1.min(2.0));
        let overlap = (zc.1 - zc.0).max(0.0) * ring_slab_area(bx, w, 0.5, 1.5);
        let want = volume(&c) + volume(&shaft) - overlap;
        let decls = wall_decls(&c, &shaft);
        eprintln!(
            "ZIPLEVER {name} peg@{deg0} begins: {} declarations",
            decls.coincident_faces.len()
        );
        let r = topo::union_with(&c, &shaft, &decls, t);
        println!(
            "ZIPLEVER {name} peg@{deg0}: collar ∪ (peg ∪ box) => {} [overlap {overlap:.9}]",
            outcome(r, want, t)
        );
    }
}

/// **The tangent lever.** A slab resting on a rounded plate with its
/// west wall flush on the plate's west wall, where the plate's fillet
/// starts (`join2_r1_probes`' grid pose `r = 1, x ∈ (0, 3), y ∈ (m1,
/// 3)`): the join refuses the tangent plane×cylinder germ pair, so the
/// union is the REST zip's. Each pose adds a box to the slab
/// (`slab ∪ box`, undeclared, through the join) that also dips into the
/// plate, so the interiors overlap in `box ∩ plate`, a brick. Both
/// operand orders; `flush_declarations` for each.
#[test]
#[ignore = "measurement: run with --ignored --nocapture"]
fn zip_rest_admission_tangent_lever() {
    let t = tol();
    let m1 = 1.0 - core::f64::consts::FRAC_1_SQRT_2;
    let plate = sweep::test_support::extruded(
        sweep::test_support::sketch_at(0.0),
        vec![crate::join2_r1_probes::rounded(6.0, 4.0, 1.0)],
        1.0,
        t,
    );
    let plate = finished("the plate", plate, t);
    let slab = box_of((0.0, 3.0), (m1, 3.0), (1.0, 2.0));
    // (name, box x, box y, box z); the overlap is the box's part in
    // x ∈ (1, 5) ∪ …, y ∈ (0, 4), z ∈ (0, 1) — every box here sits over
    // the plate's straight south edge or its interior.
    let poses: [(&str, Span, Span, Span); 4] = [
        ("control", (1.5, 2.5), (0.5, 1.5), (1.5, 2.5)),
        ("over-south-wall", (1.5, 2.5), (-0.5, 0.5), (0.8, 1.5)),
        ("inside-top", (1.5, 2.5), (1.5, 2.5), (0.8, 1.5)),
        ("over-south-wall-deep", (1.5, 2.5), (-0.5, 0.5), (0.2, 1.5)),
    ];
    for (name, bx, by, bz) in poses {
        let bb = box_of(bx, by, bz);
        let upper = match topo::union(&slab, &bb, t) {
            Ok(BooleanResult::Body(r)) => r.body,
            other => {
                println!("ZIPTAN {name}: slab ∪ box => {}", outcome(other, 0.0, t));
                continue;
            }
        };
        let ov = |s: Span, lo: f64, hi: f64| (s.1.min(hi) - s.0.max(lo)).max(0.0);
        let overlap = ov(bx, 0.0, 6.0) * ov(by, 0.0, 4.0) * ov(bz, 0.0, 1.0);
        let vol = |b: &AtRestBody<f64>| topo::mass_properties(b, t).unwrap().volume;
        let want = vol(&upper) + vol(&plate) - overlap;
        for (order, a, b) in [("PA", &upper, &plate), ("QA", &plate, &upper)] {
            let d = flush_declarations(a, b, t);
            eprintln!("ZIPTAN {name} {order} begins");
            let r = topo::union_with(a, b, &d, t);
            println!(
                "ZIPTAN {name} {order}: ∪ => {} [overlap {overlap}]",
                outcome(r, want, t)
            );
        }
    }
}

/// A box's volume inside another's.
fn box_overlap(p: (Span, Span, Span), q: (Span, Span, Span)) -> f64 {
    let ov = |s: Span, r: Span| (s.1.min(r.1) - s.0.max(r.0)).max(0.0);
    ov(p.0, q.0) * ov(p.1, q.1) * ov(p.2, q.2)
}

/// The union of bricks, folded with `flush_declarations` at each step.
fn bricks(parts: &[(Span, Span, Span)]) -> Result<AtRestBody<f64>, String> {
    let t = tol();
    let mut acc = box_of(parts[0].0, parts[0].1, parts[0].2);
    for &(x, y, z) in &parts[1..] {
        let next = box_of(x, y, z);
        match topo::union_with(&acc, &next, &flush_declarations(&acc, &next, t), t) {
            Ok(BooleanResult::Body(bb)) => acc = bb.body,
            other => return Err(outcome(other, 0.0, t)),
        }
    }
    Ok(acc)
}

/// **The reflex corner, without the shear.** `a` is the 6 × 4 plate
/// with its south-east quarter `[4, 6] × [0, 2]` notched out, z ∈ (0, 1).
/// `b`'s block `K` stands in the notch flush on its west wall `x = 4`
/// (a `Rest`) and runs north across the notch's other wall `y = 2` into
/// `a`'s material: the reflex probe's shape with straight faces. The
/// other parts are slabs resting on `a`'s top with corners inside it
/// and outside it, the join2 grid's lever. Both operand orders; the
/// closed form is box arithmetic.
#[test]
#[ignore = "measurement: run with --ignored --nocapture"]
fn zip_rest_admission_notched_plate() {
    use crate::common::differential::ccw;
    let t = tol();
    let outline = ccw(vec![
        (0.0, 0.0),
        (4.0, 0.0),
        (4.0, 2.0),
        (6.0, 2.0),
        (6.0, 4.0),
        (0.0, 4.0),
    ]);
    let a = finished(
        "the notched plate",
        topo::test_support::prism_z::<f64>(&outline, 0.0, 1.0, t).body,
        t,
    );
    let big = ((0.0, 6.0), (0.0, 4.0), (0.0, 1.0));
    let notch = ((4.0, 6.0), (0.0, 2.0), (0.0, 1.0));
    let in_a = |q: (Span, Span, Span)| box_overlap(q, big) - box_overlap(q, notch);
    let k = ((4.0, 5.0), (1.0, 3.0), (0.5, 1.5));
    let poses: Vec<(&str, Vec<(Span, Span, Span)>)> = vec![
        ("K", vec![k]),
        ("K-flush-top", vec![((4.0, 5.0), (1.0, 3.0), (0.5, 1.0))]),
        ("slab-lever", vec![((2.0, 7.0), (2.5, 3.5), (1.0, 2.0))]),
        ("K+slab-east", vec![k, ((2.0, 7.0), (2.5, 3.5), (1.0, 2.0))]),
        (
            "K+slab-north",
            vec![k, ((4.2, 4.8), (2.5, 5.0), (1.0, 2.0))],
        ),
        (
            "K+slab-west",
            vec![k, ((-1.0, 4.5), (2.5, 3.5), (1.0, 2.0))],
        ),
    ];
    for (name, parts) in poses {
        let b = match bricks(&parts) {
            Ok(b) => b,
            Err(e) => {
                println!("ZIPNOTCH {name}: b => {e}");
                continue;
            }
        };
        // Pairwise inclusion–exclusion, exact for at most two parts.
        let mut overlap: f64 = parts.iter().map(|&p| in_a(p)).sum();
        if let [p, q] = parts[..] {
            let ov = |s: Span, r: Span| (s.0.max(r.0), s.1.min(r.1));
            overlap -= in_a((ov(p.0, q.0), ov(p.1, q.1), ov(p.2, q.2)));
        }
        let vol = |b: &AtRestBody<f64>| topo::mass_properties(b, t).unwrap().volume;
        let want = vol(&a) + vol(&b) - overlap;
        for (order, p, q) in [("AB", &a, &b), ("BA", &b, &a)] {
            let d = flush_declarations(p, q, t);
            let join =
                topo::test_support::boolean_join_refusal(topo::BooleanOp::Union, p, q, &d, t);
            eprintln!("ZIPNOTCH {name} {order} begins; the join alone: {join:?}");
            let r = topo::union_with(p, q, &d, t);
            println!(
                "ZIPNOTCH {name} {order}: ∪ => {} [overlap {overlap}]",
                outcome(r, want, t)
            );
        }
    }
}

/// **The notched plate with the tangent lever.** `a` is the rounded
/// 6 × 4 plate (`r = 1`) less its south-east quarter `x > 4, y < 2`, so
/// its south-west fillet stays; `b` holds a slab on `a`'s top whose west
/// wall is flush with `a`'s where that fillet starts (the join refuses
/// the tangent plane×cylinder pair), together with the block `K` of
/// [`zip_rest_admission_notched_plate`]: flush on the notch's west wall
/// (`Rest`) and across its north wall into `a`'s material.
#[test]
#[ignore = "measurement: run with --ignored --nocapture"]
fn zip_rest_admission_notched_lever() {
    let t = tol();
    let m1 = 1.0 - core::f64::consts::FRAC_1_SQRT_2;
    let plate = sweep::test_support::extruded(
        sweep::test_support::sketch_at(0.0),
        vec![crate::join2_r1_probes::rounded(6.0, 4.0, 1.0)],
        1.0,
        t,
    );
    let plate = finished("the plate", plate, t);
    let cut = box_of((4.0, 7.0), (-1.0, 2.0), (-1.0, 2.0));
    let a = match topo::subtract(&plate, &cut, t) {
        Ok(BooleanResult::Body(bb)) => bb.body,
        other => panic!("the notch: {}", outcome(other, 0.0, t)),
    };
    let vol = |b: &AtRestBody<f64>| topo::mass_properties(b, t).unwrap().volume;
    // `a`'s material over a box lying in x ∈ (1, 6), y ∈ (0, 4), clear of
    // the fillets: the box less the notch.
    let big = ((0.0, 6.0), (0.0, 4.0), (0.0, 1.0));
    let notch = ((4.0, 6.0), (0.0, 2.0), (0.0, 1.0));
    let in_a = |q: (Span, Span, Span)| box_overlap(q, big) - box_overlap(q, notch);
    let k = ((4.0, 5.0), (1.0, 3.0), (0.5, 1.5));
    let k_low = ((4.0, 5.0), (1.0, 3.0), (0.5, 1.0));
    let slab = |x1: f64| ((0.0, x1), (m1, 3.0), (1.0, 2.0));
    let poses: Vec<(&str, Vec<(Span, Span, Span)>)> = vec![
        ("lever", vec![slab(3.0)]),
        ("K", vec![k]),
        ("K+lever4.5", vec![slab(4.5), k]),
        ("K+lever5", vec![slab(5.0), k]),
        ("K+lever5.5", vec![slab(5.5), k]),
        ("Klow+lever5", vec![slab(5.0), k_low]),
        ("Klow+lever4.5", vec![slab(4.5), k_low]),
    ];
    for (name, parts) in poses {
        let b = match bricks(&parts) {
            Ok(b) => b,
            Err(e) => {
                println!("ZIPNL {name}: b => {e}");
                continue;
            }
        };
        // The slab's own overlap is nil (it rests on `a`), so `a ∩ b` is
        // `a ∩ K` less nothing: K's part above z = 1 is outside `a`.
        let overlap: f64 = parts.iter().map(|&p| in_a(p)).sum();
        let want = vol(&a) + vol(&b) - overlap;
        for (order, p, q) in [("AB", &a, &b), ("BA", &b, &a)] {
            let d = flush_declarations(p, q, t);
            let join =
                topo::test_support::boolean_join_refusal(topo::BooleanOp::Union, p, q, &d, t);
            eprintln!("ZIPNL {name} {order} begins; the join alone: {join:?}");
            let r = topo::union_with(p, q, &d, t);
            println!(
                "ZIPNL {name} {order}: ∪ => {} [overlap {overlap}]",
                outcome(r, want, t)
            );
        }
    }
}

/// **The reflex probe with a lever.** The reflex pose's `b` (sheared
/// `sqQ1`, flush on the notch wall, its cap dipping across the 45° wall
/// into `a`) is joined, through a bridge high above `a`, to a post
/// resting on `a`'s top whose rounded footprint's west wall is flush
/// with `a`'s west wall `x = −2`: the join refuses the post's tangent
/// plane×cylinder sites, so the union is the REST zip's again. The
/// closed form is `vol a + vol b′ − v∩`, `v∩` the pose's own.
#[test]
#[ignore = "measurement: run with --ignored --nocapture"]
fn zip_rest_admission_reflex_lever() {
    use crate::common::differential::reflex_pose;
    use geom_core::{OrthoFrame, Point3};
    let t = tol();
    // The post at `x0 = −2` is flush with `a`'s west wall (the lever);
    // at `x0 = −1.9` it is not, and the join builds the union itself.
    let post_at = |x0: f64| {
        finished(
            "the post",
            sweep::test_support::extruded(
                profile::SketchPlane::from_frame(OrthoFrame::axes_xy(Point3::new(x0, -1.5, 1.0))),
                vec![crate::join2_r1_probes::rounded(1.0, 1.0, 0.25)],
                1.8,
                t,
            ),
            t,
        )
    };
    let bridge = box_of((-1.5, 0.7), (-1.2, 0.7), (2.45, 3.45));
    let vol = |b: &AtRestBody<f64>| topo::mass_properties(b, t).unwrap().volume;
    let join_of = |p: &AtRestBody<f64>, q: &AtRestBody<f64>| match topo::union(p, q, t) {
        Ok(BooleanResult::Body(bb)) => Ok(bb.body),
        other => Err(outcome(other, 0.0, t)),
    };
    for (profile, sx, sy, x0) in [
        ("sqQ1", -0.5, 0.25, -1.9),
        ("eBot", -0.25, -0.5, -1.9),
        ("sqQ1", 0.25, 0.25, -2.0),
        ("sqQ1", -0.5, 0.25, -2.0),
        ("sqQ1", -0.75, 0.1, -2.0),
        ("sqQ1", -0.3, 0.25, -2.0),
        ("sqQ1", -0.25, 0.1, -2.0),
        ("sqQ1", -0.5, -0.25, -2.0),
        ("eBot", -0.5, -0.25, -2.0),
        ("eBot", -0.25, -0.5, -2.0),
        ("dRight", -0.25, -0.5, -2.0),
    ] {
        let p = reflex_pose(profile, 0.0, sx, sy, t);
        let what = format!("{profile} ({sx}, {sy}) post at x0 = {x0}");
        let post = post_at(x0);
        let b = match join_of(&p.b, &bridge).and_then(|bb| join_of(&bb, &post)) {
            Ok(b) => b,
            Err(e) => {
                println!("ZIPRL {what}: b′ => {e}");
                continue;
            }
        };
        let want = vol(&p.a) + vol(&b) - p.want[0];
        for (order, x, y) in [("AB", &p.a, &b), ("BA", &b, &p.a)] {
            let d = flush_declarations(x, y, t);
            let join =
                topo::test_support::boolean_join_refusal(topo::BooleanOp::Union, x, y, &d, t);
            eprintln!(
                "ZIPRL {what} {order} begins ({} declarations); the join alone: {join:?}",
                d.coincident_faces.len()
            );
            let r = topo::union_with(x, y, &d, t);
            println!(
                "ZIPRL {what} {order}: ∪ => {} [v∩ {}]",
                outcome(r, want, t),
                p.want[0]
            );
        }
    }
}
