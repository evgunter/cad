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
//! cannot both pass against a hand-written word. The two words the
//! measure road used to spell by hand were both CORRECT — these rows
//! pin the rule, they do not repair a bug.
//!
//! These refusals are DOCUMENT-REACHABLE: the strings here are what an
//! author reads, and they are what a door answering a constant kind
//! breaks. The rule BEHIND them — that the kind is the door's answer
//! and not a road's — is carried by the type system rather than by a
//! census: `eval::entity_door::Found` is mintable only inside the
//! door. [`source_rules`] below guards the one thing left over, which
//! is placement.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::measure::{MeasureExpr, MeasurePrimitive};
use editor_core::{
    CancelToken, CapEnd, Datum, EntityKind, EvalOptions, Node, NodeErrorKind, NodeResult,
    ProfileDoc, ProfileVertexRef, RecipeNodeId, SitedRef, StableName, evaluate,
};
use fixture::{ang, fname, insert, len, on_frame, square, wall};
use geom_core::Tol;

/// A vertex name at `node` — the extrude's own END cap vertex on the
/// document's one outer loop, so the name RESOLVES and the refusal is
/// about its kind rather than about a name that names nothing.
fn end_cap_vertex(node: RecipeNodeId, vertex: u32) -> StableName {
    fixture::cap_vertex(
        node,
        CapEnd::End,
        ProfileVertexRef {
            loop_index: 0,
            vertex,
        },
    )
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
    let vertex = end_cap_vertex(body, 0);
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
/// hand-written word CAN get wrong — this road's two were right, and
/// what changed is that a word is no longer written at all.
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
        let name = if select == EntityKind::Edge {
            edge
        } else {
            vertex
        };
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

/// **The one source rule the compiler does not already carry.**
///
/// The rule this unit is about — *the kind in a refusal is the door's
/// answer, never a road's* — **is not guarded here, because it is not
/// guarded by text at all.** `eval::entity_door::Found` has a private
/// field and `entity` is the only thing that can mint one, so a road
/// that wanted to answer the question itself has nothing to write. An
/// earlier draft of this module policed that rule with a census over
/// closure parameters; a review defeated it three ways in one sitting
/// (an inner closure, an IIFE, a `let`-bound closure), which is the
/// argument for the type and against the census. The door's own module
/// doc states the mechanism.
///
/// What is left for text is one placement claim the compiler has no
/// opinion about: **every refusal that answers *"what was it instead"*
/// is built in `eval/wire.rs`, exactly once, and by one of the two
/// doors.** It is worth an assertion because those refusals are the
/// sentences a user reads: an answer built somewhere else, or by
/// neither door, is another vocabulary for one question — the shape
/// `work/wire/the-entity-kind-door-has-six-spellings.md` was opened
/// about. It covers the VALUE door's answer too, because the two are
/// one question over two subjects and a row that saw only one of them
/// would report a clean file while the other drifted.
///
/// **The two sets are derived from different files by different
/// needles**, and that is load-bearing. A census whose second set is
/// built by looking for the members of its first passes at the lower
/// count whenever a member leaves the first — this one's earlier draft
/// did exactly that, and a re-spelled field type hid a carrier from
/// both sides at once. Here the declarations come from `eval/mod.rs`'s
/// enum body and the constructions from a walk of `eval/wire.rs` that
/// never consults them, so a carrier that leaves one side stays in the
/// other and reds.
///
/// # What this row cannot see
///
/// The general form first, because it is the sharp one: **a census
/// that finds its sites by the spelling it is normalising can only
/// find the ones that already comply.** This row keys on
/// `found: …Found`, so:
///
/// - **`DeclareUnsupportedPair`**, in the very enum walked below and
///   built in the very file scanned below (`resolve_declarations`),
///   carries `kinds: (EntityKind, EntityKind)` rather than a `found:`
///   — and reads them off the authored `StableName`s. That is the
///   DECIDED answer there, not a residue: the refusal is raised
///   before either name is resolved, because a pair the vocabulary
///   has no step for is unsupported however many entities answer to
///   either name, so no key exists yet to read the word off. The name
///   table makes the two sources agree (`insert_ref` and
///   `insert_tied_ref` are its only writers and both refuse a row
///   whose name's kind is not its key's); the one place they could
///   differ is a broken table, which that door answers off the KEYS
///   under a `debug_assert!`. What this row still cannot see is the
///   site at all.
/// - **A kind refusal on another error type**: `names::interrogate`'s
///   `kind_mismatch`, `clearance.rs`'s `SelectionRefusal::NotAFace`,
///   `names::role`'s `NotAFaceName`, `mate/member.rs`'s recipe road.
///   Rows and dispositions on
///   `work/wire/the-entity-kind-door-has-six-spellings.md`.
/// - **A refusal built in a test**: the walk stops at `eval/wire.rs`'s
///   own inline `#[cfg(test)] mod` and reads no other file.
/// - **A road that renames the type it imports**: `use … as Kind`
///   leaves a field this row cannot recognise as a carrier. That
///   direction fails RED (the carrier leaves `declared` while its
///   construction stays in `built`), which is why it is a limit rather
///   than a hole.
mod source_rules {
    use test_utils::source;
    use test_utils::source::{boundary_before, line};

    const MOD: &str = include_str!("../src/eval/mod.rs");
    const WIRE: &str = include_str!("../src/eval/wire.rs");

    /// `eval/wire.rs`'s code, with its inline test module cut off.
    ///
    /// The subject is the SHIPPED roads. The cut is at the
    /// `#[cfg(test)]` that introduces a `mod` — **not at the first
    /// `#[cfg(test)]` of any kind**, which would let an attribute on a
    /// helper `fn` or a `use` silently shorten the scan and take real
    /// roads with it. Offsets survive because it only truncates.
    fn wire_code() -> String {
        let code = source::blanked(source::code_only, "eval/wire.rs", WIRE);
        let cut = code.match_indices("#[cfg(test)]").find(|(at, _)| {
            code[at + "#[cfg(test)]".len()..]
                .trim_start()
                .starts_with("mod ")
        });
        match cut {
            Some((at, _)) => code[..at].to_string(),
            None => code,
        }
    }

    /// **Every `NodeErrorKind` variant, and the `found:` field type of
    /// those that have one** — derived from `eval/mod.rs`'s own enum
    /// body.
    ///
    /// Both come off one walk because the second set below needs the
    /// roster: a construction is recognised by its variant NAME, and
    /// that roster has to come from somewhere other than a
    /// hand-written list.
    ///
    /// The `found:` field is recorded with its TYPE TAIL, which is
    /// what tells the two doors apart — the entity door's token
    /// (`Found`) from the value door's word (`&'static str`) — without
    /// naming either variant.
    fn variants(code: &str) -> (Vec<&str>, Vec<(&str, &str)>) {
        let head = code
            .find("pub enum NodeErrorKind")
            .expect("`NodeErrorKind` is declared");
        let open = head + code[head..].find('{').expect("the enum has a body");
        let close = source::balanced_end(code, open).expect("the enum body closes");
        let body = &code[open + 1..close];
        let (mut all, mut answers) = (Vec::new(), Vec::new());
        let mut variant: Option<&str> = None;
        for text in body.lines() {
            if let Some(head) = text.strip_prefix("    ")
                && head.starts_with(|c: char| c.is_ascii_uppercase())
                && let Some(name) = head
                    .split(|c: char| !c.is_alphanumeric() && c != '_')
                    .next()
            {
                variant = Some(name);
                all.push(name);
            }
            if let Some(ty) = text.trim().strip_prefix("found:") {
                let ty = ty.trim().trim_end_matches(',');
                let tail = ty
                    .rsplit(|c: char| !c.is_alphanumeric() && c != '_')
                    .next()
                    .unwrap_or(ty);
                answers.push((variant.expect("a field sits inside a variant"), tail));
            }
        }
        assert!(
            !all.is_empty(),
            "eval/mod.rs: `NodeErrorKind`'s body yielded no variant at all — the enum moved \
             or the scan drifted from its layout"
        );
        assert!(
            !answers.is_empty(),
            "eval/mod.rs: `NodeErrorKind` declares no `found:` field at all — the refusal \
             vocabulary moved or the scan drifted from its layout"
        );
        (all, answers)
    }

    /// **Every refusal that answers *"what was it instead"* is built in
    /// `eval/wire.rs`, exactly once, and by one of the two doors.**
    ///
    /// The two sets, equated by variant name as a multiset and derived
    /// from different files by different needles:
    ///
    /// - **declared** — the variants `eval/mod.rs` gives a `found:`
    ///   field, whatever its type.
    /// - **built** — the brace-form constructions found by walking
    ///   `eval/wire.rs` for EVERY variant name the enum declares, and
    ///   keeping the ones whose field list sets `found`. This side is
    ///   not steered by the declarations, so a carrier that leaves them
    ///   stays here and reds.
    ///
    /// The variant token is matched BARE, so a `use NodeErrorKind as
    /// NEK;` or a braced import does not walk past this — the same
    /// narrowing `wire_operand_door.rs`'s construction census makes,
    /// for the same reason, rather than the opposite one.
    ///
    /// **And each construction must belong to one of the two doors**,
    /// told apart by the declared type rather than by name: the entity
    /// door's `Found` token, or the value door, whose refusal is
    /// written inside the `OPERAND-DOOR` sentinels and whose own rules
    /// are `wire_operand_door.rs`'s. A third `found:` answer, on a
    /// third vocabulary with no census, reds here — and so does a
    /// carrier whose token type is re-spelled out of recognition
    /// (`use … as Kind`), which is the direction that keeps this row
    /// from passing over a carrier it has stopped recognising.
    ///
    /// Three failures a lane can make, all live: a carrier refused in
    /// another module (in `declared`, not in `built`); a road that
    /// stopped refusing (the converse); an answer built by neither
    /// door.
    #[test]
    fn every_found_answer_is_built_by_a_door() {
        let mod_code = source::blanked(source::code_only, "eval/mod.rs", MOD);
        let (all, answers) = variants(&mod_code);
        let mut declared: Vec<&str> = answers.iter().map(|(n, _)| *n).collect();
        let code = wire_code();
        let operand = source::sentinel_region(
            WIRE,
            "eval/wire.rs",
            "OPERAND-DOOR BEGIN",
            "OPERAND-DOOR END",
        );
        let mut built: Vec<&str> = Vec::new();
        for name in &all {
            for (at, _) in code.match_indices(name) {
                if !boundary_before(&code, at) {
                    continue;
                }
                let rest = &code[at + name.len()..];
                let pad = rest.len() - rest.trim_start().len();
                if !rest[pad..].starts_with('{') {
                    continue; // a tuple variant, a match arm, a type position
                }
                let open = at + name.len() + pad;
                let close = source::balanced_end(&code, open).expect("the brace closes");
                let fields = &code[open + 1..close];
                let sets_found = source::top_level_split(fields, ',').into_iter().any(|r| {
                    fields[r]
                        .split(':')
                        .next()
                        .is_some_and(|n| n.trim() == "found")
                });
                if !sets_found {
                    continue;
                }
                let token = answers.iter().find(|(n, _)| n == name).map(|(_, t)| *t);
                assert!(
                    token == Some("Found") || operand.contains(&at),
                    "eval/wire.rs line {}: `{name}` answers `found` as `{}`, and is written \
                     outside the operand door — an answer to what an input WAS comes from \
                     one of two doors, the entity door's unforgeable token or the value \
                     door's own home, and a third one is a vocabulary with no census",
                    line(WIRE, at),
                    token.unwrap_or("(undeclared)")
                );
                built.push(name);
            }
        }
        declared.sort_unstable();
        built.sort_unstable();
        assert_eq!(
            declared, built,
            "the `NodeErrorKind` variants declaring a `found:` and the refusals built in \
             `eval/wire.rs` are the same multiset; the list above is `eval/mod.rs`'s \
             declarations, the list below `eval/wire.rs`'s constructions"
        );
    }
}
