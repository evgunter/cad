//! **Every entity-kind refusal `eval::wire` can put in front of an
//! author, byte-exact, plus the source rules behind them.**
//!
//! The question is one question — *read a name, test what kind of
//! entity it denotes, refuse* — and the crate used to answer it in
//! three copies of five lines plus a fourth spelling over a selection.
//! It is one door now (`entity`, and `named_entity` for the roads that
//! resolve an authored name first), and what the door owns is the half
//! a caller must not write: `found:`, the kind the entity ACTUALLY
//! has.
//!
//! Four roads over three found kinds, so two independent things are
//! pinned and a door that lost either goes red:
//!
//! - **The sentence varies with the ROAD.** A shell designation, a
//!   blend's selection under its verb, a derived frame's face and a
//!   measure's scope are four refusals with four identities, and a door
//!   that flattened them into one fails every row but one.
//! - **`found:` varies with the ENTITY, under a fixed road.** The two
//!   shell rows differ only in what was designated, and so do the two
//!   measure rows — so a door that answered a constant, or the negation
//!   of its own wanted kind (*"denotes not a face"*), fails while the
//!   road's own sentence beside it stays right.
//!
//! The article is part of that: it agrees with the kind ("an edge", "a
//! face"), so a row that found an edge and a row that found a vertex
//! cannot both pass against a hand-written word.
//!
//! These refusals are DOCUMENT-REACHABLE: the strings here are what an
//! author reads. The SOURCE rules behind them — `EntityKey::kind` has
//! one call site, and every `NodeErrorKind` variant carrying a
//! `found: EntityKind` is built by handing the door a refusal — are
//! guarded by [`source_rules`] below.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, CapEnd, Datum, EntityKind, EvalOptions, Node, NodeErrorKind, NodeResult,
    ProfileDoc, ProfileVertexRef, RecipeNodeId, RoleSeg, SitedRef, StableName, evaluate,
};
use editor_core::measure::{MeasureExpr, MeasurePrimitive};
use fixture::{ang, fname, insert, len, on_frame, square, wall};
use geom_core::Tol;

/// A vertex name at `node` — the extrude's own cap vertex, so the name
/// RESOLVES and the refusal is about its kind rather than about a name
/// that names nothing.
fn vname(node: RecipeNodeId, vertex: u32) -> StableName {
    StableName {
        kind: EntityKind::Vertex,
        node,
        path: vec![RoleSeg::CapVertex(
            CapEnd::End,
            ProfileVertexRef {
                loop_index: 0,
                vertex,
            },
        )],
    }
}

/// A square prism and the four names every row below miswires with: a
/// face of it, one of its lateral edges, one of its cap vertices.
fn solid() -> (ProfileDoc, RecipeNodeId, StableName, StableName, StableName) {
    let doc = ProfileDoc::empty_derived("wire_entity_door", Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 1.0)],
    );
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let face = fname(body, wall(2));
    let edge = fixture::prism_edges(body, 4).remove(2);
    let vertex = vname(body, 0);
    (doc, body, face, edge, vertex)
}

/// The refusal `node` evaluates to, rendered.
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

/// **The shell's open designation names a FACE.** Two rows under one
/// sentence: the road is fixed and what was designated is not, so the
/// pair fails any door that stopped reading the key it was handed.
#[test]
fn a_shell_designation_of_another_kind_refuses_naming_what_it_found() {
    for (what, designate, want) in [
        (
            "an edge",
            EntityKind::Edge,
            "the shell open-face name minted by node 2 denotes an edge, not a face",
        ),
        (
            "a vertex",
            EntityKind::Vertex,
            "the shell open-face name minted by node 2 denotes a vertex, not a face",
        ),
    ] {
        let (doc, body, _, edge, vertex) = solid();
        let name = if designate == EntityKind::Edge {
            edge
        } else {
            vertex
        };
        let (doc, shell) = insert(doc, Node::shell(body, len(0.1), vec![name]));
        let got = refusal(&doc, shell);
        assert!(
            matches!(got, NodeErrorKind::ShellOpenKind { .. }),
            "{what}: the shell's own refusal, not another road's: {got:?}"
        );
        assert_eq!(got.to_string(), want, "{what}");
    }
}

/// **A blend's selection names an EDGE**, and its refusal carries the
/// verb — the identity a door that flattened the three refusals into
/// one would have dropped.
#[test]
fn a_blend_selection_of_another_kind_refuses_under_its_verb() {
    for (what, node, want) in [
        (
            "fillet",
            Node::fillet as fn(RecipeNodeId, editor_core::Expr, Vec<StableName>) -> _,
            "the fillet selection name minted by node 2 denotes a face, not an edge",
        ),
        (
            "chamfer",
            Node::chamfer as fn(RecipeNodeId, editor_core::Expr, Vec<StableName>) -> _,
            "the chamfer selection name minted by node 2 denotes a face, not an edge",
        ),
    ] {
        let (doc, body, face, _, _) = solid();
        let (doc, blend) = insert(doc, node(body, len(0.1), vec![face]));
        let got = refusal(&doc, blend);
        assert!(
            matches!(got, NodeErrorKind::BlendSelectionKind { .. }),
            "{what}: the blend's own refusal: {got:?}"
        );
        assert_eq!(got.to_string(), want, "{what}");
    }
}

/// **A derived frame's face name names a FACE** — the same wanted kind
/// as the shell's row and a different sentence, which is what a door
/// that answered one refusal for both would lose.
#[test]
fn a_derived_frame_named_on_another_kind_refuses_in_its_own_words() {
    let (doc, _, _, edge, _) = solid();
    let (doc, frame) = insert(
        doc,
        Node::Datum(Datum::FaceFrame {
            at: RecipeNodeId(2),
            face: edge,
            spin: ang(0.0),
        }),
    );
    let got = refusal(&doc, frame);
    assert!(
        matches!(got, NodeErrorKind::FaceFrameKind { .. }),
        "the frame's own refusal: {got:?}"
    );
    assert_eq!(
        got.to_string(),
        "the derived frame's name minted by node 2 denotes an edge, not a face"
    );
}

/// **A measure's reference is a SCOPE** — a whole body or one of its
/// faces. The one road whose refusal names no designation, so it takes
/// the inner door directly; two rows, because the article is the half a
/// hand-written word gets wrong.
#[test]
fn a_measure_reference_that_is_no_scope_refuses_naming_what_it_found() {
    for (what, select, want) in [
        (
            "an edge",
            EntityKind::Edge,
            "`min_clearance` measures between two selections — a whole body or one of its \
             faces — and this reference resolves to an edge",
        ),
        (
            "a vertex",
            EntityKind::Vertex,
            "`min_clearance` measures between two selections — a whole body or one of its \
             faces — and this reference resolves to a vertex",
        ),
    ] {
        let (doc, _, face, edge, vertex) = solid();
        let name = if select == EntityKind::Edge { edge } else { vertex };
        let (doc, measure) = insert(
            doc,
            Node::measure(
                MeasureExpr::primitive(MeasurePrimitive::MinClearance { a: 0, b: 1 }),
                vec![SitedRef::at_mint(name), SitedRef::at_mint(face)],
            )
            .expect("both indices in range"),
        );
        let got = refusal(&doc, measure);
        assert!(
            matches!(got, NodeErrorKind::MeasureSelectionKind { .. }),
            "{what}: the measure's own refusal: {got:?}"
        );
        assert_eq!(got.to_string(), want, "{what}");
    }
}

/// **The source rules behind the four sentences above.**
///
/// The document rows say the refusals are right today. These say they
/// stay right by construction: there is one place that answers what an
/// entity IS, and every refusal that carries the answer takes it from
/// there.
///
/// **Every row here is an EQUALITY or a count of one, never a floor.**
/// A floor over a hand-written roster is the same defect one level up
/// — it cannot see one arm of its own scan go to zero — so each row
/// names the two sets it equates and asserts its own derived set is
/// non-empty. `crates/test-utils/tests/reader_census.rs` states the
/// rule this module follows.
///
/// **What these rows cannot see**, stated so the receipt is honest:
/// a kind test written against a type that is not `NodeErrorKind` (the
/// crate has three — `names::interrogate`'s `kind_mismatch`,
/// `stackup.rs` and `mate/member.rs`, all enumerated on
/// `work/wire/the-entity-kind-door-has-six-spellings.md`), a refusal
/// built in a TEST rather than in `src/`, and a kind answered through
/// something other than `EntityKey::kind` — a hand-written `match` over
/// `EntityKey` in a `refuse` closure would be read as a forwarded
/// binding by the second row and caught only by the first if it spelled
/// `.kind()`.
mod source_rules {
    use test_utils::source;

    const MOD: &str = include_str!("../src/eval/mod.rs");
    const WIRE: &str = include_str!("../src/eval/wire.rs");

    /// The line `at` is on, for a failure a reader can open.
    fn line(text: &str, at: usize) -> usize {
        text[..at].lines().count()
    }

    /// Is the byte before `at` part of an identifier? Used to make a
    /// name match a whole name.
    fn boundary_before(code: &str, at: usize) -> bool {
        code[..at]
            .chars()
            .next_back()
            .is_none_or(|c| !c.is_alphanumeric() && c != '_')
    }

    /// The byte range of the entity door, located once.
    fn door() -> std::ops::Range<usize> {
        source::sentinel_region(WIRE, "eval/wire.rs", "ENTITY-DOOR BEGIN", "ENTITY-DOOR END")
    }

    /// **Every `NodeErrorKind` variant that carries an entity kind**,
    /// derived from `eval/mod.rs`'s own declaration rather than listed
    /// here: a fifth such refusal is measured the moment it is
    /// declared, and one renamed cannot fall out of a roster that does
    /// not exist.
    ///
    /// The walk is over the enum's own body, so a `found: EntityKind`
    /// on some other type is not read as one of these.
    fn carriers(code: &str) -> Vec<&str> {
        let head = code
            .find("pub enum NodeErrorKind")
            .expect("`NodeErrorKind` is declared");
        let open = head + code[head..].find('{').expect("the enum has a body");
        let close = source::balanced_end(code, open).expect("the enum body closes");
        let body = &code[open + 1..close];
        // The enclosing VARIANT is the last line at the enum's own
        // indentation that opens a field list; a field's line is
        // indented one level further, so the two cannot be confused.
        let mut out = Vec::new();
        let mut variant: Option<&str> = None;
        for line in body.lines() {
            if let Some(head) = line.strip_prefix("    ")
                && head.starts_with(|c: char| c.is_ascii_uppercase())
                && let Some(name) = head.split(|c: char| !c.is_alphanumeric() && c != '_').next()
            {
                variant = Some(name);
            }
            if line.contains("found: crate::names::EntityKind") {
                out.push(variant.expect("a field sits inside a variant"));
            }
        }
        out
    }

    /// **`EntityKey::kind` is called in exactly one place in
    /// `eval/wire.rs`, and that place is inside the entity door.**
    ///
    /// That call IS `found:`. A second one is a second site deciding
    /// what an entity is, which is the shape this unit removed — three
    /// copies of `found: ent.key.kind()`, one per road.
    ///
    /// The two sets: every `.kind()` call in the file, against the one
    /// the door makes. `found.len() == 1` is the whole guard — it fails
    /// on a second call anywhere, INCLUDING one inside the sentinels,
    /// which a containment test alone would miss.
    #[test]
    fn the_entity_kind_is_read_in_one_place() {
        let code = source::blanked(source::code_only, "eval/wire.rs", WIRE);
        let door = door();
        let mut found = Vec::new();
        for (at, _) in code.match_indices(".kind()") {
            assert!(
                door.contains(&at),
                "eval/wire.rs line {}: a kind is read outside the entity door — `found:` is \
                 the door's to answer and no road's to write",
                line(WIRE, at)
            );
            found.push(at);
        }
        assert_eq!(
            found.len(),
            1,
            "`eval/wire.rs` reads a kind {} times, not once (lines {:?})",
            found.len(),
            found.iter().map(|a| line(WIRE, *a)).collect::<Vec<_>>()
        );
    }

    /// **Every entity-kind refusal is built exactly once, and its
    /// `found` is a binding it was HANDED.**
    ///
    /// The two sets, equated by variant name: the carriers `eval/mod.rs`
    /// declares ([`carriers`]), and the variants `eval/wire.rs`
    /// constructs. Both directions carry a failure a lane can make — a
    /// fifth carrier declared and refused somewhere else reds, a road
    /// that stopped refusing reds, and a scan that matched nothing reds
    /// rather than passing over an empty pair.
    ///
    /// The `found` field must be the bare binding `found` — the door's
    /// closure parameter. A construction that computed its own word
    /// (`found: EntityKind::Face`, or a `match` over the key) fails the
    /// form test, and that is the rule as stated rather than a proxy
    /// for it: the assertion checks what the message claims.
    #[test]
    fn every_entity_kind_refusal_takes_found_from_the_door() {
        let mod_code = source::blanked(source::code_only, "eval/mod.rs", MOD);
        let mut declared = carriers(&mod_code);
        assert!(
            !declared.is_empty(),
            "`eval/mod.rs` declares no `found: crate::names::EntityKind` field at all — the \
             enum moved and this row is reading the wrong file"
        );
        let code = source::blanked(source::code_only, "eval/wire.rs", WIRE);
        let mut built: Vec<&str> = Vec::new();
        for name in &declared {
            for (at, _) in code.match_indices(name.trim()) {
                if !boundary_before(&code, at) {
                    continue;
                }
                let rest = &code[at + name.len()..];
                let brace = rest.len() - rest.trim_start().len();
                if !rest[brace..].starts_with('{') {
                    continue; // a `use`, a type position, a doc mention
                }
                let open = at + name.len() + brace;
                let close = source::balanced_end(&code, open).expect("the brace closes");
                let fields = &code[open + 1..close];
                let mut fs = source::top_level_split(fields, ',')
                    .into_iter()
                    .map(|r| fields[r].trim());
                let Some(found) = fs.find(|f| f.split(':').next().is_some_and(|n| n.trim() == "found"))
                else {
                    continue; // a pattern binding nothing, not a construction
                };
                assert!(
                    found == "found" || found.split(':').nth(1).is_some_and(|v| v.trim() == "found"),
                    "eval/wire.rs line {}: `{name}` sets `found` to `{found}` — the entity \
                     door answers what was found and a road takes the word it is handed",
                    line(WIRE, at)
                );
                built.push(name);
            }
        }
        declared.sort_unstable();
        built.sort_unstable();
        assert_eq!(
            declared, built,
            "every `NodeErrorKind` variant carrying a `found: EntityKind` is built in \
             `eval/wire.rs` exactly once and every such construction is one of them; the \
             list above is `eval/mod.rs`'s declarations, the list below `eval/wire.rs`'s \
             constructions"
        );
    }
}
