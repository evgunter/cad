//! The measurement door reads sphere seam regions as tier 3's check 6
//! does, and these rows hold it there on the corpus of PR 5556's review
//! (`review/5556-01D2cmWe`): the door names the face tier 3 names, refuses
//! an over-cover whose region the pole read encodes, and refuses a body
//! whose structure its walk cannot follow typed, never by a panic.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::seam_region_common::{flipped, seam_region, split_outside};
use geom::Surface;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::test_support::at_rest;
use sweep::test_support::{
    ball_poled, ball_poled_y, ball_poled_z, brick, dome, domed_cavity, finished, lantern, waisted,
};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{Body, FaceKey};
type AB = topo::AtRestBody<f64>;

fn cyl(cx: f64, cy: f64, r: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(cx, cy), r, tol).unwrap();
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, z0));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(
        &profile,
        Extrusion::Distance {
            depth: z1 - z0,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body
}

fn turned(b: &Body<f64>, axis: Vec3<f64>, angle: f64) -> Body<f64> {
    topo::transform_rigid(
        b,
        &geom_core::test_support::rigid(
            &Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), axis, angle),
            Tol::witness(),
        ),
        Tol::witness(),
    )
    .unwrap()
}

fn g<E>(r: Result<topo::BooleanResult<f64>, E>) -> Option<AB> {
    r.ok().and_then(|r| r.body().map(|b| b.body.clone()))
}

fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

/// Every sphere-carrying body this reviewer could build.
fn corpus() -> Vec<(String, AB)> {
    let tol = Tol::witness();
    let mut out: Vec<(String, AB)> = Vec::new();
    let mut push = |name: String, b: Option<AB>| {
        if let Some(b) = b {
            out.push((name, b));
        } else {
            eprintln!("SKIP {name}");
        }
    };
    let bool3 = |a: &AB, b: &AB| -> Vec<(&'static str, Option<AB>)> {
        vec![
            ("u", g(topo::boolean::union(a, b, tol))),
            ("i", g(topo::boolean::intersect(a, b, tol))),
            ("a-b", g(topo::boolean::subtract(a, b, tol))),
            ("b-a", g(topo::boolean::subtract(b, a, tol))),
        ]
    };
    for &r in &[1e-3, 0.37, 1.0, 250.0] {
        for (pi, pole) in [v(0.0, 0.0, 1.0), v(0.3, -0.5, 0.8), v(1.0, 0.0, 0.0)]
            .iter()
            .enumerate()
        {
            let p = *pole / pole.norm();
            let c = v(0.7 * r + 3.0, -2.0, 1.5);
            push(
                format!("ball r={r} pole{pi}"),
                Some(finished("b", ball_poled(r, c, p, tol), tol)),
            );
        }
    }
    push(
        "ball_y".into(),
        Some(finished("b", ball_poled_y(1.0, v(0.2, 0.0, 0.0), tol), tol)),
    );
    push(
        "ball_z".into(),
        Some(finished(
            "b",
            ball_poled_z(1.0, v(0.0, 0.3, -1.0), tol),
            tol,
        )),
    );
    push("dome".into(), Some(at_rest(&dome(1.0, tol), tol)));
    push("lantern".into(), Some(at_rest(&lantern(tol), tol)));
    push(
        "domed_cavity".into(),
        Some(at_rest(&domed_cavity(tol), tol)),
    );
    push("waisted".into(), Some(at_rest(&waisted(tol), tol)));
    let unit = finished(
        "a",
        ball_poled(1.0, v(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), tol),
        tol,
    );
    let tilt = finished(
        "a",
        ball_poled(1.0, v(0.1, 0.0, 0.0), v(0.6, 0.0, 0.8), tol),
        tol,
    );
    let small_in = finished(
        "s",
        ball_poled(0.4, v(0.1, 0.2, -0.1), v(0.0, 1.0, 0.0), tol),
        tol,
    );
    let small_in_z = finished(
        "s",
        ball_poled(0.4, v(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), tol),
        tol,
    );
    let off = finished(
        "o",
        ball_poled(0.8, v(0.9, 0.3, 0.2), v(0.0, 0.6, 0.8), tol),
        tol,
    );
    let far = finished(
        "f",
        ball_poled(0.5, v(5.0, 0.0, 0.0), v(0.0, 0.0, 1.0), tol),
        tol,
    );
    let bigbrick = finished("k", brick((-2.0, 2.0), (-2.0, 2.0), (-2.0, 2.0), tol), tol);
    let halfbrick = finished("k", brick((-2.0, 2.0), (-2.0, 2.0), (0.2, 2.0), tol), tol);
    let tallbar = finished("k", brick((-0.3, 0.3), (-0.2, 0.2), (-2.0, 2.0), tol), tol);
    let rod = at_rest(&cyl(0.1, 0.0, 0.3, -2.0, 2.0), tol);
    let rodx = at_rest(
        &turned(&cyl(0.0, 0.1, 0.25, -2.0, 2.0), v(1.0, 0.0, 0.0), 0.7),
        tol,
    );
    let stub = at_rest(&cyl(0.5, 0.2, 0.4, 0.3, 2.0), tol);
    let eqbrick = finished("k", brick((-2.0, 2.0), (-2.0, 2.0), (0.0, 2.0), tol), tol);
    for (an, a) in [("unit", &unit), ("tilt", &tilt)] {
        for (bn, b) in [
            ("small_in", &small_in),
            ("small_in_z", &small_in_z),
            ("off", &off),
            ("far", &far),
            ("bigbrick", &bigbrick),
            ("halfbrick", &halfbrick),
            ("tallbar", &tallbar),
            ("rod", &rod),
            ("rodx", &rodx),
            ("stub", &stub),
            ("eqbrick", &eqbrick),
        ] {
            for (op, r) in bool3(a, b) {
                push(format!("{an} {op} {bn}"), r);
            }
        }
    }
    // A hollow ball with a rod through it, a void in a brick next to a ball.
    if let Some(hollow) = g(topo::boolean::subtract(&unit, &small_in, tol)) {
        push(
            "hollow - rod".into(),
            g(topo::boolean::subtract(&hollow, &rod, tol)),
        );
        push(
            "hollow u far".into(),
            g(topo::boolean::union(&hollow, &far, tol)),
        );
    }
    if let Some(void) = g(topo::boolean::subtract(&bigbrick, &small_in, tol)) {
        push(
            "void - unit?".into(),
            g(topo::boolean::subtract(&void, &off, tol)),
        );
        push(
            "void u far".into(),
            g(topo::boolean::union(&void, &far, tol)),
        );
    }
    if let Some(two) = g(topo::boolean::union(&unit, &far, tol)) {
        push(
            "two - rod".into(),
            g(topo::boolean::subtract(&two, &rod, tol)),
        );
    }
    let mut rng = test_utils::fuzz::start("review_5556");
    for i in 0..12 {
        let rb = rng.range(0.4, 1.2);
        let dx = rng.range(-1.5, 1.5);
        let dy = rng.range(-1.5, 1.5);
        let dz = rng.range(-1.5, 1.5);
        let p = v(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(0.1, 1.0),
        );
        let b = finished("b", ball_poled(rb, v(dx, dy, dz), p / p.norm(), tol), tol);
        for (op, r) in bool3(&tilt, &b) {
            push(format!("fuzz{i} {op}"), r);
        }
    }
    out
}

fn sphere_faces(body: &Body<f64>) -> Vec<(FaceKey, topo::SurfaceKey)> {
    body.faces()
        .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Sphere { .. })))
        .map(|(k, f)| (k, f.surface))
        .collect()
}

/// **The door names the face tier 3 names.** Every sphere face of every
/// valid body of the corpus flipped alone: where tier 3 names the flipped
/// face (`CurvedSenseInverted`), the door refuses that face, not a
/// neighbour whose bit is right. A region whose pole read encodes its side
/// refuses only the members it contradicts; one whose read encodes none,
/// the lamina's members that encode none alone
/// (`topo::props::seam_refusals`).
#[test]
fn the_door_names_the_flipped_face_tier_3_names() {
    let tol = Tol::witness();
    let mut misnamed = Vec::new();
    for (name, b) in corpus() {
        if topo::validate_geometric(&b, tol).is_err() {
            continue;
        }
        for (f, _) in sphere_faces(&b) {
            let fl = flipped(&b, &[f]);
            let errs = topo::validate_geometric(&fl, tol).err().unwrap_or_default();
            let named = errs.iter().any(
                |e| matches!(e, topo::ValidationError::CurvedSenseInverted { face } if *face == f),
            );
            // Raw door: the subject is a flipped face tier 3 refuses (CurvedSenseInverted), and which face the door names.
            if let Err(topo::MassPropsError::Face { face, source }) =
                topo::test_support::measure_unvalidated(
                    &fl,
                    tol,
                    topo::test_support::pinned_resolution(tol),
                )
                && named
                && face != f
            {
                misnamed.push(format!(
                    "{name}: flipped {f:?}, the door names {face:?} ({source:?}); region {:?}",
                    seam_region(&b, f).0
                ));
            }
        }
    }
    assert!(
        misnamed.is_empty(),
        "the door names a correct neighbour: {misnamed:#?}"
    );
}

/// **An over-cover the pole read can encode is the census's**: an
/// exterior meridian split on a BOUNDED region (a union of two balls, and
/// one less the other) covers it twice, and both doors refuse it by the
/// census, though the region's side reads.
#[test]
fn an_over_cover_whose_region_encodes_its_side_is_refused_by_the_census() {
    let tol = Tol::witness();
    let unit = finished(
        "a",
        ball_poled(1.0, v(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), tol),
        tol,
    );
    let off = finished(
        "o",
        ball_poled(0.8, v(0.9, 0.3, 0.2), v(0.0, 0.6, 0.8), tol),
        tol,
    );
    for (name, r) in [
        ("u off", topo::boolean::union(&unit, &off, tol)),
        ("a-b off", topo::boolean::subtract(&unit, &off, tol)),
    ] {
        let b = Body::clone(&r.unwrap().body().unwrap().body);
        let mut split = false;
        for d in [
            v(0.0, 1.0, 0.0),
            v(1.0, 0.0, 0.0),
            v(0.0, 0.0, 1.0),
            v(-0.6, 0.0, 0.8),
        ] {
            let mut c = b.clone();
            if split_outside(&mut c, d, tol) {
                let errs = topo::validate_geometric(&c, tol).unwrap_err();
                assert!(
                    errs.iter()
                        .any(|e| matches!(e, topo::ValidationError::SeamRegionOvercovered { .. })),
                    "{name} {d:?}: tier 3 refuses the over-cover by the census: {errs:?}"
                );
                // Raw door: the subject is an over-cover tier 3 refuses (SeamRegionOvercovered), and the door's own refusal of it.
                let door = topo::test_support::measure_unvalidated(
                    &c,
                    tol,
                    topo::test_support::pinned_resolution(tol),
                );
                assert!(
                    matches!(
                        door,
                        Err(topo::MassPropsError::Face {
                            source: geom_brep::props::PropsError::SeamRegionOvercovered { .. },
                            ..
                        })
                    ),
                    "{name} {d:?}: the door refuses it by the census: {door:?}"
                );
                split = true;
                break;
            }
        }
        assert!(split, "{name}: an exterior meridian splits a face");
    }
}

/// **A corrupt body never finishes, and the raw door panics on it only
/// where it always did**: a two-ball union with one entity removed at a
/// time (a face, a loop, an edge or a half-edge). No removal passes
/// `AtRestBody::validate`, so the typed door never reads one. Read
/// through the raw door, the seam pass refuses what it cannot follow
/// `MassPropsError::Corrupt`, and six removals panic in the walk's own
/// face resolution (`live.rs`, a shell's or a face's record naming a
/// removed key) before any seam is read — as they did before the seam
/// pass — so the row admits those six and no other.
#[test]
fn a_corrupt_body_does_not_finish_and_the_raw_door_panics_only_where_it_did() {
    use topo::EntityId;
    let tol = Tol::witness();
    let unit = finished(
        "a",
        ball_poled(1.0, v(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), tol),
        tol,
    );
    let off = finished(
        "o",
        ball_poled(0.8, v(0.9, 0.3, 0.2), v(0.0, 0.6, 0.8), tol),
        tol,
    );
    let b = g(topo::boolean::union(&unit, &off, tol)).unwrap();
    let mut ids: Vec<EntityId> = Vec::new();
    ids.extend(b.faces().map(|(k, _)| EntityId::Face(k)));
    for (_, f) in b.faces() {
        ids.push(EntityId::Loop(f.outer));
    }
    ids.extend(b.edges().map(|(k, _)| EntityId::Edge(k)));
    ids.extend(b.half_edges().map(|(k, _)| EntityId::HalfEdge(k)));
    let mut panicked = Vec::new();
    for id in ids {
        let c = b.with_entity_removed_for_tests(id).unwrap();
        // The typed door is not reached: no removal finishes.
        assert!(
            topo::AtRestBody::validate(c.clone(), tol).is_err(),
            "{id:?}: the body with it removed finishes"
        );
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // Raw door: the subject is a body with an entity removed, which tier 1 refuses, and whether the door panics on it.
            topo::test_support::measure_unvalidated(
                &c,
                tol,
                topo::test_support::pinned_resolution(tol),
            )
        })) {
            Ok(_) => {}
            Err(_) => panicked.push(format!("{id:?}")),
        }
    }
    let before_any_seam = [
        "Face(FaceKey(1v1))",
        "Face(FaceKey(2v1))",
        "Face(FaceKey(3v5))",
        "Face(FaceKey(6v5))",
        "Loop(LoopKey(1v1))",
        "HalfEdge(HalfEdgeKey(25v1))",
    ];
    let new: Vec<_> = panicked
        .iter()
        .filter(|id| !before_any_seam.contains(&id.as_str()))
        .collect();
    assert!(
        new.is_empty(),
        "the door panics past its face resolution on: {new:?}"
    );
}

/// **A whole sphere shell flipped in a body of two shells**: the unit
/// ball less a smaller ball inside it, every face of one shell flipped.
/// Tier 3 refuses either flip, so the typed door never reads one; read
/// through the raw door, the inner shell's flip measures a positive,
/// wrong volume with no refusal, the shell's sign flipping under a total
/// that stays positive
/// (`a-sphere-shell-flipped-whole-in-a-multi-shell-body-measures-wrong-at-the-door`, closed by PR 5590).
#[test]
fn a_whole_flipped_shell_of_two_does_not_finish_and_the_raw_door_measures_it_wrong() {
    let tol = Tol::witness();
    let unit = finished(
        "a",
        ball_poled(1.0, v(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), tol),
        tol,
    );
    let small_in = finished(
        "s",
        ball_poled(0.4, v(0.1, 0.2, -0.1), v(0.0, 1.0, 0.0), tol),
        tol,
    );
    let hollow = g(topo::boolean::subtract(&unit, &small_in, tol)).expect("the hollow ball");
    let measured = topo::test_support::pinned_volume("the measured body", &hollow, tol).volume;
    assert_eq!(hollow.shells().count(), 2, "the hollow ball has two shells");
    let mut wrong = 0;
    for (shell, s) in hollow.shells() {
        let fl = flipped(&hollow, &s.faces);
        assert!(
            topo::AtRestBody::validate(fl.clone(), tol).is_err(),
            "{shell:?} flipped does not finish"
        );
        // Raw door: the subject is a flipped shell tier 3 refuses, and what the door reads of it.
        match topo::test_support::measure_unvalidated(
            &fl,
            tol,
            topo::test_support::pinned_resolution(tol),
        )
        .map(|m| m.enclosure.volume)
        {
            // The inner shell's sign flips and the total stays positive:
            // the solid ball's volume, not the hollow's.
            Ok(v) if v > measured => wrong += 1,
            // The outer shell's flip reads the whole body inside out.
            Ok(v) if v <= 0.0 => {}
            other => panic!("{shell:?} flipped: the raw door reads {other:?}"),
        }
    }
    assert_eq!(
        wrong, 1,
        "the raw door measures the flipped inner shell positive and wrong, past the \
         hollow's {measured}"
    );
}

/// **A member's own contradicted read is named over the region's
/// over-cover**: the double cover split until each face reads its own
/// side (`a_seam_region_covering_the_sphere_twice`'s `p2`), with every bit
/// flipped. Each face's own boundary contradicts its bit, so the door
/// leaves it to its own read in the flux arm, which refuses
/// `SenseContradicted`, rather than naming the census's over-cover.
#[test]
fn a_members_own_contradicted_read_is_named_over_the_over_cover() {
    use crate::seam_region_common::{
        sphere_faces, split_faces_at_equator, split_meridians_at_equator,
    };
    let tol = Tol::witness();
    let mut body = Body::clone(&finished(
        "ball",
        ball_poled(1.0, v(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), tol),
        tol,
    ));
    assert!(split_outside(&mut body, v(0.0, 1.0, 0.0), tol));
    split_meridians_at_equator(&mut body, tol);
    split_faces_at_equator(&mut body, tol);
    let faces = sphere_faces(&body);
    assert_eq!(faces.len(), 6, "each lune split at the equator");
    // Raw door: the subject is a double cover with every bit flipped, which tier 3 refuses, and which refusal the door names.
    let door = topo::test_support::measure_unvalidated(
        &flipped(&body, &faces),
        tol,
        topo::test_support::pinned_resolution(tol),
    );
    assert!(
        matches!(
            door,
            Err(topo::MassPropsError::Face {
                face,
                source: geom_brep::props::PropsError::SenseContradicted,
            }) if faces.contains(&face)
        ),
        "the flipped faces are named by their own reads: {door:?}"
    );
}
