//! **SMELL-SCAN §S49 — who owns a planar face on a curved solid.**
//!
//! The census's cross-solid proximity arm (arm 1) used to skip every
//! pair of PLANAR faces, justified by a claim about planar-only
//! SOLIDS: *"a solid bounded by planes has straight edges at its face
//! junctions"*. A cylinder's cap is a planar face on a solid that is
//! not planar-only, and its rim is an arc — so the skip fired on
//! pairs the justification never covered.
//!
//! The premise is really about EDGES, not solids: the census snapshot
//! keeps line edges and drops curved ones, so only a wholly
//! line-bounded planar face has its whole boundary in front of the
//! exact vertex/line sweeps. Arm 1 now skips exactly that pair and
//! keeps the rest, because neither other lane will take them: the
//! conformal arm groups CURVED faces by carrier key, and the confirm
//! pass examines DECLARED pairs only.
//!
//! These rows need a real arc-bounded planar face, so they live here
//! rather than in `topo`'s own suite. The fixture is #S16's extruded
//! three-arc cylinder (radius 0.5, three arc edges per cap).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Affine3, Point2, Tolerance, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, ContactRecords, EntityId, FaceKey, ValidationError, validate_pseudomanifold};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The three-arc cylinder of §S16, at `z ∈ [z0, z0 + 1]` and turned
/// by `rot` degrees about its axis: radius 0.5, three ARC edges per
/// cap, three cap vertices at `rot + {0°, 120°, 240°}`.
fn cylinder(z0: f64, rot: f64) -> Body<f64> {
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th: f64 = (deg + rot).to_radians();
        p2(0.5 * th.cos(), 0.5 * th.sin())
    };
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(at(0.0), b120),
        ProfileVertex::new(at(120.0), b120),
        ProfileVertex::new(at(240.0), b120),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0)).unwrap().body
}

/// A planar-only brick: half-width `h` about the axis, `z ∈ [z0, z0 +
/// 1]`. Every edge is a line, so every face is line-bounded.
fn brick(z0: f64, h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(
        [(-h, -h), (h, -h), (h, h), (-h, h)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    );
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0)).unwrap().body
}

/// The pair as one two-instance arena.
fn assembly(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let mut out = a.clone();
    topo::graft_disjoint(&mut out, b).unwrap();
    out
}

fn is_planar(body: &Body<f64>, f: FaceKey) -> bool {
    body.get_face(f)
        .and_then(|d| body.get_surface(d.surface))
        .is_some_and(|s| matches!(s, Surface::Plane { .. }))
}

/// The planar faces whose plane is `z = h`. In these fixtures that is
/// exactly the two faces in contact, so a row can name the pair it
/// means instead of accepting any planar refusal.
fn faces_in_plane(body: &Body<f64>, h: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| match body.get_surface(f.surface) {
            Some(Surface::Plane { origin, normal, .. }) => {
                normal.x.abs() < 1e-12 && normal.y.abs() < 1e-12 && (origin.z - h).abs() < 1e-12
            }
            _ => false,
        })
        .map(|(k, _)| k)
        .collect()
}

/// Does some refusal name exactly the unordered pair `want`?
fn names_pair(pairs: &[(FaceKey, FaceKey)], want: (FaceKey, FaceKey)) -> bool {
    pairs.iter().any(|&(a, b)| (a, b) == want || (b, a) == want)
}

/// Every `CensusUndecidable` naming a pair of PLANAR faces. The only
/// planar faces in these fixtures are the caps, and a same-solid pair
/// never reaches the arm, so such a finding IS the cross-solid cap
/// pair.
fn planar_pair_refusals(body: &Body<f64>, errors: &[ValidationError]) -> Vec<(FaceKey, FaceKey)> {
    errors
        .iter()
        .filter_map(|e| match e {
            ValidationError::CensusUndecidable {
                a: EntityId::Face(a),
                b: EntityId::Face(b),
                ..
            } if is_planar(body, *a) && is_planar(body, *b) => Some((*a, *b)),
            _ => None,
        })
        .collect()
}

/// **The regression row.** Two cylinders in cap-to-cap conformal rest,
/// undeclared, with the upper one turned so no cap vertex of either
/// lands in the other cap's interior: at vertex/line granularity there
/// is nothing to see (the caps' rims are arcs, and `snapshot` drops
/// curved edges), the conformal arm never looks at a planar face, and
/// there is no record for the confirm pass. If arm 1 does not own this
/// pair, NO lane examines it.
///
/// It is swept over the turn so the row cannot pass on the one
/// alignment that happens to work: 60 degrees puts each cap's vertices
/// on the other's rim, 30/45 put them clear of it, and 0 stacks them
/// coincident.
#[test]
fn a_cap_pair_in_rest_is_examined_by_the_proximity_arm() {
    for &turn in &[0.0_f64, 30.0, 45.0, 60.0] {
        let body = assembly(&cylinder(0.0, 0.0), &cylinder(1.0, turn));
        let errors = validate_pseudomanifold(&body, &ContactRecords::default())
            .expect_err("two solids in undeclared rest must never clear");
        let caps = faces_in_plane(&body, 1.0);
        assert_eq!(
            caps.len(),
            2,
            "turn {turn}: expected two caps at z = 1: {caps:?}"
        );
        let want = (caps[0], caps[1]);
        let pairs = planar_pair_refusals(&body, &errors);
        assert!(
            names_pair(&pairs, want),
            "turn {turn}: the proximity arm must name the cap pair {want:?} in the \
             contact plane — the other three cross-solid cap pairs are two metres \
             away and clear on a definite margin; planar refusals were {pairs:?}, \
             all findings {errors:?}"
        );
    }
}

/// The other direction, so the row above cannot pass by refusing
/// everything: the same two cylinders with a 2 m gap between the
/// facing caps (`z = 1` against `z = 3`) validate with no findings at
/// all - the caps are boxable, so the arm CLEARS them on a definite
/// margin rather than refusing every cap pair it now examines.
#[test]
fn cylinders_apart_still_clear_at_their_caps() {
    let body = assembly(&cylinder(0.0, 0.0), &cylinder(3.0, 60.0));
    assert_eq!(
        validate_pseudomanifold(&body, &ContactRecords::default()),
        Ok(()),
        "a definitely separated pair must clear"
    );
}

/// **The premise, pinned.** Two line-bounded planar faces in rest stay
/// with the exact sweeps: the small brick's bottom vertices lie inside
/// the big one's top face, so the vertex-on-face sweep reports the
/// undeclared contact and the backstop stays out of it. The pair is
/// within reach - the two boxes share the plane `z = 1` - so a
/// backstop that examined every planar pair would refuse here instead,
/// on a body whose contact the exact sweeps already decided.
#[test]
fn line_bounded_planar_faces_in_rest_stay_with_the_exact_sweeps() {
    let body = assembly(&brick(0.0, 0.5), &brick(1.0, 0.2));
    let errors = validate_pseudomanifold(&body, &ContactRecords::default())
        .expect_err("an undeclared rest must be reported");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, ValidationError::UndeclaredContact { .. })),
        "the exact sweeps must see the rest: {errors:?}"
    );
    assert!(
        planar_pair_refusals(&body, &errors).is_empty(),
        "a wholly line-bounded planar pair is the exact sweeps' - the backstop \
         must not double-refuse it: {errors:?}"
    );
}

/// The mixed pair, which the old predicate also skipped: a cylinder
/// standing on a brick. The cap is arc-bounded and the brick's face is
/// not, so the pair is arm 1's - one line-bounded side is not enough
/// to put the whole interface in the snapshot.
#[test]
fn a_cap_resting_on_a_line_bounded_face_is_examined_too() {
    let body = assembly(&brick(0.0, 1.0), &cylinder(1.0, 0.0));
    let errors = validate_pseudomanifold(&body, &ContactRecords::default())
        .expect_err("two solids in undeclared rest must never clear");
    let touching = faces_in_plane(&body, 1.0);
    assert_eq!(
        touching.len(),
        2,
        "expected the cap and the face it rests on: {touching:?}"
    );
    let want = (touching[0], touching[1]);
    assert!(
        names_pair(&planar_pair_refusals(&body, &errors), want),
        "the proximity arm must name the cap x face pair {want:?}, got {errors:?}"
    );
}
