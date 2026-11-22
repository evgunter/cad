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
    Joined { ev, built, union }
}

/// The eight elbows that build: either cap, the axis on either side,
/// either sign of the turn. (The other eight spellings put the profile
/// across the axis, which the revolve refuses on its own.) Off the End
/// cap (z = 2) a turn of +π/2 bends back into the pipe and −π/2 away
/// from it; off the Start cap (z = 0) the reverse.
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

#[test]
fn a_pipe_and_an_elbow_off_either_cap_pass_the_glue_door_in_every_pose() {
    for (end, off) in the_eight_poses() {
        stops_at_the_crossing_layer(
            &join(true, end, off),
            &format!("pipe, {end:?} cap, {off:?}"),
        );
    }
}

#[test]
fn a_rod_and_its_elbow_pass_the_glue_door() {
    let (end, off) = the_eight_poses()[0];
    stops_at_the_crossing_layer(&join(false, end, off), "rod");
}

/// The neutral points moved off the routing's stations: the axis turned
/// a few ε about the cap's normal, and far.
#[test]
fn an_elbow_whose_neutral_points_sit_between_the_stations_passes_the_glue_door() {
    for k in [2.0, 1e3] {
        let off = OffCap::Elbow {
            axis_x: BEND,
            heading: -FRAC_PI_2 + k * Tol::witness().eps(),
            angle: -FRAC_PI_2,
        };
        stops_at_the_crossing_layer(&join(true, CapEnd::End, off), &format!("turned {k}ε"));
    }
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

/// **R1 probe (review of PR 7432)**: the elbow's axis turned about the
/// cap's normal by `k·ε` over a ladder of `k`, which slides the two
/// neutral points (κ_rel = 0) past the routing's stations. Every pose
/// must stop at the crossing layer: a refusal at the glue door's
/// routing would be the bend refusing a π seam (`work/select/a-seam-
/// decided-pi-refuses-where-only-its-bend-is-undecided.md`).
#[test]
fn r1_an_elbow_turned_by_any_fraction_of_eps_passes_the_glue_door() {
    let mut off_door = Vec::new();
    for k in [0.05, 0.1, 0.2, 0.3, 0.5, 0.7, 1.0, 1.5, 3.0, 5.0, 10.0, 30.0, 100.0] {
        let off = OffCap::Elbow {
            axis_x: BEND,
            heading: -FRAC_PI_2 + k * Tol::witness().eps(),
            angle: -FRAC_PI_2,
        };
        let j = join(true, CapEnd::End, off);
        let kind = j.ev.node_error(j.union).map(|e| e.kind.to_string());
        println!("R1PROBE k={k}: {kind:?}");
        let at_crossing = matches!(
            j.ev.node_error(j.union).map(|e| &e.kind),
            Some(editor_core::NodeErrorKind::Boolean(
                topo::BooleanError::CurvedPierceUnsupported { .. }
            ))
        );
        if !at_crossing {
            off_door.push((k, kind));
        }
    }
    assert!(off_door.is_empty(), "poses that did not reach the crossing layer: {off_door:?}");
}
