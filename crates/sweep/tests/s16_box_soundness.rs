//! **The box-soundness rows that need a real curved body**, so they
//! live here rather than in `topo`'s own suite.
//!
//! # 1. The census's instance-containment arm
//!
//! It must bound the CONTAINING solid by a superset, not by its
//! vertices.
//!
//! Arm 2 of the cross-solid backstop clears a pair when some extent
//! margin is definitely negative — when the inner solid pokes out of
//! the outer one. That reading is sound only if the OUTER box contains
//! the outer solid's whole locus. Built from vertices it does not: a
//! curved solid's vertex hull is an inscribed polytope, so a body
//! sitting between the inscribed hull and the true surface pokes out
//! of the hull while being entirely INSIDE the solid, and the backstop
//! whose whole job is "decide or refuse, never silently not-examine"
//! silently clears it.
//!
//! The fixture is the extruded three-arc cylinder (radius 0.5, six
//! vertices at 0°/120°/240° on two caps). Its vertex hull is the
//! inscribed triangular prism, whose x extent starts at −0.25 while
//! the cylinder's starts at −0.5. Every nested probe below lives in
//! that annulus.
//!
//! The two counter-rows — the ones that keep the row above from
//! passing by refusing everything — separate in **x** and in **z**.
//! Both directions are needed and neither is decorative: the cylinder
//! arm widens the reach box radially by construction and not at all
//! along its axis, so a rule that lost the axial extent, or grew it,
//! would keep an x-only file green.
//!
//! # 2. The NURBS extent re-gate's blocker
//!
//! `topo`'s fallback re-gate has no end-to-end path today, and one of
//! the two reasons is a `sweep` fact: a lofted body's NURBS EDGES are
//! refused before any face box is built. That blocker is pinned below,
//! with the operand's face class asserted so the row cannot outlive the
//! premise it argues from.
//!
//! # 3. The conic edge box as a PRUNE, through the sweep
//!
//! The boolean sweep's candidate generation reads each edge's box and
//! examines only the faces whose box it overlaps. A conic edge's box
//! is the exact extremal construction (`topo`'s `EdgeBoxRule`, its
//! conic arm), so it ends at the arc's own extreme — and the extreme
//! of this file's cylinder rims lies mid-arc, strictly between the
//! vertices. Both directions of that claim are pinned here through the
//! public doors: a pair whose loci meet is examined (soundness, the
//! differential suite's superset pin on a conic corpus), and a plate
//! clear of the rim by more than the pad is not (tightness — the
//! width the box does not have).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::operands::{
    nested_box, rim_plate, rounded_plate, small_box, three_arc_cylinder, top_rim_plate,
};
use geom_core::Tol;
use geom_core::{Affine3, Point2, Vec3};
use std::collections::BTreeSet;
use sweep::test_support::brick;
use topo::{
    Body, BooleanError, BooleanResult, ContactRecords, EntityId, FaceKey, SweepStrategy,
    SweepTrace, ValidationError, sweep_traces, validate_pseudomanifold,
};

/// The corpus cylinder, its base at `z0`, `height` tall (the rows'
/// cylinder is `cylinder(0.0, 1.0)`; the blind bore's tool is a raised
/// one): this suite poses it by lifting the sketch plane. The hull of
/// its six vertices is the inscribed triangular prism,
/// `x ∈ [−0.25, 0.5]`, `y ∈ [−0.433, 0.433]`.
fn cylinder(z0: f64, height: f64) -> Body<f64> {
    three_arc_cylinder(Point2::new(0.0, 0.0), 0.5, z0, height, 0.0)
}

/// The nested pair as one two-instance arena.
fn assembly(outer: &Body<f64>, inner: &Body<f64>) -> Body<f64> {
    let mut out = outer.clone();
    topo::graft_disjoint(&mut out, inner, Tol::witness()).unwrap();
    out
}

/// **The regression row.** A body wholly inside the cylinder, but
/// outside the hull of the cylinder's six vertices, must be REFUSED —
/// never cleared. Arm 1 refuses the cylinder wall × box-face pairs
/// (the proximity class), AND the material test decides the pair: the
/// box's vertices are strictly inside the cylinder's material, so the
/// arm reports the interference beside arm 1's refusals.
///
/// `cx` is swept across the whole annulus between the inscribed hull's
/// face (x = −0.25) and the true wall (x = −0.5), so the row goes red
/// for any rule that recovers only part of the outer solid's extent,
/// not merely for the one offset that happens to break the vertex
/// hull. Every probe is checked to be genuinely nested first, so a
/// refusal can never be explained by the probe poking out for real.
#[test]
fn a_body_nested_inside_a_curved_solid_is_never_silently_cleared() {
    let outer = cylinder(0.0, 1.0);
    let h = 0.05;
    for &cx in &[-0.22_f64, -0.26, -0.30, -0.35, -0.40] {
        // Genuinely inside the cylinder: the far corner is within r.
        let far = ((cx.abs() + h).powi(2) + h * h).sqrt();
        assert!(
            far < 0.5,
            "probe at {cx} is not nested (corner radius {far} ≥ 0.5)"
        );
        let inner = nested_box(cx, h);
        let body = assembly(&outer, &inner);
        // Tier 3 alone cannot see it — that is the #382 finding, and
        // the reason arm 2 exists at all.
        assert_eq!(
            topo::validate_geometric(&body, Tol::witness()),
            Ok(()),
            "probe at {cx}: tier 3 does not compare solids"
        );
        let errors = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness())
            .expect_err("a nested instance must refuse, never clear");
        // Arm 1's refusals stand (the box is within the wall's reach),
        // and an `In` vertex is decided whatever else stands.
        assert!(
            errors.iter().any(|e| matches!(
                e,
                ValidationError::CensusUndecidable {
                    a: EntityId::Face(_),
                    b: EntityId::Face(_),
                    what,
                } if what.contains("a curved face of one is within reach of the other")
            )),
            "probe at {cx}: arm 1 refuses the wall pairs first, got {errors:?}"
        );
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::InstanceInterference { .. })),
            "probe at {cx}: the material test decides the nested pair, got {errors:?}"
        );
        assert!(
            !errors.iter().any(|e| matches!(
                e,
                ValidationError::CensusUndecidable {
                    a: EntityId::Solid(_),
                    b: EntityId::Solid(_),
                    ..
                }
            )),
            "probe at {cx}: decided, so no solid-pair undecidable rides beside it: {errors:?}"
        );
    }
}

/// **A part in a blind BORE** (issue 750's fourth placement), MEASURED
/// against arm 1 rather than claimed: a 2 m block with a 0.5 m-radius
/// bore cut 0.5 m deep from its top, and a 0.2 m box floating in the
/// bore — inside the block's box, outside its material. The box's
/// planar faces and the bore's cylindrical wall are cross-solid faces
/// within reach with a curved side, which is arm 1's proximity class
/// and refuses before any material test; arm 2 then reads those
/// standing face-pair refusals as "this pair is not certified
/// crossing-free" and refuses its own examination typed. So the bore
/// does NOT clear today, and this row pins exactly that shape: arm
/// 1's face-pair refusals name the bore's wall, arm 2's refusal is
/// the precondition's, and no interference verdict and no clear
/// appear. The bore stays the exclusion ring's case (arm 1's), and
/// this row is what moves the day that ring lands.
#[test]
fn a_part_in_a_blind_bore_is_refused_by_arm_1_before_the_material_test() {
    let block = brick((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.0), Tol::witness());
    let tool = cylinder(0.5, 1.0);
    let BooleanResult::Body(bored) = topo::subtract(&block, &tool, Tol::witness()).unwrap() else {
        panic!("the bore cuts a body");
    };
    let bored = bored.body;
    assert_eq!(
        topo::validate_geometric(&bored, Tol::witness()),
        Ok(()),
        "the bored block is a sound single solid"
    );
    let bore_walls: Vec<FaceKey> = bored
        .faces()
        .filter(|(_, f)| {
            matches!(
                bored.get_surface(f.surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .map(|(k, _)| k)
        .collect();
    assert!(!bore_walls.is_empty(), "the bore has a cylindrical wall");
    let part = small_box(0.0, 0.1, 0.6);
    let body = assembly(&bored, &part);
    let errors = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness())
        .expect_err("measured: the bore refuses today");
    let arm1: Vec<&ValidationError> = errors
        .iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::CensusUndecidable {
                    a: EntityId::Face(_),
                    b: EntityId::Face(_),
                    ..
                }
            )
        })
        .collect();
    // Every arm-1 refusal is the proximity class, and the bore's
    // cylindrical wall is among the faces named (the block's planar
    // top and the bore's floor carry the bore's arc rims, so they are
    // arm 1's too and are named beside it).
    assert!(
        !arm1.is_empty()
            && arm1.iter().all(|e| match e {
                ValidationError::CensusUndecidable { what, .. } => {
                    what.contains("a curved face of one is within reach of the other")
                }
                _ => false,
            })
            && arm1.iter().any(|e| match e {
                ValidationError::CensusUndecidable {
                    a: EntityId::Face(a),
                    b: EntityId::Face(b),
                    ..
                } => bore_walls.contains(a) || bore_walls.contains(b),
                _ => false,
            }),
        "arm 1 refuses the wall × part-face pairs first: {errors:?}"
    );
    let arm2: Vec<&'static str> = errors
        .iter()
        .filter_map(|e| match e {
            ValidationError::CensusUndecidable {
                a: EntityId::Solid(_),
                b: EntityId::Solid(_),
                what,
            } => Some(*what),
            _ => None,
        })
        .collect();
    assert_eq!(arm2.len(), 1, "{errors:?}");
    assert!(arm2[0].contains("another finding"), "{}", arm2[0]);
    assert!(
        !errors
            .iter()
            .any(|e| matches!(e, ValidationError::InstanceInterference { .. })),
        "{errors:?}"
    );
    assert_eq!(
        errors.len(),
        arm1.len() + 1,
        "nothing else refuses: {errors:?}"
    );
}

/// The other direction, so the row above cannot pass by refusing
/// everything: a body clearly OUTSIDE the cylinder, and outside its
/// reach box, is still cleared by the containment arm.
#[test]
fn a_body_beside_the_cylinder_is_still_cleared_by_containment() {
    let outer = cylinder(0.0, 1.0);
    // The nested box only as a FAR operand: its lift carries no claim
    // here, so the `z` it shares with section 1 decides nothing.
    let beside = nested_box(3.0, 0.2);
    let body = assembly(&outer, &beside);
    // The whole verdict, not a filtered slice of it. Filtering to
    // `CensusUndecidable` and asserting that list empty makes a row
    // named "still cleared" green whenever the pair starts refusing for
    // some OTHER reason — the shape this file's section 2 exists about.
    let verdict = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness());
    assert!(
        verdict.is_ok(),
        "a separated pair must validate cleanly, not refuse: {verdict:?}"
    );
}

/// **The same claim in z, which nothing in this file separated in.**
/// The row above separates at `cx = 3.0` in **x** — the axis the
/// cylinder arm does not widen — so a reach box that grew along its
/// own AXIS would leave this file entirely green. Here the probe sits
/// directly over the cylinder's top cap, radially INSIDE it, so `z`
/// is the only axis that can clear the pair: the row goes red for any
/// rule that loses the containing solid's axial extent, and for any
/// that unbounds it.
///
/// **How far over, swept from touching to far.** The cylinder tops out
/// at `z = 1` and its reach box now stops there: the radius widens the
/// slab PERPENDICULAR to its own axis and not along it. So the whole
/// range a full-radius axial over-claim used to swallow — `z` just
/// above 1, where the probe is genuinely above the solid and touching
/// nothing — is assertable, and this row sweeps it rather than
/// standing clear of it. Any rule that widens the containing solid's
/// axial extent reds the near offsets; any that loses it reds the far
/// ones (the pair stops being clearable at all).
#[test]
fn a_body_above_the_cylinder_is_still_cleared_by_containment() {
    let outer = cylinder(0.0, 1.0);
    // Half-width 0.2 against radius 0.5: radially inside the wall, so
    // `z` is the only axis that can clear any of these pairs.
    for &z0 in &[1.01, 1.1, 1.25, 1.5, 2.0] {
        let above = small_box(0.0, 0.2, z0);
        let body = assembly(&outer, &above);
        let verdict = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness());
        assert!(
            verdict.is_ok(),
            "a pair separated in z alone (probe at z0 = {z0}, solid tops out at 1.0) \
             must validate cleanly, not refuse: {verdict:?}"
        );
    }
}

// ---------------------------------------------------------------------
// 2. The NURBS extent re-gate, end to end
// ---------------------------------------------------------------------

/// A lofted body: squares at z = 0 and z = 2 with a trapezoid between,
/// so its walls are genuine `Surface::Nurbs` with a real control net —
/// not the `mvfs` placeholder, whose net is poison.
fn lofted() -> Body<f64> {
    sweep::test_support::loft_prism(Tol::witness())
}

/// **Why `NurbsExtentUnsupported` has no end-to-end row, pinned so the
/// day that changes is loud.**
///
/// The fallback re-gate (`ops.rs`'s `sphere_extent_scan`) fires when a
/// NURBS-FACED operand reaches the sphere-extent fallback. To get
/// there an operand must survive the per-arm operand gate and produce
/// no crossings. No constructor mints such a body today:
///
/// - a lofted body has NURBS **edges**, and the gate refuses those
///   typed (`CurvedEdgeUnsupported`) before any face box is built —
///   this row;
/// - the `mvfs` placeholder surface gives NURBS faces with line edges,
///   but its control net is poison, so its box is poison, so it is
///   never pruned and meets the crossing layer first.
///
/// So the re-gate is defensive depth, pinned at the mechanism in
/// `topo`'s own suite. When a rung-3 operand gate admits NURBS edges
/// this row goes red, and whoever lifts it owes the end-to-end row.
#[test]
fn a_lofted_operand_is_refused_at_its_nurbs_edges_before_any_face_box() {
    let a = lofted();
    // The operand is IN the class the re-gate exists for. Nothing else
    // in the row says so: the refusal below is about its EDGES.
    let nurbs_faces = a
        .faces()
        .filter(|(_, f)| matches!(a.get_surface(f.surface), Some(geom::Surface::Nurbs(_))))
        .count();
    assert!(
        nurbs_faces > 0,
        "the lofted operand carries NURBS FACES — the class the re-gate exists for"
    );

    let b = nested_box(20.0, 0.5);
    let err = topo::boolean::union(&a, &b, Tol::witness())
        .expect_err("a NURBS operand must refuse typed");
    assert!(
        matches!(err, BooleanError::CurvedEdgeUnsupported { .. }),
        "the operand gate's edge arm is what a lofted body meets, got {err:?}"
    );
}

// ---------------------------------------------------------------------
// 3. The conic edge box as a PRUNE, through the sweep
// ---------------------------------------------------------------------

/// A plate straddling the TOP rim about its x-extremum — the second
/// fixture whose loci meet a rim mid-arc, so the through-the-door
/// soundness pin does not rest on one.
///
/// Deliberately NOT `common::operands`'s, though its two siblings are:
/// this suite is the only one that builds it, and a helper one suite
/// uses stays in that suite.
fn top_rim_x_plate(x_max: f64) -> Body<f64> {
    brick((-0.9, x_max), (-0.15, 0.15), (0.9, 1.1), Tol::witness())
}

/// The cylinder shifted along `x` by `1 + gap`: two rims a gap apart
/// at their nearest points, which are mid-arc on both.
fn cylinder_apart(gap: f64) -> Body<f64> {
    topo::transform_rigid(
        &cylinder(0.0, 1.0),
        &Affine3::translation(Vec3::new(1.0 + gap, 0.0, 0.0)),
        Tol::witness(),
    )
    .unwrap()
}

/// The conic corpus: (name, A, B), every B placed against an arc of A
/// — inside its hull, across its extreme, or clear of it by a stated
/// margin.
fn conic_corpus() -> Vec<(String, Body<f64>, Body<f64>)> {
    let cyl = cylinder(0.0, 1.0);
    let rounded = rounded_plate();
    let mut v = vec![
        (
            "cylinder × nested box".to_string(),
            cyl.clone(),
            nested_box(-0.3, 0.05),
        ),
        (
            "cylinder × box beside".to_string(),
            cyl.clone(),
            nested_box(3.0, 0.2),
        ),
        (
            "cylinder × plate across the rim's x-extreme".to_string(),
            cyl.clone(),
            rim_plate(-0.499),
        ),
        (
            "cylinder × plate across the top rim's x-extreme".to_string(),
            cyl.clone(),
            top_rim_x_plate(-0.499),
        ),
        // The same two crossings against the cylinder authored from its
        // 240° vertex. The Idealized reference refuses the crossings
        // above (`CurvedPierceUnsupported`) and accepts these, on the
        // same point set — the reference's answer depends on the rim
        // edges' minting order (filed:
        // `work/issues/idealized-sweep-refuses-a-rim-crossing-by-the-loops-authored-start.md`),
        // so the pin is held on both authorings and binds where the
        // reference answers.
        (
            "cylinder from 240° × plate across the rim's x-extreme".to_string(),
            three_arc_cylinder(Point2::new(0.0, 0.0), 0.5, 0.0, 1.0, 240.0),
            rim_plate(-0.499),
        ),
        (
            "cylinder from 240° × plate across the top rim's x-extreme".to_string(),
            three_arc_cylinder(Point2::new(0.0, 0.0), 0.5, 0.0, 1.0, 240.0),
            top_rim_x_plate(-0.499),
        ),
        (
            "cylinder × cylinder 1e-3 apart".to_string(),
            cyl.clone(),
            cylinder_apart(1e-3),
        ),
        (
            "rounded plate × box clear of the round".to_string(),
            rounded.clone(),
            brick((1.2, 1.6), (0.36, 0.6), (0.2, 0.5), Tol::witness()),
        ),
        (
            "rounded plate × box grazing the round".to_string(),
            rounded,
            brick((1.18, 1.6), (0.33, 0.6), (0.2, 0.5), Tol::witness()),
        ),
    ];
    for &x_max in &[-0.5003, -0.5006, -0.501, -0.502, -0.51, -0.6] {
        v.push((
            format!("cylinder × rim plate clear by {}", -0.5 - x_max),
            cyl.clone(),
            rim_plate(x_max),
        ));
    }
    for &y_min in &[0.5006, 0.502, 0.51] {
        v.push((
            format!("cylinder × top rim plate clear by {}", y_min - 0.5),
            cyl.clone(),
            top_rim_plate(y_min),
        ));
    }
    v
}

type Pair = (topo::EdgeKey, topo::FaceKey);

fn examined(t: &SweepTrace) -> BTreeSet<Pair> {
    t.examined.iter().copied().collect()
}

/// **Soundness, at the door that prunes.** On every corpus pair where
/// the brute-force reference runs, the realized sweep's candidate set
/// contains every pair the reference ACCEPTED an event on — per
/// direction. A conic edge box that ended short of its arc's extreme
/// would lose the plate across the rim's x-extreme, and the row says
/// so by name. Non-vacuous on two fixtures, not one: the plates across
/// the bottom and the top rim's x-extreme each accept events.
#[test]
fn conic_pruning_never_loses_an_accepted_pair() {
    let mut accepting: Vec<String> = Vec::new();
    for (name, a, b) in conic_corpus() {
        let real = sweep_traces(&a, &b, SweepStrategy::Realized, None, Tol::witness())
            .unwrap_or_else(|e| panic!("{name}: the realized sweep refused: {e:?}"));
        // The reference examines every pair, including ones the
        // realized sweep prunes, so it can meet a class the exact
        // lanes refuse typed; such a corpus row carries no pin.
        let Ok(ideal) = sweep_traces(&a, &b, SweepStrategy::Idealized, None, Tol::witness()) else {
            continue;
        };
        for (dir, r, i) in [("A→B", &real.0, &ideal.0), ("B→A", &real.1, &ideal.1)] {
            let ex = examined(r);
            let lost: Vec<Pair> = i
                .accepted
                .iter()
                .copied()
                .filter(|p| !ex.contains(p))
                .collect();
            assert!(
                lost.is_empty(),
                "{name} {dir}: the realized sweep never examined accepted pairs {lost:?}"
            );
            if !i.accepted.is_empty() && !accepting.contains(&name) {
                accepting.push(name.clone());
            }
        }
    }
    assert!(
        accepting.len() >= 2,
        "the pin rests on fewer than two accepting fixtures: {accepting:?}"
    );
}

/// **Tightness, at the same door.** A plate clear of the rim's extreme
/// by `3e-4` and more — and a second cylinder a gap of `1e-3` away —
/// is examined against NOTHING in either direction:
/// the rim's box ends at `x = −0.5` (`y = 0.5` on the top rim), and the
/// gap exceeds the sweep pad — `escalate + 2·zero` of the linear band
/// at the witness tolerance, which is `1.2e-5` at ε = 1e-6 and smaller
/// at every tighter row — so the pair is pruned at every ε the suite
/// runs. Non-vacuous by the plate across the extreme, which IS
/// examined. A box carrying a subdivision charge or a full-turn
/// amplitude examines the near plates, and that is the width the
/// exact form does not have.
#[test]
fn a_plate_clear_of_the_rim_by_more_than_the_pad_is_not_examined() {
    let cyl = cylinder(0.0, 1.0);
    let count = |b: &Body<f64>| {
        let (ab, ba) =
            sweep_traces(&cyl, b, SweepStrategy::Realized, None, Tol::witness()).unwrap();
        ab.examined.len() + ba.examined.len()
    };
    assert!(
        count(&rim_plate(-0.499)) > 0,
        "the plate across the extreme must be examined"
    );
    for &x_max in &[-0.5003, -0.5006, -0.501, -0.502, -0.51] {
        assert_eq!(
            count(&rim_plate(x_max)),
            0,
            "a rim plate clear by {} must be pruned outright",
            -0.5 - x_max
        );
    }
    for &y_min in &[0.5006, 0.502, 0.51] {
        assert_eq!(
            count(&top_rim_plate(y_min)),
            0,
            "a top rim plate clear by {} must be pruned outright",
            y_min - 0.5
        );
    }
    // Two cylinders whose rims come within 1e-3 of each other mid-arc:
    // the exact boxes end at each rim, so no face pair is examined.
    let apart = cylinder_apart(1e-3);
    let (ab, ba) =
        sweep_traces(&cyl, &apart, SweepStrategy::Realized, None, Tol::witness()).unwrap();
    assert_eq!(
        ab.examined.len() + ba.examined.len(),
        0,
        "two cylinders 1e-3 apart must be pruned outright"
    );
}
