//! **A pipe unions with what is built off its own end cap.** The pipe
//! is the ring `circle(1)`, `circle(0.6)` extruded 2; the same loops
//! drawn on one of its caps (`draw_on_face`, `OnFace::nearest`) are
//! revolved a quarter turn about a line in the cap's plane three units
//! off the pipe's axis, and the union of the pipe and that elbow is
//! measured against its closed form (Pappus: the section's area times
//! the path its centroid runs). The neighbours: the same with a solid
//! rod, and an extrude off the same cap.
//!
//! The pipe's walls and the elbow's both end on the cap's two rims: a
//! junction, which the glue door decides on each rim's identity and its
//! π wedge, so the straight wall and the bend, whose relative curvature
//! passes through zero at two points of each rim, are no second-order
//! question there. An edge of either body that ends on a rim, tangent to
//! the other's wall there, has that end's root factored by the crossing
//! layer (`geom_brep::curve_face::end_root`), so the four poses that bend
//! away from the pipe pass it and stop at the finish, where the seams'
//! edges still carry a construction's stand-in description. The four that
//! bend back into the pipe are no junction (their walls leave the rim on
//! one side) and refuse their rims at the glue door
//! (`work/tang/an-elbow-bent-back-into-its-pipe-meets-it-along-a-rim-and-a-crossing.md`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::{FRAC_PI_2, PI};

use editor_core::test_support::{ang, len};
use editor_core::{
    BodyRead, BooleanValue, CancelToken, CapEnd, DocEdit, EntityKind, EvalOptions, Evaluation,
    Formula, LoopProgram, Node, OnFace, Operand, ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg,
    StableName, ValuePayload, evaluate,
};
use geom_core::Tol;
use topo::ContactRecords;

const OUTER: f64 = 1.0;
const INNER: f64 = 0.6;
const LENGTH: f64 = 2.0;
/// The revolve axis's distance from the pipe's axis.
const BEND: f64 = 3.0;

/// What is built off the cap.
#[derive(Clone, Copy, Debug)]
enum OffCap {
    /// A quarter turn about the line through `(axis_x, axis_y)` heading
    /// `heading`, turned through `angle`.
    Elbow {
        axis_x: f64,
        axis_y: f64,
        heading: f64,
        angle: f64,
    },
    /// An extrude of the cap's loops, this far.
    Extrude(f64),
}

/// What the union of the pipe (or rod) and what is built off its cap
/// evaluated to.
struct Joined {
    ev: Evaluation<f64>,
    /// The pipe (or rod).
    base: RecipeNodeId,
    built: RecipeNodeId,
    union: RecipeNodeId,
}

fn insert(
    doc: ProfileDoc,
    node: Node<ProfileProgram<Formula>, Formula>,
) -> (ProfileDoc, RecipeNodeId) {
    let applied = doc
        .apply(
            &DocEdit::InsertNode {
                node: Box::new(node),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .unwrap();
    (applied.doc, applied.record.minted.unwrap())
}

fn circle(r: f64) -> LoopProgram<Formula> {
    LoopProgram::Circle {
        centre: [len(0.0), len(0.0)],
        radius: len(r),
    }
}

fn loops(hollow: bool) -> Vec<LoopProgram<Formula>> {
    if hollow {
        vec![circle(OUTER), circle(INNER)]
    } else {
        vec![circle(OUTER)]
    }
}

/// The section's area: the ring's, or the rod's disc.
fn section(hollow: bool) -> f64 {
    PI * (OUTER.powi(2) - if hollow { INNER.powi(2) } else { 0.0 })
}

/// The union's closed-form volume: the straight part plus what is
/// built off the cap, which meet only on the cap.
fn closed_form(hollow: bool, off: OffCap) -> f64 {
    let a = section(hollow);
    a * LENGTH
        + match off {
            OffCap::Elbow { angle, .. } => a * BEND * angle.abs(),
            OffCap::Extrude(d) => a * d,
        }
}

fn join(hollow: bool, end: CapEnd, off: OffCap) -> Joined {
    join_at(hollow, LENGTH, end, off)
}

/// [`join`], the pipe `length` long.
fn join_at(hollow: bool, length: f64, end: CapEnd, off: OffCap) -> Joined {
    let doc = ProfileDoc::empty_derived("a_pipe_and_its_elbow", Tol::witness());
    let (doc, section) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: loops(hollow),
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, pipe) = insert(
        doc,
        Node::Extrude {
            profile: section.into(),
            distance: len(length),
        },
    );
    let cap = StableName {
        kind: EntityKind::Face,
        node: pipe,
        path: vec![RoleSeg::Cap(end)],
    };
    let drawn = editor_core::draw_on_face(
        &doc,
        BodyRead::plain(Operand::output(pipe, 0)),
        cap,
        loops(hollow),
        OnFace::nearest(),
        &EvalOptions::default(),
        &CancelToken::new(),
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("the loops draw on the {end:?} cap: {e}"));
    let built = match off {
        OffCap::Elbow {
            axis_x,
            axis_y,
            heading,
            angle,
        } => Node::Revolve {
            profile: drawn.profile.into(),
            axis_through: [len(axis_x), len(axis_y)],
            axis_heading: ang(heading),
            angle: ang(angle),
        },
        OffCap::Extrude(d) => Node::Extrude {
            profile: drawn.profile.into(),
            distance: len(d),
        },
    };
    let (doc, built) = insert(drawn.doc, built);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                pipe.into(),
                BodyRead::plain(Operand::output(built, 0)),
            ]),
        },
    );
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    assert!(
        ev.node_error(built).is_none(),
        "what is built off the {end:?} cap builds ({off:?}): {:?}",
        ev.node_error(built).map(|e| e.kind.to_string())
    );
    Joined {
        ev,
        base: pipe,
        built,
        union,
    }
}

/// The eight elbows that build: either cap, the axis on either side,
/// either sign of the turn. (The other eight spellings put the profile
/// across the axis, which the revolve refuses on its own.) A turn of
/// +π/2 bends back into the pipe and −π/2 away from it, off either cap:
/// the Start cap's frame faces the other way, so off the End cap (z = 2)
/// the inward elbow spans z ∈ [−2, 2] and off the Start cap (z = 0)
/// z ∈ [0, 4].
fn the_eight_poses() -> [(CapEnd, OffCap); 8] {
    let q = FRAC_PI_2;
    let elbow = |axis_x, heading, angle| OffCap::Elbow {
        axis_x,
        axis_y: 0.0,
        heading,
        angle,
    };
    [
        (CapEnd::End, elbow(BEND, -q, q)),
        (CapEnd::End, elbow(BEND, -q, -q)),
        (CapEnd::End, elbow(-BEND, q, q)),
        (CapEnd::End, elbow(-BEND, q, -q)),
        (CapEnd::Start, elbow(BEND, -q, q)),
        (CapEnd::Start, elbow(BEND, -q, -q)),
        (CapEnd::Start, elbow(-BEND, q, q)),
        (CapEnd::Start, elbow(-BEND, q, -q)),
    ]
}

/// Whether a pose bends away from the pipe: a turn of −π/2.
fn bends_away(off: OffCap) -> bool {
    matches!(off, OffCap::Elbow { angle, .. } if angle < 0.0)
}

/// The union's refusal.
#[track_caller]
fn refusal<'j>(j: &'j Joined, what: &str) -> &'j topo::BooleanError {
    let refusal =
        j.ev.node_error(j.union)
            .unwrap_or_else(|| panic!("{what}: the union refuses"));
    match &refusal.kind {
        editor_core::NodeErrorKind::Boolean(e) => e,
        other => panic!("{what}: a Boolean refusal: {other:?}"),
    }
}

/// A pose bent away passes the crossing layer and stops at the finish,
/// on the seams' descriptions: a construction's stand-in left on them, or
/// (off the neutral alignment) one the tangency lane does not certify.
#[track_caller]
fn stops_at_the_finish(j: &Joined, what: &str) {
    let e = refusal(j, what);
    let said = format!("{e:?}");
    assert!(
        matches!(e, topo::BooleanError::ResultInvalid { .. }) && said.contains("ScaffoldAtRest")
            || matches!(e, topo::BooleanError::EdgeUndescribed { .. }),
        "{what}: past the crossing layer, at the finish's seam descriptions: {said}"
    );
}

/// **The four elbows bent away from the pipe glue a junction on both
/// rims**: every pair of walls ending on a rim, the pipe's cylinder and
/// the elbow's torus, is the routing's π seam with the faces leaving on
/// opposite sides and each body's own rim edge convex, and no other pair
/// glues. The union then passes the crossing layer, where an edge of one
/// body ending on the rim tangent to the other's wall has that end's root
/// factored, and stops at the finish.
#[test]
fn an_elbow_bent_away_from_its_pipe_glues_a_junction_on_both_rims() {
    for (end, off) in the_eight_poses()
        .into_iter()
        .filter(|&(_, off)| bends_away(off))
    {
        let what = format!("pipe, {end:?} cap, {off:?}");
        let j = join(true, end, off);
        glues_its_walls_a_junction(&j, 8, &what);
        stops_at_the_finish(&j, &what);
    }
}

/// **The four elbows bent back into the pipe refuse at the glue door,
/// as before the junction**: the two walls leave each rim on one side, so
/// no junction glues, and the door reads the rim as it always has, the
/// tangency table along its turn, where the bend's relative curvature
/// passes through zero and lands in the band. What these poses owe is
/// TANG's (`work/tang/an-elbow-bent-back-into-its-pipe-meets-it-along-a-rim-and-a-crossing.md`).
#[test]
fn an_elbow_bent_back_into_its_pipe_refuses_its_rims_at_the_glue_door() {
    for (end, off) in the_eight_poses()
        .into_iter()
        .filter(|&(_, off)| !bends_away(off))
    {
        let what = format!("pipe, {end:?} cap, {off:?}");
        let j = join(true, end, off);
        let (walls, _) = door(&j);
        assert!(
            walls.iter().all(|d| matches!(
                d,
                topo::boolean::RimDoor::Refused(topo::BooleanErrorKind::Escalated)
            )),
            "{what}: each wall pair refuses in band at the table: {walls:?}"
        );
        let refusal =
            j.ev.node_error(j.union)
                .unwrap_or_else(|| panic!("{what}: the union refuses"));
        assert!(
            matches!(
                &refusal.kind,
                editor_core::NodeErrorKind::Boolean(topo::BooleanError::Escalated { .. })
            ),
            "{what}: the door's in-band refusal: {:?}",
            refusal.kind
        );
    }
}

#[test]
fn a_rod_and_its_elbow_glue_one_junction() {
    let (end, off) = the_eight_poses()[1];
    assert!(bends_away(off));
    let j = join(false, end, off);
    glues_its_walls_a_junction(&j, 4, "rod");
    stops_at_the_finish(&j, "rod");
}

/// The neutral points moved off the routing's stations: the axis turned
/// two ε about the cap's normal, an outward pose.
#[test]
fn an_elbow_whose_neutral_points_sit_between_the_stations_glues_a_junction() {
    for k in [2.0, 1e3] {
        let off = OffCap::Elbow {
            axis_x: BEND,
            axis_y: 0.0,
            heading: -FRAC_PI_2 + k * Tol::witness().eps(),
            angle: -FRAC_PI_2,
        };
        let what = format!("turned {k}ε");
        let j = join(true, CapEnd::End, off);
        glues_its_walls_a_junction(&j, 8, &what);
        stops_at_the_finish(&j, &what);
    }
}

/// **A hooked elbow that comes back into its pipe is never a wrong body**
/// (R1's guard on PR 7432's second review): the axis 1.5 off the pipe's,
/// turned −3π/2 and −5π/4 off the End cap, the rod and the pipe. The
/// torus wall leaves the rim G1, so every wall pair on the rim glues a
/// junction, then comes back round into the pipe's wall away from the
/// rim. The arcs ending on the rim turn past a quarter turn, so their
/// windows are not the whole span and the crossing is the root doors'.
/// The union passes the crossing layer and stops at the finish (the
/// seams' descriptions); once the finish builds, the row owes a typed
/// refusal or the hand-checked body, never a wrong one.
#[test]
fn a_hooked_elbow_that_comes_back_into_its_pipe_is_never_a_wrong_body() {
    use topo::boolean::RimDoor;
    for hollow in [false, true] {
        for angle in [-1.5 * PI, -1.25 * PI] {
            let what = format!("hollow {hollow}, turned {angle}");
            let off = OffCap::Elbow {
                axis_x: 1.5,
                axis_y: 0.0,
                heading: -FRAC_PI_2,
                angle,
            };
            let j = join(hollow, CapEnd::End, off);
            let (walls, rest) = door(&j);
            assert!(
                !walls.is_empty() && walls.iter().all(|d| *d == RimDoor::Junction),
                "{what}: every wall pair on the rim a junction: {walls:?}"
            );
            assert!(
                rest.iter().all(|d| *d == RimDoor::NotTangent),
                "{what}: every other pair meets at a corner: {rest:?}"
            );
            stops_at_the_finish(&j, &what);
        }
    }
}

/// The door's row for a pose bent away: `walls` pairs of a cylinder and a
/// torus sharing a rim, each glued a junction, and no other pair glued.
#[track_caller]
fn glues_its_walls_a_junction(j: &Joined, walls: usize, what: &str) {
    use topo::boolean::RimDoor;
    let (w, rest) = door(j);
    assert_eq!(w.len(), walls, "{what}: the wall pairs on the rims");
    assert!(
        w.iter().all(|d| *d == RimDoor::Junction),
        "{what}: every wall pair a junction: {w:?}"
    );
    assert!(
        rest.iter().all(|d| *d == RimDoor::NotTangent),
        "{what}: every other pair meets at a corner: {rest:?}"
    );
}

/// The extrude's walls continue the pipe's on one carrier, and each wall
/// meets the other body's coplanar cap at a corner all round its rim:
/// no junction is glued there, and the union is the one straight pipe.
#[test]
fn a_pipe_unions_with_an_extrude_off_its_own_cap() {
    let off = OffCap::Extrude(1.0);
    let j = join(true, CapEnd::End, off);
    let built = built(&j);
    assert_eq!(built.solids, 1, "one solid");
    assert_eq!(built.records, 0, "the continuation records no contact");
    assert!(built.tier3.is_ok(), "tier 3: {:?}", built.tier3);
    assert!(
        built.tier3_prime.is_ok(),
        "tier 3′: {:?}",
        built.tier3_prime
    );
    let want = closed_form(true, off);
    assert!(
        (built.volume - want).abs() <= 1e-12 * want,
        "volume {} against the closed form {want}",
        built.volume
    );
}

/// **What the glue door returns for each pair of the two bodies' faces
/// that share a rim** (`topo::boolean::rim_door`), the walls' pairs
/// apart from the rest: the pairs whose kinds are a cylinder and a
/// torus, and every other.
fn door(j: &Joined) -> (Vec<topo::boolean::RimDoor>, Vec<topo::boolean::RimDoor>) {
    let at_rest = |node| {
        topo::test_support::finished(
            "an operand",
            crate::corpus::body_of(&j.ev, node).clone(),
            Tol::witness(),
        )
    };
    let walls = |(ka, kb): (geom::SurfaceKind, geom::SurfaceKind)| {
        matches!(
            (ka, kb),
            (geom::SurfaceKind::Cylinder, geom::SurfaceKind::Torus)
        )
    };
    let rows = topo::boolean::rim_door(&at_rest(j.base), &at_rest(j.built), Tol::witness());
    let (w, rest): (Vec<_>, Vec<_>) = rows.into_iter().partition(|(kinds, _)| walls(*kinds));
    (
        w.into_iter().map(|(_, d)| d).collect(),
        rest.into_iter().map(|(_, d)| d).collect(),
    )
}

/// What a union that builds came to.
struct Built {
    solids: usize,
    records: usize,
    tier3: Result<(), Vec<topo::ValidationError>>,
    tier3_prime: Result<(), Vec<topo::ValidationError>>,
    volume: f64,
}

#[track_caller]
fn built(j: &Joined) -> Built {
    if let Some(e) = j.ev.node_error(j.union) {
        panic!("the union builds: {}", e.kind);
    }
    let body = crate::corpus::body_of(&j.ev, j.union);
    let none = ContactRecords::default();
    let value = j.ev.value(j.union).expect("a value");
    let contacts = match &value.payload {
        ValuePayload::Boolean(BooleanValue::Body { .. }) => &*value.carry.contacts,
        _ => &none,
    };
    let finished = topo::test_support::finished("the union", body.clone(), Tol::witness());
    Built {
        solids: body.solids().count(),
        records: contacts.len(),
        tier3: topo::validate_geometric(&body, Tol::witness()),
        tier3_prime: topo::validate_pseudomanifold(&body, contacts, Tol::witness()),
        volume: topo::test_support::pinned_volume("the union", &finished, Tol::witness()).volume,
    }
}

/// The elbow bent away about the line through `3·(−sin δ, cos δ)`
/// heading `δ`: the bend's neutral point on each rim (where its relative
/// curvature across the rim passes through zero) sits `δ` round the rim
/// from the pipe's seam ruling and the rim's vertex, where edges end.
fn neutral_off_the_seam(delta: f64) -> OffCap {
    OffCap::Elbow {
        axis_x: -BEND * delta.sin(),
        axis_y: BEND * delta.cos(),
        heading: delta,
        angle: -FRAC_PI_2,
    }
}

/// **An edge ending at or near a neutral point stops where its distance
/// from it says** (the end-root door, `geom_brep::curve_face::end_root`,
/// through the union):
///
/// - on it, the end's root is of order four, read past, and the union
///   stops at the finish;
/// - `ε` off it, the quadratic term reads in band: the end's order
///   escalates (`JunctionEndOrder`);
/// - `√ε/3` off it (`1e-5` at `ε = 1e-9`), the quadratic term decides but
///   no window holds its spread: the end is unread (`JunctionEndUnread`,
///   `Window`), the belt
///   `work/heldrecords/the-junction-end-window-refuses-in-a-belt-around-a-neutral-point.md`
///   files.
#[test]
fn an_edge_ending_near_a_neutral_point_stops_where_its_distance_says() {
    let on = join(true, CapEnd::End, neutral_off_the_seam(0.0));
    stops_at_the_finish(&on, "on the neutral point");
    let eps = Tol::witness().eps();
    let near = join(true, CapEnd::End, neutral_off_the_seam(eps));
    let e = refusal(&near, "ε off it");
    assert!(
        matches!(
            e,
            topo::BooleanError::CurvedPierceEscalated {
                decision: topo::BooleanDecision::JunctionEndOrder,
                ..
            }
        ),
        "ε off it, the order in band: {e:?}"
    );
    let belt = join(true, CapEnd::End, neutral_off_the_seam(eps.sqrt() / 3.0));
    let e = refusal(&belt, "√ε/3 off it");
    assert!(
        matches!(
            e,
            topo::BooleanError::JunctionEndUnread {
                gap: topo::JunctionEndGap::Window,
                ..
            }
        ),
        "√ε/3 off it, no window: {e:?}"
    );
}

/// **A root of the rest of the span is found, not covered.** A pipe 4
/// long: its seam ruling `x = 1` meets the bend's carrier again at
/// `z = 4 − √12`, inside its span and off the bend's face. The window past
/// the rim end stops short of it, the root doors place it off the face,
/// and the union passes the crossing layer.
#[test]
fn a_long_pipe_whose_ruling_meets_the_bends_carrier_again_passes_the_crossing_layer() {
    let (end, off) = the_eight_poses()[1];
    assert!(bends_away(off));
    stops_at_the_finish(&join_at(true, 4.0, end, off), "a pipe 4 long");
}
