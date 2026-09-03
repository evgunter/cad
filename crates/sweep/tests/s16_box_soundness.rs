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

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::{Affine3, Point2, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, Section, extrude, loft_body};
use topo::{
    Body, BooleanError, ContactRecords, EntityId, ValidationError, validate_pseudomanifold,
};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The three-arc cylinder: radius 0.5 about the z axis, `z ∈ [0, 1]`.
/// Six vertices; the hull of them is the inscribed triangular prism,
/// `x ∈ [−0.25, 0.5]`, `y ∈ [−0.433, 0.433]`.
fn cylinder() -> Body<f64> {
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th: f64 = deg.to_radians();
        p2(0.5 * th.cos(), 0.5 * th.sin())
    };
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(at(0.0), b120),
        ProfileVertex::new(at(120.0), b120),
        ProfileVertex::new(at(240.0), b120),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body
}

/// A small axis-aligned box of half-width `h` centred at `(cx, 0, ·)`,
/// spanning `z in [z0, z0 + 0.4]`.
fn small_box(cx: f64, h: f64, z0: f64) -> Body<f64> {
    let lp = ProfileLoop::new(
        [(cx - h, -h), (cx + h, -h), (cx + h, h), (cx - h, h)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    );
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(0.4), Tol::witness())
        .unwrap()
        .body
}

/// The same box at `z in [0.3, 0.7]`. Against section 1's cylinder
/// (`z in [0, 1]`) that is clear of both caps, so a pair there is
/// nested or separated in x alone, never touching; section 2 uses the
/// same box only as a far operand, where the lift carries no claim.
fn nested_box(cx: f64, h: f64) -> Body<f64> {
    small_box(cx, h, 0.3)
}

/// The nested pair as one two-instance arena.
fn assembly(outer: &Body<f64>, inner: &Body<f64>) -> Body<f64> {
    let mut out = outer.clone();
    topo::graft_disjoint(&mut out, inner, Tol::witness()).unwrap();
    out
}

/// **The regression row.** A body wholly inside the cylinder, but
/// outside the hull of the cylinder's six vertices, must be REFUSED as
/// the C6 interference class — never cleared.
///
/// `cx` is swept across the whole annulus between the inscribed hull's
/// face (x = −0.25) and the true wall (x = −0.5), so the row goes red
/// for any rule that recovers only part of the outer solid's extent,
/// not merely for the one offset that happens to break the vertex
/// hull. Every probe is checked to be genuinely nested first, so a
/// refusal can never be explained by the probe poking out for real.
#[test]
fn a_body_nested_inside_a_curved_solid_is_never_silently_cleared() {
    let outer = cylinder();
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
        assert!(
            errors.iter().any(|e| matches!(
                e,
                ValidationError::CensusUndecidable {
                    a: EntityId::Solid(_),
                    b: EntityId::Solid(_),
                    ..
                }
            )),
            "probe at {cx}: the containment arm must name the solid pair, got {errors:?}"
        );
    }
}

/// The other direction, so the row above cannot pass by refusing
/// everything: a body clearly OUTSIDE the cylinder, and outside its
/// reach box, is still cleared by the containment arm.
#[test]
fn a_body_beside_the_cylinder_is_still_cleared_by_containment() {
    let outer = cylinder();
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
    let outer = cylinder();
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
    let quad = |pts: [(f64, f64); 4]| -> Section {
        vec![ProfileLoop::polygon(pts.iter().map(|&(x, y)| p2(x, y)))]
    };
    let square = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let trapezoid = [(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let sections = vec![quad(square), quad(trapezoid), quad(square)];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
    ];
    loft_body::<f64>(&sections, &places, 2, Tol::witness())
        .unwrap()
        .body
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
