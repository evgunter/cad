//! The merge door and a declared pair on a CYLINDER carrier.
//!
//! A declared `Rest` pair whose faces both survive a boolean is handed
//! to `merge_coplanar_faces_declared` whatever its carrier, and the door
//! reads every meeting edge on the carrier ladder, planar and curved
//! alike, declared or not: faces their margins decide one carrier
//! facing the same way glue. A curved run whose glue would close its
//! carrier's full period stays in its cut form and is recorded as a
//! `SkippedMerge` carrying `PeriodClosure` — visible in
//! `BooleanNaming::merge_skipped`, never a refusal blaming the caller,
//! and never a record of the declaration itself. Scenes A, B, C, D and
//! F ship honest bodies whose only records are their wall runs' period
//! closures; scenes C and D build through the chord join, which
//! consumes the bore side of the pair before the door; scene E, the
//! same stack with only the caps declared or nothing, is scene F's body
//! (D10).
//!
//! Scenes A–D are `mate2_common`'s; scene D's plate/peg builders are
//! copied from `r1_probes_m9_3` (private there).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::operands::{plate6, plate6_cyl};
use crate::mate2_common;
use geom::SurfaceKind;
use geom_core::{Affine3, Point2, Tol, Vec2, Vec3};
use mate2_common::{
    assert_additive, boolean_body, collar, collar_at, continuations, peg_at, plane_face, volume,
    wall_decls, walls_at,
};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::ExtrudeSide;
use sweep::test_support::finished;
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{
    AtRestBody, Body, BooleanBody, BooleanDeclarations, ContactClass, FacePairDeclaration,
    FaceSurface, MergeCoplanarError, Rechart, SkippedMerge, validate_closed, validate_geometric,
    validate_pseudomanifold,
};

const BORE_R: f64 = 0.5;

/// Scene A: the peg floats in the bore (z ∈ [1.5, 2.5] against a bore
/// z ∈ [1, 2]); nine wall `Rest`s.
fn scene_a() -> (AtRestBody<f64>, AtRestBody<f64>, BooleanDeclarations) {
    let c = collar();
    let p = peg_at(0.0, 1.5, 1.0);
    let d = wall_decls(&c, &p);
    let tol = Tol::witness();
    (
        finished("the collar", c, tol),
        finished("the peg", p, tol),
        d,
    )
}

/// Scene B: the peg ends mid-bore (z ∈ [0.5, 1.5]).
fn scene_b() -> (AtRestBody<f64>, AtRestBody<f64>, BooleanDeclarations) {
    let c = collar_at(0.0);
    let p = peg_at(0.0, 0.5, 1.0);
    let d = wall_decls(&c, &p);
    let tol = Tol::witness();
    (
        finished("the collar", c, tol),
        finished("the peg", p, tol),
        d,
    )
}

/// Scene C: flush at the bottom, proud at the top (z ∈ [1, 2.5]).
fn scene_c() -> (AtRestBody<f64>, AtRestBody<f64>, BooleanDeclarations) {
    let c = collar_at(0.0);
    let p = peg_at(0.0, 1.0, 1.5);
    let d = wall_decls(&c, &p);
    let tol = Tol::witness();
    (
        finished("the collar", c, tol),
        finished("the peg", p, tol),
        d,
    )
}

/// Scene D: partial engagement — a plate with a peg reaching halfway up
/// the through-bore of a plate above it; one planar `Rest` (the plates'
/// mating faces) plus nine wall `Rest`s.
fn scene_d() -> (AtRestBody<f64>, AtRestBody<f64>, BooleanDeclarations) {
    let tol = Tol::witness();
    let p = boolean_body(
        topo::union(
            &finished("the lower plate", plate6(0.0), tol),
            &finished("the peg", plate6_cyl(2.0, 0.4, 1.1, BORE_R), tol),
            tol,
        )
        .unwrap(),
    )
    .body;
    let q = boolean_body(
        topo::subtract(
            &finished("the upper plate", plate6(1.0), tol),
            &finished("the bore", plate6_cyl(2.0, 0.8, 1.4, BORE_R), tol),
            tol,
        )
        .unwrap(),
    )
    .body;
    // The plates' flush outer walls are continuations.
    let mut d = continuations(&p, &q);
    d.coincident_faces.push(FacePairDeclaration::new(
        plane_face(&p, 1.0, true),
        plane_face(&q, 1.0, false),
        ContactClass::Rest,
    ));
    for &fa in &walls_at(&p, BORE_R) {
        for &fb in &walls_at(&q, BORE_R) {
            d.coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    (p, q, d)
}

/// The union must run, be exactly additive, and be tier-3 and
/// 3′-clean — the skip has exposed the zip's body, and that body has
/// to be right on its own.
fn union_honest(
    label: &str,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    d: &BooleanDeclarations,
) -> BooleanBody<f64> {
    let bb = boolean_body(
        topo::union_with(a, b, d, Tol::witness())
            .unwrap_or_else(|e| panic!("{label}: the door must not refuse: {e:?}")),
    );
    assert_additive(volume(&bb.body), volume(a), volume(b));
    assert_eq!(validate_closed(&bb.body), Ok(()), "{label}: tier 2");
    assert_eq!(
        validate_geometric(&bb.body, Tol::witness()),
        Ok(()),
        "{label}: tier 3"
    );
    assert_eq!(
        validate_pseudomanifold(&bb.body, &bb.contacts, Tol::witness()),
        Ok(()),
        "{label}: tier 3′"
    );
    sweep::test_support::assert_legal_operand(label, &bb.body, Tol::witness());
    bb
}

fn is_cylinder(body: &Body<f64>, face: topo::FaceKey) -> bool {
    body.get_face(face)
        .and_then(|f| body.get_surface(f.surface))
        .is_some_and(|s| matches!(s, geom::Surface::Cylinder { .. }))
}

/// The merge records of a boolean result: each is a curved run's
/// `PeriodClosure`, its faces live and on a cylinder. Returns them.
fn assert_period_closures<'a>(label: &str, bb: &'a BooleanBody<f64>) -> Vec<&'a SkippedMerge> {
    for skip in &bb.naming.merge_skipped {
        assert!(
            matches!(skip.reason, MergeCoplanarError::PeriodClosure { .. }),
            "{label}: unexpected skip reason {:?}",
            skip.reason
        );
        assert!(!skip.faces.is_empty(), "{label}: a record names its faces");
        for &f in &skip.faces {
            assert!(
                bb.body.get_face(f).is_some(),
                "{label}: recorded face {f:?} is live"
            );
            assert!(
                is_cylinder(&bb.body, f),
                "{label}: recorded face {f:?} is on a cylinder"
            );
        }
    }
    bb.naming.merge_skipped.iter().collect()
}

/// The surviving r = 0.5 faces: `(face, sense)` in face-arena order.
fn bore_radius_faces(body: &Body<f64>) -> Vec<(topo::FaceKey, bool)> {
    walls_at(body, BORE_R)
        .into_iter()
        .map(|f| (f, body.get_face(f).unwrap().sense))
        .collect()
}

fn face_of(body: &Body<f64>, he: topo::HalfEdgeKey) -> topo::FaceKey {
    body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
        .unwrap()
        .face
}

/// Edges shared by two DISTINCT faces of `set`, as `(edge, f1, f2)`.
fn shared_edges(
    body: &Body<f64>,
    set: &[topo::FaceKey],
) -> Vec<(topo::EdgeKey, topo::FaceKey, topo::FaceKey)> {
    body.edges()
        .filter_map(|(ek, e)| {
            let (f1, f2) = (face_of(body, e.he_plus), face_of(body, e.he_minus));
            (f1 != f2 && set.contains(&f1) && set.contains(&f2)).then_some((ek, f1, f2))
        })
        .collect()
}

/// Every live face on either key, sorted.
fn faces_on(body: &Body<f64>, k1: topo::SurfaceKey, k2: topo::SurfaceKey) -> Vec<topo::FaceKey> {
    let mut faces: Vec<_> = body
        .faces()
        .filter(|(_, f)| f.surface == k1 || f.surface == k2)
        .map(|(k, _)| k)
        .collect();
    faces.sort();
    faces
}

fn sorted(mut faces: Vec<topo::FaceKey>) -> Vec<topo::FaceKey> {
    faces.sort();
    faces
}

/// Row 1 (C): the proud peg flush at the bottom ships an honest body
/// whose only records are period closures. The peg's bottom rim lies on
/// the bore's bottom rim, so the section segments there are edges of
/// both solids; the chord join builds the union itself (JOIN-1), and
/// the bore wall it discards takes its surface with it. What the door
/// records is the two surviving wall runs (the peg's and the collar's
/// outer wall), each of whose glue would close its full period.
#[test]
fn proud_peg_declared_walls_union_builds_through_the_join() {
    let (c, p, d) = scene_c();
    let bb = union_honest("C", &c, &p, &d);
    assert_eq!(
        assert_period_closures("C", &bb).len(),
        2,
        "C: two wall runs"
    );
}

/// `face` alone onto a fresh key holding `surface`, outward-facing,
/// through the describing door with the stored description of every
/// edge the move strands restated on it. Returns the new key.
fn on_a_key_of_its_own(
    body: &mut Body<f64>,
    face: topo::FaceKey,
    surface: geom::Surface<f64>,
) -> topo::SurfaceKey {
    let charts = vec![Rechart::new(surface, face, true)];
    let specs = body.carried_redescriptions(&charts).unwrap();
    // Lifts RechartUnvouched: the generators between sectors name the key the neighbours keep, on a curved chart whose residuals no door reads; the sectors on keys of their own are the row.
    let [key] = body
        .lifting_rechart_refusals_for_tests(|body| {
            body.set_face_surfaces_describing(charts, &specs, Tol::witness())
        })
        .unwrap()[..]
    else {
        panic!("one chart, one key")
    };
    key
}

/// A peg whose three wall sectors each sit on their OWN surface key
/// (same description): no hard rung fires anywhere, so the door has
/// nothing to merge and only the declaration to answer.
fn peg_with_split_wall_keys() -> (Body<f64>, Vec<topo::SurfaceKey>) {
    let mut body = peg_at(0.0, 0.0, 1.0);
    let mut keys = Vec::new();
    for f in walls_at(&body, BORE_R) {
        let described = body
            .get_surface(body.get_face(f).unwrap().surface)
            .unwrap()
            .clone();
        keys.push(on_a_key_of_its_own(&mut body, f, described));
    }
    assert_eq!(keys.len(), 3);
    assert_eq!(validate_closed(&body), Ok(()));
    (body, keys)
}

/// Row 1′ (public door): a declared cylinder pair on a peg whose three
/// wall sectors sit on keys of their own is read on the carrier ladder,
/// and the three sectors are one run whose glue would close the full
/// period: the door records that `PeriodClosure`, naming every sector,
/// commits nothing, and copies of the pair change nothing.
#[test]
fn a_declared_full_period_pair_is_recorded_as_its_period_closure() {
    let (mut body, keys) = peg_with_split_wall_keys();
    let before = format!("{body:?}");
    let pair = (keys[0], keys[1]);
    let outcome = body
        .merge_coplanar_faces_declared(&[pair, pair, pair], Tol::witness())
        .expect("a full-period run is recorded, never refused");
    assert!(outcome.groups.is_empty(), "{:?}", outcome.groups);
    let [skip] = &outcome.skipped[..] else {
        panic!("one record: {:?}", outcome.skipped);
    };
    assert!(
        matches!(skip.reason, MergeCoplanarError::PeriodClosure { .. }),
        "{:?}",
        skip.reason
    );
    assert_eq!(
        sorted(skip.faces.clone()),
        sorted(walls_at(&body, BORE_R)),
        "the record names every sector of the run"
    );
    assert_eq!(
        format!("{body:?}"),
        before,
        "nothing commits: the body is untouched"
    );
}

/// Row 1″ (public door): a planar pair and a cylinder pair in ONE
/// call, in either order, are each read by their own carriers — the
/// planar pair (two cap planes that never meet) licenses nothing, and
/// the cylinder pair's run is recorded as its period closure.
#[test]
fn mixed_list_classifies_each_pair_by_its_own_kind() {
    let (mut body, keys) = peg_with_split_wall_keys();
    let caps = (
        body.get_face(plane_face(&body, 0.0, false))
            .unwrap()
            .surface,
        body.get_face(plane_face(&body, 1.0, true)).unwrap().surface,
    );
    let cyl = (keys[0], keys[1]);
    let before = format!("{body:?}");
    for declared in [[caps, cyl], [cyl, caps]] {
        let outcome = body
            .merge_coplanar_faces_declared(&declared, Tol::witness())
            .unwrap_or_else(|e| panic!("{declared:?}: each pair is its own kind: {e:?}"));
        assert!(outcome.groups.is_empty(), "{:?}", outcome.groups);
        assert!(
            matches!(
                outcome.skipped[..],
                [SkippedMerge {
                    reason: MergeCoplanarError::PeriodClosure { .. },
                    ..
                }]
            ),
            "{declared:?}: the run's period closure: {:?}",
            outcome.skipped
        );
        assert_eq!(
            format!("{body:?}"),
            before,
            "nothing commits: the body is untouched"
        );
    }
}

/// A prism whose y = 0 wall is split in two coplanar faces at a
/// straight-angle vertex, and whose right end is two arcs of one
/// circle (centre (2.25, 0.5), meeting the straight walls at 26.6°, so
/// no joint is tangent), split in two cylinder faces at their shared
/// vertex. The extrude builds each run as ONE wall with one rim edge
/// on each cap; the station is cut into both rims and a chord `mef`
/// between the two copies splits the wall. Returns the two wall keys
/// (made distinct) and two keys of the one cylinder (made distinct).
fn d_prism_with_split_keys() -> (
    Body<f64>,
    (topo::SurfaceKey, topo::SurfaceKey),
    (topo::SurfaceKey, topo::SurfaceKey),
) {
    // The arc's centre is (2.25, 0.5); its far point (2.25 + r, 0.5)
    // splits the 233° sweep into two equal arcs, bulge = tan(θ/4).
    let r = (0.25f64 * 0.25 + 0.5 * 0.5).sqrt();
    let sweep = 2.0 * core::f64::consts::PI - 2.0 * 0.5f64.atan2(0.25);
    let bulge = (sweep / 2.0 / 4.0).tan();
    let lp = bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 0.0),
        (Point2::new(2.0, 0.0), bulge),
        (Point2::new(2.25 + r, 0.5), bulge),
        (Point2::new(2.0, 1.0), 0.0),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let built = extrude(
        &profile,
        Extrusion::Distance {
            depth: 1.0,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .unwrap();
    let mut body = built.body;
    // Each run wall split at its station by a chord `mef` between the
    // station's two copies (the straight ruling, on the plane or the
    // cylinder alike). The sweep carries each rim as one edge over the
    // run, so the station is cut into both rims first: it is each run's
    // midpoint (the line run's two sides are equal, as are its arcs).
    let mut split_at_station = |run: &sweep::SideWall| {
        let mut station = |rim: topo::EdgeKey| {
            let curve = body.get_edge(rim).unwrap().curve;
            let (t0, t1) = body
                .get_curve_geom(curve)
                .unwrap()
                .certified()
                .unwrap()
                .params();
            body.split_edge(rim, (t0 + t1) * 0.5, Tol::witness())
                .unwrap()
                .vertex
        };
        let (bottom, top) = (station(run.bottom_rim), station(run.top_rim));
        let leaving = |v: topo::VertexKey| {
            body.half_edges()
                .find(|(h, he)| he.start == v && body.face_of_half_edge(*h) == Some(run.face))
                .unwrap()
                .0
        };
        let (he1, he2) = (leaving(bottom), leaving(top));
        body.mef_chord(topo::MefSite::Chords { he1, he2 }, Tol::witness())
            .unwrap();
    };
    let (line_run, arc_run) = (&built.walls[0][0], &built.walls[0][1]);
    assert_eq!(line_run.segments, vec![0, 1], "the y = 0 side is one run");
    assert_eq!(arc_run.segments, vec![2, 3], "the two arcs are one run");
    split_at_station(line_run);
    split_at_station(arc_run);
    let y0_walls: Vec<_> = body
        .faces()
        .filter(|(_, f)| match body.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, normal, .. }) => {
                origin.y.abs() < 1e-12 && normal.y < -0.5
            }
            _ => false,
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(
        y0_walls.len(),
        2,
        "the straight-angle vertex splits the y = 0 wall"
    );
    let walls = distinct_keys(&mut body, y0_walls[0], y0_walls[1]);
    let arcs = walls_at(&body, r);
    assert_eq!(arcs.len(), 2, "two arcs of one circle");
    let cyls = distinct_keys(&mut body, arcs[0], arcs[1]);
    assert_eq!(validate_closed(&body), Ok(()));
    (body, walls, cyls)
}

/// The two faces' keys, re-homing `b` onto a fresh copy of its
/// description when the constructor put both on one key.
fn distinct_keys(
    body: &mut Body<f64>,
    a: topo::FaceKey,
    b: topo::FaceKey,
) -> (topo::SurfaceKey, topo::SurfaceKey) {
    let ka = body.get_face(a).unwrap().surface;
    let kb = body.get_face(b).unwrap().surface;
    if ka != kb {
        return (ka, kb);
    }
    let described = body.get_surface(kb).unwrap().clone();
    // Lifts RechartStrandsDescriptions: the row's premise is the key the face left, which its descriptions still name.
    let fresh = body
        .set_face_surface_unvouched_for_tests(
            b,
            FaceSurface::New {
                surface: described,
                sense: true,
            },
        )
        .unwrap();
    (ka, fresh)
}

/// Row 1‴ (public door): a `(Plane, Plane)` pair and a
/// `(Cylinder, Cylinder)` pair that each genuinely meet at an edge BOTH
/// glue — the D-prism's split `y = 0` wall and its arc run, short of a
/// full period — in one call, nothing recorded.
#[test]
fn planar_and_cylinder_pairs_glue_in_one_call() {
    let (mut body, walls, cyls) = d_prism_with_split_keys();
    let faces_before = body.faces().count();
    let outcome = body
        .merge_coplanar_faces_declared(&[walls, cyls], Tol::witness())
        .unwrap_or_else(|e| panic!("{e:?}"));
    assert_eq!(outcome.groups.len(), 2, "{:?}", outcome.groups);
    for group in &outcome.groups {
        assert_eq!(group.absorbed.len(), 1, "{group:?}");
        assert!(
            body.get_face(group.kept).is_some() && body.get_face(group.absorbed[0]).is_none(),
            "the pair glued: {group:?}"
        );
    }
    assert!(outcome.skipped.is_empty(), "{:?}", outcome.skipped);
    assert_eq!(body.faces().count(), faces_before - 2);
    assert_eq!(validate_closed(&body), Ok(()));
}

/// Row 1⁗ (public door): a declared pair across two keys of one peg
/// wall — two sectors on `k`, the third on `k2` — is one run with the
/// same-key sectors, and the three close the full period: no group
/// commits, and the one record names the three live sectors.
/// (Construction after the R1 review probe and R2's MAJOR 1.)
#[test]
fn a_run_across_keys_is_recorded_whole() {
    let mut body = peg_at(0.0, 0.0, 1.0);
    let walls = walls_at(&body, BORE_R);
    let k = body.get_face(walls[0]).unwrap().surface;
    let described = body.get_surface(k).unwrap().clone();
    let k2 = on_a_key_of_its_own(&mut body, walls[2], described);
    let faces_before = body.faces().count();
    let outcome = body
        .merge_coplanar_faces_declared(&[(k, k2)], Tol::witness())
        .unwrap_or_else(|e| panic!("{e:?}"));
    assert!(outcome.groups.is_empty(), "{:?}", outcome.groups);
    assert_eq!(body.faces().count(), faces_before);
    let [skip] = &outcome.skipped[..] else {
        panic!("one record: {:?}", outcome.skipped);
    };
    assert!(
        matches!(skip.reason, MergeCoplanarError::PeriodClosure { .. }),
        "{:?}",
        skip.reason
    );
    for &f in &skip.faces {
        assert!(body.get_face(f).is_some(), "recorded face {f:?} is live");
    }
    assert_eq!(
        sorted(skip.faces.clone()),
        faces_on(&body, k, k2),
        "the record names every live face on either key"
    );
}

/// Row 1⁵ (public door): a declared pair with NO live face on either
/// surface — a surface kept alive only by an edge description after
/// every face left it — licenses nothing: the door's outcome and body
/// are the undeclared door's, whose only records are the body's own
/// wall runs' period closures; and so is a pair of that surface with a
/// live key. (Construction after the R1/R2 review probes: scene C's
/// shipped body with every peg-wall face re-keyed.)
#[test]
fn pair_with_no_live_faces_is_the_undeclared_merge() {
    let (c, p, d) = scene_c();
    let mut body = union_honest("C", &c, &p, &d).body.into_body();
    let walls = walls_at(&body, BORE_R);
    let pk = body.get_face(walls[0]).unwrap().surface;
    assert!(
        walls
            .iter()
            .all(|&f| body.get_face(f).unwrap().surface == pk)
    );
    let mut fresh = Vec::new();
    for &f in &walls {
        let described = body.get_surface(pk).unwrap().clone();
        fresh.push(
            // Lifts RechartStrandsDescriptions: the row's premise is the key the face left, which its descriptions still name.
            body.set_face_surface_unvouched_for_tests(
                f,
                FaceSurface::New {
                    surface: described,
                    sense: true,
                },
            )
            .unwrap(),
        );
    }
    assert!(
        body.get_surface(pk).is_some(),
        "{pk:?} stays live with zero faces (held by an edge description) — if the \
         orphan rule changed, re-pin this row on another construction"
    );
    assert!(faces_on(&body, pk, pk).is_empty());
    assert_eq!(validate_closed(&body), Ok(()));
    // At rest: the re-charted walls store their rows again, so the
    // door's closing mint has nothing to change.
    topo::mint_pcurves(&mut body, Tol::witness()).unwrap();
    let mut plain = body.clone();
    let want = plain.merge_coplanar_faces(Tol::witness()).unwrap();
    assert!(
        want.groups.is_empty()
            && want
                .skipped
                .iter()
                .all(|r| matches!(r.reason, MergeCoplanarError::PeriodClosure { .. })),
        "the undeclared door records only period closures: {want:?}"
    );
    for pair in [(pk, pk), (pk, fresh[0])] {
        let mut declared = body.clone();
        let got = declared
            .merge_coplanar_faces_declared(&[pair], Tol::witness())
            .unwrap_or_else(|e| panic!("{pair:?}: {e:?}"));
        assert_eq!(
            format!("{got:?}"),
            format!("{want:?}"),
            "{pair:?}: the undeclared door's outcome"
        );
        assert_eq!(
            format!("{declared:?}"),
            format!("{plain:?}"),
            "{pair:?}: the undeclared door's body"
        );
    }
}

/// A ball of radius `r` (a revolved semicircle).
fn ball(r: f64) -> Body<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(0.0, -r), 1.0),
        (Point2::new(0.0, r), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    revolve(&profile, axis_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

/// A donut (a revolved circle off the axis).
fn donut() -> Body<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(1.0, -0.3), 1.0),
        (Point2::new(1.0, 0.3), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    revolve(&profile, axis_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

fn axis_y() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    }
}

/// Two distinct keys of one kind on `body`: the first two faces of that
/// kind, one re-homed onto a fresh copy of its description when the
/// constructor put them all on one key.
fn two_keys_of(body: &mut Body<f64>, kind: SurfaceKind) -> (topo::SurfaceKey, topo::SurfaceKey) {
    let faces: Vec<_> = body
        .faces()
        .filter(|(_, f)| body.get_surface(f.surface).map(geom::Surface::kind) == Some(kind))
        .map(|(k, _)| k)
        .collect();
    assert!(faces.len() >= 2, "{kind:?}: need two faces, got {faces:?}");
    let keys: std::collections::BTreeSet<_> = faces
        .iter()
        .map(|&f| body.get_face(f).unwrap().surface)
        .collect();
    if keys.len() >= 2 {
        let mut it = keys.into_iter();
        return (it.next().unwrap(), it.next().unwrap());
    }
    distinct_keys(body, faces[0], faces[1])
}

/// Row 1⁶ (public door): a declared sphere pair and a declared torus
/// pair — a ball's two faces, a donut's two — are each one run closing
/// its carrier's full period: the record is that `PeriodClosure`,
/// naming both faces, the undeclared door's outcome and body. (After
/// the R1 review probe.)
#[test]
fn sphere_and_torus_pairs_record_their_period_closure() {
    for (label, mut body, kind) in [
        ("ball", ball(0.7), SurfaceKind::Sphere),
        ("donut", donut(), SurfaceKind::Torus),
    ] {
        assert_eq!(validate_closed(&body), Ok(()), "{label}: tier 2");
        let (k1, k2) = two_keys_of(&mut body, kind);
        let mut plain = body.clone();
        let want = plain.merge_coplanar_faces(Tol::witness()).unwrap();
        let outcome = body
            .merge_coplanar_faces_declared(&[(k1, k2)], Tol::witness())
            .unwrap_or_else(|e| panic!("{label}: {e:?}"));
        assert_eq!(
            format!("{outcome:?}"),
            format!("{want:?}"),
            "{label}: the undeclared door's outcome"
        );
        assert_eq!(
            format!("{body:?}"),
            format!("{plain:?}"),
            "{label}: the undeclared door's body"
        );
        assert!(outcome.groups.is_empty(), "{label}: {:?}", outcome.groups);
        let [skip] = &outcome.skipped[..] else {
            panic!("{label}: one record: {:?}", outcome.skipped);
        };
        assert!(
            matches!(skip.reason, MergeCoplanarError::PeriodClosure { .. }),
            "{label}: {:?}",
            skip.reason
        );
        assert_eq!(sorted(skip.faces.clone()), faces_on(&body, k1, k2));
    }
}

/// Row 2 (A, B): the floating and mid-bore pegs ship honest bodies —
/// additive volume, tiers 2, 3 and 3′ — whose only records are their
/// three wall runs' period closures. Each peg rim cuts the bore wall mid-height, and the
/// section loops' roles resolve on the rim arcs' own midpoints; the
/// result holds the peg exactly where it is, read at points, which an
/// additive volume alone does not pin.
#[test]
fn floating_and_mid_bore_pegs_ship_honest_with_their_period_closures() {
    use topo::SolidContainment::{In, Out};
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    for (label, (a, b, d), pts) in [
        (
            "A",
            scene_a(),
            [
                ((0.0, 0.0, 1.25), Out),
                ((0.0, 0.0, 1.75), In),
                ((0.0, 0.0, 2.25), In),
                ((1.0, 0.0, 1.5), In),
                ((1.0, 0.0, 2.25), Out),
            ],
        ),
        (
            "B",
            scene_b(),
            [
                ((0.0, 0.0, 0.75), In),
                ((0.0, 0.0, 1.25), In),
                ((0.0, 0.0, 1.75), Out),
                ((1.0, 0.0, 1.5), In),
                ((1.0, 0.0, 0.75), Out),
            ],
        ),
    ] {
        let bb = union_honest(label, &a, &b, &d);
        assert_eq!(
            assert_period_closures(label, &bb).len(),
            3,
            "{label}: three wall runs"
        );
        assert_eq!(bb.body.solids().count(), 1, "{label}: one solid");
        for ((x, y, z), want) in pts {
            assert_eq!(
                topo::point_in_solid(
                    &bb.body,
                    geom_core::Point3::new(x, y, z),
                    band,
                    Tol::witness()
                )
                .ok(),
                Some(want),
                "{label} at ({x}, {y}, {z})"
            );
        }
    }
}

/// Row 3 (C, D): one side of the declared pair is CONSUMED by the
/// union before the door runs — scene C's bore wall, scene D's peg wall
/// — and its surface goes with it, so the door records only the
/// surviving wall runs' period closures, and the body is one in
/// whichever order the caller lists the declarations. Both build
/// through the chord join (row 1).
#[test]
fn consumed_side_of_the_pair_leaves_only_period_closures() {
    let (c, p, d) = scene_c();
    let bb = union_honest("C", &c, &p, &d);
    assert_eq!(
        assert_period_closures("C", &bb).len(),
        2,
        "C: two wall runs"
    );

    let (p, q, d) = scene_d();
    let bb = union_honest("D", &p, &q, &d);
    assert_eq!(assert_period_closures("D", &bb).len(), 1, "D: one wall run");
    let mut reversed = BooleanDeclarations::none();
    reversed.coincident_faces = d.coincident_faces.iter().rev().cloned().collect();
    let rb = union_honest("D (planar pair last)", &p, &q, &reversed);
    assert_eq!(
        format!("{:?}", rb.body),
        format!("{:?}", bb.body),
        "D (planar pair last): one body"
    );
}

/// A peg body's planar cap key and one cylinder wall key.
fn plane_and_cylinder_keys(body: &Body<f64>) -> (topo::SurfaceKey, topo::SurfaceKey) {
    let plane = body.get_face(plane_face(body, 0.0, false)).unwrap().surface;
    let cyl = body.get_face(walls_at(body, BORE_R)[0]).unwrap().surface;
    (plane, cyl)
}

/// Row 4: a pair of two KINDS is a torn argument and keeps refusing at
/// the public door, typed, naming the SECOND key (the one whose kind
/// disagrees with the first's), with the body untouched — never
/// swallowed as a carrier record; and a torn pair anywhere in a longer
/// list refuses the whole call.
#[test]
fn mixed_kind_declared_pair_still_refuses_typed() {
    let mut body = peg_at(0.0, 0.0, 1.0);
    let (plane, cyl) = plane_and_cylinder_keys(&body);
    let before = format!("{body:?}");
    for (k1, k2) in [(plane, cyl), (cyl, plane)] {
        let err = body
            .merge_coplanar_faces_declared(&[(k1, k2)], Tol::witness())
            .expect_err("a plane/cylinder pair refuses");
        assert!(
            matches!(
                err,
                MergeCoplanarError::InvalidDeclaration {
                    surface,
                    what: "declared surfaces are not one kind",
                } if surface == k2
            ),
            "({k1:?}, {k2:?}): {err:?}"
        );
        assert_eq!(format!("{body:?}"), before, "body untouched on refusal");
    }
    let caps = (
        plane,
        body.get_face(plane_face(&body, 1.0, true)).unwrap().surface,
    );
    for list in [
        vec![(plane, cyl), caps, (cyl, cyl)],
        vec![caps, (plane, cyl), (cyl, cyl)],
        vec![caps, (cyl, cyl), (plane, cyl)],
    ] {
        let err = body
            .merge_coplanar_faces_declared(&list, Tol::witness())
            .expect_err("a torn pair anywhere in the list refuses");
        assert!(
            matches!(
                err,
                MergeCoplanarError::InvalidDeclaration {
                    surface,
                    what: "declared surfaces are not one kind",
                } if surface == cyl
            ),
            "{list:?}: {err:?}"
        );
        assert_eq!(format!("{body:?}"), before, "body untouched on refusal");
    }
}

/// Row 5: a key that does not resolve refuses BEFORE any kind is
/// read, whichever kind its partner has.
#[test]
fn unresolved_declared_key_still_refuses() {
    let mut body = peg_at(0.0, 0.0, 1.0);
    let (plane, cyl) = plane_and_cylinder_keys(&body);
    // The collar has more surfaces than the peg, so its highest key
    // names a slot the peg's arena does not have.
    let dangling = collar().surfaces().map(|(k, _)| k).max().unwrap();
    assert!(
        body.get_surface(dangling).is_none(),
        "{dangling:?} must not resolve in the peg"
    );
    let before = format!("{body:?}");
    for pair in [
        (dangling, plane),
        (plane, dangling),
        (dangling, cyl),
        (cyl, dangling),
    ] {
        let err = body
            .merge_coplanar_faces_declared(&[pair], Tol::witness())
            .expect_err("an unresolved key refuses");
        assert!(
            matches!(
                err,
                MergeCoplanarError::InvalidDeclaration {
                    surface,
                    what: "declared surface key does not resolve",
                } if surface == dangling
            ),
            "{pair:?}: {err:?}"
        );
        assert_eq!(format!("{body:?}"), before, "body untouched on refusal");
    }
}

/// Row 7 (E, F): two equal pegs stacked end to end, their walls one
/// cylinder carried on across the caps — continuations. With every
/// wall pair declared a continuation (F) the union is exact and
/// honest, and its six wall faces are one run, all of one sense, each
/// lower sector meeting its upper across the z = 1 rim, whose glue
/// would close the full period: the one record is that
/// `PeriodClosure`. With only the caps declared, or nothing (E), the
/// walls are one carrier by margin and the union is F's body bit for
/// bit (D10).
#[test]
fn stacked_equal_pegs_same_sense_walls() {
    let lo = finished("the lower peg", peg_at(0.0, 0.0, 1.0), Tol::witness());
    let hi = finished("the upper peg", peg_at(0.0, 1.0, 1.0), Tol::witness());
    let mut caps = BooleanDeclarations::none();
    caps.coincident_faces.push(FacePairDeclaration::new(
        plane_face(&lo, 1.0, true),
        plane_face(&hi, 1.0, false),
        ContactClass::Rest,
    ));
    let ce = topo::union_with(&lo, &hi, &caps, Tol::witness());
    let ue = topo::union(&lo, &hi, Tol::witness());
    let mut both = caps.clone();
    for &fa in &walls_at(&lo, BORE_R) {
        for &fb in &walls_at(&hi, BORE_R) {
            both.coincident_faces
                .push(FacePairDeclaration::continuation(fa, fb));
        }
    }
    let bb = union_honest("F", &lo, &hi, &both);
    let v = volume(&bb.body);
    assert!(
        (v - core::f64::consts::FRAC_PI_2).abs()
            <= 4.0 * f64::EPSILON * core::f64::consts::FRAC_PI_2,
        "F: two unit-height r = 1/2 pegs: {v}"
    );
    let [skip] = &assert_period_closures("F", &bb)[..] else {
        panic!("F: one record: {:?}", bb.naming.merge_skipped);
    };
    assert_eq!(
        skip.faces.len(),
        6,
        "F: the record names both walls: {skip:?}"
    );
    for (what, r) in [("E, caps only", ce), ("E, undeclared", ue)] {
        let Ok(topo::BooleanResult::Body(e)) = r else {
            panic!("{what}: builds: {r:?}");
        };
        assert_eq!(
            format!("{:?}", e.body),
            format!("{:?}", bb.body),
            "{what}: F's body"
        );
    }
    let faces = bore_radius_faces(&bb.body);
    assert_eq!(
        faces.len(),
        6,
        "F: both walls survive, split at z = 1: {faces:?}"
    );
    assert!(
        faces.iter().all(|&(_, s)| s == faces[0].1),
        "F: one sense throughout — a continuation, not a slit: {faces:?}"
    );
    let keys: Vec<_> = faces.iter().map(|&(f, _)| f).collect();
    let cross: Vec<_> = shared_edges(&bb.body, &keys)
        .into_iter()
        .filter(|&(_, f1, f2)| {
            bb.body.get_face(f1).unwrap().surface != bb.body.get_face(f2).unwrap().surface
        })
        .collect();
    assert_eq!(
        cross.len(),
        3,
        "F: each lower sector meets its upper across the z = 1 rim: {cross:?}"
    );
}

/// The cycle halves of `f`'s loops, outer first.
fn face_halves(body: &Body<f64>, f: topo::FaceKey) -> Vec<topo::HalfEdgeKey> {
    let face = body.get_face(f).unwrap();
    core::iter::once(face.outer)
        .chain(face.rings.iter().copied())
        .filter_map(|lk| match body.get_loop(lk).unwrap().boundary {
            topo::LoopBoundary::Cycle { first } => Some(body.loop_cycle(first).unwrap()),
            topo::LoopBoundary::Empty { .. } => None,
        })
        .flatten()
        .collect()
}

/// `f`'s `(rows stored, rows missing)`.
fn row_count(body: &Body<f64>, f: topo::FaceKey) -> (usize, usize) {
    let halves = face_halves(body, f);
    let stored = halves
        .iter()
        .filter(|&&he| body.pcurve(he).is_some())
        .count();
    (stored, halves.len() - stored)
}

/// The merge door on a curved sub-period run whose sectors arrive one
/// never minted, the other complete, through public doors alone: the
/// D-prism's two arc faces, one re-seated onto a new key and then onto
/// the other's key, which the setter's chart change leaves rowless.
/// The door holds a band and re-mints the staged result, so it merges
/// the run whichever sector was the rowless one — the survivor or the
/// absorbed sector — and leaves no face half-minted. (A full-period
/// run, such as a peg's three wall sectors, closes its period and is
/// recorded instead, so the run here is the prism's 233° arc.)
///
/// Adopted from the review of the loop-reparenting doors' re-mint
/// (its `reviewer_c2_merge_door_public_only` probe).
#[test]
fn a_curved_run_with_one_rowless_sector_merges_whichever_sector_it_is() {
    let tol = Tol::witness();
    let r = (0.25f64 * 0.25 + 0.5 * 0.5).sqrt();
    let mut rowless_kept = Vec::new();
    for i in 0..2usize {
        let (mut body, _, _) = d_prism_with_split_keys();
        topo::mint_pcurves(&mut body, tol).unwrap();
        let arcs = walls_at(&body, r);
        assert_eq!(arcs.len(), 2, "the prism's arc run is two sectors");
        let other = arcs[1 - i];
        let k = body.get_face(other).unwrap().surface;
        let described = body.get_surface(k).unwrap().clone();
        let sense = body.get_face(arcs[i]).unwrap().sense;
        // Lifts RechartStrandsDescriptions: the row's premise is the key the face left, which its descriptions still name.
        body.set_face_surface_unvouched_for_tests(
            arcs[i],
            FaceSurface::New {
                surface: described,
                sense,
            },
        )
        .unwrap();
        // Lifts RechartUnvouched: the row's premise is the rowless sector the chart change leaves.
        body.set_face_surface_unvouched_for_tests(arcs[i], FaceSurface::Shared { key: k, sense })
            .unwrap();
        assert_eq!(
            row_count(&body, arcs[i]).0,
            0,
            "case {i}: arc {i} arrives rowless"
        );
        assert_eq!(
            row_count(&body, other).1,
            0,
            "case {i}: the other sector arrives complete"
        );
        let outcome = body
            .merge_coplanar_faces(tol)
            .unwrap_or_else(|e| panic!("case {i}: the merge door refused: {e:?}"));
        assert!(
            outcome.skipped.is_empty(),
            "case {i}: the run is merged, not recorded as skipped: {:?}",
            outcome
                .skipped
                .iter()
                .map(|s| &s.reason)
                .collect::<Vec<_>>()
        );
        let run = [arcs[i], other];
        let groups: Vec<_> = outcome
            .groups
            .iter()
            .filter(|g| run.contains(&g.kept))
            .collect();
        let [group] = groups[..] else {
            panic!(
                "case {i}: the two-sector run is one group: {:?}",
                outcome.groups
            );
        };
        assert!(
            group.absorbed.len() == 1 && run.contains(&group.absorbed[0]),
            "case {i}: the group is the run: kept {:?}, absorbed {:?}",
            group.kept,
            group.absorbed
        );
        rowless_kept.push(group.kept == arcs[i]);
        let half_minted: Vec<_> = body
            .faces()
            .map(|(f, _)| (f, row_count(&body, f)))
            .filter(|&(_, (stored, missing))| stored > 0 && missing > 0)
            .collect();
        assert!(
            half_minted.is_empty(),
            "case {i}: half-minted faces after the merge: {half_minted:?}"
        );
    }
    assert!(
        rowless_kept.contains(&true) && rowless_kept.contains(&false),
        "the rowless sector is the survivor in one case and the absorbed sector in the other: \
         {rowless_kept:?}"
    );
}
