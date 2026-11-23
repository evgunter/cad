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
//! question there. Every elbow pose, the four that bend back into the
//! pipe among them, then stops at the crossing layer: an edge of one
//! body ends on a rim tangent to the other's wall, which that layer
//! cannot yet settle (`work/heldrecords/a-pipe-does-not-union-with-an-elbow-revolved-off-its-own-cap.md`).
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
    /// A quarter turn about the line through `(axis_x, 0)` heading
    /// `heading`, turned through `angle`.
    Elbow {
        axis_x: f64,
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
            distance: len(LENGTH),
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
            heading,
            angle,
        } => Node::Revolve {
            profile: drawn.profile.into(),
            axis_through: [len(axis_x), len(0.0)],
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
            members: editor_core::Bodies::spelled(vec![
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
/// −π/2 bends away from the pipe off either cap, and +π/2 back into it
/// (measured: the glue door's rows below).
fn the_eight_poses() -> [(CapEnd, OffCap); 8] {
    let q = FRAC_PI_2;
    let elbow = |axis_x, heading, angle| OffCap::Elbow {
        axis_x,
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

/// The refusal the union reached, which must be the crossing layer's
/// for an edge ending on a rim: past the glue door, which no longer
/// reads the rim's second order.
#[track_caller]
fn stops_at_the_crossing_layer(j: &Joined, what: &str) {
    let refusal =
        j.ev.node_error(j.union)
            .unwrap_or_else(|| panic!("{what}: the union refuses"));
    assert!(
        matches!(
            &refusal.kind,
            editor_core::NodeErrorKind::Boolean(topo::BooleanError::CurvedPierceUnsupported { .. })
        ),
        "{what}: the crossing layer's refusal of an edge ending on the rim: {:?}",
        refusal.kind
    );
}

/// **The four elbows bent away from the pipe glue a junction on both
/// rims**: every pair of walls ending on a rim, the pipe's cylinder and
/// the elbow's torus, is the routing's π seam with the faces leaving on
/// opposite sides and each body's own rim edge convex, and no other pair
/// glues. The union then stops at the crossing layer: an edge of one body
/// ends on the rim tangent to the other's wall
/// (`work/heldrecords/a-pipe-does-not-union-with-an-elbow-revolved-off-its-own-cap.md`).
#[test]
fn an_elbow_bent_away_from_its_pipe_glues_a_junction_on_both_rims() {
    for (end, off) in the_eight_poses()
        .into_iter()
        .filter(|&(_, off)| bends_away(off))
    {
        let what = format!("pipe, {end:?} cap, {off:?}");
        let j = join(true, end, off);
        glues_its_walls_a_junction(&j, 8, &what);
        stops_at_the_crossing_layer(&j, &what);
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
    stops_at_the_crossing_layer(&j, "rod");
}

/// The neutral points moved off the routing's stations: the axis turned
/// a few ε about the cap's normal, and far.
#[test]
fn an_elbow_whose_neutral_points_sit_between_the_stations_glues_a_junction() {
    for k in [2.0, 1e3] {
        let off = OffCap::Elbow {
            axis_x: BEND,
            heading: -FRAC_PI_2 + k * Tol::witness().eps(),
            angle: -FRAC_PI_2,
        };
        let what = format!("turned {k}ε");
        let j = join(true, CapEnd::End, off);
        glues_its_walls_a_junction(&j, 8, &what);
        stops_at_the_crossing_layer(&j, &what);
    }
}

/// **A hooked elbow that comes back into its pipe is never a wrong body**
/// (R1's guard on PR 7432's second review): the axis 1.5 off the pipe's,
/// turned −3π/2 and −5π/4 off the End cap, the rod and the pipe. The
/// torus wall leaves the rim G1, so every wall pair on the rim glues a
/// junction, then comes back round into the pipe's wall away from the
/// rim. That crossing is the crossing layer's, which refuses it typed
/// here; once the crossing layer reads it (PR 7471's top of the stack)
/// the row owes a typed refusal or the hand-checked body, never a wrong
/// one.
#[test]
fn a_hooked_elbow_that_comes_back_into_its_pipe_is_never_a_wrong_body() {
    use topo::boolean::RimDoor;
    for hollow in [false, true] {
        for angle in [-1.5 * PI, -1.25 * PI] {
            let what = format!("hollow {hollow}, turned {angle}");
            let off = OffCap::Elbow {
                axis_x: 1.5,
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
            stops_at_the_crossing_layer(&j, &what);
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

/// What the union came to: its refusal, or its volume against the
/// closed form and tier 3 and 3′ on the result.
fn outcome(j: &Joined, expect: f64) -> String {
    if let Some(e) = j.ev.node_error(j.union) {
        return format!("REFUSES {}", e.kind);
    }
    let body = crate::corpus::body_of(&j.ev, j.union);
    let none = ContactRecords::default();
    let value = j.ev.value(j.union).expect("a value");
    let contacts = match &value.payload {
        ValuePayload::Boolean(BooleanValue::Body { .. }) => &*value.carry.contacts,
        _ => &none,
    };
    let t3 = topo::validate_geometric(&body, Tol::witness()).map_err(|e| format!("{e:?}"));
    let t3p = topo::validate_pseudomanifold(&body, contacts, Tol::witness())
        .map_err(|e| format!("{e:?}"));
    let vol = match &t3 {
        Ok(()) => {
            let f = topo::test_support::finished("the union", body.clone(), Tol::witness());
            format!(
                "{:.12}",
                topo::test_support::pinned_volume("the union", &f, Tol::witness()).volume
            )
        }
        Err(_) => "-".into(),
    };
    format!(
        "BUILDS solids {} volume {vol} expect {expect:.12} records {} t3 {t3:?} t3' {t3p:?}",
        body.solids().count(),
        contacts.len()
    )
}

/// **A probe, asserting nothing**: what every pose, the rod's, the
/// turned axes' and the extrude's unions come to, printed. Run with
/// `cargo nextest run -p editor-core --test all --run-ignored only zz_probe_outcomes --no-capture`
/// (`probes/heldrecords-elbow/` for the instrumented runs).
#[test]
#[ignore = "a probe: prints, asserts nothing"]
fn zz_probe_outcomes() {
    for (end, off) in the_eight_poses() {
        let j = join(true, end, off);
        let pipe_b = crate::corpus::body_of(&j.ev, j.built);
        let zs: Vec<f64> = pipe_b.points().map(|(_, p)| p.to_array()[2]).collect();
        println!(
            "PROBE elbow z range {:?}",
            (
                zs.iter().cloned().fold(f64::MAX, f64::min),
                zs.iter().cloned().fold(f64::MIN, f64::max)
            )
        );
        println!(
            "PROBE pipe {end:?} {off:?}: {}",
            outcome(&j, closed_form(true, off))
        );
    }
    for (end, off) in the_eight_poses() {
        println!(
            "PROBE rod {end:?} {off:?}: {}",
            outcome(&join(false, end, off), closed_form(false, off))
        );
    }
    for k in [0.5, 1.0, 2.0, 5.0, 20.0, 1e3, 1e6] {
        let d = k * Tol::witness().eps();
        let off = OffCap::Elbow {
            axis_x: BEND,
            heading: -FRAC_PI_2 + d,
            angle: -FRAC_PI_2,
        };
        println!(
            "PROBE near-neutral k={k}: {}",
            outcome(&join(true, CapEnd::End, off), closed_form(true, off))
        );
    }
    let off = OffCap::Extrude(1.0);
    println!(
        "PROBE extrude: {}",
        outcome(&join(true, CapEnd::End, off), closed_form(true, off))
    );
}
