//! R2 review probes (verbs/pierce-r2-probes) for PR #1068 — the disc
//! class's ADJACENT shapes, attacked rather than assumed. Not for
//! merge.
//!
//! - annular cap (a washer): TWO circle loops on one planar face —
//!   outer disc + concentric ring;
//! - a box through the washer's HOLE (the ring must be visible now);
//! - a box through the washer's SOLID part (must not read disjoint);
//! - a MIXED arc+line cap (half-disc): the stated blind spot — is the
//!   remainder honest (typed) or a silent wrong body?

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A washer: annulus (outer r, hole rh) extruded z0..z1 at the origin.
fn washer(r: f64, rh: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let outer = profile::circle(p2(0.0, 0.0), r, tol).unwrap();
    let hole = ProfileLoop::new(vec![
        ProfileVertex::new(p2(rh, 0.0), 1.0),
        ProfileVertex::new(p2(-rh, 0.0), 1.0),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![outer.into(), hole])
        .validate(tol)
        .unwrap();
    extrude(&profile, Extrusion::Distance(z1 - z0), tol)
        .unwrap()
        .body
}

fn boxx(x0: f64, x1: f64, y0: f64, y1: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp: ProfileLoop<f64> = RawLoop::polygon([p2(x0, y0), p2(x1, y0), p2(x1, y1), p2(x0, y1)]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(z1 - z0), tol)
        .unwrap()
        .body
}

/// A half-cylinder: half-disc (semicircular arc + diameter), extruded.
fn half_cyl(r: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    // bulge 1 = semicircle from (-r,0) to (r,0)... vertex order and
    // sign chosen so the arc bows through (0, r).
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(r, 0.0), 0.0),
        ProfileVertex::new(p2(-r, 0.0), 1.0),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(z1 - z0), tol)
        .unwrap()
        .body
}

/// A thin box dropped straight through the washer's HOLE, touching
/// nothing: the ring loop is a disc-class loop, and a point inside the
/// hole must read OUTSIDE the annular face. The operands are honestly
/// disjoint and the volumes add exactly.
#[test]
fn r2_a_box_through_the_washer_hole_reads_disjoint() {
    let tol = Tol::witness();
    let a = washer(1.0, 0.5, 0.0, 2.0);
    let b = boxx(-0.2, 0.2, -0.2, 0.2, -1.0, 3.0);
    let topo::BooleanResult::Body(out) = topo::union(&a, &b, tol).expect("disjoint operands")
    else {
        panic!("washer + through-hole box is two shells");
    };
    assert_eq!(out.body.shells().count(), 2, "two disjoint shells");
    let v = topo::mass_properties(&out.body, tol).unwrap().volume;
    let truth = PI * (1.0 - 0.25) * 2.0 + 0.4 * 0.4 * 4.0;
    assert!((v - truth).abs() < 1e-12, "{v} vs {truth}");
}

/// A box through the washer's SOLID part: crossings on the ANNULAR cap
/// exist, so this must never read disjoint and double-count. Either a
/// typed refusal or the correct union is honest; the old silent shape
/// (volumes adding while the bodies overlap) is the red.
#[test]
fn r2_a_box_through_the_washer_solid_part_is_never_silent() {
    let tol = Tol::witness();
    let a = washer(1.0, 0.3, 0.0, 2.0);
    // Centered at (0.65, 0): x in [0.5, 0.8] — strictly between hole
    // (0.3) and rim (1.0) at y in [-0.15, 0.15].
    let b = boxx(0.5, 0.8, -0.15, 0.15, 1.0, 3.0);
    match topo::union(&a, &b, tol) {
        Err(e) => println!("washer-solid union refuses typed: {e:?}"),
        Ok(topo::BooleanResult::Body(out)) => {
            let v = topo::mass_properties(&out.body, tol).unwrap().volume;
            let disjoint_wrong = PI * (1.0 - 0.09) * 2.0 + 0.3 * 0.3 * 2.0;
            assert!(
                (v - disjoint_wrong).abs() > 1e-9,
                "silent double-count: {v}"
            );
            let truth = PI * (1.0 - 0.09) * 2.0 + 0.3 * 0.3 * 1.0;
            assert!((v - truth).abs() < 1e-12, "{v} vs {truth}");
        }
        Ok(other) => panic!("unexpected: {other:?}"),
    }
}

/// The blind spot BEYOND the stated one: a LENS cap — two arcs of two
/// DIFFERENT circles, no line edge at all, so it is outside the disc
/// class AND outside any "mixes arcs and lines" description, and its
/// polygon through two vertices has zero area like every other member
/// of the class.
///
/// **RED-BY-DESIGN, FLIPPED.** Written against the shipped PR it
/// measured a silent wrong body; the loop-shape gate refuses it typed
/// now. The volume comparison is kept underneath the refusal so the
/// row still names the wrong answer it is standing in front of.
#[test]
fn r2_a_box_through_a_lens_cap_measures_the_all_arc_remainder() {
    let tol = Tol::witness();
    // Lens: from (-1,0) to (1,0) via a deep arc (bulge 0.6), back via
    // a shallow arc of a DIFFERENT circle (bulge 0.35 on the return).
    // SYMMETRIC lens: equal bulges on both legs bow outward on
    // opposite sides (mirror-image circles, distinct carriers).
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(-1.0, 0.0), 0.6),
        ProfileVertex::new(p2(1.0, 0.0), 0.6),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.0)));
    let profile = Profile::new(plane, vec![lp]).validate(tol).unwrap();
    let a = extrude(&profile, Extrusion::Distance(2.0), tol)
        .unwrap()
        .body;
    let va = topo::mass_properties(&a, tol).unwrap().volume;
    // A small box through the cap near (0, 0.3) — inside the lens for
    // these bulges (upper arc reaches y=0.6... sagitta = bulge*half-chord).
    println!(
        "lens operand volume {}",
        topo::mass_properties(&a, tol).unwrap().volume
    );
    let b = boxx(-0.1, 0.1, -0.1, 0.1, 1.0, 3.0);
    let silent_wrong = va + 0.2 * 0.2 * 2.0;
    match topo::union(&a, &b, tol) {
        Err(e) => assert!(
            matches!(e, BooleanError::ArcLoopContainmentUnsupported { .. }),
            "the lens cap has no walk and must say so; got {e:?}"
        ),
        Ok(topo::BooleanResult::Body(out)) => {
            let v = topo::mass_properties(&out.body, tol).unwrap().volume;
            panic!(
                "ALL-ARC LENS SILENT WRONG BODY: {v} (operand {va}, \
                 silent-wrong {silent_wrong} — the overlap double-counted)"
            );
        }
        Ok(other) => panic!("unexpected: {other:?}"),
    }
}

/// A box driven up through a HALF-cylinder's cap, in the semicircular
/// region: the cap's loop is an arc plus a chord over two vertices, so
/// the polygon through them is a zero-area segment.
///
/// **RED-BY-DESIGN, FLIPPED — and re-signed.** As authored this row
/// ran ONE bulge sense and read `3.266592653589793` as the silent
/// wrong body against a truth of `3.204092653589793`. Measured in the
/// fix pass, that half-disc bows AWAY from its box: nothing overlaps,
/// and `3.266592653589793` is the correct disjoint answer. The row
/// therefore takes the R1 row's two-sense design, which needs no such
/// judgement — exactly one of the two senses contains the box, the
/// containing one has no walk for its cap and refuses typed, and the
/// other is honestly disjoint. Both answering the same number is the
/// silent wrong body.
#[test]
fn r2_a_box_through_a_half_disc_cap_measures_the_mixed_loop_remainder() {
    let tol = Tol::witness();
    // Cap crossings at (0.1..0.35, 0.3..0.55, z=2): inside the
    // half-disc that bows to y > 0, outside the one that bows to y < 0.
    let b = boxx(0.1, 0.35, 0.3, 0.55, 1.0, 3.0);
    let half = PI / 2.0 * 2.0; // half-disc area * height = pi
    let disjoint_answer = half + 0.25 * 0.25 * 2.0;
    let buried_truth = disjoint_answer - 0.25 * 0.25 * 1.0;
    let mut refused = 0;
    let mut bodies = 0;
    for bulge in [1.0, -1.0] {
        let lp = ProfileLoop::new(vec![
            ProfileVertex::new(p2(1.0, 0.0), 0.0),
            ProfileVertex::new(p2(-1.0, 0.0), bulge),
        ]);
        let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.0)));
        let profile = Profile::new(plane, vec![lp]).validate(tol).unwrap();
        let a = extrude(&profile, Extrusion::Distance(2.0), tol)
            .unwrap()
            .body;
        match topo::union(&a, &b, tol) {
            Err(e) => {
                assert!(
                    matches!(e, BooleanError::ArcLoopContainmentUnsupported { .. }),
                    "bulge={bulge}: the half-disc cap has no walk; got {e:?}"
                );
                refused += 1;
            }
            Ok(topo::BooleanResult::Body(out)) => {
                let v = topo::mass_properties(&out.body, tol).unwrap().volume;
                println!("half-disc cap bulge={bulge}: BODY volume {v}");
                assert!(
                    (v - disjoint_answer).abs() < 1e-9,
                    "bulge={bulge}: the non-containing sense is honestly disjoint \
                     ({disjoint_answer}); got {v}"
                );
                bodies += 1;
            }
            Ok(other) => panic!("bulge={bulge}: unexpected {other:?}"),
        }
    }
    assert_eq!(
        (refused, bodies),
        (1, 1),
        "one sense contains the box and must refuse; the other must answer \
         disjoint ({disjoint_answer}). Both answering it is the silent wrong \
         body against a buried truth of {buried_truth}"
    );
}

/// The door-table claim names `Join(SectionLoopMixed)` for the
/// box-through-cap row; the shipped test asserts only `Join(_)`.
/// Print the exact payload.
#[test]
fn r2_the_box_cap_refusal_payload_is_printed() {
    let tol = Tol::witness();
    let lp = profile::circle(p2(0.0, 0.0), 1.0, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.0)));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    let a = extrude(&profile, Extrusion::Distance(2.0), tol)
        .unwrap()
        .body;
    let b = boxx(-0.3, 0.3, -0.3, 0.3, 1.0, 3.0);
    match topo::union(&a, &b, tol) {
        Err(e) => {
            println!("box-through-cap refusal: {e:?}");
            assert!(
                matches!(
                    e,
                    BooleanError::Join(topo::SplitJoinError::SectionLoopMixed { .. })
                ),
                "the door table names this payload, so it is pinned: {e:?}"
            );
        }
        Ok(r) => panic!("expected the typed refusal, got {r:?}"),
    }
}

/// Calibration for the PR's cosurface design claim (door-2 evidence
/// item 4): what does the PLANAR substrate do with an undeclared
/// value-coincident cap-to-cap union (two stacked boxes)? If this
/// unions without a declaration, "their honest destination is the
/// declaration ladder" reads differently for coaxial-stacked than the
/// PR implies; if it refuses, the curved rows follow the precedent.
#[test]
fn r2_stacked_boxes_calibrate_the_cosurface_claim() {
    let tol = Tol::witness();
    let a = boxx(0.0, 1.0, 0.0, 1.0, 0.0, 1.0);
    let b = boxx(0.0, 1.0, 0.0, 1.0, 1.0, 2.0);
    match topo::union(&a, &b, tol) {
        Err(e) => println!("stacked boxes refuse: {e:?}"),
        Ok(topo::BooleanResult::Body(out)) => {
            let v = topo::mass_properties(&out.body, tol).unwrap().volume;
            println!("stacked boxes union OK, volume {v}");
        }
        Ok(other) => println!("stacked boxes: {other:?}"),
    }
}

/// The `point_in_face` sibling site (PR sweep table: "measured, not
/// fixed"), attacked at a pose the shipped guard does not reach: a
/// PANCAKE cylinder (r much larger than h) buries a box at its centre,
/// so most of the ray schedule's directions exit through the CAPS —
/// the faces whose trim the ray lane still reads through the
/// zero-area two-vertex polygon. If the cap hits are dropped, the box
/// reads as outside and the union double-counts it.
#[test]
fn r2_a_box_buried_in_a_pancake_cylinder_attacks_the_ray_cap_trim() {
    let tol = Tol::witness();
    let cyl = {
        let lp = profile::circle(p2(0.0, 0.0), 5.0, tol).unwrap();
        let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.0)));
        let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
        extrude(&profile, Extrusion::Distance(0.4), tol)
            .unwrap()
            .body
    };
    let b = boxx(-0.1, 0.1, -0.1, 0.1, 0.1, 0.3);
    match topo::union(&cyl, &b, tol) {
        Err(e) => panic!("the buried box must be swallowed, not refused: {e:?}"),
        Ok(topo::BooleanResult::Body(out)) => {
            let v = topo::mass_properties(&out.body, tol).unwrap().volume;
            let truth = PI * 25.0 * 0.4;
            println!("pancake union volume {v}; truth {truth}");
            assert!(
                (v - truth).abs() < 1e-9,
                "RAY-CAP-TRIM WRONG BODY: {v} vs {truth} (buried box not swallowed)"
            );
        }
        Ok(other) => panic!("unexpected: {other:?}"),
    }
}

/// The #1032 seam measurement re-run (PR body "Point 2, measured"):
/// declaring every cross-solid cylindrical wall pair in the m9_2b_r2
/// fixture must clear exactly the wall-on-wall undecidables and leave
/// the curved-x-planar ones. Prints both counts.
#[test]
fn r2_the_1032_declaration_measurement_reproduces() {
    use profile::SketchPlane as SP;
    let tol = Tol::witness();
    // The m9_2b_r2 fixture, restated (holed plate + through-boss).
    let outer = ProfileLoop::new(
        [(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]
            .map(|(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .to_vec(),
    );
    let hole = ProfileLoop::new(vec![
        ProfileVertex::new(p2(2.5, 2.0), 1.0),
        ProfileVertex::new(p2(1.5, 2.0), 1.0),
    ]);
    let plate_profile = Profile::new(SP::xy(), vec![outer, hole])
        .validate(tol)
        .unwrap();
    let plate = extrude(&plate_profile, Extrusion::Distance(1.0), tol)
        .unwrap()
        .body;
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th = (deg as f64).to_radians();
        p2(2.0 + 0.5 * th.cos(), 2.0 + 0.5 * th.sin())
    };
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(at(90.0), b120),
        ProfileVertex::new(at(210.0), b120),
        ProfileVertex::new(at(330.0), b120),
    ]);
    let plane = SP::new(Affine3::translation(Vec3::new(0.0, 0.0, -0.2)));
    let boss_profile = Profile::new(plane, vec![lp]).validate(tol).unwrap();
    let boss = extrude(&boss_profile, Extrusion::Distance(1.6), tol)
        .unwrap()
        .body;

    let mut body = plate.clone();
    let plate_faces: std::collections::BTreeSet<_> = body.faces().map(|(k, _)| k).collect();
    topo::graft_disjoint(&mut body, &boss, tol).unwrap();
    let is_cyl = |body: &Body<f64>, f: &topo::Face| {
        matches!(
            body.get_surface(f.surface),
            Some(geom::Surface::Cylinder { .. })
        )
    };
    let plate_walls: Vec<_> = body
        .faces()
        .filter(|(k, f)| plate_faces.contains(k) && is_cyl(&body, f))
        .map(|(k, _)| k)
        .collect();
    let boss_walls: Vec<_> = body
        .faces()
        .filter(|(k, f)| !plate_faces.contains(k) && is_cyl(&body, f))
        .map(|(k, _)| k)
        .collect();
    println!(
        "plate cylinder walls: {}, boss cylinder walls: {}",
        plate_walls.len(),
        boss_walls.len()
    );

    let count_undecidable =
        |contacts: &topo::ContactRecords| match topo::validate_pseudomanifold(&body, contacts, tol)
        {
            Ok(()) => 0,
            Err(errs) => {
                for e in &errs {
                    if matches!(e, topo::ValidationError::CensusUndecidable { .. }) {
                        println!("  undecidable: {e:?}");
                    }
                }
                errs.iter()
                    .filter(|e| matches!(e, topo::ValidationError::CensusUndecidable { .. }))
                    .count()
            }
        };
    let undeclared = count_undecidable(&topo::ContactRecords::default());
    let mut declared = topo::ContactRecords::default();
    for &pw in &plate_walls {
        for &bw in &boss_walls {
            declared.patches.push(topo::PatchContact {
                face_a: pw,
                face_b: bw,
            });
        }
    }
    let with_decl = count_undecidable(&declared);
    println!("undeclared: {undeclared} undecidable; declared: {with_decl}");
    assert_eq!(undeclared, 11, "the PR measured 11 undeclared");
    assert_eq!(with_decl, 6, "the PR measured 6 under declaration");
}
