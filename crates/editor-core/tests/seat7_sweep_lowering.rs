//! **The sweeps run on the verb substrate, the profile's own radius
//! reaches the walls they mint, and a document's cyl×cyl pinch reads as
//! its kernel-built twin's.**
//!
//! Three claims, three groups of rows, in that order.
//!
//! # 1. Nothing observable moved (the SEAT-4/5 method)
//!
//! `Node::Extrude` and `Node::Revolve` build a `verbs::Verb`, run it
//! through the profile door and read their birth record out of the
//! closed record channel. The wire format is pinned by a byte-identical
//! round trip over a document carrying both sweeps, and each
//! sweep-carrying corpus document's evaluation is pinned to a committed
//! digest that says WHICH document moved. `m10_p_fence` digests every
//! body point's bits corpus-wide and `lib_g16_corpus_name_digests`
//! digests every name table; neither says which document moved.
//!
//! # 2. The profile's radius, at the walls and in the key
//!
//! An extruded circle's wall is a cylinder whose stored radius is that
//! circle's, a revolved circle's wall is a torus whose minor radius is,
//! and a chain's arc steps each draw their own wall at their own
//! radius. The expression a profile's carrier radius is spelled in is
//! an input to the profile's content key, so a re-spelling never shares
//! a memo entry with its old spelling.
//!
//! # 3. The germ, end to end from a document
//!
//! Two extruded circles at one declared `r`, spun off the pinch,
//! unioned: the pinch refusal the kernel returns names the equal-radius
//! configuration, exactly as the kernel-direct twin at bit-identical
//! radii does — the margins decide the radii equal, and no recipe
//! channel enters the decision.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use editor_core::ExtrudeSide;

use crate::corpus;
use crate::fixture;

use corpus::{body_of, eval, failures};
use editor_core::{
    CancelToken, Dimension, DocEdit, DocumentId, EvalOptions, Evaluation, Formula, FreeVar,
    LoopProgram, Node, ProfileDoc, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget,
    RecipeNodeId, SlotId, StepArg, VarName, evaluate, persist,
};
use fixture::digest::digest;
use fixture::{ang, axis_in_plane, frame, insert, len, scl, square, step, tol, xy_frame};
use geom_core::{Affine3, Point2, Point3, Vec3};
use topo::{Body, BooleanError, FaceKey};

/// The declared radius every document below draws its circles at,
/// meters (dyadic).
const R: f64 = 1.0;
/// The second declared radius (dyadic).
const Q: f64 = 0.25;
/// The extrusion half-height (dyadic).
const H: f64 = 1.2;
/// The spin that takes both seams off the pinch — the germ fixture's
/// own angle (`sweep`'s `verbs_germarms2`).
const PHI: f64 = PI / 4.0;

fn param(name: &'static str) -> Formula {
    Formula::named(VarName::from_static(name), Dimension::Length)
}

/// A document declaring `r`.
fn doc_with_r(name: &'static str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(name), tol());
    step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::from_static("r"),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, R)),
        },
    )
    .0
}

/// A frame at `z` and a circle of radius `radius` drawn on it —
/// returns the doc, the frame node and the profile node.
fn circle_on_frame(
    doc: ProfileDoc,
    z: f64,
    radius: Formula,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, plane) = insert(doc, frame([0.0, 0.0, z], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            frame: plane.into(),
            loops: vec![LoopProgram::Circle {
                centre: [len(0.0), len(0.0)],
                radius,
            }],
            ids: Vec::new(),
        }),
    );
    (doc, plane, profile)
}

/// A cylinder about `z` of radius `radius`, `z ∈ [−H, H]` — the
/// document spelling of the germ fixture's `cyl`.
fn cylinder(doc: ProfileDoc, radius: Formula) -> (ProfileDoc, RecipeNodeId) {
    let (doc, _, profile) = circle_on_frame(doc, -H, radius);
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(2.0 * H),
            side: ExtrudeSide::Along,
        },
    )
}

/// A rigid rotation of `input` about the origin.
fn spin(
    doc: ProfileDoc,
    input: RecipeNodeId,
    axis: [f64; 3],
    angle: f64,
) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::transform(
            input,
            editor_core::Step::Rigid {
                translation: [len(0.0), len(0.0), len(0.0)],
                axis: axis.map(scl),
                angle: ang(angle),
            },
        ),
    )
}

// ------------------------------------------------------------------
// 1. Nothing observable moved
// ------------------------------------------------------------------

/// The two-sweep fixture: what to save, what to evaluate, and the two
/// sweep nodes' ids.
struct BothSweeps {
    snapshot: ProfileDoc,
    doc: ProfileDoc,
    edits: Vec<editor_core::DocEdit<ProfileProgram>>,
    sweeps: [RecipeNodeId; 2],
}

/// **One document carrying both sweep nodes**: a square extruded, and a
/// square revolved about an axis written in its own frame — so a single
/// file exercises both wire spellings and both lowering paths.
fn both_sweeps() -> BothSweeps {
    let mut r = corpus::Recorder::new();
    let snapshot = r.doc.clone();
    let square_loop = LoopProgram::polygon(square(0.0, 0.0, 0.5)).unwrap();
    let frame = r.insert(xy_frame());
    let profile = r.insert(Node::Profile(ProfileProgram {
        frame: frame.into(),
        loops: vec![square_loop],
        ids: Vec::new(),
    }));
    let extruded = r.insert(Node::Extrude {
        profile: profile.into(),
        distance: len(1.0),
        side: ExtrudeSide::Along,
    });
    // The revolve's own profile: a square clear of the axis, drawn on
    // its own frame, spun about an axis written in that same frame.
    let rev_frame = r.insert(xy_frame());
    let rev_profile = r.insert(Node::Profile(ProfileProgram {
        frame: rev_frame.into(),
        loops: vec![LoopProgram::polygon(square(0.0, -2.0, 0.5)).unwrap()],
        ids: Vec::new(),
    }));
    let axis = r.insert(axis_in_plane(rev_frame, (0.0, 0.0), (1.0, 0.0)));
    let revolved = r.insert(Node::Revolve {
        profile: rev_profile.into(),
        axis: axis.into(),
        angle: ang(PI / 2.0),
    });
    BothSweeps {
        snapshot,
        doc: r.doc,
        edits: r.edits,
        sweeps: [extruded, revolved],
    }
}

/// **The wire format is untouched**: save → load → save reproduces the
/// bytes exactly, for a document carrying an extrude and a revolve.
///
/// Byte equality is the whole assertion. A schema bump, a field rename,
/// a reordered payload or a changed number format each break it, and
/// none of them would be visible in an evaluation digest.
#[test]
fn an_extrude_and_revolve_document_round_trips_byte_identical() {
    let fixture = both_sweeps();
    let first =
        persist::save(&fixture.snapshot, &fixture.edits, tol()).expect("the document saves");
    let loaded = persist::load(&first, tol()).expect("its own bytes load back");
    assert_eq!(loaded.edits, fixture.edits, "the edit log did not survive");
    assert!(
        loaded.doc.bit_eq(&fixture.doc),
        "the replayed document is not bit-identical to the authored one"
    );
    let second = persist::save(&loaded.snapshot, &loaded.edits, tol())
        .expect("the loaded document re-saves");
    assert_eq!(
        first, second,
        "an extrude+revolve document does not round-trip byte-identically"
    );
}

/// **Both sweep nodes evaluate**, in one document, through the one
/// generic lowering, each with a full name table under its own id.
#[test]
fn both_sweeps_evaluate_in_one_document() {
    let fixture = both_sweeps();
    let ev = eval::<f64>(&fixture.doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "the two-sweep document failed: {bad:?}");
    for id in fixture.sweeps {
        let value = ev.value(id).expect("the sweep node produced a value");
        assert!(
            value.name_table.iter().count() > 0,
            "node {id:?} produced an empty name table"
        );
    }
}

/// **The sweep-carrying corpus documents' evaluations are
/// bit-identical**, body and name table, one committed number each
/// (`fixture::digest::digest`, the one feed every verb suite shares).
///
/// The registry is FULL of extrudes — every solid in it starts as one —
/// so this is the widest differential the verb migration has had. The
/// five rows are chosen to cover the shapes the lowering can differ on:
/// `die` and `corner_table` are polygon extrudes, `cut_cylinder` and
/// `boss_union` carry the two carrier loop forms (`circle` and
/// `circle_split`), and `kitchen_sink` is the registry's revolve.
///
/// RE-MINTED when the digest moved to its one home
/// (`fixture::digest`) and gained the boolean and split arms every
/// suite now shares: each of these five documents carries a boolean
/// (the die's pip subtracts, the table's and the boss's unions) or a
/// split (the cut cylinder, the kitchen sink) that this suite's own
/// copy of the feed never read, so the numbers moved with the FEED
/// and not with any evaluation. The differential was re-taken on the
/// extracted merge base with the shared feed: all five reproduce
/// there, and `cut_cylinder`'s and `kitchen_sink`'s are now the same
/// numbers the split suite pins — one feed, one number per document.
///
/// They are goldens in the ordinary sense — when one moves the question
/// is whether the new behaviour is right, never how to restore the old
/// number.
///
/// RE-BLESSED for the orthonormal basis's world-axis comparison: the
/// digest feeds each surface's `Debug`, and every planar carrier's
/// stored `u_ref` is now `normalize(e_z × n)` or `normalize(e_y × n)`
/// by `|n.z| ≤ max(|n.x|, |n.y|)/2`. The plane's LOCUS did not move —
/// origin and normal are bit-identical, which the STEP fixtures'
/// record-level diff shows directly — and the id-free body rows
/// (`m4_pr8_corpus`'s exact mass pins, `m5_pr8_bvh_diff`'s
/// realized-vs-idealized bit equality) were green across the change
/// untouched.
///
/// RE-BLESSED, `die` and `kitchen_sink` only, when declaring a variable
/// began minting its id on the document's chain: every node minted
/// after a declare was renumbered, and this digest feeds ids. The
/// id-free body rows (`m4_pr8_corpus`'s exact mass pins,
/// `m5_pr8_bvh_diff`) held untouched, and every row of a document that
/// declares nothing held its word.
///
/// RE-BLESSED (`boss_union` alone) when `circle_split` began storing
/// its authored carrier (centre and `|r|`) instead of re-deriving each
/// arc's carrier from its chord: the boss's split rims moved in the
/// last bits. `cut_cylinder`'s `circle` did not move — its chord
/// lowering returned the authored centre and radius bit for bit.
///
/// Re-blessed when contact records gained the `(vertex, edge)` and
/// edge-edge kinds: the digest feeds the records' `Debug`, which now
/// prints empty `ve` and `ee` lists; with those fields stripped every
/// constant here held.
///
/// RE-BLESSED for INTENT-LITERALS PR C (a slot holds a variable): every
/// node is minted from slots holding variable ids, so every id moved
/// and this digest feeds ids. No outcome or point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held untouched.
///
/// RE-BLESSED, `cut_cylinder` and `boss_union` only, when a chart
/// image's flag became `wrap` (the wrap edge, D1): the digest feeds each
/// curve's `Debug`, whose field name moved; with `wrap: ` read back as
/// `seam: ` the feed reproduces every old constant, so no evaluation
/// moved.
///
/// RE-BLESSED for INTENT-LITERALS PR D (`Expr` holds no float):
/// `kitchen_sink` alone, whose formulas hold written quantities that
/// now mint variables of their own, so its ids moved. No outcome or
/// point moved (the id-free fence held).
///
/// RE-BLESSED, `cut_cylinder` and `boss_union` only, when a swept point's description began
/// carrying a `geom_brep::SweepRange` (`range`) beside its angle or
/// vector, and a restricted one kept its placement instead of composing
/// the split's motion into it: the digest feeds each curve's `Debug`,
/// and these are the documents whose bodies store a swept-point
/// description, split or whole.
///
/// RE-BLESSED, all five, for INTENT stage 2 PR C (the product is the
/// world): each document now places its bodies, and every placement is
/// a node with a value and a name table of its own, so the evaluation
/// this digest walks holds those copies. No node evaluated before moved:
/// `intent_s2_c_world`'s migration check holds each product to its
/// pre-C digest.
///
/// RE-BLESSED, `cut_cylinder` and `boss_union` only, when restriction
/// moved onto the description as a whole (`MappedCurve { source,
/// range }`): every curve's `Debug` now nests its source under
/// `source` beside one `range`, and these are the documents whose
/// bodies store a sketch pushforward. No point moved (`m10_p_fence`'s
/// f64 and Interval rows held), and no name table did.
///
/// RE-BLESSED, all five, for INTENT stage 4 E (booleans glue on Zero):
/// a body no longer carries provenance side tables (`GeomSource`
/// stamps, field and axis sources), and the digest feeds each body's
/// `Debug`. No outcome or point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held.
#[test]
fn the_sweep_documents_evaluate_to_their_committed_digests() {
    let rows: [(&str, u64); 5] = [
        ("die", 0x3fb1_d208_1d37_5a93),
        ("corner_table", 0xdb95_b8fc_06b7_a993),
        ("cut_cylinder", 0x4318_92f9_c696_0fd1),
        ("boss_union", 0x05b4_17a8_c844_6429),
        ("kitchen_sink", 0x98c0_fba7_09be_e02d),
    ];
    let mut moved: Vec<String> = Vec::new();
    for (name, want) in rows {
        let doc = corpus::documents()
            .into_iter()
            .find(|d| d.name == name)
            .expect("the document is registered");
        let ev = eval::<f64>(&doc.doc);
        let bad = failures(&ev);
        assert!(bad.is_empty(), "{name} failed to evaluate: {bad:?}");
        let got = digest(&ev);
        println!("seat7 {name}: {got:#018x}");
        if got != want {
            moved.push(format!("{name}: {got:#018x} (want {want:#018x})"));
        }
    }
    assert!(
        moved.is_empty(),
        "these documents' evaluations moved — body or name table:\n{}",
        moved.join("\n")
    );
}

// ------------------------------------------------------------------
// 2. The profile's radius, at the walls and in the key
// ------------------------------------------------------------------

/// A profile of `loops` on a frame at `z`, extruded — returns the doc,
/// the profile node and the swept body node.
fn extruded(
    doc: ProfileDoc,
    z: f64,
    loops: Vec<LoopProgram<Formula>>,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, plane) = insert(doc, frame([0.0, 0.0, z], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            frame: plane.into(),
            loops,
            ids: Vec::new(),
        }),
    );
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(2.0 * H),
            side: ExtrudeSide::Along,
        },
    );
    (doc, profile, body)
}

/// A circle at the origin of the given radius expression.
fn circle_loop(radius: Formula) -> LoopProgram<Formula> {
    LoopProgram::Circle {
        centre: [len(0.0), len(0.0)],
        radius,
    }
}

/// Every cylindrical face of `body`, with the radius its carrier
/// stores.
fn cylinder_walls(body: &Body<f64>) -> Vec<(FaceKey, f64)> {
    topo::query::all_faces(body)
        .into_iter()
        .filter_map(|f| {
            let s = body.get_surface(body.get_face(f)?.surface)?;
            match s {
                geom::Surface::Cylinder { radius, .. } => Some((f, *radius)),
                _ => None,
            }
        })
        .collect()
}

/// **A revolve whose profile has an ON-AXIS edge mints no wall for
/// it**, and the arc's wall is a torus at the arc's own radius.
///
/// A segment lying on the axis of revolution sweeps nothing: the record
/// exports `None` at its position, and the positions after it are still
/// their own segments' walls. The chain puts the on-axis edge FIRST, so
/// a reader that closed that hole would displace the whole rest of the
/// loop.
#[test]
fn a_revolve_over_an_on_axis_edge_mints_no_wall_for_it() {
    let doc = doc_with_r("seat7-revolve-on-axis");
    let (doc, plane) = insert(doc, xy_frame());
    // Negative y is the door's half-plane about the +x axis, and the
    // first leg runs ALONG that axis from the origin.
    let mut steps = vec![
        ProgramStep::At([len(0.0), len(0.0)]),
        ProgramStep::Toward {
            dx: scl(1.0),
            dy: scl(0.0),
        },
        ProgramStep::Line(len(4.0)),
        ProgramStep::Toward {
            dx: scl(0.0),
            dy: scl(-1.0),
        },
        ProgramStep::Line(len(2.0)),
    ];
    steps.extend(tangent_arc(param("r"), profile::ArcSide::Right));
    steps.push(ProgramStep::LineTo(ProgramTarget::Start));
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            frame: plane.into(),
            loops: vec![LoopProgram::Chain(steps)],
            ids: Vec::new(),
        }),
    );
    let (doc, axis) = insert(doc, axis_in_plane(plane, (0.0, 0.0), (1.0, 0.0)));
    let (doc, solid) = insert(
        doc,
        Node::Revolve {
            profile: profile.into(),
            axis: axis.into(),
            angle: ang(2.0 * PI),
        },
    );
    let ev = eval::<f64>(&doc);
    let bad = failures(&ev);
    assert!(
        bad.is_empty(),
        "on-axis revolve document:\n{}",
        bad.join("\n")
    );
    let editor_core::ValuePayload::Profile(pv) =
        &ev.value(profile).expect("the profile evaluates").payload
    else {
        panic!("the profile node carries a profile");
    };
    assert_eq!(
        pv.edge_radii[0].len(),
        4,
        "the chain replays to four segments, one of them the on-axis leg"
    );
    // A full revolution splits each CURVED wall at its seam and builds
    // the plane disc whole, so the three segments that mint a wall are
    // five faces; a degenerate wall from the on-axis edge would be more.
    let body = body_of(&ev, solid);
    assert_eq!(
        topo::query::all_faces(body).len(),
        5,
        "three of the four segments minted a wall, each curved one split at the seam"
    );
    let mut tori = 0;
    for face in topo::query::all_faces(body) {
        let carrier = body
            .get_face(face)
            .and_then(|f| body.get_surface(f.surface))
            .expect("a live face on a carrier");
        if let geom::Surface::Torus { minor_radius, .. } = carrier {
            assert!(
                (minor_radius - R).abs() < 1e-9,
                "{face:?}: the arc's wall has minor radius {minor_radius}, not the arc's {R}"
            );
            tori += 1;
        }
    }
    assert!(
        tori > 0,
        "the fixture's arc minted a wall at all, or the row above is vacuous"
    );
}

/// One evaluation, optionally served from a prior one.
fn memo_eval(doc: &editor_core::ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        prior,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol(),
    )
}

/// **The memo never serves a sweep under a spelling the document no
/// longer holds.**
///
/// A and B extrude circles at `r`; then A's profile radius is
/// re-spelled as the literal of the same value. The geometry is
/// bit-identical, so a key over resolved values alone would hand A's
/// profile straight back out of the memo as the `r` spelling — and when
/// `r` then moves, A would follow it. The profile's key moves at the
/// re-spelling, and after the move A stays at the literal's radius
/// while B re-runs at the new one.
#[test]
fn the_memo_never_serves_a_sweep_a_stale_spelling() {
    let doc = doc_with_r("seat7-memo");
    let (doc, profile_a, a) = extruded(doc, -H, vec![circle_loop(param("r"))]);
    let (doc, _, b) = extruded(doc, 10.0, vec![circle_loop(param("r"))]);
    let ev1 = memo_eval(&doc, None);
    assert!(failures(&ev1).is_empty(), "{:?}", failures(&ev1));

    // A's carrier radius becomes the LITERAL of the same value.
    let (doc, _) = step(
        doc,
        DocEdit::SetParam {
            node: profile_a,
            slot: SlotId::Profile {
                loop_: 0,
                step: 0,
                arg: StepArg::Radius,
            },
            value: len(R).into(),
            fresh: Vec::new(),
        },
    );
    let ev2 = memo_eval(&doc, Some(&ev1));
    assert!(failures(&ev2).is_empty(), "{:?}", failures(&ev2));
    assert!(
        ev2.reused > 0,
        "B's half of the document is memo-served, or the row proves nothing about the memo"
    );
    assert_ne!(
        key_of(&ev1, profile_a),
        key_of(&ev2, profile_a),
        "a carrier radius re-spelled value-preservingly must not share a memo entry \
         with its old spelling"
    );

    // Now move `r`. B re-runs at the new radius; A stays at the literal's.
    let (doc, _) = step(
        doc,
        DocEdit::DefineVar {
            var: VarName::from_static("r").into(),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, 2.0 * R)),
            fresh: Vec::new(),
        },
    );
    let ev3 = memo_eval(&doc, Some(&ev2));
    assert!(failures(&ev3).is_empty(), "{:?}", failures(&ev3));
    let ra = cylinder_walls(body_of(&ev3, a))[0].1;
    let rb = cylinder_walls(body_of(&ev3, b))[0].1;
    assert_eq!(
        ra, R,
        "A is the literal {R}; a memo-served A would follow `r`"
    );
    assert_eq!(rb, 2.0 * R, "B is `r`, which moved to {}", 2.0 * R);
}

// ------------------------------------------------------------------
// 2b. A CHAIN loop's per-step arc radii
//
// A carrier loop is drawn at one radius; a chain's arc steps each carry
// their own, so two walls of ONE loop can stand at two radii.
// ------------------------------------------------------------------

/// One quarter-turn arc at `r`, departing along the incoming tangent.
///
/// The `Sweep` mode is the endpoint-FREE arc leg — the arc analog of
/// `line(len)` — so it needs a bound departure direction and
/// `ProgramStep::Tangent` is what binds one from the incoming tangent.
/// Neither of those two steps emits a segment; the arc emits exactly
/// one.
fn tangent_arc(r: Formula, side: profile::ArcSide) -> [ProgramStep<Formula>; 2] {
    [
        ProgramStep::Tangent,
        ProgramStep::ArcTo(ProgramArcData::Sweep {
            r,
            side,
            angle: ang(PI / 2.0),
        }),
    ]
}

/// A chain heading `+x` from the origin, straight for `4`, then one
/// quarter-turn arc at `r`, then straight back to the start: three
/// segments, of which the arc's wall is the one cylinder.
fn one_arc_chain(r: Formula) -> LoopProgram<Formula> {
    let mut steps = vec![
        ProgramStep::At([len(0.0), len(0.0)]),
        ProgramStep::Toward {
            dx: scl(1.0),
            dy: scl(0.0),
        },
        ProgramStep::Line(len(4.0)),
    ];
    steps.extend(tangent_arc(r, profile::ArcSide::Left));
    steps.push(ProgramStep::LineTo(ProgramTarget::Start));
    LoopProgram::Chain(steps)
}

/// The step [`one_arc_chain`]'s arc sits at — the address a re-spelling
/// edit writes through.
const ARC_STEP: u32 = 4;

/// The same chain with a SECOND quarter-turn arc at `r2`, after two
/// more units of straight: five segments, two of them arcs.
///
/// `side` turns the whole chain: `Left` authors it counterclockwise and
/// canonicalization leaves the segment numbering alone, `Right` authors
/// the mirror image and canonicalization REVERSES it, so canonical
/// segment `k` is a different edge from program segment `k`.
fn two_arc_chain(r1: Formula, r2: Formula, side: profile::ArcSide) -> LoopProgram<Formula> {
    let mut steps = vec![
        ProgramStep::At([len(0.0), len(0.0)]),
        ProgramStep::Toward {
            dx: scl(1.0),
            dy: scl(0.0),
        },
        ProgramStep::Line(len(4.0)),
    ];
    steps.extend(tangent_arc(r1, side));
    steps.push(ProgramStep::Tangent);
    steps.push(ProgramStep::Line(len(2.0)));
    steps.extend(tangent_arc(r2, side));
    steps.push(ProgramStep::LineTo(ProgramTarget::Start));
    LoopProgram::Chain(steps)
}

/// The profile node's content key, out of one evaluation.
fn key_of(ev: &Evaluation<f64>, node: RecipeNodeId) -> editor_core::ContentKey {
    ev.value(node).expect("the profile evaluates").content_key
}

/// **A chain arc's radius SPELLING is an input to the profile's content
/// key.**
///
/// Re-spell the arc's radius as the LITERAL of the same value: the
/// geometry is bit-identical, so a key over resolved values alone would
/// serve this profile — and the extrude above it — straight back out of
/// the memo under the old spelling. The key moves, the rest of the
/// document is memo-served, and the wall stands where it stood.
///
/// (The spelling pair is param → literal, not two unit spellings of one
/// number: display units never enter the key by ratified design (D7,
/// `switch_program_key::display_units_never_enter_the_key`), so
/// `10 mm` → `0.01 m` is the same expression and moves nothing.)
#[test]
fn a_chain_arcs_radius_spelling_moves_the_profiles_key() {
    let doc = doc_with_r("seat7-chain-arc");
    let (doc, profile_a, a) = extruded(doc, -H, vec![one_arc_chain(param("r"))]);
    let (doc, _, peg) = extruded(doc, 10.0, vec![circle_loop(param("r"))]);
    let ev1 = memo_eval(&doc, None);
    assert!(failures(&ev1).is_empty(), "{:?}", failures(&ev1));
    let walls = cylinder_walls(body_of(&ev1, a));
    assert_eq!(
        walls.len(),
        1,
        "the chain has exactly one arc, so exactly one cylindrical wall"
    );
    assert_eq!(walls[0].1, R, "the arc wall is the one drawn at `r`");

    // The arc's radius becomes the LITERAL of the same value.
    let (doc, _) = step(
        doc,
        DocEdit::SetParam {
            node: profile_a,
            slot: SlotId::Profile {
                loop_: 0,
                step: ARC_STEP,
                arg: StepArg::CarrierRadius,
            },
            value: len(R).into(),
            fresh: Vec::new(),
        },
    );
    let ev2 = memo_eval(&doc, Some(&ev1));
    assert!(failures(&ev2).is_empty(), "{:?}", failures(&ev2));
    assert_ne!(
        key_of(&ev1, profile_a),
        key_of(&ev2, profile_a),
        "a chain radius re-spelled value-preservingly must not share a memo entry with \
         its old spelling"
    );
    assert!(
        ev2.reused > 0,
        "the peg's half of the document is memo-served, or the row proves nothing about \
         the memo"
    );
    assert_eq!(
        format!("{:?}", body_of(&ev2, peg)),
        format!("{:?}", body_of(&ev1, peg)),
        "the memo-served peg is the peg"
    );
    let walls = cylinder_walls(body_of(&ev2, a));
    assert_eq!(
        walls.len(),
        1,
        "the re-spelled chain still has one arc wall"
    );
    assert_eq!(walls[0].1, R, "the literal is the value `r` held");
}

/// **Two arcs of one chain mint their walls at their own steps'
/// radii.**
///
/// The mutant this reds is a lowering that draws every arc of a loop
/// at the loop's FIRST arc radius — the shape a carrier form's
/// one-radius-per-loop rule invites, and one no single-arc row sees.
#[test]
fn each_arc_of_a_chain_mints_its_wall_at_its_own_steps_radius() {
    assert_two_arcs_at_their_own_radii("seat7-chain-two-arcs", profile::ArcSide::Left, false);
}

/// **The same claim on a chain canonicalization REVERSED**, where
/// canonical segment `k` is not program segment `k`.
#[test]
fn each_arc_of_a_reversed_chain_mints_its_wall_at_its_own_steps_radius() {
    assert_two_arcs_at_their_own_radii("seat7-chain-two-arcs-cw", profile::ArcSide::Right, true);
}

/// The shared body of the two rows above: a two-arc chain at `r` and
/// `q`, whose walls stand one at each radius.
fn assert_two_arcs_at_their_own_radii(
    id: &'static str,
    side: profile::ArcSide,
    want_reversed: bool,
) {
    let doc = doc_with_r(id);
    let (doc, _) = step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::from_static("q"),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, Q)),
        },
    );
    let (doc, profile_node, chain) =
        extruded(doc, -H, vec![two_arc_chain(param("r"), param("q"), side)]);
    let ev = eval::<f64>(&doc);
    let bad = failures(&ev);
    assert!(
        bad.is_empty(),
        "two-arc chain document:\n{}",
        bad.join("\n")
    );
    // The fixture's own premise: a row written to be the reversed case
    // and silently canonicalized to the identity one proves nothing.
    let editor_core::ValuePayload::Profile(pv) = &ev
        .value(profile_node)
        .expect("the profile evaluates")
        .payload
    else {
        panic!("{id}: the profile node carries a profile");
    };
    assert_eq!(
        pv.naming.loops[0].reversed,
        want_reversed,
        "{id}: the fixture is written to be the {} case",
        if want_reversed {
            "reversed"
        } else {
            "identity"
        }
    );
    let mut radii: Vec<f64> = cylinder_walls(body_of(&ev, chain))
        .into_iter()
        .map(|(_, r)| r)
        .collect();
    radii.sort_by(f64::total_cmp);
    // A chain arc's carrier radius reaches the wall through the replay's
    // own construction, so it is `R` and `Q` to within a rounding step
    // and not to the bit.
    assert_eq!(radii.len(), 2, "{id}: two arc steps, two cylindrical walls");
    assert!(
        (radii[0] - Q).abs() < 1e-9 && (radii[1] - R).abs() < 1e-9,
        "{id}: the walls stand at {radii:?}, not one at q = {Q} and one at r = {R}"
    );
}

// ------------------------------------------------------------------
// 3. The germ, end to end
// ------------------------------------------------------------------

/// The kernel-direct twin of [`cylinder`]: the same profile, the same
/// extrude, at the same radius — and no recipe layer above them.
fn raw_cylinder(r: f64, h: f64) -> Body<f64> {
    let lp = profile::circle(Point2::new(0.0, 0.0), r, tol()).unwrap();
    let plane = profile::SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, -h)));
    let sketch = profile::Profile::new(plane, vec![lp.into()])
        .validate(tol())
        .unwrap();
    sweep::extrude(
        &sketch,
        sweep::Extrusion::Distance {
            depth: 2.0 * h,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body
}

/// A kernel-direct rigid spin, for the twin.
fn raw_spin(b: &Body<f64>, axis: Vec3<f64>, angle: f64) -> Body<f64> {
    topo::transform_rigid(
        b,
        &Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), axis, angle),
        tol(),
    )
    .unwrap()
}

/// Whether a cyl×cyl pinch refusal decided its radii equal, or a loud
/// failure.
fn pinch_equal_radii(err: &BooleanError) -> bool {
    match err {
        BooleanError::GermFrameCylinderPinch { equal_radii, .. } => *equal_radii,
        other => panic!("the equal-radius pair refused with {other:?}, not the pinch"),
    }
}

/// **THE END-TO-END ROW**: one declared radius parameter, two extruded
/// circles, one boolean — and the germ decides the radii equal, as it
/// does for the kernel-built twin at bit-identical radii.
///
/// The pose is the germ fixture's own (`sweep`'s `verbs_germarms2`):
/// the classic Steinmetz pair never reaches the join at all, because
/// both operands' seams sit ON the pinch points and die a layer earlier
/// at the tangency door, so each operand is spun about its OWN axis —
/// a motion a cylinder of revolution is invariant under, which moves
/// the charts and not the surfaces.
///
/// What the union RETURNS is a refusal: the equal-radius
/// intersecting-axes locus is two ellipses crossing at two valence-4
/// pinch vertices, which is not one conic and therefore has no frame
/// to hand over. The refusal names which locus it is.
#[test]
fn one_declared_radius_pinches_at_the_germ_as_its_kernel_twin_does() {
    let doc = doc_with_r("seat7-germ");
    let (doc, a) = cylinder(doc, param("r"));
    let (doc, b) = cylinder(doc, param("r"));
    // B's axis becomes +y; then each is spun about its own axis to take
    // the seams off the pinch.
    let (doc, b) = spin(doc, b, [1.0, 0.0, 0.0], PI / 2.0);
    let (doc, a) = spin(doc, a, [0.0, 0.0, 1.0], PHI);
    let (doc, b) = spin(doc, b, [0.0, 1.0, 0.0], PHI);
    let (doc, union) = insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a: a.into(),
            b: b.into(),
            declare: Vec::new(),
        },
    );
    let ev = eval::<f64>(&doc);
    let from_document = match ev.nodes.get(&union) {
        Some(editor_core::NodeResult::Failed(e)) => match &e.kind {
            editor_core::NodeErrorKind::Boolean(err) => pinch_equal_radii(err),
            other => panic!("the union refused with {other:?}, not a boolean refusal"),
        },
        other => panic!("the union did not refuse: {other:?}"),
    };
    assert!(
        from_document,
        "one declared parameter's two walls must pinch as equal radii"
    );

    let raw_a = raw_spin(&raw_cylinder(R, H), Vec3::new(0.0, 0.0, 1.0), PHI);
    let raw_b = raw_spin(
        &raw_spin(&raw_cylinder(R, H), Vec3::new(1.0, 0.0, 0.0), PI / 2.0),
        Vec3::new(0.0, 1.0, 0.0),
        PHI,
    );
    let raw_a = topo::test_support::finished("the raw spun cylinder A", raw_a, tol());
    let raw_b = topo::test_support::finished("the raw spun cylinder B", raw_b, tol());
    let raw = topo::union(&raw_a, &raw_b, tol()).expect_err("this family has no join arm");
    assert!(
        pinch_equal_radii(&raw),
        "the kernel-built twin at bit-identical radii pinches as equal radii too"
    );
}
