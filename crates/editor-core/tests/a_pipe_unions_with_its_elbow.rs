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
//! layer (`geom_brep::curve_face::end_root`), and the seam the zip leaves
//! is described where it rests, its bend under-determined. So the four
//! poses that bend away from the pipe union at the closed form. The four
//! that bend back into the pipe are no junction (their walls leave the
//! rim on one side) and stop at the crossing layer
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
/// across the axis, which the revolve refuses on its own.) A turn of
/// +π/2 bends back into the pipe and −π/2 away from it, off either cap:
/// the Start cap's frame faces the other way, so off the End cap (z = 2)
/// the inward elbow spans z ∈ [−2, 2] and off the Start cap (z = 0)
/// z ∈ [0, 4].
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

/// Whether the pose bends the elbow away from the pipe: a turn of −π/2
/// off either cap.
fn outward(off: OffCap) -> bool {
    match off {
        OffCap::Elbow { angle, .. } => angle < 0.0,
        OffCap::Extrude(_) => true,
    }
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

/// An outward pose passes the crossing layer and stops at the finish on
/// the seams' stand-in descriptions; an inward one stops at the crossing
/// layer.
#[track_caller]
fn stops_where_its_pose_does(j: &Joined, off: OffCap, want: f64, what: &str) {
    if outward(off) {
        let built = built(j);
        assert_eq!(built.solids, 1, "{what}: one solid");
        assert!(built.tier3.is_ok(), "{what}: tier 3: {:?}", built.tier3);
        assert!(
            built.tier3_prime.is_ok(),
            "{what}: tier 3′: {:?}",
            built.tier3_prime
        );
        assert!(
            (built.volume - want).abs() <= 1e-12 * want,
            "{what}: volume {} against the closed form {want}",
            built.volume
        );
    } else {
        let e = refusal(j, what);
        assert!(
            matches!(e, topo::BooleanError::CurvedPierceUnsupported { .. }),
            "{what}: bent back into the pipe, at the crossing layer: {e:?}"
        );
    }
}

#[test]
fn a_pipe_and_an_elbow_off_either_cap_unions_or_stops_where_its_pose_does() {
    for (end, off) in the_eight_poses() {
        stops_where_its_pose_does(
            &join(true, end, off),
            off,
            closed_form(true, off),
            &format!("pipe, {end:?} cap, {off:?}"),
        );
    }
}

#[test]
fn a_rod_and_its_elbow_union_or_stop_where_their_pose_does() {
    for (end, off) in [the_eight_poses()[0], the_eight_poses()[1]] {
        stops_where_its_pose_does(
            &join(false, end, off),
            off,
            closed_form(false, off),
            &format!("rod, {off:?}"),
        );
    }
}

/// An outward elbow whose axis is turned `δ` about the cap's normal,
/// through the same point: its bend's radius is `BEND·cos δ`.
fn turned(delta: f64) -> (OffCap, f64) {
    let off = OffCap::Elbow {
        axis_x: BEND,
        heading: -FRAC_PI_2 + delta,
        angle: -FRAC_PI_2,
    };
    let a = section(true);
    (off, a * LENGTH + a * BEND * delta.cos() * FRAC_PI_2)
}

/// **The neutral points between the routing's stations**: the axis
/// turned a few ε, or a thousandth of a radian, about the cap's normal.
/// Off the stations the bend's relative curvature changes sign between
/// two of them, which is no jet-determinate seam, so the seam is
/// described where it rests, its bend under-determined.
#[test]
fn an_elbow_whose_neutral_points_sit_between_the_stations_unions() {
    for delta in [2.0 * Tol::witness().eps(), 1e-3] {
        let (off, want) = turned(delta);
        stops_where_its_pose_does(
            &join(true, CapEnd::End, off),
            off,
            want,
            &format!("turned {delta:e}"),
        );
    }
}

/// **A neutral point within the band of a station refuses on the seam's
/// bend**, though its wedge is a decided π: the second-order walk reads
/// that station's sagitta in band
/// (`work/select/a-seam-decided-pi-refuses-where-only-its-bend-is-undecided.md`).
#[test]
fn an_elbow_whose_neutral_point_sits_in_band_of_a_station_refuses_on_its_bend() {
    let (off, _) = turned(20.0 * Tol::witness().eps());
    let j = join(true, CapEnd::End, off);
    let e = refusal(&j, "turned 20ε");
    assert!(
        matches!(
            e,
            topo::BooleanError::Escalated {
                decision: topo::BooleanDecision::SeamJet,
                ..
            }
        ),
        "the seam's bend in band: {e:?}"
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

/// R2 probe: the built outward elbow's topology and its edges' descriptions.
#[test]
fn r2_probe_the_built_elbow() {
    for (hollow, end, off) in [
        (true, CapEnd::End, the_eight_poses()[1].1),
        (false, CapEnd::End, the_eight_poses()[1].1),
        (true, CapEnd::End, turned(1e-3).0),
    ] {
        let j = join(hollow, end, off);
        if let Some(e) = j.ev.node_error(j.union) {
            eprintln!("R2E {hollow} {off:?}: REFUSES {}", e.kind);
            continue;
        }
        let body = crate::corpus::body_of(&j.ev, j.union);
        let (f, e, v) = (body.faces().count(), body.edges().count(), body.vertices().count());
        eprintln!("R2E {hollow} {off:?}: faces {f} edges {e} vertices {v} solids {} shells {}", body.solids().count(), body.shells().count());
        for (k, edge) in body.edges() {
            let c = body.get_curve_geom(edge.curve).and_then(|c| c.certified());
            match c {
                Some(c) => {
                    let d = format!("{:?}", c.description());
                    eprintln!("R2E   edge {k:?}: {:?} params {:?} desc {}", c.carrier().kind(), c.params(), &d[..d.len().min(60)]);
                }
                None => eprintln!("R2E   edge {k:?}: uncertified"),
            }
        }
        let b = built(&j);
        eprintln!("R2E   t3 {:?} t3' {:?} vol {} want {}", b.tier3.is_ok(), b.tier3_prime.is_ok(), b.volume, { let OffCap::Elbow { heading, .. } = off else { unreachable!() }; section(hollow) * LENGTH + section(hollow) * BEND * (heading + FRAC_PI_2).cos() * FRAC_PI_2 });
    }
}

/// R2 probe: the neutral points in the half-arcs' end gaps.
#[test]
fn r2_probe_neutral_points_in_the_end_gaps() {
    for delta in [1.15, 1.18, 1.21, -1.21] {
        let (off, want) = turned(delta);
        for hollow in [true, false] {
            let want = if hollow { want } else { section(false) * LENGTH + section(false) * BEND * delta.cos() * FRAC_PI_2 };
            let j = join(hollow, CapEnd::End, off);
            eprintln!("R2G delta {delta} hollow {hollow}: {}", outcome(&j, want));
        }
    }
}
