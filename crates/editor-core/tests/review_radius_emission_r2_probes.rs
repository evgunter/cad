//! **Review probes (lane `radius-r2`) for the per-edge radius door's
//! new record reading** (`edit/radius-emission-record`, PR 2892).
//!
//! Two facts no row on the branch pins. First, that the emission
//! record survives a NON-IDENTITY permutation: §5's wall rows are all
//! authored counter-clockwise from the lexicographic minimum, so the
//! identity map `CheckedRecords::edge_of` applies is never exercised
//! against a reversed-and-rotated loop that also holds a fillet.
//! Second, that `segment_radii`'s CARRIER arm and `profile_edges_of`
//! read the same record — the property `checked_records` was factored
//! out to make true.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]

use crate::fixture;

use editor_core::{
    CancelToken, EntityKey, Entry, EvalOptions, Evaluation, LoopProgram, Node, ProfileDoc,
    ProfileEdgeRef, ProfileProgram, ProgramStep, ProgramTarget, RecipeNodeId, RoleSeg,
    ValuePayload, eval::ProfileNaming, evaluate,
};
use fixture::{insert, len, tol};
use profile::{ProfileStructure, SketchPlane};
use topo::FaceKey;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol(),
    )
}

/// The records the door reads, rebuilt through the same public doors
/// the evaluation's pre-pass runs (`edit_step_segments`'s `records`
/// says at length why a rebuild is the only route from outside).
fn records(doc: &ProfileDoc, program: &ProfileProgram) -> ProfileStructure {
    let env = doc.param_env::<f64>();
    let resolved = program.resolve::<f64>(&env).expect("the fixture resolves");
    let mut loops = Vec::new();
    let mut replay = Vec::new();
    for steps in &resolved {
        let (lp, record) = profile::replay_recording(steps, tol()).expect("the fixture replays");
        loops.push(lp);
        replay.push(record);
    }
    let assembled = profile::Profile::new(SketchPlane::xy(), loops);
    let (_, canonical) = assembled
        .validate_recording(tol())
        .expect("the fixture validates");
    ProfileStructure { replay, canonical }
}

/// The cylindrical radius of the wall a profile edge swept, `None`
/// where that wall is not a cylinder.
fn wall_radius(ev: &Evaluation<f64>, ext: RecipeNodeId, e: ProfileEdgeRef) -> Option<f64> {
    let name = fixture::fname(ext, RoleSeg::Lateral(e));
    let face: FaceKey = match ev.value(ext)?.name_table.lookup(&name)? {
        Entry::Unique(r) => match r.key {
            EntityKey::Face(f) => f,
            _ => return None,
        },
        Entry::Tied(_) => return None,
    };
    let ValuePayload::Body(body) = &ev.value(ext)?.payload else {
        return None;
    };
    match body.get_surface(body.get_face(face)?.surface)? {
        geom::Surface::Cylinder { radius, .. } => Some(*radius),
        _ => None,
    }
}

/// **A fillet's radius reaches its own arc's wall on a loop
/// canonicalization both REVERSES and ROTATES.**
///
/// `CheckedRecords::edge_of` passes the PRE-CANONICAL segment index
/// straight through, on the strength of `checked_records`' proof that
/// the two records describe one permutation and that the anchor
/// rewrite undoes it. §5's wall rows never put that together with a
/// fillet: the arc rows that are reversed hold no fillet, and the
/// fillet rows are all identity-anchored. This is the missing
/// combination — a clockwise-authored chain whose program start is not
/// its lexicographic minimum — measured in 3-space, so a door that
/// permuted its own answer names a neighbouring wall and reds.
#[test]
fn a_fillets_radius_reaches_its_wall_on_a_reversed_and_rotated_loop() {
    let pt = |x: f64, y: f64| [len(x), len(y)];
    let radius = len(0.5);
    // `a_fillets_radius_reaches_its_arcs_wall`'s chain mirrored in y:
    // the same corner rounded at the same radius, authored the other
    // way round the loop and entered away from the corner the
    // canonical start is at.
    let filleted = LoopProgram::Chain(vec![
        ProgramStep::At(pt(0.0, 0.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(3.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(3.0, -1.0))),
        ProgramStep::Toward {
            dx: fixture::scl(-1.0),
            dy: fixture::scl(0.0),
        },
        ProgramStep::Fillet(radius.clone()),
        ProgramStep::Toward {
            dx: fixture::scl(0.0),
            dy: fixture::scl(-1.0),
        },
        ProgramStep::FarEndTo(pt(1.0, -3.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, -3.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let doc = ProfileDoc::empty_derived("r2-reversed-fillet", tol());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![filleted],
        }),
    );
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let ev = run(&doc);
    let Some(Node::Profile(program)) = doc.node(profile) else {
        panic!("the profile node is a program");
    };
    let ValuePayload::Profile(pv) = &ev.value(profile).expect("evaluates").payload else {
        panic!("carries a profile");
    };
    let structure = records(&doc, program);
    let canonical = &structure.canonical.loops[0];
    assert!(
        canonical.reversed,
        "the fixture is authored clockwise, so canonicalization REVERSES it"
    );
    assert_ne!(
        canonical.start, 0,
        "and its canonical start is not the program's, so it ROTATES it too"
    );
    let answer = program
        .segment_radii(&structure, &pv.naming, 0)
        .expect("the door answers");
    let [(edge, expr)] = answer[..] else {
        panic!("one fillet, one arc, one pair — got {answer:?}");
    };
    assert_eq!(*expr, radius);
    let got = wall_radius(&ev, ext, edge)
        .unwrap_or_else(|| panic!("{edge:?} names no cylindrical wall, so it is a neighbour's"));
    assert!(
        (got - 0.5).abs() < 1e-9,
        "the answered wall is the fillet arc's own, not a neighbour at {got}"
    );
    // The loop's ONLY cylinder is that one, so the row would not pass
    // by accident if the door had named a different segment.
    let cylinders = (0..pv.naming.loops[0].len)
        .filter(|s| {
            wall_radius(
                &ev,
                ext,
                ProfileEdgeRef {
                    loop_index: 0,
                    segment: *s,
                },
            )
            .is_some()
        })
        .count();
    assert_eq!(cylinders, 1, "the chain has exactly one arc");
}

/// **`segment_radii`'s CARRIER arm reads no span, so it can disagree
/// with `profile_edges_of` about the same record.**
///
/// The PR's stated property for factoring `checked_records` out is
/// that "the two cannot disagree" about which segments of a loop the
/// records describe. For a chain that holds: both doors mint their
/// refs through `CheckedRecords::edge_of`. For a CARRIER form it does
/// not: `profile_edges_of` reads `replay.steps[0]` and bounds-checks
/// the span, while `segment_radii` enumerates `0..segments` off the
/// CANONICAL record and never looks at the span at all. A record whose
/// step-0 span is short is answered by one door and refused by the
/// other — the same class of divergence the factoring was for, one
/// arm further out.
#[test]
fn a_carrier_records_short_span_is_refused_by_one_door_and_answered_by_the_other() {
    let doc = ProfileDoc::empty_derived("r2-carrier-span", tol());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::CircleSplit {
                centre: [len(0.0), len(0.0)],
                radius: len(1.0),
                n: 4,
                phase: fixture::ang(0.0),
            }],
        }),
    );
    let ev = run(&doc);
    let Some(Node::Profile(program)) = doc.node(profile) else {
        panic!("the profile node is a program");
    };
    let ValuePayload::Profile(pv) = &ev.value(profile).expect("evaluates").payload else {
        panic!("carries a profile");
    };
    let structure = records(&doc, program);
    let naming: &ProfileNaming = &pv.naming;
    let segments = structure.canonical.loops[0].segments.len();
    assert_eq!(segments, 4, "the split loop has four segments");
    assert_eq!(
        program
            .segment_radii(&structure, naming, 0)
            .expect("the door answers")
            .len(),
        segments,
        "and every one of them is answered at the loop's one radius"
    );

    // Now the same records with step 0's recorded span cut short —
    // a record of a DIFFERENT program with the right step count,
    // which is exactly the class `checked_records`' shape check and
    // `profile_edges_of`'s span bound exist to refuse.
    let mut short = structure.clone();
    short.replay[0].steps[0] = profile::StepSpan::new(0, segments + 3);
    let step_door = program.profile_edges_of(&short, naming, 0, 0);
    let radius_door = program.segment_radii(&short, naming, 0);
    assert!(
        step_door.is_err(),
        "the per-step door refuses a span off the loop: {step_door:?}"
    );
    assert!(
        radius_door.is_ok(),
        "and the per-edge radius door answers the same record without reading it — \
         the two doors disagree about the record `checked_records` handed both: \
         {radius_door:?}"
    );
}
