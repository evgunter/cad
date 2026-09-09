//! **The `Shell` recipe node** — `Node::Shell` wires the verb seat's
//! `Verb::Shell` (`topo::shell_open`) over a target body and an
//! ORDERED designation of the faces to open into rims.
//!
//! The documents under test are `corpus/cup.rs` (a box with its top
//! opened; exact closed forms) and `corpus/vessel.rs` (a revolved pot
//! with its two-faced mouth opened). Both sit beside the corpus
//! registry rather than in it, so this module runs the rows the
//! registry would have run on them — both lanes, persistence, the
//! bump — and pins the reason they are outside: a dual has no shell
//! door.
//!
//! # The oracles, derived rather than measured
//!
//! `corpus/cup.rs`'s module docs derive the open cup's closed forms
//! for a blank `L × L × H` and wall `t`:
//!
//! ```text
//! V = L²H − (L−2t)²(H−t)
//! A = 2L² + 4LH + 4(L−2t)(H−t)
//! ```
//!
//! and the sealed hollow's (`open = []`), whose cavity is a complete
//! box `(L−2t) × (L−2t) × (H−2t)`:
//!
//! ```text
//! V = L²H − (L−2t)²(H−2t)
//! A = 2L² + 4LH + 2(L−2t)² + 4(L−2t)(H−2t)
//! ```
//!
//! Every dimension is dyadic, so both are exact in `f64` and asserted
//! with `==`, never metered.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use corpus::{body_of, cup, eval, failures, vessel};
use editor_core::{
    CancelToken, DocEdit, EntityKind, EvalOptions, EvalOutcome, Node, NodeErrorKind, NodeResult,
    PersistError, ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg, SlotId, StableName, apply,
    evaluate, load, save,
};
use geom_core::{Dual64, Tol};
use topo::ShellError;

/// A name under the shell node wrapping one source name in a role.
fn shelled(shell: RecipeNodeId, kind: EntityKind, seg: RoleSeg) -> StableName {
    StableName {
        kind,
        node: shell,
        path: vec![seg],
    }
}

/// The three names the cup's rows read: the rim of the top, the cavity
/// twin of the bottom, the outer wall carried through.
fn cup_names(blank: RecipeNodeId, shell: RecipeNodeId) -> [StableName; 3] {
    [
        shelled(
            shell,
            EntityKind::Face,
            RoleSeg::Rim(Box::new(cup::top(blank))),
        ),
        shelled(
            shell,
            EntityKind::Face,
            RoleSeg::Inner(Box::new(cup::bottom(blank))),
        ),
        editor_core::carried(shell, fixture::fname(blank, fixture::wall(0))),
    ]
}

/// The blank the cup shells (its extrude node), found by kind.
fn blank_of(doc: &ProfileDoc) -> RecipeNodeId {
    doc.order()
        .iter()
        .copied()
        .find(|&id| matches!(doc.node(id), Some(Node::Extrude { .. })))
        .expect("the cup's blank")
}

/// Exact mass equality against a dyadic oracle.
fn assert_exact(body: &topo::Body<f64>, pin: corpus::MassPin, what: &str) {
    let m = topo::mass_properties(body, Tol::witness()).expect("mass properties");
    assert_eq!(
        m.volume, pin.volume,
        "{what}: volume is not the closed form"
    );
    assert_eq!(
        Some(m.surface_area),
        pin.area,
        "{what}: area is not the closed form"
    );
}

// ---------------------------------------------------------------
// 1. The cup: green, exact, named, rebuilt
// ---------------------------------------------------------------

/// **The f64 row**: the cup evaluates green, its head is a valid closed
/// solid with eleven faces, and its mass IS the closed form.
#[test]
fn the_cup_evaluates_green_and_is_exactly_its_closed_form() {
    let d = cup::document();
    let ev = eval::<f64>(&d.doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "cup:\n{}", bad.join("\n"));
    assert_eq!(ev.outcome, EvalOutcome::Completed);
    assert_eq!(ev.order.len(), d.len());

    let body = body_of(&ev, d.result.expect("the cup is the head"));
    assert_eq!(topo::validate(body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(body), Ok(()), "closed");
    // Five outer faces, the rim annulus, the cavity floor and its four
    // walls: the opened top is a face that became a rim, not a hole.
    assert_eq!(body.faces().count(), 11, "5 outer + 1 rim + 5 cavity");
    assert_eq!(body.solids().count(), 1);
    assert_eq!(
        body.shells().count(),
        1,
        "the rim fused the cavity into the outer shell"
    );
    assert_exact(body, d.pin.expect("the cup pins"), "cup");
}

/// **The interval row**: both documents, green at the certified
/// scalar — the lane the registry would have run.
#[cfg(feature = "interval")]
#[test]
fn both_documents_evaluate_green_at_the_interval_scalar() {
    let d = cup::document();
    let ev = eval::<geom_core::Interval>(&d.doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "cup (interval):\n{}", bad.join("\n"));
    assert_eq!(ev.outcome, EvalOutcome::Completed);
    let v = vessel::document();
    let ev = eval::<geom_core::Interval>(&v.doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "vessel (interval):\n{}", bad.join("\n"));
}

/// **The sealed row**: an empty `open` is the sealed hollow — legal, a
/// closed thin solid with TWO shells in one solid, each of genus 0.
#[test]
fn an_empty_open_list_is_the_sealed_hollow() {
    let d = cup::document();
    let blank = blank_of(&d.doc);
    let (doc, sealed) = fixture::insert(
        d.doc.clone(),
        Node::shell(blank, fixture::len(cup::T), Vec::new()),
    );
    let ev = eval::<f64>(&doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "sealed:\n{}", bad.join("\n"));
    let body = body_of(&ev, sealed);
    assert_eq!(topo::validate(body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(body), Ok(()), "closed");
    assert_eq!(body.solids().count(), 1, "one solid");
    assert_eq!(
        body.shells().count(),
        2,
        "an outer shell and a cavity shell"
    );
    assert_eq!(body.faces().count(), 12, "two complete boxes");
    // Euler on the two boxes: genus 0 each, so V − E + F = 2 per shell.
    let (v, e, f) = (
        body.vertices().count() as i64,
        body.edges().count() as i64,
        body.faces().count() as i64,
    );
    assert_eq!(v - e + f, 4, "two genus-0 shells");
    assert_exact(
        body,
        cup::sealed_forms(cup::L, cup::H, cup::T),
        "sealed cup",
    );
    // Every face is a survivor or a cavity twin: no rim was minted.
    let table = &ev.value(sealed).expect("a value").name_table;
    assert!(
        table.iter().all(|(n, _)| !matches!(
            n.path.first(),
            Some(RoleSeg::Rim(_) | RoleSeg::HoleRim { .. })
        )),
        "a sealed hollow mints no rim"
    );
}

/// **The naming row**: `Rim(top)`, `Inner(bottom)` and
/// `FromTarget(side)` each resolve to exactly one face of the cup, the
/// designated face's own name VANISHES, and the table is total.
#[test]
fn the_rim_inner_and_outer_names_resolve() {
    let d = cup::document();
    let blank = blank_of(&d.doc);
    let shell = d.result.expect("head");
    let ev = eval::<f64>(&d.doc);
    let value = ev.value(shell).expect("the cup evaluated");
    let table = &value.name_table;
    for name in cup_names(blank, shell) {
        assert!(
            matches!(table.lookup(&name), Some(editor_core::Entry::Unique(_))),
            "{name:?} must resolve to one face"
        );
    }
    // The designated face's own name is gone: what a selector says for
    // the mouth is `Rim(top)`, never `FromTarget(top)`.
    let carried_top = editor_core::carried(shell, cup::top(blank));
    assert!(
        table.lookup(&carried_top).is_none(),
        "the opened face's own name must vanish"
    );
    // Every role the cup can produce is produced, and nothing else.
    let mut roles: Vec<&'static str> = table
        .iter()
        .map(|(n, _)| match n.path.first().expect("a role path") {
            RoleSeg::OutputBody => "body",
            RoleSeg::FromTarget(_) => "survivor",
            RoleSeg::Inner(_) => "inner",
            RoleSeg::Rim(_) => "rim",
            RoleSeg::HoleRim { .. } => "hole rim",
            other => panic!("a non-shell role leaked into the shell's table: {other:?}"),
        })
        .collect();
    roles.sort_unstable();
    roles.dedup();
    assert_eq!(roles, ["body", "inner", "rim", "survivor"]);
    // The ring: the rim's inner loop is the cavity twins of the top's
    // four edges, named `Inner(⟨boundary edge⟩)` — no second role.
    let ring: usize = table
        .iter()
        .filter(|(n, _)| {
            n.kind == EntityKind::Edge
                && matches!(
                    n.path.first(),
                    Some(RoleSeg::Inner(src)) if matches!(src.path.first(), Some(RoleSeg::RimEdge(editor_core::CapEnd::End, _)))
                )
        })
        .count();
    assert_eq!(
        ring, 4,
        "the rim's ring is the top's four boundary edges, twinned"
    );
    let body = body_of(&ev, shell);
    assert_eq!(
        table.iter().count(),
        1 + 11 + 24 + 16,
        "body + faces + edges + vertices"
    );
    assert_eq!(body.edges().count(), 24);
    assert_eq!(body.vertices().count(), 16);
}

/// **The rebuild row, above all**: bump the blank's height AND the wall
/// through `DocEdit`, re-evaluate, and the same three names resolve to
/// the same roles while the closed forms move to the bumped values.
#[test]
fn a_rebuild_moves_the_forms_and_keeps_the_names() {
    let d = cup::document();
    let blank = blank_of(&d.doc);
    let shell = d.result.expect("head");
    let before = eval::<f64>(&d.doc);
    let names = cup_names(blank, shell);
    let resolved_before: Vec<_> = names
        .iter()
        .map(|n| before.value(shell).unwrap().name_table.lookup(n).cloned())
        .collect();

    let bumped = d.bumped();
    let bumped = apply(
        &bumped,
        &DocEdit::SetParam {
            node: shell,
            slot: SlotId::ShellThickness,
            expr: fixture::len(cup::T_BUMPED),
        },
        Tol::witness(),
    )
    .expect("the wall bumps")
    .doc;
    let after = eval::<f64>(&bumped);
    let bad = failures(&after);
    assert!(bad.is_empty(), "bumped cup:\n{}", bad.join("\n"));
    let body = body_of(&after, shell);
    assert_exact(
        body,
        cup::closed_forms(cup::L, cup::H_BUMPED, cup::T_BUMPED),
        "bumped cup",
    );
    let table = &after
        .value(shell)
        .expect("the bumped cup evaluated")
        .name_table;
    for (name, was) in names.iter().zip(&resolved_before) {
        let now = table.lookup(name).cloned();
        assert!(
            matches!(now, Some(editor_core::Entry::Unique(_))),
            "{name:?} must still resolve after the bump"
        );
        assert_eq!(
            &now, was,
            "{name:?} must resolve to the same entity after the bump"
        );
    }
    // The bump moved the key: a different body must not memo-hit.
    assert_ne!(
        before.value(shell).unwrap().content_key,
        after.value(shell).unwrap().content_key
    );
}

// ---------------------------------------------------------------
// 2. The vessel: the two-faced mouth
// ---------------------------------------------------------------

/// **The revolved mouth**: both halves designated, the rim is ONE
/// annular face named for the FIRST designated half, and the body is a
/// valid closed thin solid.
#[test]
fn the_vessel_opens_its_two_faced_mouth_into_one_rim() {
    let d = vessel::document();
    let ev = eval::<f64>(&d.doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "vessel:\n{}", bad.join("\n"));
    let shell = d.result.expect("head");
    let body = body_of(&ev, shell);
    assert_eq!(topo::validate(body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(body), Ok(()), "closed");
    // Outer: base ×2, foot ×2, belly ×2; the rim; cavity: the same six.
    assert_eq!(body.faces().count(), 13, "6 outer + 1 rim + 6 cavity");
    let pot = d.doc.node(shell).map(|n| n.inputs()[0]).expect("the pot");
    let table = &ev.value(shell).expect("evaluated").name_table;
    let rim = shelled(
        shell,
        EntityKind::Face,
        RoleSeg::Rim(Box::new(editor_core::band(pot, vessel::SEG_MOUTH))),
    );
    assert!(
        matches!(table.lookup(&rim), Some(editor_core::Entry::Unique(_))),
        "the rim is named for the first designated half"
    );
    let other = shelled(
        shell,
        EntityKind::Face,
        RoleSeg::Rim(Box::new(editor_core::band_pi(pot, vessel::SEG_MOUTH))),
    );
    assert!(
        table.lookup(&other).is_none(),
        "the second half's name does not carry the rim"
    );
    let rims = table
        .iter()
        .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::Rim(_))))
        .count();
    assert_eq!(rims, 1, "one chart, one rim");
}

/// **The order of `open` is the rim's identity, and the key says so**:
/// the same two mouth faces named the other way round mint the other
/// half's `Rim` and key apart.
#[test]
fn the_designation_order_moves_the_rim_and_the_content_key() {
    let a = vessel::document();
    let b = vessel::document_with_open(|pot| {
        [
            editor_core::band_pi(pot, vessel::SEG_MOUTH),
            editor_core::band(pot, vessel::SEG_MOUTH),
        ]
    });
    let (sa, sb) = (a.result.unwrap(), b.result.unwrap());
    let (ea, eb) = (eval::<f64>(&a.doc), eval::<f64>(&b.doc));
    assert!(failures(&eb).is_empty(), "{:?}", failures(&eb));
    assert_ne!(
        ea.value(sa).unwrap().content_key,
        eb.value(sb).unwrap().content_key,
        "the rim's identity moved, so the key must"
    );
    let pot = b.doc.node(sb).map(|n| n.inputs()[0]).unwrap();
    let rim_pi = shelled(
        sb,
        EntityKind::Face,
        RoleSeg::Rim(Box::new(editor_core::band_pi(pot, vessel::SEG_MOUTH))),
    );
    assert!(
        matches!(
            eb.value(sb).unwrap().name_table.lookup(&rim_pi),
            Some(editor_core::Entry::Unique(_))
        ),
        "named the other way round, the other half carries the rim"
    );
}

// ---------------------------------------------------------------
// 3. The refusal families
// ---------------------------------------------------------------

/// The typed refusal a shell node produced at `f64`.
fn refusal(doc: &ProfileDoc, node: RecipeNodeId) -> NodeErrorKind {
    let mut ev = evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    match ev.nodes.remove(&node) {
        Some(NodeResult::Failed(e)) => e.kind,
        other => panic!("expected a refusal at {node:?}, got {other:?}"),
    }
}

/// A cup document whose shell node is replaced by `shell`.
fn cup_with(
    shell: impl FnOnce(RecipeNodeId) -> Node<ProfileProgram>,
) -> (ProfileDoc, RecipeNodeId) {
    let d = cup::document();
    let blank = blank_of(&d.doc);
    fixture::insert(d.doc, shell(blank))
}

/// **The four document-layer refusals and the kernel's own, typed and
/// pinned as text** — the blends' `lib_g16_blend_messages.rs` shape.
/// Merged into one row: each rebuilds the cup, and the split would pay
/// the fixture five times over.
#[test]
fn the_refusals_are_typed_and_their_texts_pinned() {
    // (a) a name the target never minted — Vanished through N5.
    let ghost = |blank| fixture::fname(blank, fixture::wall(7));
    let (doc, n) = cup_with(|blank| Node::shell(blank, fixture::len(cup::T), vec![ghost(blank)]));
    let e = refusal(&doc, n);
    assert!(matches!(e, NodeErrorKind::ShellOpenResolve { .. }), "{e:?}");
    assert_eq!(
        e.to_string(),
        "a shell open-face name failed to resolve: the face name minted by node 2 no longer \
         resolves in this evaluation: the recorded reference disagrees with the recipe as it \
         stands on the derivation path (node 2's payload differs)"
    );

    // (b) a name of the wrong KIND: an edge that really is there, so
    // it resolves and then fails the door's faces-only check.
    let (doc, n) = cup_with(|blank| {
        Node::shell(
            blank,
            fixture::len(cup::T),
            vec![fixture::prism_edges(blank, 4)[0].clone()],
        )
    });
    let e = refusal(&doc, n);
    assert!(
        matches!(
            &e,
            NodeErrorKind::ShellOpenKind {
                found: EntityKind::Edge,
                ..
            }
        ),
        "{e:?}"
    );
    assert_eq!(
        e.to_string(),
        "the shell open-face name minted by node 2 denotes an edge, not a face"
    );

    // (c) a non-positive thickness: the kernel's gate, carried WITH its
    // number — at f64 the fold is the identity, so the value comes
    // back bit for bit.
    let (doc, n) =
        cup_with(|blank| Node::shell(blank, fixture::len(-0.125), vec![cup::top(blank)]));
    let e = refusal(&doc, n);
    match &e {
        NodeErrorKind::Shell(inner) => match **inner {
            ShellError::Thickness { thickness } => assert_eq!(thickness, -0.125),
            ref other => panic!("expected the thickness gate, got {other:?}"),
        },
        other => panic!("expected the shell op's refusal, got {other:?}"),
    }
    assert_eq!(
        e.to_string(),
        "the shell op refused: shell: the wall thickness (-0.125 m) is not certifiably \
         positive, so there is no thin solid to build"
    );

    // (d) a half-chart designation on the vessel: the kernel's
    // `OpenFaceChartPartial`, carried verbatim — the document layer
    // completes no chart on the author's behalf.
    let v = vessel::document_with_open(|pot| {
        [
            editor_core::band(pot, vessel::SEG_MOUTH),
            editor_core::band(pot, vessel::SEG_BELLY),
        ]
    });
    // Replace the two-name designation by the single half: the door
    // above needs two names, so re-author with one.
    let pot = v
        .doc
        .node(v.result.unwrap())
        .map(|n| n.inputs()[0])
        .unwrap();
    let (doc, n) = fixture::insert(
        v.doc.clone(),
        Node::shell(
            pot,
            fixture::len(vessel::WALL),
            vec![editor_core::band(pot, vessel::SEG_MOUTH)],
        ),
    );
    let e = refusal(&doc, n);
    match &e {
        NodeErrorKind::Shell(inner) => {
            assert!(
                matches!(**inner, ShellError::OpenFaceChartPartial { .. }),
                "expected the partial-chart gate, got {inner:?}"
            );
        }
        other => panic!("expected the shell op's refusal, got {other:?}"),
    }
    // The op row's tail quotes arena keys, so it is prefix-pinned.
    let text = e.to_string();
    assert!(
        text.starts_with("the shell op refused: shell: ")
            && text.contains("shares its chart and was not"),
        "the partial-chart refusal text moved: {text}"
    );
    // (e) a CURVED designated face: the belly is a sphere zone, and a
    // rim on it would be a curved face carrying a ring — the kernel's
    // `OpenFaceRingUnsupported`, carried with the surface kind. This
    // gate sits BEFORE the chart check in the kernel's own order, so
    // the mouth's missing half is not what this designation hears
    // about first.
    let e = refusal(&v.doc, v.result.unwrap());
    match &e {
        NodeErrorKind::Shell(inner) => match **inner {
            ShellError::OpenFaceRingUnsupported { kind, .. } => {
                assert_eq!(kind, geom_brep::SurfaceKind::Sphere);
            }
            ref other => panic!("expected the ring gate on the belly, got {other:?}"),
        },
        other => panic!("expected the shell op's refusal, got {other:?}"),
    }
}

/// **The construction door keeps order and drops repeats keeping the
/// first**, and `Rebind` onto an already-designated face shrinks the
/// list the same way.
#[test]
fn the_shell_door_keeps_designation_order_and_drops_repeats() {
    let a = fixture::fname(RecipeNodeId(1), fixture::wall(0));
    let b = fixture::fname(RecipeNodeId(1), fixture::wall(1));
    let node: Node<ProfileProgram> = Node::shell(
        RecipeNodeId(1),
        fixture::len(0.1),
        vec![b.clone(), a.clone(), b.clone(), a.clone()],
    );
    let Node::Shell { open, .. } = &node else {
        panic!("the door builds a shell");
    };
    assert_eq!(
        open,
        &vec![b.clone(), a.clone()],
        "order kept, first occurrence kept"
    );
    assert_eq!(node.payload_names(), vec![&b, &a]);
    assert_eq!(node.slots(), vec![SlotId::ShellThickness]);
    assert_eq!(
        SlotId::ShellThickness.dimension(),
        editor_core::Dimension::Length
    );
    assert_eq!(SlotId::ShellThickness.label(), "shell thickness");
    assert!(!SlotId::ShellThickness.is_structural());
}

/// **The load door refuses a repeated `open` entry** as a corrupt file,
/// never quietly deduplicating it — through the one definition the
/// insert door asks too (`Node::input_fault`), so the two doors refuse
/// alike (`lib_g17_r2_probes::p2_*` is the insert door's half).
#[test]
fn a_repeated_open_entry_is_refused_at_load() {
    let d = cup::document();
    let text = save(&d.doc, &[], Tol::witness()).expect("the cup saves");
    // The wire form of `open` is the name's own serde encoding inside
    // an `"open"` list; the one entry names the blank's END cap, and
    // the pin reads that spelling rather than assuming it.
    let open = text
        .find("\"open\"")
        .expect("the open list reaches the wire");
    let start = open + text[open..].find('[').expect("a list");
    let mut depth = 0usize;
    let mut end = start;
    for (i, ch) in text[start..].char_indices() {
        match ch {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    end = start + i;
                    break;
                }
            }
            _ => {}
        }
    }
    let entry = text[start + 1..end].trim().to_owned();
    assert!(
        entry.contains("\"Cap\": \"End\""),
        "the one designated face is the extrude's end cap: {entry}"
    );
    // Doubling the list's one entry: `[x]` → `[x, x]`.
    let corrupt = format!("{}[{entry}, {entry}]{}", &text[..start], &text[end + 1..]);
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(editor_core::SnapshotError::InputList {
            fault: editor_core::InputFault::RepeatedDesignation { first: 0, again: 1 },
            ..
        })) => {}
        other => panic!("a repeated designation must refuse typed, got {other:?}"),
    }
    // The uncorrupted text round-trips, so the refusal above is the
    // repeat's and not the surgery's.
    let back = load(&text, Tol::witness()).expect("the cup loads");
    assert!(back.doc.bit_eq(&d.doc));
}

/// **A dual has no shell door**: the reason the two documents sit
/// beside the registry, pinned as the typed refusal it is rather than
/// left to a red in another program's row.
#[test]
fn a_dual_evaluation_refuses_the_shell_typed() {
    for d in [cup::document(), vessel::document()] {
        let ev = eval::<Dual64>(&d.doc);
        let shell = d.result.expect("head");
        match ev.nodes.get(&shell) {
            Some(NodeResult::Failed(e)) => {
                assert!(
                    matches!(e.kind, NodeErrorKind::ShellLaneUnsupported { lane: "Dual" }),
                    "{}: expected the lane refusal, got {:?}",
                    d.name,
                    e.kind
                );
            }
            other => panic!(
                "{}: expected a typed refusal at Dual64, got {other:?}",
                d.name
            ),
        }
        // Everything upstream of the shell built: the refusal is the
        // shell's alone.
        let upstream_bad: Vec<_> = failures(&ev)
            .into_iter()
            .filter(|s| !s.starts_with(&format!("{shell:?}")))
            .collect();
        assert!(upstream_bad.is_empty(), "{}: {upstream_bad:?}", d.name);
    }
}

/// **Both documents round-trip through persistence**, the tube pair's
/// row: the wire form of an ordered `open` list comes back bit for bit.
#[test]
fn both_documents_round_trip_through_persistence() {
    for d in [cup::document(), vessel::document()] {
        let empty = ProfileDoc::empty_derived("lib-g17-roundtrip", Tol::witness());
        let mut expected = empty.clone();
        for edit in &d.edits {
            expected = apply(&expected, edit, Tol::witness())
                .expect("a corpus edit applies")
                .doc;
        }
        let text = save(&empty, &d.edits, Tol::witness()).expect("saves");
        let back = load(&text, Tol::witness()).expect("loads");
        assert!(back.snapshot.bit_eq(&empty), "{}: snapshot", d.name);
        assert_eq!(back.edits, d.edits, "{}: edit log", d.name);
        assert!(back.doc.bit_eq(&expected), "{}: replayed document", d.name);
        assert_eq!(
            save(&back.snapshot, &back.edits, Tol::witness()).expect("re-saves"),
            text,
            "{}: save bytes not canonical",
            d.name
        );
        assert!(
            back.doc
                .order()
                .iter()
                .any(|&id| matches!(back.doc.node(id), Some(Node::Shell { .. }))),
            "{}: the round-tripped recipe carries no shell",
            d.name
        );
    }
}
