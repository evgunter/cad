//! **The opening probe** for the cylinder×cylinder germ lane: which
//! door refuses #347's cases, measured rather than assumed, on bodies
//! this suite authors through the public extrude door.
//!
//! Two families, and they no longer share a fate:
//!
//! - #347's cylinder unions (coaxial, parallel, Steinmetz) STILL refuse
//!   at `CurvedPierceUnsupported` — the curved sweep arm's frontier.
//!   That is the crossing layer, not the join, and opening it needs a
//!   pierce/split substrate that is its own unit; the rows below pin
//!   the refusals so that unit starts from a measurement.
//! - #347's bracket bound is GONE. It used to read `r ≤ 4` passes,
//!   `r ≥ 5` refuses — exactly `2r > 8`, the corner round's CARRIER
//!   reaching the pocket's `x = 8` wall while its ARC stayed 2 mm
//!   clear. Both halves of that are retired: the rim arc and the wall
//!   face are boxed by the arc they occupy rather than the circle they
//!   ride, and the line-clearance dip clamps its vertex to the
//!   segment. Every radius cuts, and the rows meter the result.
//!
//! One row is the **no-crossings silence**: a cylinder pair whose walls
//! cross in one closed loop touching no edge of either operand reaches
//! the vertex probe with no extent certificate, and refuses typed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A circle-derived cylinder: circle (cx, cy) of radius r, extruded
/// from z0 to z1 — exactly #347's "`circle`-derived cylinder".
fn cyl(cx: f64, cy: f64, r: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(p2(cx, cy), r, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(z1 - z0), tol)
        .unwrap()
        .body
}

fn union_err(a: &Body<f64>, b: &Body<f64>) -> BooleanError {
    topo::union(a, b, Tol::witness()).expect_err("this pair has no arm yet")
}

/// The carrier kind of the edge a refusal names — the datum that says
/// WHICH family a row belongs to, since the line × wall roots exist in
/// closed form and the circle × wall roots do not exist anywhere.
fn refused_carrier(body: &Body<f64>, err: &BooleanError) -> &'static str {
    let BooleanError::CurvedPierceUnsupported { edge, .. } = err else {
        panic!("not a pierce refusal: {err:?}");
    };
    let Some(topo::CurveGeom::Certified(c)) = body
        .get_edge(*edge)
        .and_then(|e| body.get_curve_geom(e.curve))
    else {
        panic!("the named edge has no certified curve");
    };
    match c.carrier() {
        topo::Curve3::Line { .. } => "line",
        topo::Curve3::Circle { .. } => "circle",
        _ => "other",
    }
}

/// #347's "two `circle`-derived cylinders refuse to union at all
/// (coaxial or not)": every crossing pose meets the CURVED SWEEP ARM's
/// frontier — the pierce door, not a kind gate and not a join refusal.
///
/// **The four rows are not one family, and the ring lane moves none of
/// them.** Each is named with the pair that actually raises, measured
/// rather than inferred, and with the carrier kind pinned because that
/// is what decides whose work the row is waiting on:
///
/// 1. `coaxial-equal-r` — A's rim CIRCLE lies on B's wall carrier. An
///    undeclared value-coincident contact; CONTACT-DESIGN C2/C4 forbid
///    inferring the gluing at any ε, so its destination is the
///    declaration ladder and no crossing or join arm moves it.
/// 2. `coaxial-stacked` — A's seam LINE lies on B's wall carrier
///    (residual identically zero). Same class, same destination; the
///    binding coincidence is the cap discs, a plane × plane rest.
/// 3. `parallel-equal-r` — A's rim CIRCLE genuinely crosses B's wall.
///    A real pierce, and the one this lane cannot serve: the circle ×
///    wall event parameters are the roots of a degree-2 TRIGONOMETRIC
///    polynomial, and no quartic, cubic or resolvent lane exists in
///    this tree at all.
/// 4. `steinmetz` — A's seam RULING is TANGENT to B's wall. The two
///    walls meet in two ellipses that CROSS at `(±1, 0, 0)`, where the
///    surfaces are mutually tangent, and the extruded circle puts its
///    seam at azimuth 0 and π — exactly on those two singular points.
///    The clearance there is exactly zero, and the dip bound is not
///    loose about it: `m = 0` centres the parabola's vertex, where the
///    charge `q/8` IS the true dip. A tangency is not a crossing at any
///    order the pierce machinery reads, so the ring lane leaves it
///    where it is; what moves this row is a second-order lane or a
///    declaration, not an arm.
///
/// Rows 1–2 and row 4 all refuse for reasons the ARMS unit does not
/// touch either, which is why this table pins the reason and not just
/// the variant.
#[test]
fn cylinder_unions_refuse_at_the_curved_pierce_door() {
    let turned = topo::transform_rigid(
        &cyl(0.0, 0.0, 1.0, -2.0, 2.0),
        &Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            PI / 2.0,
        ),
        Tol::witness(),
    )
    .unwrap();
    let a = cyl(0.0, 0.0, 1.0, 0.0, 2.0);
    let a_tall = cyl(0.0, 0.0, 1.0, -2.0, 2.0);
    let rows: [(&str, BooleanError); 4] = [
        // Coaxial, equal radius, overlapping heights: B's rim circles
        // lie ON A's wall carrier, so the circle row's residual
        // extremes are Zero and the incidence is undeclared.
        (
            "coaxial-equal-r",
            union_err(&a, &cyl(0.0, 0.0, 1.0, 1.0, 3.0)),
        ),
        // Coaxial, equal radius, stacked cap-to-cap.
        (
            "coaxial-stacked",
            union_err(&a, &cyl(0.0, 0.0, 1.0, 2.0, 4.0)),
        ),
        // Parallel axes, definitely crossing walls.
        (
            "parallel-equal-r",
            union_err(&a, &cyl(1.2, 0.0, 1.0, 0.0, 2.0)),
        ),
        // Perpendicular axes, equal radius (the Steinmetz pair).
        ("steinmetz", union_err(&a_tall, &turned)),
    ];
    // The carrier of the edge each row names — the family datum (doc
    // above): a `line` row waits on second-order or declaration work, a
    // `circle` row waits on the trigonometric root lane that does not
    // exist.
    let carriers = ["circle", "line", "circle", "line"];
    for ((name, err), want) in rows.into_iter().zip(carriers) {
        assert!(
            matches!(err, BooleanError::CurvedPierceUnsupported { .. }),
            "{name}: expected the curved pierce door, got {err:?}"
        );
        // Every row names an edge of operand A (the refusals are
        // measured to be A-side), so A is the body the carrier is read
        // from; `coaxial-*` and `parallel-*` share one A, `steinmetz`
        // has the tall one.
        let owner = if name == "steinmetz" { &a_tall } else { &a };
        assert_eq!(refused_carrier(owner, &err), want, "{name}");
    }
}

/// The COAXIAL UNEQUAL-radius pose (a boss on a shaft) is not a pierce
/// case, and it unions. Metered rather than asserted `is_ok`, because
/// a door that opened onto a wrong body would pass that check: the
/// closed form is a shaft plus the boss's protruding stub,
/// `π·1²·2 + π·0.5²·1`.
///
/// The two rows behind it are the cap's: a `Circle` boundary is
/// decided by its exact arc rows, not by a chord that runs through the
/// disc; and a loop of arcs of one circle is decided by its radius,
/// not by the polygon through its two vertices.
#[test]
fn the_coaxial_boss_unions_and_meters_at_the_closed_form() {
    let tol = Tol::witness();
    let topo::BooleanResult::Body(out) = topo::union(
        &cyl(0.0, 0.0, 1.0, 0.0, 2.0),
        &cyl(0.0, 0.0, 0.5, 1.0, 3.0),
        tol,
    )
    .expect("the boss unions") else {
        panic!("a boss on a shaft is one solid");
    };
    assert_eq!(topo::validate_geometric(&out.body, tol), Ok(()), "tier 3");
    let v = topo::mass_properties(&out.body, tol).unwrap().volume;
    let truth = PI * 2.0 + PI * 0.25;
    assert!((v - truth).abs() < 1e-12, "{v} vs {truth}");
}

/// **#347's bound is GONE, and the flip is this row.** It used to
/// assert what the substrate measured: the pocket cut passed at
/// `r ≤ 4 mm` and refused at `r ≥ 5 mm`, which is exactly `2r > 8` —
/// the corner round's CARRIER reaching the pocket's `x = 8` wall while
/// the round's own ARC stayed 2 mm clear. Two conservatisms in series
/// produced that: the rim arc and the wall face were both boxed by the
/// whole circle they ride, which made the pocket edge a candidate at
/// all; and the line-clearance dip charged a centred-vertex dip to an
/// edge whose nearest approach is an endpoint.
///
/// Both are trim-scoped now, so every radius cuts — and the row meters
/// the RESULT rather than merely asserting the absence of a refusal: a
/// door that opened onto a wrong body would pass an `is_ok` check.
/// The closed form is `bracket.py`'s own: an 80×40 plate less what four
/// corner rounds of radius `r` take off, times 8 thick, less the
/// pocket's 5 mm bite.
#[test]
fn the_bracket_rounds_at_every_radius_and_meters_exactly() {
    let tol = Tol::witness();
    for r in [3.0_f64, 4.0, 5.0, 6.0] {
        let plate = rounded_plate(80.0, 40.0, r, 8.0);
        let pocket = slab((8.0, 28.0), (10.0, 30.0), (-2.0, 5.0));
        let out = topo::subtract(&plate, &pocket, tol)
            .unwrap_or_else(|e| panic!("r = {r} mm must cut: {e:?}"));
        let topo::BooleanResult::Body(bb) = out else {
            panic!("r = {r} mm: the cut cannot empty the plate");
        };
        let v = topo::mass_properties(&bb.body, tol).unwrap().volume;
        // Plate area less the four corner bites, times the thickness,
        // less the pocket's 20×20×5 bite.
        let expect =
            (80.0 * 40.0 - r * r * (4.0 - core::f64::consts::PI)) * 8.0 - 20.0 * 20.0 * 5.0;
        assert!(
            (v - expect).abs() < 1e-9,
            "r = {r} mm: metered {v}, closed form {expect}"
        );
    }
}

/// #347's own headline radius, called out on its own so the issue can
/// be closed against a named row: **the bracket rounds at 6 mm.**
#[test]
fn the_bracket_rounds_at_six_millimetres() {
    let tol = Tol::witness();
    let out = topo::subtract(
        &rounded_plate(80.0, 40.0, 6.0, 8.0),
        &slab((8.0, 28.0), (10.0, 30.0), (-2.0, 5.0)),
        tol,
    )
    .expect("#347's requested radius cuts");
    assert!(matches!(out, topo::BooleanResult::Body(_)));
}

/// **The no-crossings silence, and the posture that closes it.**
///
/// The pair below genuinely interpenetrates, yet no vertex of either
/// operand is inside the other and no edge of either meets a face of
/// the other. Before D10 the containment fallback kept both shells
/// whole and metered the union as two DISJOINT solids — volume exactly
/// the SUM of the operands', the shared lens counted twice, a WRONG
/// ANSWER rather than a refusal. The sum is asserted here so the row
/// still carries the measurement it was opened with.
///
/// It now refuses typed at the curved-extent scan's wall×wall gate.
///
/// **The base reproduction, recorded here rather than as a row.** At
/// this unit's merge base the same two bodies returned
/// `BooleanResult::Body` from `union`, with `shells().count() == 2` and
/// `mass_properties(..).volume == 94.2477796076938` — 30π to the last
/// digit, i.e. exactly `va + vb`, with the interpenetrating lens
/// contributing nothing. The independent review reproduced it at base.
/// It cannot be a test row on this branch, because the row would have
/// to ASSERT the wrong answer; the sum is asserted below instead, so
/// the yardstick that made the defect visible still ships with the
/// fix.
#[test]
fn a_fully_crossing_cylinder_pair_with_no_edge_event_refuses_typed() {
    let tol = Tol::witness();
    let (a, b) = crossing_pair_without_edge_events();
    let va = topo::mass_properties(&a, tol).unwrap().volume;
    let vb = topo::mass_properties(&b, tol).unwrap().volume;
    // The wrong answer the silence used to give, kept as the row's own
    // yardstick: 10π + 20π, the lens counted twice.
    assert!((va + vb - 30.0 * core::f64::consts::PI).abs() < 1e-9);
    let err = topo::union(&a, &b, tol).expect_err("the silence never re-opens");
    let BooleanError::FallbackExtentUnsupported { what, .. } = err else {
        panic!("expected the extent scan's wall pair gate, got {err:?}");
    };
    assert!(what.contains("two cylinder walls"), "{what}");
}

/// Cylinder operands the gate must NOT touch: two walls standing clear
/// of each other still answer, because non-overlapping boxes prove the
/// boundaries do not meet. The gate is reach-first, so the ordinary
/// disjoint-operands fallback keeps working.
#[test]
fn cylinders_standing_clear_of_each_other_still_answer() {
    let tol = Tol::witness();
    let a = cyl(0.0, 0.0, 1.0, 0.0, 2.0);
    let b = cyl(5.0, 0.0, 1.0, 0.0, 2.0);
    let topo::BooleanResult::Body(out) = topo::union(&a, &b, tol).unwrap() else {
        panic!("two disjoint solids union into a two-shell body");
    };
    assert_eq!(out.body.shells().count(), 2);
    let v = topo::mass_properties(&out.body, tol).unwrap().volume;
    assert!((v - 4.0 * core::f64::consts::PI).abs() < 1e-9, "{v}");
}

/// The pair whose walls cross in ONE closed loop that reaches no edge
/// of either operand — A's seam meridians sit at y = 0, outside the
/// loop's θ band, and B's at z = 4 and z = 6, clear of A entirely. The
/// reduction therefore finds no crossing at all and the operation falls
/// through to the containment fallback with the boundaries genuinely
/// meeting: the S12-silence shape, for a cylinder pair.
pub(crate) fn crossing_pair_without_edge_events() -> (Body<f64>, Body<f64>) {
    let tol = Tol::witness();
    let a = cyl(0.0, 0.0, 1.0, 0.0, 10.0);
    let rod = cyl(0.0, 0.0, 1.0, -10.0, 10.0);
    let turn = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        PI / 2.0,
    );
    let lie = topo::transform_rigid(&rod, &turn, tol).unwrap();
    let b =
        topo::transform_rigid(&lie, &Affine3::translation(Vec3::new(0.0, 1.5, 5.0)), tol).unwrap();
    (a, b)
}

/// `bracket.py`'s `rounded_plate`, in millimetres.
fn rounded_plate(w: f64, h: f64, r: f64, thick: f64) -> Body<f64> {
    let tol = Tol::witness();
    let outline = profile::Open
        .at(p2(w / 2.0, 0.0))
        .toward(1.0, 0.0, tol)
        .unwrap()
        .fillet(r, tol)
        .unwrap()
        .toward(0.0, 1.0, tol)
        .unwrap()
        .to(p2(w, h / 2.0), tol)
        .unwrap()
        .fillet(r, tol)
        .unwrap()
        .toward(-1.0, 0.0, tol)
        .unwrap()
        .to(p2(w / 2.0, h), tol)
        .unwrap()
        .fillet(r, tol)
        .unwrap()
        .toward(0.0, -1.0, tol)
        .unwrap()
        .to(p2(0.0, h / 2.0), tol)
        .unwrap()
        .fillet(r, tol)
        .unwrap()
        .to(profile::Start, tol)
        .unwrap();
    let plane = SketchPlane::new(Affine3::identity());
    let prof = Profile::new(plane, vec![outline.into()])
        .validate(tol)
        .unwrap();
    extrude(&prof, Extrusion::Distance(thick), tol)
        .unwrap()
        .body
}

/// `bracket.py`'s `slab`, in millimetres.
fn slab(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    let tol = Tol::witness();
    let lp =
        profile::ProfileLoop::polygon([p2(x.0, y.0), p2(x.1, y.0), p2(x.1, y.1), p2(x.0, y.1)]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z.0)));
    let prof = Profile::new(plane, vec![lp]).validate(tol).unwrap();
    extrude(&prof, Extrusion::Distance(z.1 - z.0), tol)
        .unwrap()
        .body
}

// ---------------------------------------------------------------- D3 --
//
// The curved containment door, pinned in BOTH directions on a real
// cylinder chart. The operand is an extruded circle, so its wall is two
// half-cylinder faces: azimuth window a half turn wide, height range
// [z0, z1], and boundary edges that are rim arcs and seam meridians —
// exactly the iso-bounded class the trim is exact for.

/// The wall faces of `body`, in face-arena order.
fn wall_faces(body: &Body<f64>) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// A point on the unit cylinder about z at azimuth `theta`, height `h`.
fn on_wall(theta: f64, h: f64) -> Point3<f64> {
    Point3::new(theta.cos(), theta.sin(), h)
}

#[test]
fn the_containment_door_answers_both_directions_on_a_cylinder_chart() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let body = cyl(0.0, 0.0, 1.0, 0.0, 2.0);
    let walls = wall_faces(&body);
    assert_eq!(walls.len(), 2, "an extruded circle mints two half-walls");

    // Every azimuth strictly inside ONE of the two windows, at a height
    // strictly inside the band: the point is IN exactly one wall face
    // and OUT of the other.
    for theta in [0.4_f64, 1.2, 2.4, 3.6, 4.8, 6.0] {
        let q = on_wall(theta, 1.0);
        let verdicts: Vec<_> = walls
            .iter()
            .map(|&f| topo::curved_face_containment(&body, f, q, band).unwrap())
            .collect();
        let ins = verdicts
            .iter()
            .filter(|v| **v == Some(topo::FaceContainment::In))
            .count();
        let outs = verdicts
            .iter()
            .filter(|v| **v == Some(topo::FaceContainment::Out))
            .count();
        assert_eq!(
            (ins, outs),
            (1, 1),
            "θ = {theta}: exactly one wall holds the point, got {verdicts:?}"
        );
    }

    // ON the carrier but ABOVE and BELOW the height band: out of both.
    for h in [-0.5_f64, 2.5] {
        for &f in &walls {
            assert_eq!(
                topo::curved_face_containment(&body, f, on_wall(0.4, h), band).unwrap(),
                Some(topo::FaceContainment::Out),
                "h = {h} is outside the wall's height range"
            );
        }
    }

    // OFF the carrier entirely (the surface's own residual definitely
    // non-zero): out of both, from the trim's radial row.
    for r in [0.5_f64, 1.5] {
        let q = Point3::new(r * 0.4_f64.cos(), r * 0.4_f64.sin(), 1.0);
        for &f in &walls {
            assert_eq!(
                topo::curved_face_containment(&body, f, q, band).unwrap(),
                Some(topo::FaceContainment::Out),
                "radius {r} is off the wall's carrier"
            );
        }
    }
}

/// The BAND EDGES: a point on a rim, on a seam meridian, or on a
/// boundary vertex is a BOUNDARY answer or the honest `None` — never
/// an interior/exterior verdict the trim cannot stand behind.
#[test]
fn the_containment_door_reports_the_trim_boundary_rather_than_guessing() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let body = cyl(0.0, 0.0, 1.0, 0.0, 2.0);
    let walls = wall_faces(&body);

    // A rim point (height exactly at a band edge, azimuth interior):
    // the boundary walk's CIRCLE arm names the rim edge it lies on.
    for h in [0.0_f64, 2.0] {
        let q = on_wall(1.0, h);
        let named = walls.iter().any(|&f| {
            matches!(
                topo::curved_face_containment(&body, f, q, band).unwrap(),
                Some(topo::FaceContainment::OnEdge(_))
            )
        });
        assert!(named, "a rim point at h = {h} must name the rim edge");
    }

    // A seam-meridian point (azimuth exactly at a window edge, height
    // interior): the LINE arm names the meridian edge.
    for theta in [0.0_f64, core::f64::consts::PI] {
        let q = on_wall(theta, 1.0);
        let named = walls.iter().any(|&f| {
            matches!(
                topo::curved_face_containment(&body, f, q, band).unwrap(),
                Some(topo::FaceContainment::OnEdge(_))
            )
        });
        assert!(named, "a seam point at θ = {theta} must name the meridian");
    }

    // A boundary VERTEX (both band edges at once): the vertex pass wins,
    // ahead of either edge arm.
    for (theta, h) in [(0.0_f64, 0.0_f64), (core::f64::consts::PI, 2.0)] {
        let q = on_wall(theta, h);
        let named = walls.iter().any(|&f| {
            matches!(
                topo::curved_face_containment(&body, f, q, band).unwrap(),
                Some(topo::FaceContainment::OnVertex(_))
            )
        });
        assert!(named, "a corner at (θ = {theta}, h = {h}) is a vertex hit");
    }
}

/// The class gate: a wall the chart trim CANNOT express answers `None`,
/// never a verdict. A tilted cut leaves the wall bounded by a section
/// ELLIPSE, whose height extreme is interior to an edge, so the
/// rectangle the boundary vertices pin misstates the face in both
/// directions — and the door refuses to speak rather than read it.
#[test]
fn a_wall_the_trim_cannot_express_gets_no_verdict() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let post = cyl(0.0, 0.0, 1.0, 0.0, 2.0);
    let phi = 0.3_f64;
    let plane = topo::splitting::SplitPlane {
        origin: Point3::new(0.0, 0.0, 1.0),
        normal: Vec3::new(phi.sin(), 0.0, phi.cos()),
    };
    let result = topo::splitting::split(&post, &plane, tol).unwrap();
    let topo::splitting::SplitPart::Body(below) = &result.below else {
        panic!("the tilted cut leaves material below");
    };
    let walls = wall_faces(below);
    assert!(!walls.is_empty(), "the cut post still has wall faces");
    // On the carrier, well inside the surviving stub: an iso-bounded
    // wall answers In or Out here; an ellipse-bounded one must not.
    let verdicts: Vec<_> = walls
        .iter()
        .map(|&f| topo::curved_face_containment(below, f, on_wall(0.4, 0.5), band).unwrap())
        .collect();
    assert!(
        verdicts.iter().all(Option::is_none),
        "a wall closed by a tilted section must get no verdict, got {verdicts:?}"
    );
}

/// **The dip clamp's own row.** The blinded review found the clamp
/// shipped unpinned: reverting `bool_line_cylinder_clearance`'s charge
/// to the unconditional `q/8` left the whole tree green, because the
/// bracket the clamp was written for stopped being a candidate pair
/// once the boxes were trim-scoped. Two fixes for one defect, and each
/// hid the other's evidence.
///
/// This is the bracket's shape reduced until only the clamp decides. A
/// unit wall, and a brick whose lower edges run radially away from it
/// starting just outside the surface at `y = 0.999`:
///
/// - the boxes DO overlap (the brick's edge starts at `x = 0.5`, inside
///   the wall's `x ∈ [−1, 1]`), so the pair is genuinely examined;
/// - the line's nearest approach to the axis is its START — the
///   parabola's vertex sits at `x = 0`, outside the span `[0.5, 2.5]` —
///   so the true clearance is the endpoint residual, about 0.124 m;
/// - the endpoint gap `m ≈ 3.0` exceeds `q/2 = 2.0`, so the clamp
///   charges EXACTLY ZERO and the clearance stands;
/// - the unconditional `q/8 = 0.5` would swamp 0.124 and refuse.
///
/// So this row reds — with `CurvedPierceUnsupported` — the moment the
/// clamp is reverted, and it is the only row in the tree that does.
#[test]
fn the_line_clearance_clamp_is_what_lets_a_radial_edge_clear() {
    let tol = Tol::witness();
    let wall = cyl(0.0, 0.0, 1.0, 0.0, 2.0);
    let lp =
        profile::ProfileLoop::polygon([p2(0.5, 0.999), p2(2.5, 0.999), p2(2.5, 3.0), p2(0.5, 3.0)]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.9)));
    let brick = extrude(
        &Profile::new(plane, vec![lp]).validate(tol).unwrap(),
        Extrusion::Distance(0.2),
        tol,
    )
    .unwrap()
    .body;
    let out = topo::union(&wall, &brick, tol)
        .expect("the radial edge clears the wall; only the clamp proves it");
    let topo::BooleanResult::Body(bb) = out else {
        panic!("two disjoint solids union into a body");
    };
    // Disjoint operands, so the volumes add: π·1²·2 + 2.0·2.001·0.2.
    let v = topo::mass_properties(&bb.body, tol).unwrap().volume;
    let expect = core::f64::consts::PI * 2.0 + 2.0 * 2.001 * 0.2;
    assert!((v - expect).abs() < 1e-9, "metered {v}, expected {expect}");
}
