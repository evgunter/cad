//! MATE-7a — the torus declared-`Rest` lane's first unit (issue 968
//! items 1–2 and the ratified π arm of
//! `docs/MATE-7-TANGENCY-DESIGN.md`).
//!
//! Three things are pinned here, on torus geometry a producer actually
//! mints (`sweep::tube_along_arc`, solid and hollow):
//!
//! 1. **The operand gate's covered-pair rung.** A torus pair the
//!    caller's declarations speak for is no longer refused by the KIND
//!    roster; an undeclared one is refused exactly as before.
//! 2. **The carrier ladder's torus rung**, reached through the public
//!    door: a declared torus×torus `Rest` pair on one carrier is
//!    VERIFIED rather than turned away at the declaration door for
//!    lack of a rung, and a declaration the carrier data contradicts
//!    is refused loudly.
//! 3. **The ruling's shared-rim routing**, both arms, on real rims: a
//!    G1 tube chain classifies wedge π (the seam — the built arm), and
//!    a kissing torus pair classifies wedge 2π (the cusp family, whose
//!    certified rim witness is defined and unbuilt). Each refuses
//!    typed naming the arm the geometry earned.
//!
//! **What this suite also RECORDS is where the lane stops**, because
//! the stopping point is the unit's measurement and not an omission:
//! an admitted torus pair reaches the crossing layer and refuses there
//! at the curved-pierce frontier. Every edge a torus-walled body
//! carries is a CIRCLE, and the circle×face clearance enclosure the
//! frontier consults has no torus arm, so it declines before the
//! declared-cover rung behind it can be consulted at all. Two rows
//! below hold that boundary still, so the day the enclosure grows an
//! arm they are what changes.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use geom_core::{Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use revolve_common::{axis_y, p2, validated};
use sweep::{Revolution, TubeWindow, revolve, tube_along_arc, tube_along_arc_hollow};
use topo::query;
use topo::{
    Body, BooleanDeclarations, BooleanError, BooleanResult, ContactClass, FaceKey,
    FacePairDeclaration,
};

const TUBE: f64 = 0.06;
/// The lower stem arc's ring — the lily's own numbers, so the fixtures
/// are the demand signal's geometry rather than a rounder one.
const RING: f64 = 5.0;
const TURN: f64 = 22.0;

fn axis() -> Vec3<f64> {
    Vec3::new(0.0, -1.0, 0.0)
}

fn window(deg: f64) -> TubeWindow<f64> {
    TubeWindow::Arc {
        t0: 0.0,
        t1: deg.to_radians(),
    }
}

/// Segment A of the chain: a tube along the lily's lower stem arc,
/// starting at the origin heading `+z`.
fn segment_a() -> Body<f64> {
    tube_along_arc(
        Point3::new(-RING, 0.0, 0.0),
        axis(),
        Vec3::new(1.0, 0.0, 0.0),
        RING,
        window(TURN),
        TUBE,
        Tol::witness(),
    )
    .expect("segment A builds")
    .body
}

/// Segment B: the SAME tube radius on a tighter ring, continuing from
/// A's end with A's own end tangent — so the two walls meet G1 along
/// the shared terminal meridian circle and the composed material runs
/// smoothly through it.
fn segment_b() -> Body<f64> {
    let turn = TURN.to_radians();
    let end = Point3::new(-RING + RING * turn.cos(), 0.0, RING * turn.sin());
    let tangent = Vec3::new(-turn.sin(), 0.0, turn.cos());
    // The tighter ring's centre sits perpendicular to that tangent, on
    // the same side A is turning toward.
    let inward = Vec3::new(-tangent.z, 0.0, tangent.x);
    let center = end + inward * 1.1;
    tube_along_arc(
        center,
        axis(),
        (end - center).normalize(),
        1.1,
        window(170.0),
        TUBE,
        Tol::witness(),
    )
    .expect("segment B builds")
    .body
}

/// The toroidal SOCKET: a hollow elbow whose BORE is a torus of tube
/// radius [`TUBE`] — the curved spelling of the bored plate.
fn socket() -> Body<f64> {
    tube_along_arc_hollow(
        Point3::new(-RING, 0.0, 0.0),
        axis(),
        Vec3::new(1.0, 0.0, 0.0),
        RING,
        window(TURN),
        0.09,
        0.09 - TUBE,
        Tol::witness(),
    )
    .expect("socket builds")
    .body
}

/// Two FULL tori KISSING along one circle: coaxial and coplanar, ring
/// radii `R` and `R + 2r`, so the two tube centre circles are `2r`
/// apart and the tubes touch externally along the midplane circle of
/// radius `R + r`. That circle is the outer equator seam of the inner
/// torus and the inner equator seam of the outer one, so it is a
/// boundary edge of a face on each side — a rim, not an interior
/// touch.
fn kissing_pair() -> (Body<f64>, Body<f64>) {
    (full_torus(RING), full_torus(RING + 2.0 * TUBE))
}

/// A full solid torus of ring radius `major` and tube [`TUBE`], about
/// the origin on `+z`. It carries its two wall faces and nothing else.
fn full_torus(major: f64) -> Body<f64> {
    tube_along_arc(
        Point3::origin(),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
        major,
        TubeWindow::Full,
        TUBE,
        Tol::witness(),
    )
    .expect("the full torus builds")
    .body
}

/// The torus faces of a body whose tube radius is `minor`.
fn torus_faces(body: &Body<f64>, minor: f64) -> Vec<FaceKey> {
    let hits: Vec<_> = query::all_faces(body)
        .into_iter()
        .filter(|&f| {
            matches!(
                body.get_face(f).and_then(|fd| body.get_surface(fd.surface)),
                Some(geom::Surface::Torus { minor_radius, .. })
                    if (*minor_radius - minor).abs() < 1e-12
            )
        })
        .collect();
    assert!(
        !hits.is_empty(),
        "the fixture must carry torus faces at tube radius {minor}"
    );
    hits
}

/// Every torus face pair across the two operands, declared under
/// `class`.
fn wall_declarations(
    a: &Body<f64>,
    b: &Body<f64>,
    minor: f64,
    class: ContactClass,
) -> BooleanDeclarations {
    let mut decls = BooleanDeclarations::none();
    for &fa in &torus_faces(a, minor) {
        for &fb in &torus_faces(b, minor) {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(fa, fb, class));
        }
    }
    decls
}

// -------------------------------------------------------------------
// 1. The declaration door: the carrier ladder's torus rung.
// -------------------------------------------------------------------

/// **The rung exists.** Before it, a `Rest` declaration on a torus
/// face was turned away at the front door — the carrier inventory
/// named plane, sphere and cylinder, so no torus pair could be stated
/// at all, let alone verified. The socket's bore and the peg's wall
/// are ONE carrier with opposed material sides, which is what `Rest`
/// means; the declaration is now admitted and the ladder runs on it.
///
/// What the op then does is the next row's subject. This one is about
/// the door, so it asserts only that the refusal is no longer the
/// door's.
#[test]
fn a_declared_torus_rest_pair_passes_the_declaration_door() {
    let (s, p) = (socket(), segment_a());
    let decls = wall_declarations(&s, &p, TUBE, ContactClass::Rest);
    assert!(
        !decls.coincident_faces.is_empty(),
        "the socket's bore and the peg's wall must both be torus faces"
    );
    let err = topo::union_with(&s, &p, &decls, Tol::witness())
        .expect_err("the lane still stops downstream — see the frontier row");
    assert!(
        !matches!(
            err,
            BooleanError::InvalidDeclaration { .. }
                | BooleanError::ContactContradicted { .. }
                | BooleanError::UndeclaredCoincidence { .. }
        ),
        "the torus Rest declaration must be admitted and verified, not refused at the \
         declaration door: {err:?}"
    );
}

/// **The rung DECIDES, it does not merely admit.** A peg whose tube is
/// definitely thinner than the bore is not the bore's carrier, and
/// declaring it `Rest` is a false statement the ladder contradicts at
/// the minor-radius margin — the torus arm's version of the 0.4 peg
/// against the 0.5 bore.
#[test]
fn a_contradicted_torus_rest_declaration_refuses_loudly() {
    let s = socket();
    let thin = tube_along_arc(
        Point3::new(-RING, 0.0, 0.0),
        axis(),
        Vec3::new(1.0, 0.0, 0.0),
        RING,
        window(TURN),
        TUBE * 0.75,
        Tol::witness(),
    )
    .expect("the thin peg builds")
    .body;
    let mut decls = BooleanDeclarations::none();
    for &fa in &torus_faces(&s, TUBE) {
        for &fb in &torus_faces(&thin, TUBE * 0.75) {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    let err = topo::union_with(&s, &thin, &decls, Tol::witness())
        .expect_err("a thin peg declared Rest against a wider bore must contradict");
    match err {
        BooleanError::ContactContradicted { margin, .. } => {
            assert_eq!(
                margin.predicate,
                Some("carrier_torus_minor_radius"),
                "the refusal must name the datum that decided: {margin:?}"
            );
        }
        other => panic!("expected ContactContradicted, got {other:?}"),
    }
}

// -------------------------------------------------------------------
// 2. The operand gate's covered-pair rung.
// -------------------------------------------------------------------

/// **Undeclared, the KIND roster still refuses at the gate**, and it
/// refuses pair-scoped with `op: None` — the invariant half. Nothing
/// about the widening loosens what an undeclared torus operand
/// reaches.
#[test]
fn an_undeclared_torus_pair_still_gates() {
    let err = topo::union(&socket(), &segment_a(), Tol::witness())
        .expect_err("an undeclared torus pair must still refuse");
    assert!(
        matches!(
            err,
            BooleanError::CurvedPairUnsupported {
                op: None,
                kind: geom_brep::SurfaceKind::Torus,
                ..
            }
        ),
        "the undeclared refusal stays the operand gate's, naming the torus: {err:?}"
    );
}

/// **Fully covered, the gate has nothing left to say.** A full torus
/// carries NOTHING but its two wall faces, so declaring every cross
/// pair covers every pair the KIND roster could object to — and the
/// operation reaches the classification layer, which is the depth
/// admission buys.
///
/// The two operands are the SAME torus, which is what makes the
/// declaration true rather than convenient: one carrier, one material
/// side — the ladder's honest `SameOriented`, a merge-stage pair, and
/// a verdict the `Rest` door accepts.
#[test]
fn a_fully_covered_torus_pair_reaches_past_the_operand_gate() {
    let (a, b) = (full_torus(RING), full_torus(RING));
    assert_eq!(
        a.faces().count(),
        torus_faces(&a, TUBE).len(),
        "a full torus must carry nothing but wall faces, or the covering below is partial"
    );
    let decls = wall_declarations(&a, &b, TUBE, ContactClass::Rest);
    let undeclared =
        topo::union(&a, &b, Tol::witness()).expect_err("undeclared, the KIND roster refuses");
    assert!(
        matches!(
            undeclared,
            BooleanError::CurvedPairUnsupported {
                op: None,
                kind: geom_brep::SurfaceKind::Torus,
                ..
            }
        ),
        "the undeclared refusal is the operand gate's: {undeclared:?}"
    );
    let declared = topo::union_with(&a, &b, &decls, Tol::witness())
        .expect_err("the lane still stops downstream of the gate");
    assert!(
        !matches!(declared, BooleanError::CurvedPairUnsupported { .. }),
        "every offending pair is covered, so the gate must not be what refuses: {declared:?}"
    );
}

/// **Coverage is per PAIR, and a partial covering does not open the
/// gate.** The socket carries an OUTER wall as well as its bore; the
/// declarations name the bore against the peg, so the outer wall
/// against the peg stays uncovered and the gate still refuses — naming
/// that pair rather than a declared one.
///
/// That refusal is also this unit's measurement of what BINDS. The two
/// loci it names never meet: the socket's outer wall stands 0.03 m
/// clear of everything the peg occupies. What overlaps is the
/// whole-torus BOX, which reads nothing from the face's boundary.
///
/// **What that shows, stated no wider than the fixture supports.** An
/// undeclared face defeats the covered rung when its box meets the
/// whole-RING box of a torus face on the other operand — which is what
/// happens here, and what happens easily, since a windowed torus is
/// boxed as its entire ring. It is NOT the claim that any extra face
/// defeats it: a face whose box clears the ring is exactly what the
/// pair-scoped gate was built to let through, and it would not gate.
/// The two regimes this suite actually demonstrates are the row above
/// (operands carrying nothing but wall faces, so every pair is
/// coverable) and this one (an extra face whose box does meet the
/// ring). Where a boundary-tight torus box would move the line between
/// them is issue 1488's question, not this unit's.
#[test]
fn a_partly_covered_torus_pair_still_gates_on_the_uncovered_one() {
    let (s, p) = (socket(), segment_a());
    let decls = wall_declarations(&s, &p, TUBE, ContactClass::Rest);
    let err = topo::union_with(&s, &p, &decls, Tol::witness())
        .expect_err("the socket's outer wall is nobody's declared pair");
    match err {
        BooleanError::CurvedPairUnsupported {
            face, other_face, ..
        } => {
            assert!(
                !decls
                    .coincident_faces
                    .iter()
                    .any(|d| (d.a, d.b) == (face, other_face)),
                "the gate must refuse an UNCOVERED pair, never the declared one"
            );
        }
        other => panic!("expected the operand gate's refusal, got {other:?}"),
    }
}

/// **Where the lane stops once the gate is past, held still.** The
/// admitted pair reaches the crossing layer and refuses at the
/// curved-pierce frontier: every edge of a torus-walled body is a
/// CIRCLE, and the circle-versus-face clearance enclosure the frontier
/// consults has no torus arm, so it declines before the declared-cover
/// rung behind it is consulted at all. This is a boundary, not a
/// verdict about the declaration — and it is asserted so that growing
/// that enclosure shows up here as a change rather than as silence.
#[test]
fn the_admitted_torus_lane_stops_at_the_curved_pierce_frontier() {
    let (a, b) = (full_torus(RING), full_torus(RING));
    let decls = wall_declarations(&a, &b, TUBE, ContactClass::Rest);
    let err = topo::union_with(&a, &b, &decls, Tol::witness())
        .expect_err("the circle-versus-torus clearance has no enclosure yet");
    assert!(
        matches!(err, BooleanError::CurvedPierceUnsupported { .. }),
        "the lane's stopping point is the curved-pierce frontier: {err:?}"
    );
}

// -------------------------------------------------------------------
// 3. The ratified routing, both arms.
// -------------------------------------------------------------------

/// **The π arm, on a real rim.** Two same-radius tube segments meeting
/// G1 along their shared terminal meridian circle: the outward normals
/// agree across the rim, so the material runs smoothly through it and
/// the routing classifies the seam.
///
/// The declaration is `Tangent` because that is the only class an
/// author has to state a rim contact with today — and the routing's
/// answer is precisely that this rim does not want one: a wedge-π rim
/// is structural. The refusal is its own variant so it can say that
/// without also claiming the π arm is unbuilt, which it is not.
#[test]
fn the_g1_tube_chain_rim_routes_to_the_smooth_seam() {
    let (a, b) = (segment_a(), segment_b());
    let decls = wall_declarations(&a, &b, TUBE, ContactClass::Tangent);
    let err = topo::union_with(&a, &b, &decls, Tol::witness())
        .expect_err("a seam takes no declaration, and the join wiring is not built");
    assert!(
        matches!(err, BooleanError::RimSeamNotDeclarable { .. }),
        "a G1 chain's rim is the wedge-π seam: {err:?}"
    );
    let text = format!("{err}");
    assert!(
        text.contains("MATE-7-TANGENCY-DESIGN"),
        "the refusal must cite the ruling of record: {text}"
    );
    assert!(
        !text.contains("UNBUILT"),
        "the π arm IS built — the seam refusal must not claim otherwise: {text}"
    );
}

/// **The 0/2π arm.** Two tori touching externally along one circle:
/// the outward normals oppose across it, the void between them is the
/// vanishing crescent, and the wedge is 2π — the knife slit. That is
/// the declared-cusp family, whose certified rim witness is DEFINED
/// BUT UNBUILT, and the refusal cites the ruling that defines it
/// rather than reporting the class as merely unsupported.
#[test]
fn a_kissing_torus_rim_routes_to_the_unbuilt_cusp_family() {
    let (a, b) = kissing_pair();
    let decls = wall_declarations(&a, &b, TUBE, ContactClass::Tangent);
    let err = topo::union_with(&a, &b, &decls, Tol::witness())
        .expect_err("the cusp family is defined and unbuilt");
    match err {
        BooleanError::RimCuspArmUnbuilt { wedge, .. } => {
            assert_eq!(
                wedge,
                geom_brep::MaterialWedge::Slit,
                "two solids kissing along a circle leave the crescent VOID: wedge 2π"
            );
        }
        other => panic!("expected the routing's verdict, got {other:?}"),
    }
    let text = format!("{err}");
    assert!(
        text.contains("MATE-7-TANGENCY-DESIGN") && text.contains("UNBUILT"),
        "the refusal must cite the ruling and name the arm as unbuilt: {text}"
    );
}

/// **The routing is taken on the geometry, not on the kinds.** The two
/// arms above differ only in where the second body sits; nothing about
/// "torus" chooses between them, and swapping the fixtures swaps the
/// verdicts. Asserted as one row so a routing that had quietly become
/// a constant would fail here even if both rows above still passed
/// individually.
#[test]
fn the_two_arms_are_decided_by_the_rim_and_not_by_the_kind() {
    let chain = {
        let (a, b) = (segment_a(), segment_b());
        let d = wall_declarations(&a, &b, TUBE, ContactClass::Tangent);
        topo::union_with(&a, &b, &d, Tol::witness()).expect_err("chain refuses")
    };
    let kiss = {
        let (a, b) = kissing_pair();
        let d = wall_declarations(&a, &b, TUBE, ContactClass::Tangent);
        topo::union_with(&a, &b, &d, Tol::witness()).expect_err("kiss refuses")
    };
    // The two arms are DIFFERENT VARIANTS, which is the sharper form
    // of the same claim: the routing did not merely fill one payload
    // two ways, it sent the two geometries down two paths.
    assert!(
        matches!(chain, BooleanError::RimSeamNotDeclarable { .. }),
        "the G1 chain routes to the seam: {chain:?}"
    );
    assert!(
        matches!(
            kiss,
            BooleanError::RimCuspArmUnbuilt {
                wedge: geom_brep::MaterialWedge::Slit,
                ..
            }
        ),
        "the kissing pair routes to the cusp family: {kiss:?}"
    );
}

/// **A pair with no shared rim keeps the bare class refusal.** The
/// routing speaks only where there is a rim to speak about; a torus
/// pair whose faces share no circle is still simply outside the
/// witness lane, and saying otherwise would be inventing a rim.
#[test]
fn a_torus_pair_with_no_shared_rim_keeps_the_class_refusal() {
    let a = segment_a();
    // The same elbow displaced far along `y`, so nothing meets and no
    // boundary circle is shared.
    let b = tube_along_arc(
        Point3::new(-RING, 40.0, 0.0),
        axis(),
        Vec3::new(1.0, 0.0, 0.0),
        RING,
        window(TURN),
        TUBE,
        Tol::witness(),
    )
    .expect("the distant elbow builds")
    .body;
    let decls = wall_declarations(&a, &b, TUBE, ContactClass::Tangent);
    let err = topo::union_with(&a, &b, &decls, Tol::witness())
        .expect_err("a Tangent declaration outside the witness lane is refused");
    assert!(
        matches!(
            err,
            BooleanError::UnsupportedDeclarationClass {
                class: ContactClass::Tangent
            }
        ),
        "no rim, no routing — the class refusal stands verbatim: {err:?}"
    );
}

// -------------------------------------------------------------------
// 4. What the widening did NOT do.
// -------------------------------------------------------------------

/// **∖ and ∩ keep their roster verbatim.** The operand gate admits a
/// declared pair because a declaration supplies the VERDICT a germ arm
/// would have; the revert roster is a different claim — which kinds
/// have a seam lane to revert through — and no declaration supplies
/// one. So a declared torus pair under a subtract is exactly as
/// refused as an undeclared one.
#[test]
fn a_declared_torus_pair_under_subtract_is_still_refused_by_the_revert_roster() {
    let (s, p) = (socket(), segment_a());
    let decls = wall_declarations(&s, &p, TUBE, ContactClass::Rest);
    let err = topo::subtract_with(&s, &p, &decls, Tol::witness())
        .expect_err("no declaration supplies a revert seam lane");
    assert!(
        matches!(
            err,
            BooleanError::CurvedPairUnsupported {
                op: Some(topo::BooleanOp::Subtract),
                kind: geom_brep::SurfaceKind::Torus,
                ..
            }
        ),
        "the revert roster refuses the declared pair unchanged: {err:?}"
    );
}

/// **A cone declaration is still refused at the inventory**, which is
/// what keeps the gate's covered-pair rung kind-generic without being
/// kind-blind: the carrier ladder has no cone rung, so no cone pair
/// can survive into a declaration, so none can be covered at the gate
/// either. One list decides both.
#[test]
fn a_cone_face_still_cannot_be_declared_at_all() {
    let cone = {
        let vp = validated(vec![ProfileLoop::polygon([
            p2(0.0, 0.0),
            p2(1.0, 0.0),
            p2(0.0, 1.0),
        ])]);
        revolve(&vp, axis_y(), Revolution::Full, Tol::witness())
            .expect("the cone body builds")
            .body
    };
    let cone_face = cone
        .faces()
        .find(|(_, f)| {
            matches!(
                cone.get_surface(f.surface),
                Some(geom::Surface::Cone { .. })
            )
        })
        .map(|(k, _)| k)
        .expect("the revolve mints a cone face");
    let other = segment_a();
    let mut decls = BooleanDeclarations::none();
    decls.coincident_faces.push(FacePairDeclaration::new(
        cone_face,
        torus_faces(&other, TUBE)[0],
        ContactClass::Rest,
    ));
    let err = topo::union_with(&cone, &other, &decls, Tol::witness())
        .expect_err("a cone face cannot be declared");
    match err {
        BooleanError::InvalidDeclaration { what, .. } => {
            assert!(
                what.contains("certified inventory"),
                "the refusal must name the inventory: {what}"
            );
        }
        other => panic!("expected InvalidDeclaration, got {other:?}"),
    }
}

/// The chain's two segments really are the tube chain the π row claims
/// — same tube radius, and their shared terminal circle is one circle,
/// not two near ones. Without this the π row could be passing on a
/// fixture that is not the geometry it names.
#[test]
fn the_chain_fixture_is_g1_with_one_shared_rim() {
    let (a, b) = (segment_a(), segment_b());
    let radii = |body: &Body<f64>| {
        body.faces()
            .filter_map(|(_, f)| match body.get_surface(f.surface) {
                Some(&geom::Surface::Torus { minor_radius, .. }) => Some(minor_radius),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    for r in radii(&a).into_iter().chain(radii(&b)) {
        assert_eq!(r, TUBE, "both segments carry the same tube radius");
    }
    let plane_origins = |body: &Body<f64>| {
        body.faces()
            .filter_map(|(_, f)| match body.get_surface(f.surface) {
                Some(&geom::Surface::Plane { origin, .. }) => Some(origin),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    let shared = plane_origins(&a)
        .into_iter()
        .flat_map(|oa| {
            plane_origins(&b)
                .into_iter()
                .map(move |ob| (oa - ob).norm())
        })
        .filter(|d| *d < 1e-12)
        .count();
    assert_eq!(
        shared, 1,
        "exactly one cap plane is shared between the segments"
    );
    match topo::union_with(&a, &b, &BooleanDeclarations::none(), Tol::witness()) {
        Ok(BooleanResult::Body(_) | BooleanResult::Empty) => {
            panic!("an undeclared torus chain must not silently succeed")
        }
        Err(_) => {}
    }
}
