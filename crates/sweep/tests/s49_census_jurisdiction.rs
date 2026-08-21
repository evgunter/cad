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
//!
//! **§H14's row is here too**, on the same fixtures and for the same
//! reason: arm 1's v-on-f deferral named the other SOLID where the
//! finding it suppresses is about the other FACE, and separating the
//! two needs one solid with several faces reaching the same plane
//! without sharing a vertex — which the three-arc cylinder's three
//! wall faces give. See the banner below.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Affine3, Point2, Tolerance, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, ContactRecords, EntityId, FaceKey, ValidationError, validate_pseudomanifold};
use geom_core::Tol;

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
        .validate(Tol::witness())
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
        .validate(Tol::witness())
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

// =====================================================================
// §H14 — the same defect one deferral over, in the same arm.
//
// Arm 1's v-on-f deferral read `planar_face_bridged(a.face, b.solid)`:
// a record naming the planar face and ANY vertex of the other SOLID.
// The finding it suppresses is about the FACE PAIR, so one declared
// interface silenced that planar face against every other face of the
// same solid — including one resting on it elsewhere with no vertex
// evidence of its own, which is the class this arm exists for. It now
// requires the record's vertex to be a boundary vertex of the other
// face OF THIS PAIR.
//
// The witness needs one solid with several faces reaching the same
// plane and not sharing a vertex, which the three-arc cylinder gives
// for free: its three wall faces each own two of the three rim
// vertices, so declaring ONE rim vertex leaves exactly one wall that
// owns none of the declared ones.
// =====================================================================

/// The boundary vertices of `f`, outer loop then rings, in walk order.
fn face_vertices(body: &Body<f64>, f: FaceKey) -> Vec<topo::VertexKey> {
    let data = body.get_face(f).unwrap();
    let mut out = Vec::new();
    for &lk in core::iter::once(&data.outer).chain(&data.rings) {
        let Some(l) = body.get_loop(lk) else { continue };
        let topo::LoopBoundary::Cycle { first } = l.boundary else {
            // A lone-vertex loop has no cycle to walk. Extruded bodies
            // have none; the skip is a shape requirement of the walk,
            // not a judgement that an empty loop carries nothing —
            // which is the reading §H14's residue 2 was about.
            continue;
        };
        for he in body.loop_cycle(first).unwrap() {
            out.push(body.get_half_edge(he).unwrap().start);
        }
    }
    out
}

/// Every `CensusUndecidable` naming a pair of FACES, in either order.
fn face_pair_refusals(errors: &[ValidationError]) -> Vec<(FaceKey, FaceKey)> {
    errors
        .iter()
        .filter_map(|e| match e {
            ValidationError::CensusUndecidable {
                a: EntityId::Face(a),
                b: EntityId::Face(b),
                ..
            } => Some((*a, *b)),
            _ => None,
        })
        .collect()
}

/// The cylinder's three wall faces, split by whether they hold `v` on
/// their boundary: `(holding, not_holding)`.
fn walls_by_vertex(body: &Body<f64>, v: topo::VertexKey) -> (Vec<FaceKey>, Vec<FaceKey>) {
    let mut holding = Vec::new();
    let mut apart = Vec::new();
    for (f, data) in body.faces() {
        if !matches!(
            body.get_surface(data.surface),
            Some(Surface::Cylinder { .. })
        ) {
            continue;
        }
        let mut owns = false;
        for &lk in core::iter::once(&data.outer).chain(&data.rings) {
            let Some(l) = body.get_loop(lk) else { continue };
            let topo::LoopBoundary::Cycle { first } = l.boundary else {
                // A lone-vertex loop has no cycle to walk. Extruded
                // bodies have none; the skip is a shape requirement of
                // the walk, not a judgement that an empty loop carries
                // nothing — the reading §H14's residue 2 was about.
                continue;
            };
            for he in body.loop_cycle(first).unwrap() {
                if body.get_half_edge(he).unwrap().start == v {
                    owns = true;
                }
            }
        }
        if owns { holding.push(f) } else { apart.push(f) }
    }
    (holding, apart)
}

/// **§H14's regression row.** A three-arc cylinder standing on a
/// brick, with ONE of its three rim vertices declared v-on-f on the
/// brick's top face. The deferral that record earns must cover the
/// faces at THAT interface and no others.
///
/// Both directions are asserted from the one fixture, which is what
/// makes it a statement about granularity rather than about loudness:
/// the two walls holding the declared vertex stay deferred, and the
/// one wall holding none of them is examined and refused. A deferral
/// keyed on the other SOLID defers all three; a deferral deleted
/// outright refuses all three.
#[test]
fn a_vf_record_defers_the_faces_at_its_own_interface_and_no_others() {
    let body = assembly(&brick(0.0, 1.0), &cylinder(1.0, 0.0));
    let touching = faces_in_plane(&body, 1.0);
    assert_eq!(touching.len(), 2, "the cap and the face it rests on");
    // The brick's top face is the line-bounded one (four vertices);
    // the cap has three, joined by arcs.
    let brick_top = *touching
        .iter()
        .find(|&&f| face_vertices(&body, f).len() == 4)
        .expect("the brick's top face");
    let cap = *touching
        .iter()
        .find(|&&f| face_vertices(&body, f).len() == 3)
        .expect("the cylinder's cap");
    // Declare exactly one rim vertex on the brick's face. It is a real
    // coincidence (the rim sits in that face's region), so the confirm
    // pass has nothing to say about it.
    let rim = face_vertices(&body, cap);
    let v0 = rim[0];
    let mut records = ContactRecords::default();
    records.b_on_a.push(topo::VfContact {
        vertex: v0,
        face: brick_top,
    });
    // The fixture's own precondition, and it is what makes the
    // "stays deferred" assertion below a statement about the
    // DEFERRAL: if the record ever stops being built, this reddens
    // here rather than at an assertion that would read as the
    // deferral having been deleted. The two are indistinguishable
    // downstream — both leave every wall refused.
    assert_eq!(
        records.b_on_a.len(),
        1,
        "the fixture declares exactly one rest, on the brick's top face"
    );
    assert_eq!(records.b_on_a[0].face, brick_top);
    let (holding, apart) = walls_by_vertex(&body, v0);
    assert_eq!(holding.len(), 2, "two walls share each rim vertex");
    assert_eq!(apart.len(), 1, "one wall holds neither of its ends");

    let errors =
        validate_pseudomanifold(&body, &records).expect_err("an undeclared rest must be reported");
    assert!(
        !errors
            .iter()
            .any(|e| matches!(e, ValidationError::StaleContactDeclaration { .. })),
        "the fixture's precondition: the declared rest confirms, so no refusal \
         below is borrowed from a refuted record: {errors:?}"
    );
    let pairs = face_pair_refusals(&errors);
    assert!(
        names_pair(&pairs, (brick_top, apart[0])),
        "the wall holding no declared vertex is not at the declared interface \
         and must be examined: wanted {:?}, refusals were {pairs:?}",
        (brick_top, apart[0])
    );
    for &w in &holding {
        assert!(
            !names_pair(&pairs, (brick_top, w)),
            "the walls at the declared interface stay deferred — the narrowing \
             is a narrowing, not a deletion: {:?} in {pairs:?}",
            (brick_top, w)
        );
    }
    assert!(
        !names_pair(&pairs, (brick_top, cap)),
        "the cap holds the declared vertex and stays deferred: {pairs:?}"
    );
}
