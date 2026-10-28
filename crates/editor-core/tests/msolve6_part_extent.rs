//! MSOLVE-6 acceptance — **a bundle's lever is the copy's own
//! extent**, taken from the copied shape's evaluated body.
//!
//! The lever a bundle's angular decisions turn on is
//! `R + max over its mates of (‖on.origin‖ + ‖to.origin‖)`, `R` an
//! upper bound on the copied shape's reach from its own origin and
//! each pose's origin its distance from its coordinates' origin, with
//! no floor and no constant. These rows measure the reach against
//! bodies whose true extent is known, pin the lever to the bit, decide
//! the measure site's own example at the copy's scale (at whatever ε
//! the run is at — the expectation is derived from the run's own
//! `Band`), and pin the refusals and the exactly-once evaluation.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use topo::construction::Construct;

use std::sync::Arc;

use editor_core::mate::SurfaceKind;
use editor_core::{
    CapEnd, Coord, DocEdit, DocumentId, EvalOptions, Evaluation, Frame, Mate, MateFault, MateRef,
    Node, NodeErrorKind, NodeResult, PartFault, ProfileDoc, ReachRefusal, RecipeNodeId,
    ResolveFault, content_pin, product,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{ang, insert, len, on_frame, revolve_about, run, solve};
use geom_core::{Point3, Tol};

// ---- Substrate ----

/// **`doc` with `body` placed in its world**, as
/// [`PartStore::insert_part`] places it — so [`in_part`] spells the
/// part's caps through that placement wherever the part is stored.
fn placed_part(doc: ProfileDoc, body: RecipeNodeId) -> ProfileDoc {
    let mut scratch = PartStore::new();
    let (doc_ref, _) = scratch.insert_part((doc, body), Tol::witness());
    scratch.doc(doc_ref.id)
}

/// A box part: a square of half-side `half` at the origin, extruded
/// `height` along +z. Its farthest point from the part origin is the
/// top corner, `sqrt(2 half² + height²)` away. The body is placed in
/// the part's world ([`placed_part`]).
fn box_part(label: &str, half: f64, height: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, half)],
    );
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
        },
    );
    placed_part(doc, body)
}

/// A cylinder part: a rectangle `radius × height` in the xy plane,
/// revolved a full turn about the plane's +y through the origin. The
/// cylinder stands on the origin along +y; its farthest point from
/// the origin is the top rim, `sqrt(radius² + height²)` away. The body
/// is placed in the part's world ([`placed_part`]).
fn cylinder_part(label: &str, radius: f64, height: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.0, 0.0),
            (radius, 0.0),
            (radius, height),
            (0.0, height),
        ]],
    );
    let (doc, body) = insert(
        doc,
        revolve_about(
            profile,
            (0.0, 0.0),
            crate::fixture::along_y(),
            ang(std::f64::consts::TAU),
        ),
    );
    placed_part(doc, body)
}

/// **The node a part's caps are named on**: its one extrude or
/// revolve, found by kind.
///
/// # Panics
///
/// If `part` has no such node, or more than one.
fn body_node(part: &ProfileDoc) -> RecipeNodeId {
    let solids: Vec<RecipeNodeId> = part
        .ids()
        .iter()
        .copied()
        .filter(|&id| {
            matches!(
                part.node(id),
                Some(Node::Extrude { .. } | Node::Revolve { .. })
            )
        })
        .collect();
    let [body] = solids[..] else {
        panic!("a part here has one body node, not {solids:?}");
    };
    body
}

/// A part's reach, read off its own body the way the fold reads a
/// copy's.
fn reach(part: &ProfileDoc) -> f64 {
    editor_core::mate::part_reach(&body_of(part)).expect("the part's body is bounded")
}

/// `n` instances of `part` — the first pinned to the world at the
/// identity, the rest with empty bundles, placed by the mates a row
/// adds to them — the options that resolve them, and the part's
/// [`body_node`].
fn instances(
    label: &str,
    part: ProfileDoc,
    n: usize,
) -> (ProfileDoc, Vec<RecipeNodeId>, EvalOptions, RecipeNodeId) {
    let body = body_node(&part);
    let mut store = PartStore::new();
    let part = crate::fixture::assemble_world_copies(part);
    let doc_ref = store.insert(part, Tol::witness());
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for i in 0..n {
        let (next, id) = if i == 0 {
            fixture::insert_pinned(doc, crate::fixture::mated_instance(doc_ref), &opts)
        } else {
            insert(doc, fixture::mated_instance(doc_ref))
        };
        doc = next;
        ids.push(id);
    }
    (doc, ids, opts, body)
}

/// **`upper` seated flush on `lower`'s top cap**: `upper`'s bottom cap
/// resting on it, and its three values zero — pinned in `lower`'s
/// space. Each side is an instance of the part whose body is the
/// paired [`body_node`]. Answers the rest's address.
fn seat(
    doc: ProfileDoc,
    (lower, lower_body): (RecipeNodeId, RecipeNodeId),
    (upper, upper_body): (RecipeNodeId, RecipeNodeId),
) -> (ProfileDoc, MateRef) {
    // The rest and its three values, each 0.
    fixture::constrain_at(
        doc,
        upper,
        fixture::rest(
            fixture::head_at(upper, in_part(upper, upper_body, CapEnd::Start)),
            fixture::head_at(lower, in_part(lower, lower_body, CapEnd::End)),
        ),
    )
}

// ---- A4: refusals typed ----

/// **A part that does not resolve fails its instance in the resolver's
/// voice, carrying the `PartFault` unaltered** — and the blast radius
/// is pinned: the instance whose part is lost refuses on its own, with
/// no bundle fault, and the copy its mate reads is untouched. With no
/// resolver at all, the same arm in that voice at the copy the mate
/// reads, through which the lost instance is poisoned.
///
/// *Breaks if* a lost part reads as a mate's refusal, or reaches the
/// copy the mate is pinned against.
#[test]
fn a4_an_unresolvable_part_fails_its_instance_in_the_resolvers_voice() {
    // The part lives in ANOTHER store: the reference is well formed
    // and nothing here can resolve it.
    let mut elsewhere = PartStore::new();
    let lost_part = box_part("msolve6-a4-elsewhere", 0.5, 1.0);
    let lost_body = body_node(&lost_part);
    let doc_ref = elsewhere.insert(lost_part, Tol::witness());
    let (doc, ids, opts, body) = instances(
        "msolve6-a4-unresolved",
        box_part("msolve6-a4-part", 0.5, 1.0),
        1,
    );
    let (doc, lost) = insert(doc, fixture::mated_instance(doc_ref));
    let (doc, mate) = seat(doc, (ids[0], body), (lost, lost_body));
    assert_eq!(mate.placement, lost, "the mate is the lost instance's");
    let ev = run(&doc, &opts);
    let part_fault = |ev: &Evaluation<f64>| match ev.result(lost) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::Part { fault, .. } => fault.clone(),
            other => panic!("expected the part's own refusal, got {other:?}"),
        },
        other => panic!("expected a failure, got {other:?}"),
    };
    assert!(
        matches!(
            part_fault(&ev),
            PartFault::Unresolved {
                fault: ResolveFault::Unresolved,
                ..
            }
        ),
        "the resolver's own classification, unaltered: {:?}",
        part_fault(&ev)
    );
    let poses = solve(&doc, &opts, Tol::witness());
    assert_eq!(
        (poses.fault(lost), poses.fault(ids[0])),
        (None, None),
        "no bundle refused: the lost instance's refusal is its part's"
    );
    assert!(
        matches!(ev.result(ids[0]), Some(NodeResult::Ok(_))),
        "{:?}",
        ev.result(ids[0])
    );
    // No resolver at all: the same arm, in that voice, at the instance
    // the seat is read against — which the lost instance reads, so it
    // is poisoned through it before its own part is asked.
    let none = run(&doc, &EvalOptions::default());
    assert!(
        matches!(
            none.result(ids[0]),
            Some(NodeResult::Failed(e)) if matches!(
                &e.kind,
                NodeErrorKind::Part { fault: PartFault::NoResolver, .. }
            )
        ),
        "{:?}",
        none.result(ids[0])
    );
    assert!(
        matches!(none.result(lost), Some(NodeResult::Poisoned { through }) if *through == ids[0]),
        "{:?}",
        none.result(lost)
    );
}

// ---- A2: the reach is an upper bound, measured ----

/// **A box part's reach is at least its far corner's distance** — and
/// for a body whose every edge is a line the rim walk is EXACT, so it
/// is that distance to the bit.
#[test]
fn a2_a_box_parts_reach_is_its_far_corner() {
    let (half, height) = (0.5_f64, 1.0);
    let far = (2.0 * half * half + height * height).sqrt();
    let r = reach(&box_part("msolve6-a2-box-part", half, height));
    assert!(r >= far, "the reach {r} must bound the far corner {far}");
    // Every edge is a line, so the bound is the maximum over the
    // vertices — the corner itself, to the bit.
    assert_eq!(r, far, "a box's rim walk is exact");
}

/// **A cylinder part's reach is at least `sqrt(r² + h²)`**, its top
/// rim's distance: the rim is a circle bounded by `‖centre‖ + r`,
/// which is `h + r ≥ sqrt(r² + h²)` — an upper bound, never under.
#[test]
fn a2_a_cylinder_parts_reach_is_at_least_its_far_rim() {
    let (radius, height) = (0.3_f64, 0.7);
    let far = (radius * radius + height * height).sqrt();
    let r = reach(&cylinder_part("msolve6-a2-cyl-part", radius, height));
    assert!(r >= far, "the reach {r} must bound the far rim {far}");
    assert!(
        r <= height + radius + 1e-12,
        "and it is the rim circle's own bound, not wider: {r} vs {}",
        height + radius
    );
}

/// **A body with a face whose reach cannot be bounded refuses typed,
/// naming the face and its kind.** No document door builds such a
/// face — every surface kind a validated body can hold is bounded
/// (`mate::reach`'s per-kind table) — so the arm is pinned at the
/// unit level on a hand-built body: the Euler `mvfs` seed, whose one
/// face stands on the all-poison NURBS placeholder, which has no
/// finite hull to bound.
#[test]
fn a4_a_face_whose_reach_cannot_be_bounded_refuses_typed() {
    let mut body = topo::Body::<f64>::new();
    let made = body
        .mvfs(Point3::new(0.0, 0.0, 0.0), true)
        .expect("the seed vertex-face-shell");
    let refusal = editor_core::mate::body_reach(&body).expect_err("the placeholder has no bound");
    assert_eq!(
        refusal,
        ReachRefusal::FaceUnbounded {
            face: made.face,
            kind: SurfaceKind::Nurbs,
        }
    );
    // The same answer through the door the solve reads.
    assert_eq!(
        editor_core::mate::part_reach(&body).err(),
        Some(refusal.clone())
    );
}

// ---- A5: the evaluation and the memo ----

/// **A mated part is evaluated exactly once**: the fold's reach is the
/// copy's own, and the instantiate nodes read the one table the run
/// entered before either, so seating one instance on another crosses
/// into the part once — the same count as with no mate at all.
///
/// *Breaks if* forming a lever resolves the part a second time.
#[test]
fn a5_a_mated_part_is_evaluated_exactly_once() {
    let (doc, ids, opts, body) = instances(
        "msolve6-a5-once",
        box_part("msolve6-a5-once-part", 0.5, 1.0),
        2,
    );
    let before = run(&doc, &opts);
    assert_eq!(
        before.part_evaluations, 1,
        "two instances, one part, one crossing"
    );
    let (doc, mate) = seat(doc, (ids[0], body), (ids[1], body));
    let after = run(&doc, &opts);
    assert_eq!(
        after.part_evaluations, 1,
        "the fold's reach and the instantiate nodes share one evaluation"
    );
    assert_eq!(mate.placement, ids[1]);
    for &id in &ids {
        assert!(matches!(after.result(id), Some(NodeResult::Ok(_))));
    }
}

// ---- The edit door: no edit asks the store, and replay ----

/// Two instances of a box, the first pinned to the world and the
/// second seated flush on it, through the door: the mated
/// document the edit rows work on, with its log as the file carries
/// it, and the seat's constraints.
#[allow(clippy::type_complexity)]
fn seated(
    label: &str,
) -> (
    ProfileDoc,
    [RecipeNodeId; 2],
    Vec<MateRef>,
    EvalOptions,
    Vec<DocEdit<editor_core::ProfileProgram>>,
    RecipeNodeId,
) {
    let (doc, ids, opts, body) = instances(label, box_part(&format!("{label}-part"), 0.5, 1.0), 0);
    debug_assert!(ids.is_empty());
    let mut log = Vec::new();
    let mut doc = doc;
    let mut push = |doc: &mut ProfileDoc, edit: DocEdit<editor_core::ProfileProgram>| {
        let applied = doc.apply(&edit, Tol::witness()).expect("the edit applies");
        log.push(edit);
        *doc = applied.doc;
        (applied.record.minted, applied.record.minted_constraints)
    };
    let part_ref = {
        // The store inside `opts` already holds the part; its reference
        // is the pin of the same document.
        let part = box_part(&format!("{label}-part"), 0.5, 1.0);
        editor_core::DocRef {
            id: part.id(),
            pin: content_pin(&part, Tol::witness()).unwrap(),
        }
    };
    let insert_node = |node| DocEdit::InsertNode {
        node: Box::new(node),
        fresh: Vec::new(),
    };
    let a = push(
        &mut doc,
        insert_node(crate::fixture::mated_instance(part_ref)),
    )
    .0
    .unwrap();
    // Values on the world's axes are the first instance's whole bundle.
    push(&mut doc, pinned_by(a, &Frame::IDENTITY));
    let b = push(&mut doc, insert_node(fixture::mated_instance(part_ref)))
        .0
        .unwrap();
    // The rest and its three values, each 0, written whole.
    let rest = fixture::rest(
        fixture::head_at(b, in_part(b, body, CapEnd::Start)),
        fixture::head_at(a, in_part(a, body, CapEnd::End)),
    );
    let squared = fixture::squared(&doc, rest, &[]);
    let seat = push(&mut doc, squared.write(b))
        .1
        .into_iter()
        .map(|id| MateRef::new(b, id))
        .collect();
    (doc, [a, b], seat, opts, log, body)
}

/// The edit that writes `mate` as `placement`'s whole bundle: a world
/// pin, which pins its copy by itself.
fn pinned_by(placement: RecipeNodeId, at: &Frame) -> DocEdit<editor_core::ProfileProgram> {
    editor_core::Placed::unmated(editor_core::test_support::moves_at(at)).write(placement)
}

/// The edit that stands `placement`'s copy where it is: the identity
/// bundle written over whatever it held ([`editor_core::stand`]).
fn unseat(placement: RecipeNodeId) -> DocEdit<editor_core::ProfileProgram> {
    editor_core::stand().write(placement)
}

/// **A constraint edit is never refused for the placement it unseats,
/// and asks no store** (A11 (2)): no edit records a pose, so writing a
/// placement whose part does not resolve out of its mate — the identity
/// bundle written over its seat — applies, and stands the copy where it
/// is — no row, no solve.
///
/// *Breaks if* the unseating is refused, or invents a constraint.
#[test]
fn a6_a_mate_graph_edit_on_an_unresolvable_part_is_not_refused() {
    let mut elsewhere = PartStore::new();
    let lost_part = box_part("msolve6-a6-elsewhere", 0.5, 1.0);
    let lost_body = body_node(&lost_part);
    let lost_ref = elsewhere.insert(lost_part, Tol::witness());
    let (doc, ids, _, body) = instances(
        "msolve6-a6-unresolved",
        box_part("msolve6-a6-part", 0.5, 1.0),
        1,
    );
    let (doc, lost) = insert(doc, fixture::mated_instance(lost_ref));
    let (doc, mate) = fixture::constrain_at(
        doc,
        lost,
        fixture::rest(
            fixture::head_at(lost, in_part(lost, lost_body, CapEnd::Start)),
            fixture::head_at(ids[0], in_part(ids[0], body, CapEnd::End)),
        ),
    );
    let remove = unseat(mate.placement);
    {
        let applied = doc
            .apply(&remove, Tol::witness())
            .expect("unseating a placing mate is never refused");
        assert!(
            crate::fixture::without_anonymous(&applied.maintenance).is_empty(),
            "{:?}",
            applied.maintenance
        );
        assert!(
            fixture::bundle(&applied.doc, lost)
                .iter()
                .all(|c| c.value().is_some())
                && fixture::bundle(&applied.doc, lost).len() == 6,
            "the copy the mate pinned stands by the identity bundle, with no constraint the \
             edit invented"
        );
        assert_eq!(
            fixture::bundle(&applied.doc, ids[0]),
            fixture::bundle(&doc, ids[0]),
            "the copy it was pinned against keeps its bundle"
        );
    }
}

/// **No edit asks the store**: the edit door takes none and decides
/// only what the recipe says, so on a mated document a seat written to
/// a third instance, a world mate re-written at another pose, an
/// appearance, a union, a fourth instance, the seat unseated and the
/// first instance deleted — each applies.
///
/// *Breaks if* any of these edits is refused for a fact of the parts.
#[test]
fn a6_no_edit_asks_the_store() {
    let (doc, [a, b], seat_refs, _opts, _log, body) = seated("msolve6-a6-preserving");
    let tol = Tol::witness();
    let part_ref = match doc.node(a) {
        Some(Node::InstantiatePart { doc_ref, .. }) => *doc_ref,
        _ => panic!("an instance"),
    };
    let (doc, c) = insert(doc, fixture::mated_instance(part_ref));
    let seat_c = fixture::squared(
        &doc,
        fixture::rest(
            fixture::head_at(c, in_part(c, body, CapEnd::Start)),
            fixture::head_at(b, in_part(b, body, CapEnd::End)),
        ),
        &[],
    );
    let applied = doc.apply(&seat_c.write(c), tol).expect("a seat is written");
    assert!(
        crate::fixture::without_anonymous(&applied.maintenance).is_empty(),
        "{:?}",
        applied.maintenance
    );
    let doc = applied.doc;
    assert!(
        fixture::bundle(&doc, a).iter().all(|c| c.value().is_some()),
        "the first instance is placed in the world by values alone"
    );
    // The values taken off — the copy stood where it is — and written
    // again at another pose.
    let doc = doc
        .apply(&unseat(a), tol)
        .expect("unseating a world mate asks nothing")
        .doc;
    let doc = doc
        .apply(&pinned_by(a, &Frame::translation([0.0, 0.0, 3.0])), tol)
        .expect("a world mate re-written asks nothing")
        .doc;
    let doc = doc
        .apply(
            &DocEdit::SetAppearance {
                body: doc.read_of_node(a).expect("the instance's one output"),
                name: in_part(a, body, CapEnd::End).name,
                attr: editor_core::Attr::Color(editor_core::Rgba8::opaque(200, 30, 30)),
            },
            tol,
        )
        .expect("an appearance asks nothing")
        .doc;
    let doc = doc
        .apply(
            &DocEdit::InsertNode {
                node: Box::new(Node::Union {
                    members: editor_core::Bodies::Spelled(vec![
                        fixture::copy(b, body),
                        fixture::copy(c, body),
                    ]),
                }),
                fresh: Vec::new(),
            },
            tol,
        )
        .expect("a union asks nothing")
        .doc;
    let doc = doc
        .apply(
            &DocEdit::InsertNode {
                node: Box::new(fixture::mated_instance(part_ref)),
                fresh: Vec::new(),
            },
            tol,
        )
        .expect("a fourth instance asks nothing")
        .doc;
    let stranding = |applied: &editor_core::Applied<editor_core::ProfileProgram>| {
        assert!(
            crate::fixture::without_anonymous(&applied.maintenance)
                .iter()
                .all(|row| matches!(
                    row,
                    editor_core::Maintenance::Strand { .. }
                        | editor_core::Maintenance::StrandedRead { .. }
                        | editor_core::Maintenance::StrandedSelection { .. }
                        | editor_core::Maintenance::StrandedAppearance { .. }
                )),
            "a removal reports what it stranded and records no pose: {:?}",
            applied.maintenance
        );
    };
    // The seat unseated whole.
    assert_eq!(seat_refs.len(), 4, "the rest and its three values");
    stranding(
        &doc.apply(&unseat(b), tol)
            .expect("an unseating is never refused for the placement it unseats"),
    );
    stranding(
        &doc.apply(&DocEdit::DeleteNode { id: a }, tol)
            .expect("a removal is never refused for the placement it removes"),
    );
}

/// **A saved document whose seat is taken apart replays
/// bit-identically with no store**: the log is its edits, and replay
/// re-applies them without evaluating — the copy the unseated seat
/// pinned stands by the identity bundle, not re-pinned.
///
/// *Breaks if* replay consults a part, or re-pins what the log unseated.
#[test]
fn a6_a_saved_unseating_replays_bit_identically_with_no_store() {
    let (doc, [a, b], _, _, mut log, _) = seated("msolve6-a6-replay");
    let remove = unseat(b);
    let live = doc
        .apply(&remove, Tol::witness())
        .expect("the unseating applies")
        .doc;
    log.push(remove);
    let empty = ProfileDoc::empty(live.id(), Tol::witness());
    let text = editor_core::save(&empty, &log, Tol::witness()).expect("saves");
    let loaded = editor_core::load(&text, Tol::witness()).expect("loads with no store");
    assert!(loaded.doc.bit_eq(&live), "the document replays bit for bit");
    assert_eq!(loaded.edits, log, "the log comes back as saved");
    let replayed = ProfileDoc::replay(live.id(), &log, Tol::witness()).expect("replays");
    assert!(replayed.bit_eq(&live));
    assert!(
        fixture::bundle(&live, b)
            .iter()
            .all(|c| c.value().is_some()),
        "the copy the seat pinned stands by the identity bundle"
    );
    assert_eq!(fixture::bundle(&live, a).len(), 6, "the first stays placed");
}

/// **C5, the checked-in corpus**: every TRACKED `.pncad` (git's
/// list — the viewer's suite reaches the editor-core corpus through a
/// symlink, which is not a second corpus) — the four the row names,
/// and it asserts that is what it walked — loads with no store in
/// hand and re-saves byte for byte. The re-save is structural rather
/// than lucky: `load` returns each entry as parsed, and replay performs
/// exactly an entry's rows — a non-empty list re-applied verbatim, an
/// empty one refused if its edit performs any — so the log that comes
/// back is both the one the file holds and the one its replay did.
#[test]
fn c5_every_checked_in_document_loads_with_no_store_and_re_saves_identically() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let listed = std::process::Command::new("git")
        .args(["ls-files", "-z", "--", "*.pncad"])
        .current_dir(&root)
        .output()
        .expect("git lists the tracked files");
    assert!(listed.status.success(), "{listed:?}");
    let mut walked: Vec<String> = String::from_utf8(listed.stdout)
        .expect("paths are utf-8")
        .split('\0')
        .filter(|p| !p.is_empty())
        .map(str::to_owned)
        .collect();
    walked.sort();
    assert_eq!(
        walked,
        [
            "crates/editor-core/tests/corpus/die_tool.pncad",
            "crates/editor-core/tests/corpus/tour/die_composed_tour.pncad",
            "crates/pncad/tests/plate_param.pncad",
            "crates/viewer/tests/gallery_ring.pncad",
        ],
        "the corpus this row walks"
    );
    let found: Vec<std::path::PathBuf> = walked.iter().map(|p| root.join(p)).collect();
    let process = Tol::witness().eps();
    let mut loaded_count = 0;
    for path in &found {
        let text = std::fs::read_to_string(path).expect("readable");
        let loaded = match editor_core::load(&text, Tol::witness()) {
            Ok(loaded) => loaded,
            // D4's seam, before any replay: a document authored at
            // another ε refuses at THIS process's ε, typed. That is the
            // load door's own answer on the other ε rows and not this
            // row's subject.
            Err(editor_core::PersistError::ToleranceConflict {
                document,
                process: p,
            }) => {
                assert_ne!(
                    document,
                    p,
                    "{}: the seam refuses only a real conflict",
                    path.display()
                );
                assert_eq!(p, process);
                continue;
            }
            Err(e) => panic!("{}: loads with no store: {e}", path.display()),
        };
        loaded_count += 1;
        let again = editor_core::save(&loaded.snapshot, &loaded.edits, Tol::witness())
            .unwrap_or_else(|e| panic!("{}: re-saves: {e}", path.display()));
        assert_eq!(again, text, "{}: re-saves byte for byte", path.display());
    }
    if process == 1e-9 {
        assert_eq!(
            loaded_count,
            found.len(),
            "every checked-in document is authored at the default ε"
        );
    }
}

/// **At `Interval`, a certified body's reach is a bracket whose `hi`
/// bounds the `f64` lane's**, and an assembly over the part — two
/// instances, each pinned to the world over its own geometry —
/// evaluates `Ok` at `Interval` with the part evaluated once.
///
/// *Breaks if* the interval bracket's `hi` falls below the `f64`
/// reach, or a placement's fold at `Interval` resolves the part again.
#[test]
fn a5_at_interval_the_reach_brackets_the_f64_lanes() {
    use geom_core::{Bounds, Interval};
    let (doc, ids, opts, _) = instances(
        "msolve6-a5-interval",
        cylinder_part("msolve6-a5-interval-part", 0.3, 0.7),
        2,
    );
    let doc = fixture::pin(doc, ids[1], &Frame::translation([2.0, 0.0, 0.0]), &opts);
    let r64 = reach(&cylinder_part("msolve6-a5-interval-part", 0.3, 0.7));
    // The bracket, read off the certified body directly.
    let part = cylinder_part("msolve6-a5-interval-part", 0.3, 0.7);
    let evi = editor_core::evaluate::<Interval>(
        &part,
        None,
        &editor_core::CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let body = editor_core::product(&part, &evi, Tol::witness()).expect("the interval product");
    let bracket = editor_core::mate::part_reach(&body).expect("bounded");
    let ri = bracket.hi();
    assert!(
        ri >= r64 && ri.is_finite(),
        "hi {ri} bounds the f64 reach {r64}"
    );
    let ev = editor_core::evaluate::<Interval>(
        &doc,
        None,
        &editor_core::CancelToken::new(),
        &opts,
        Tol::witness(),
    );
    for &id in &ids {
        assert!(
            matches!(ev.result(id), Some(NodeResult::Ok(_))),
            "{:?}",
            ev.result(id)
        );
    }
    assert_eq!(ev.part_evaluations, 1);
}

// ---- The correctness arm's probes, adopted ----

/// The unit cube `[0,1]³`, placed in its world ([`placed_part`]).
fn block(label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    placed_part(doc, body)
}

/// A part's evaluated body, through the ordinary doors.
fn body_of(part: &ProfileDoc) -> topo::Body<f64> {
    let ev = run(part, &EvalOptions::default());
    product(part, &ev, Tol::witness()).expect("the part's product")
}

/// The TRUE maximum distance from the origin over the body's rims:
/// line ends exactly, circles sampled densely. For a body of plane and
/// cylinder patches the interior maximum is on a rim (convexity), so
/// this is the body's true reach up to the sampling.
fn true_reach(body: &topo::Body<f64>) -> (f64, usize) {
    let origin = Point3::new(0.0, 0.0, 0.0);
    let (mut best, mut circles) = (0.0_f64, 0usize);
    for (_, face) in body.faces() {
        for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
            let (edges, _) = topo::props::loop_edges(body, lk).expect("the loop walks");
            for e in &edges {
                match &e.carrier {
                    geom::Curve3::Line { .. } => {
                        for t in [e.t0, e.t1] {
                            best = best.max((e.carrier.eval(t) - origin).norm());
                        }
                    }
                    geom::Curve3::Circle { .. } => {
                        circles += 1;
                        let n = 4096;
                        for i in 0..=n {
                            let t = e.t0 + (e.t1 - e.t0) * f64::from(i) / f64::from(n);
                            best = best.max((e.carrier.eval(t) - origin).norm());
                        }
                    }
                    other => panic!("unexpected carrier {other:?}"),
                }
            }
        }
    }
    (best, circles)
}

/// **The reach bounds the TRUE maximum over every fixture part** —
/// boxes at three scales, the unit block, two cylinders — measured
/// against the rims themselves (line ends exactly, circles sampled),
/// against the analytic far corner or rim, and through the public
/// door bit for bit; and for an all-line body the bound IS the truth.
#[test]
fn a2_the_reach_bounds_the_true_maximum_over_every_fixture_part() {
    let cases: Vec<(&str, ProfileDoc, f64)> = vec![
        (
            "box 0.5x1",
            box_part("msolve6-p1-box", 0.5, 1.0),
            (2.0_f64 * 0.25 + 1.0).sqrt(),
        ),
        ("block [0,1]^3", block("msolve6-p1-block"), 3.0_f64.sqrt()),
        (
            "box 10mm",
            box_part("msolve6-p1-10mm", 0.005, 0.01),
            (2.0_f64 * 0.005 * 0.005 + 0.01 * 0.01).sqrt(),
        ),
        (
            "box 10m",
            box_part("msolve6-p1-10m", 5.0, 10.0),
            (2.0_f64 * 25.0 + 100.0).sqrt(),
        ),
        (
            "cyl r0.3 h0.7",
            cylinder_part("msolve6-p1-cyl-a", 0.3, 0.7),
            (0.09_f64 + 0.49).sqrt(),
        ),
        (
            "cyl r0.7 h0.3",
            cylinder_part("msolve6-p1-cyl-b", 0.7, 0.3),
            (0.49_f64 + 0.09).sqrt(),
        ),
    ];
    for (name, part, analytic) in cases {
        let body = body_of(&part);
        let (truth, circles) = true_reach(&body);
        let bound = editor_core::mate::part_reach(&body).expect("bounded");
        assert!(bound >= truth, "{name}: bound {bound} < true reach {truth}");
        assert!(
            bound >= analytic - 1e-15,
            "{name}: bound {bound} < analytic {analytic}"
        );
        if circles == 0 {
            assert_eq!(bound, truth, "{name}: all-line rims are exact");
        }
    }
}

/// **The reach fold propagates a poisoned face rather than dropping
/// it**: `body_reach` folds with `Real::max`, which is NaN-propagating
/// — where the inherent `f64::max` would drop a poisoned face's bound
/// and under-estimate silently. Pinned on the two functions, since
/// no document door builds a poisoned face.
#[test]
fn a2_the_reach_fold_propagates_a_poisoned_face_rather_than_dropping_it() {
    use geom_core::Real;
    assert!(Real::max(f64::NAN, 1.0).is_nan());
    assert!(Real::max(1.0, f64::NAN).is_nan());
    assert!(
        !f64::max(1.0, f64::NAN).is_nan(),
        "std's max drops NaN; the fold must not use it"
    );
}

/// **A mate chained to a refused one unseats like any other**: a
/// placement whose bundle its fold refuses (two rests, the second
/// beside the first past the door, as a doctored save leaves it) leaves
/// the copy seated on it poisoned through it, and unseating that copy —
/// the identity bundle written over its seat — neither asks for a
/// verdict nor refuses: no edit records a pose.
///
/// *Breaks if* a refused placement's reader evaluates, or a removal
/// decides anything.
#[test]
fn a6_a_mate_chained_to_a_refused_one_removes_like_any_other() {
    let part = box_part("msolve6-p3c-part", 5.0, 10.0);
    let body = body_node(&part);
    let mut store = PartStore::new();
    let part = crate::fixture::assemble_world_copies(part);
    let doc_ref = store.insert(part, Tol::witness());
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("msolve6-p3c"), Tol::witness());
    let (doc, b) = insert(doc, fixture::mated_instance(doc_ref));
    let (doc, c) = insert(doc, fixture::mated_instance(doc_ref));
    let rest_on = |to| Mate {
        on: fixture::plane(fixture::head_at(b, in_part(b, body, CapEnd::Start))),
        to: fixture::flip(to),
        orient: editor_core::Orientation::IDENTITY,
    };
    // A floor's top, and a second turned `0.25` about the world's `x`.
    let (doc, level, _) = fixture::floor(doc, Vec::new());
    let (doc, tilted, _) = fixture::floor(
        doc,
        vec![(Coord::Spin(Some(editor_core::TargetAxis::X)), ang(0.25))],
    );
    let (doc, _) = fixture::constrain_at(doc, b, rest_on(level));
    let (doc, _) = editor_core::test_support::beside_past_the_bundle_rule(
        &doc,
        b,
        rest_on(tilted),
        Tol::witness(),
    );
    // One rest and its values: the seat this row unseats.
    let (doc, m2) = fixture::constrain_at(
        doc,
        c,
        fixture::rest(
            fixture::head_at(c, in_part(c, body, CapEnd::Start)),
            fixture::head_at(b, in_part(b, body, CapEnd::End)),
        ),
    );
    assert!(
        matches!(
            solve(&doc, &opts, Tol::witness()).fault(b),
            Some(MateFault::Overconstrained { .. })
        ),
        "the premise: the two rests' bundle refuses"
    );
    let ev = run(&doc, &opts);
    assert!(
        matches!(ev.result(c), Some(NodeResult::Poisoned { through }) if *through == b),
        "the copy seated on the undecided one is poisoned through it: {:?}",
        ev.result(c)
    );
    let remove = unseat(m2.placement);
    {
        let applied = doc
            .apply(&remove, Tol::witness())
            .expect("unseating a copy is never refused");
        assert!(crate::fixture::without_anonymous(&applied.maintenance).is_empty());
    }
}

/// **A whole-assembly split stands its instance where it stands**: the
/// cut's instances move into the new part, and the instance left
/// behind stands where the part's world stands by the identity bundle,
/// which reads nothing of the part — so a split handed no resolver
/// stands it the same way.
///
/// *Breaks if* a split leaves an unplaced instance, or reads the part
/// to stand it.
#[test]
fn a6_a_split_stands_its_instance_by_the_identity_bundle_with_or_without_a_resolver() {
    let (doc, ids, opts, body) = instances("msolve6-p6", box_part("msolve6-p6-part", 0.5, 1.0), 2);
    let [a, b] = [ids[0], ids[1]];
    let (doc, _) = seat(doc, (a, body), (b, body));
    // Both instances, the second's bundle holding the seat.
    let cut = fixture::with_placements(&doc, &[a, b].into_iter().collect());
    let split = |resolver| {
        editor_core::split(
            &doc,
            &cut,
            DocumentId::derive("msolve6-p6-new-part"),
            Tol::witness(),
            resolver,
        )
    };
    let out = split(opts.resolver.as_ref())
        .unwrap_or_else(|e| panic!("a whole-assembly cut splits: {e}"));
    let remainder = &out.remainder;
    let placements = remainder.placements();
    assert_eq!(placements.len(), 1, "one instance is left behind");
    let bundle = fixture::bundle(remainder, placements[0]);
    assert!(
        bundle.len() == 6 && bundle.iter().all(|c| c.value().is_some()),
        "the instance stands where it stands by the identity bundle: {bundle:?}"
    );
    let unresolved = split(None).unwrap_or_else(|e| panic!("a cut splits with no resolver: {e}"));
    let placements = unresolved.remainder.placements();
    assert_eq!(
        fixture::bundle(&unresolved.remainder, placements[0]),
        bundle,
        "with no resolver the instance stands by the same identity bundle"
    );
}

/// **Two mated parts evaluate once each**: a chain over two distinct
/// parts (three instances, two seats) evaluates each part exactly
/// once — the folds' reaches share the run's cache with the
/// instantiate nodes — and every node evaluates `Ok`.
///
/// *Breaks if* a fold resolves a part the instance already did.
#[test]
fn a5_two_mated_parts_evaluate_once_each() {
    let mut store = PartStore::new();
    let (pa, pb) = (box_part("msolve6-p8-a", 0.5, 1.0), block("msolve6-p8-b"));
    let (a_body, b_body) = (body_node(&pa), body_node(&pb));
    let pa = crate::fixture::assemble_world_copies(pa);
    let ra = store.insert(pa, Tol::witness());
    let pb = crate::fixture::assemble_world_copies(pb);
    let rb = store.insert(pb, Tol::witness());
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("msolve6-p8"), Tol::witness());
    let (doc, a) = fixture::insert_pinned(doc, crate::fixture::mated_instance(ra), &opts);
    let (doc, b) = insert(doc, fixture::mated_instance(rb));
    let (doc, c) = insert(doc, fixture::mated_instance(ra));
    assert_eq!(run(&doc, &opts).part_evaluations, 2);
    let (doc, _) = seat(doc, (a, a_body), (b, b_body));
    let (doc, _) = seat(doc, (b, b_body), (c, a_body));
    let ev = run(&doc, &opts);
    assert_eq!(ev.part_evaluations, 2);
    for id in [a, b, c] {
        assert!(
            matches!(ev.result(id), Some(NodeResult::Ok(_))),
            "{:?}",
            ev.result(id)
        );
    }
}
