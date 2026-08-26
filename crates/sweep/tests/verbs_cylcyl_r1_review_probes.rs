//! Reviewer consumer probes for the PR-A substrate (blinded review of
//! the germ-lane substrate unit). Every row is RED-able: it pins a
//! behavior the unit claims, phrased so a regression flips it.
//!
//! The E2E posture: author crossing-cylinder bodies in varied poses and
//! demand that every outcome is a TYPED refusal or a correct answer —
//! the substrate must never reproduce the D10 wrong answer (a union
//! metered as the SUM of two interpenetrating operands).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError};

fn cyl(cx: f64, cy: f64, r: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(cx, cy), r, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(z1 - z0), tol)
        .unwrap()
        .body
}

fn turned(b: &Body<f64>, axis: Vec3<f64>, angle: f64) -> Body<f64> {
    topo::transform_rigid(
        b,
        &Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), axis, angle),
        Tol::witness(),
    )
    .unwrap()
}

fn moved(b: &Body<f64>, d: Vec3<f64>) -> Body<f64> {
    topo::transform_rigid(b, &Affine3::translation(d), Tol::witness()).unwrap()
}

/// Every crossing pose this reviewer could author must refuse TYPED —
/// and if any ever answers, the answer must not be the operand-volume
/// sum (the D10 double-count). Varied radii, axes, heights, offsets.
#[test]
fn every_reachable_crossing_pose_refuses_typed_or_answers_correctly() {
    let tol = Tol::witness();
    let poses: Vec<(&str, Body<f64>, Body<f64>)> = vec![
        (
            "perpendicular unequal radii",
            cyl(0.0, 0.0, 2.0, -3.0, 3.0),
            moved(
                &turned(
                    &cyl(0.0, 0.0, 0.7, -4.0, 4.0),
                    Vec3::new(1.0, 0.0, 0.0),
                    PI / 2.0,
                ),
                Vec3::new(0.0, 0.0, 0.0),
            ),
        ),
        (
            "skew axes, crossing walls",
            cyl(0.0, 0.0, 1.0, 0.0, 6.0),
            moved(
                &turned(
                    &cyl(0.0, 0.0, 0.8, -5.0, 5.0),
                    Vec3::new(1.0, 0.0, 0.0),
                    0.7,
                ),
                Vec3::new(0.9, 0.0, 3.0),
            ),
        ),
        (
            "no-edge-event lens, unequal radii",
            cyl(0.0, 0.0, 1.0, 0.0, 10.0),
            moved(
                &turned(
                    &cyl(0.0, 0.0, 0.6, -10.0, 10.0),
                    Vec3::new(0.0, 1.0, 0.0),
                    PI / 2.0,
                ),
                Vec3::new(0.0, 1.2, 5.0),
            ),
        ),
        (
            "parallel axes, shallow overlap",
            cyl(0.0, 0.0, 1.5, 0.0, 4.0),
            cyl(2.9, 0.0, 1.5, 1.0, 5.0),
        ),
        (
            "externally tangent walls",
            cyl(0.0, 0.0, 1.0, 0.0, 2.0),
            cyl(2.0, 0.0, 1.0, 0.0, 2.0),
        ),
        (
            "tall thin through short fat",
            cyl(0.0, 0.0, 0.2, -8.0, 8.0),
            moved(
                &turned(
                    &cyl(0.0, 0.0, 3.0, -1.0, 1.0),
                    Vec3::new(0.0, 1.0, 0.0),
                    PI / 2.0,
                ),
                Vec3::new(0.0, 0.0, 0.0),
            ),
        ),
    ];
    for (name, a, b) in &poses {
        let va = topo::mass_properties(a, tol).unwrap().volume;
        let vb = topo::mass_properties(b, tol).unwrap().volume;
        for (op_name, out) in [
            ("union", topo::union(a, b, tol)),
            ("subtract", topo::subtract(a, b, tol)),
            ("intersect", topo::intersect(a, b, tol)),
        ] {
            match out {
                Err(_) => {} // a typed refusal is an honest outcome
                Ok(topo::BooleanResult::Body(body)) => {
                    let v = topo::mass_properties(&body.body, tol).unwrap().volume;
                    // The one FORBIDDEN outcome: the interpenetrating
                    // union metered as the disjoint sum.
                    if op_name == "union" {
                        assert!(
                            (v - (va + vb)).abs() > 1e-6,
                            "{name}: union answered the DISJOINT SUM {v} for an \
                             interpenetrating pair — the D10 wrong answer"
                        );
                    }
                    panic!(
                        "{name}/{op_name}: answered OK (volume {v}) — no cyl×cyl arm is \
                         wired in PR-A, so an answer here needs its own audit"
                    );
                }
                Ok(other) => panic!("{name}/{op_name}: unexpected non-body result {other:?}"),
            }
        }
    }
}

/// The C6 capability cost, pinned honest: a coaxial NESTED pair (one
/// wholly inside the other) now refuses at the wall-pair extent gate.
/// Before PR-A the containment fallback answered it correctly by luck;
/// the refusal must be the gate's own typed door, not a wrong answer.
#[test]
fn the_nested_coaxial_pair_refuses_at_the_wall_pair_gate() {
    let tol = Tol::witness();
    let inner = cyl(0.0, 0.0, 1.0, 1.0, 3.0);
    let outer = cyl(0.0, 0.0, 2.0, 0.0, 4.0);
    for (a, b) in [(&inner, &outer), (&outer, &inner)] {
        let err = topo::union(a, b, tol).expect_err("the nested pair refuses under D10");
        let BooleanError::FallbackExtentUnsupported { what, .. } = err else {
            panic!("expected the wall-pair extent gate, got {err:?}");
        };
        assert!(what.contains("two cylinder walls"), "{what}");
    }
}

/// The gate's conservatism inherits the AABB of the CARRIER slab: two
/// parallel cylinders diagonally offset — axes 2.69 apart, a 0.69 m
/// true gap, robustly disjoint — refuse because their axis-aligned
/// boxes still overlap at a corner. The sharp edge of the stated
/// capability cost: an honest refusal, never a wrong answer, but a
/// refusal on a pose the old fallback answered correctly. If the gate
/// ever narrows to trimmed-wall reach, this row flips to the correct
/// two-shell answer and should be updated, loudly.
#[test]
fn a_diagonally_offset_disjoint_pair_now_refuses_at_the_gate() {
    let tol = Tol::witness();
    let a = cyl(0.0, 0.0, 1.0, 0.0, 2.0);
    let b = cyl(1.9, 1.9, 1.0, 0.0, 2.0);
    match topo::union(&a, &b, tol) {
        Err(BooleanError::FallbackExtentUnsupported { what, .. }) => {
            assert!(what.contains("two cylinder walls"), "{what}");
        }
        Err(e) => panic!("expected the extent gate, got {e:?}"),
        Ok(topo::BooleanResult::Body(body)) => {
            let v = topo::mass_properties(&body.body, tol).unwrap().volume;
            assert!((v - 4.0 * PI).abs() < 1e-9, "two disjoint units: {v}");
            panic!("the box gate no longer fires on the diagonal pose: update this row");
        }
        Ok(other) => panic!("unexpected {other:?}"),
    }
}

/// D3's carrier gate probed at MANY radii: radially-off points must be
/// `Out` from every wall face, at every height inside the band — the
/// off-carrier `In` bug's fix, swept wider than the unit's own pins.
#[test]
fn off_carrier_points_are_out_at_every_probed_radius() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let body = cyl(0.0, 0.0, 1.0, 0.0, 2.0);
    let walls: Vec<topo::FaceKey> = body
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(walls.len(), 2);
    for r in [0.05_f64, 0.5, 0.9, 0.999, 1.001, 1.1, 2.0, 10.0] {
        for theta in [0.4_f64, 2.0, 4.0] {
            for h in [0.3_f64, 1.0, 1.7] {
                let q = Point3::new(r * theta.cos(), r * theta.sin(), h);
                for &f in &walls {
                    assert_eq!(
                        topo::curved_face_containment(&body, f, q, band).unwrap(),
                        Some(topo::FaceContainment::Out),
                        "r = {r}, θ = {theta}, h = {h}: off-carrier must be Out"
                    );
                }
            }
        }
    }
}

/// The iso-bounded class gate from a DIFFERENT pose than the unit's
/// own pin: a steeper tilt, cutting the other way, on a wider post.
#[test]
fn a_steeply_tilted_cut_wall_still_answers_none() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let post = cyl(0.0, 0.0, 2.0, 0.0, 6.0);
    let phi = 0.9_f64;
    let plane = topo::splitting::SplitPlane {
        origin: Point3::new(0.0, 0.0, 3.0),
        normal: Vec3::new(-phi.sin(), 0.0, phi.cos()),
    };
    let result = topo::splitting::split(&post, &plane, tol).unwrap();
    let topo::splitting::SplitPart::Body(below) = &result.below else {
        panic!("material below the tilted cut");
    };
    let walls: Vec<topo::FaceKey> = below
        .faces()
        .filter(|(_, f)| {
            matches!(
                below.get_surface(f.surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .map(|(k, _)| k)
        .collect();
    assert!(!walls.is_empty());
    let q = Point3::new(2.0 * 0.5_f64.cos(), 2.0 * 0.5_f64.sin(), 0.5);
    for &f in &walls {
        assert!(
            topo::curved_face_containment(below, f, q, band)
                .unwrap()
                .is_none(),
            "an ellipse-bounded wall must answer None"
        );
    }
}

/// A REVOLVE-minted cylinder: a full revolution mints a FULL-TURN wall
/// face whose seam meridian is a boundary edge. This is the wall class
/// the extrude door never mints, and it probes two things at once:
/// the D10 gate must still catch a wall×wall pose built from it, and
/// the containment door must not give a wrong verdict on its chart.
fn revolved_cyl(r: f64, h: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(r, 0.0),
        Point2::new(r, h),
        Point2::new(0.0, h),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .unwrap();
    let axis = sweep::RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    sweep::revolve(&vp, axis, sweep::Revolution::Full, tol)
        .unwrap()
        .body
}

/// The gate catches a crossing pose built from REVOLVE-minted
/// (full-turn, seam-carrying) walls too — the wall-pair scan keys on
/// the surface kind, not on the extrude door's half-wall convention.
#[test]
fn revolve_minted_walls_meet_the_same_gate() {
    let tol = Tol::witness();
    let a = revolved_cyl(1.0, 10.0); // wall about y, r = 1, y in [0, 10]
    let b = moved(
        &turned(&revolved_cyl(1.0, 20.0), Vec3::new(0.0, 0.0, 1.0), PI / 2.0),
        Vec3::new(1.5, 5.0, 0.0),
    );
    match topo::union(&a, &b, tol) {
        Err(_) => {} // typed refusal: honest
        Ok(topo::BooleanResult::Body(body)) => {
            let va = topo::mass_properties(&a, tol).unwrap().volume;
            let vb = topo::mass_properties(&b, tol).unwrap().volume;
            let v = topo::mass_properties(&body.body, tol).unwrap().volume;
            assert!(
                (v - (va + vb)).abs() > 1e-6,
                "the D10 double-count on revolve-minted walls: {v}"
            );
            panic!("unexpected OK for interpenetrating revolve walls (volume {v})");
        }
        Ok(other) => panic!("unexpected {other:?}"),
    }
}

/// The containment door handed a FULL-TURN wall (revolve-minted, seam
/// in the boundary): the doc promises `None` is "the honest remainder
/// throughout — a chart form the trim cannot express" — a full-period
/// azimuth window is such a form. Measured here: the door must never
/// return a WRONG In/Out; None or a loud error are both recorded.
#[test]
fn a_full_turn_wall_never_gets_a_wrong_interior_verdict() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let body = revolved_cyl(1.0, 2.0);
    let walls: Vec<topo::FaceKey> = body
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .map(|(k, _)| k)
        .collect();
    assert!(!walls.is_empty(), "a revolved rectangle has wall faces");
    // On the carrier, mid-height, azimuth far from the seam: interior
    // of the full-turn wall.
    let q = Point3::new(2.0_f64.cos(), 1.0, -(2.0_f64.sin()));
    let mut outcomes = Vec::new();
    for &f in &walls {
        let got = topo::curved_face_containment(&body, f, q, band);
        eprintln!("full-turn wall {f:?}: {got:?}");
        match got {
            Ok(Some(topo::FaceContainment::Out)) => {
                // Only acceptable if q is genuinely off this face —
                // with a single full wall this would be a WRONG verdict.
                outcomes.push("Out");
            }
            Ok(Some(topo::FaceContainment::In)) => outcomes.push("In"),
            Ok(Some(_)) => outcomes.push("OnBoundary"),
            Ok(None) => outcomes.push("None"),
            Err(_) => outcomes.push("Err"),
        }
    }
    // The door must not claim Out on every wall face when the point is
    // on the solid's wall: that would be the wrong-verdict shape.
    assert!(
        !(outcomes.iter().all(|o| *o == "Out")),
        "an on-wall interior point reported Out of every wall face: {outcomes:?}"
    );
}

/// #347's bracket at r = 5, verified at the MECHANISM rather than the
/// door — **and the mechanism is now retired, so the row asserts its
/// absence at the same two datums.**
///
/// As written for the substrate, this row pinned the refusal's payload:
/// the pocket's `y = 10` LINE edge (whose x-span `[8, 28]` entered the
/// corner CARRIER's slab) against the corner-round CYLINDER face at
/// `(5, 5)`, `r = 5`. Keeping it green by keeping the refusal is not an
/// option a moved baseline allows, so it is re-aimed rather than
/// deleted: the same two entities are looked up by geometry, and what
/// is asserted is that their BOXES no longer meet. That is the precise
/// claim the trim-scoped conic edge box and wall face box make, it
/// fails if either scoping regresses, and it says something the
/// whole-op row (`the_bracket_rounds_at_every_radius_and_meters_exactly`)
/// cannot: WHY the cut runs.
#[test]
fn the_r5_bracket_pocket_edge_no_longer_reaches_the_corner_wall() {
    let tol = Tol::witness();
    let plate = rounded_plate(80.0, 40.0, 5.0, 8.0);
    let pocket = slab((8.0, 28.0), (10.0, 30.0), (-2.0, 5.0));

    // The cut runs at all — the door this row used to name is shut.
    topo::subtract(&plate, &pocket, tol).expect("r = 5 cuts since the boxes were trim-scoped");

    // The corner round at (5, 5), by geometry rather than by key.
    let corner = plate
        .faces()
        .find(|(_, f)| {
            matches!(
                plate.get_surface(f.surface),
                Some(geom::Surface::Cylinder { origin: o, radius, .. })
                    if (*radius - 5.0).abs() < 1e-9
                        && (o.x - 5.0).abs() < 1e-9
                        && (o.y - 5.0).abs() < 1e-9
            )
        })
        .map(|(k, _)| k)
        .expect("the corner round at (5, 5)");

    // The pocket's y = 10 wall edge, likewise.
    let wall_edge = pocket
        .edges()
        .find(|(_, e)| {
            matches!(
                pocket.get_curve_geom(e.curve).and_then(topo::CurveGeom::certified),
                Some(c) if matches!(c.carrier(),
                    geom::Curve3::Line { origin, dir }
                        if (origin.y - 10.0).abs() < 1e-9 && dir.y.abs() < 1e-9)
            )
        })
        .map(|(k, _)| k)
        .expect("the pocket's y = 10 edge");

    // The claim, read off the sweep's OWN candidate list: the round's
    // face box is the quarter ring its ARC occupies (`x, y ∈ [0, 5]`),
    // not the carrier slab (`[0, 10]`), so the pocket edge at `y = 10`
    // never pairs with it. `2r > 8` stops meaning anything.
    //
    // `examined` is the exact path's candidate set, so this asserts
    // about the pair the substrate's refusal named — not about a
    // downstream verdict that could go the right way for a wrong
    // reason.
    let (_, b_trace) = topo::boolean::sweep_traces(
        &plate,
        &pocket,
        topo::boolean::SweepStrategy::Realized,
        None,
        tol,
    )
    .expect("the sweep runs");
    assert!(
        !b_trace.examined.contains(&(wall_edge, corner)),
        "the pocket's y = 10 edge is still a candidate against the corner round"
    );
}

/// `bracket.py`'s `rounded_plate` (reviewer copy of the probe suite's
/// fixture — same numbers as #347), in millimetres.
fn rounded_plate(w: f64, h: f64, r: f64, thick: f64) -> Body<f64> {
    let tol = Tol::witness();
    let outline = profile::Open
        .at(Point2::new(w / 2.0, 0.0))
        .toward(1.0, 0.0, tol)
        .unwrap()
        .fillet(r, tol)
        .unwrap()
        .toward(0.0, 1.0, tol)
        .unwrap()
        .to(Point2::new(w, h / 2.0), tol)
        .unwrap()
        .fillet(r, tol)
        .unwrap()
        .toward(-1.0, 0.0, tol)
        .unwrap()
        .to(Point2::new(w / 2.0, h), tol)
        .unwrap()
        .fillet(r, tol)
        .unwrap()
        .toward(0.0, -1.0, tol)
        .unwrap()
        .to(Point2::new(0.0, h / 2.0), tol)
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
    let lp = profile::ProfileLoop::polygon([
        Point2::new(x.0, y.0),
        Point2::new(x.1, y.0),
        Point2::new(x.1, y.1),
        Point2::new(x.0, y.1),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z.0)));
    let prof = Profile::new(plane, vec![lp]).validate(tol).unwrap();
    extrude(&prof, Extrusion::Distance(z.1 - z.0), tol)
        .unwrap()
        .body
}

/// The D5 trap stays closed through the PUBLIC boolean door: the
/// probe's no-edge-event pair must never surface `GermFrameUnsupported`
/// TODAY (no cyl×cyl germs are minted yet) — the wall-pair gate owns
/// the refusal. If this row ever flips to `GermFrameUnsupported`, D4
/// widened the reduction without wiring the frame arm: exactly the
/// regression the trap exists to catch loudly rather than silently.
#[test]
fn the_no_edge_event_pair_refuses_at_the_gate_not_the_frame() {
    let tol = Tol::witness();
    let a = cyl(0.0, 0.0, 1.0, 0.0, 10.0);
    let rod = cyl(0.0, 0.0, 1.0, -10.0, 10.0);
    let lie = turned(&rod, Vec3::new(0.0, 1.0, 0.0), PI / 2.0);
    let b = moved(&lie, Vec3::new(0.0, 1.5, 5.0));
    let err = topo::union(&a, &b, tol).expect_err("no cyl×cyl arm exists");
    assert!(
        matches!(err, BooleanError::FallbackExtentUnsupported { .. }),
        "got {err:?}"
    );
}
