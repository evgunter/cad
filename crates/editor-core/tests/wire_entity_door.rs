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
//! author reads. The SOURCE rules behind them — `EntityKey::kind` has
//! one call site, and every `NodeErrorKind` variant carrying a
//! `found: EntityKind` is built by handing the door a refusal — are
//! guarded by [`source_rules`] below.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::measure::{MeasureExpr, MeasurePrimitive};
use editor_core::{
    CancelToken, CapEnd, Datum, EntityKind, EvalOptions, Node, NodeErrorKind, NodeResult,
    ProfileDoc, ProfileVertexRef, RecipeNodeId, RoleSeg, SitedRef, StableName, evaluate,
};
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

/// **The source rules behind the four sentences above.**
///
/// The document rows say the refusals are right today. These say they
/// stay right by construction: there is one place that answers what an
/// entity IS, and every refusal that carries the answer takes it from
/// there, in a position these rows check rather than describe.
///
/// **Every row here is an EQUALITY or a count of one, never a floor.**
/// A floor over a hand-written roster is the same defect one level up
/// — it cannot see one arm of its own scan go to zero — so each row
/// names the two sets it equates and asserts EACH derived set
/// non-empty on its own. `crates/test-utils/tests/reader_census.rs`
/// states the rule this module follows.
///
/// **The two sets are derived from different files by different
/// needles**, and that is load-bearing rather than incidental. A
/// census whose second set is built by looking for the members of its
/// first passes at the lower count whenever a member leaves the first:
/// re-spelling one carrier's field type drops it from both sides at
/// once and the equality still holds, with that carrier covered by
/// nothing. Here the declarations come from `eval/mod.rs`'s enum body
/// and the constructions from the argument lists of `eval/wire.rs`'s
/// own door calls, so a carrier that leaves one side stays in the
/// other and reds.
///
/// # What these rows cannot see
///
/// Stated so the receipt is honest, and the first line is the general
/// one: **a census that finds its sites by the spelling it is
/// normalising can only find the ones that already comply.** These
/// rows key on `NodeErrorKind` variants with a `found:` field, so the
/// class they cannot enumerate is exactly the class worth finding.
/// Known members, none of which is this module's subject:
///
/// - **`DeclareUnsupportedPair`**, in the very enum walked below and
///   built in the very file scanned below (`route_declarations`). It
///   carries `kinds: (EntityKind, EntityKind)` rather than a `found:`,
///   and it reads those kinds off the authored `StableName`s rather
///   than off the resolved keys. `work/wire/the-declared-pair-refusal-
///   reads-the-authored-kind.md` is the row.
/// - **A kind refusal on another error type**: `names::interrogate`'s
///   `kind_mismatch` (`InterrogateError`), `assembly.rs`'s
///   `RefusedRef::NotAFace`, `clearance.rs`'s
///   `SelectionRefusal::NotAFace`, `mate/member.rs`'s recipe road.
///   `work/wire/the-entity-kind-door-has-six-spellings.md` enumerates
///   them with their dispositions.
/// - **A refusal built in a test**, here or in another crate: the walk
///   below stops at `eval/wire.rs`'s own `#[cfg(test)]` module and
///   reads no other file.
/// - **A kind answered without saying `kind`**: a hand-written `match`
///   over `EntityKey` inside a `refuse` closure binds its own word and
///   would pass the form test below.
mod source_rules {
    use test_utils::source;
    use test_utils::source::{boundary_before, line};

    const MOD: &str = include_str!("../src/eval/mod.rs");
    const WIRE: &str = include_str!("../src/eval/wire.rs");

    /// `eval/wire.rs`'s code, with its inline `#[cfg(test)]` module
    /// cut off.
    ///
    /// The subject is the SHIPPED doors. An inline test that builds one
    /// of these refusals by hand is a test fixture, and counting it
    /// would red the equality below with a message describing a
    /// production defect that is not there. Offsets survive the cut
    /// because it only truncates.
    fn wire_code() -> String {
        let code = source::blanked(source::code_only, "eval/wire.rs", WIRE);
        match code.find("#[cfg(test)]") {
            Some(at) => code[..at].to_string(),
            None => code,
        }
    }

    /// The byte range of the entity door, located once.
    fn door() -> std::ops::Range<usize> {
        source::sentinel_region(WIRE, "eval/wire.rs", "ENTITY-DOOR BEGIN", "ENTITY-DOOR END")
    }

    /// **Every `fn` declared inside the entity door**, derived rather
    /// than listed: a door added between the sentinels is measured the
    /// moment it is typed, and a door renamed cannot fall out of a
    /// roster that does not exist.
    fn doors(code: &str) -> Vec<&str> {
        let door = door();
        let mut out = Vec::new();
        for (at, _) in code[door.clone()].match_indices("fn ") {
            let at = door.start + at;
            if !boundary_before(code, at) {
                continue;
            }
            let head = at + "fn ".len();
            let end = head
                + code[head..]
                    .find(|c: char| !c.is_alphanumeric() && c != '_')
                    .expect("a fn name ends");
            out.push(&code[head..end]);
        }
        out
    }

    /// **The argument list of every CALL to a door**, as byte ranges —
    /// where a road's own `refuse` constructor is allowed to be
    /// written, and the only place it is.
    ///
    /// Derived from [`doors`], so it follows a rename and covers a new
    /// door without being told.
    fn door_calls(code: &str) -> Vec<std::ops::Range<usize>> {
        let mut out = Vec::new();
        for name in doors(code) {
            for (at, _) in code.match_indices(&format!("{name}(")) {
                if !boundary_before(code, at) || code[..at].trim_end().ends_with("fn") {
                    continue; // the declaration itself
                }
                let open = at + name.len();
                let close = source::balanced_end(code, open).expect("the call's paren closes");
                out.push(open + 1..close);
            }
        }
        out
    }

    /// **Every `NodeErrorKind` variant that carries an entity kind**,
    /// derived from `eval/mod.rs`'s own declaration.
    ///
    /// The field's TYPE is matched by its tail (`EntityKind`), not by
    /// one path spelling: the enum already mixes `crate::names::`-
    /// qualified fields with imported ones, so keying on the long form
    /// would let a carrier leave this set under a re-spelling that
    /// changes nothing.
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
        for text in body.lines() {
            if let Some(head) = text.strip_prefix("    ")
                && head.starts_with(|c: char| c.is_ascii_uppercase())
                && let Some(name) = head
                    .split(|c: char| !c.is_alphanumeric() && c != '_')
                    .next()
            {
                variant = Some(name);
            }
            if text
                .trim()
                .strip_prefix("found:")
                .is_some_and(|ty| ty.trim().trim_end_matches(',').ends_with("EntityKind"))
            {
                out.push(variant.expect("a field sits inside a variant"));
            }
        }
        out
    }

    /// One brace-form `NodeErrorKind::<Variant> { … }` in a view: the
    /// variant, where it starts, and its fields.
    struct Built<'c> {
        variant: &'c str,
        at: usize,
        fields: &'c str,
    }

    /// Every brace-form `NodeErrorKind::<Variant> { … }` inside
    /// `within`, whatever the variant.
    ///
    /// The qualifier is REQUIRED here, which is the one narrowing this
    /// walk makes: it is what tells a refusal apart from any other
    /// braced path expression in an argument list, and `eval/wire.rs`
    /// writes every one of these qualified.
    fn built_in<'c>(code: &'c str, within: std::ops::Range<usize>) -> Vec<Built<'c>> {
        let mut out = Vec::new();
        for (rel, _) in code[within.clone()].match_indices("NodeErrorKind::") {
            let at = within.start + rel;
            if !boundary_before(code, at) {
                continue;
            }
            let head = at + "NodeErrorKind::".len();
            let Some(len) = code[head..].find(|c: char| !c.is_alphanumeric() && c != '_') else {
                continue;
            };
            let rest = &code[head + len..];
            let pad = rest.len() - rest.trim_start().len();
            if !rest[pad..].starts_with('{') {
                continue; // a tuple variant, a match arm, a type position
            }
            let open = head + len + pad;
            let close = source::balanced_end(code, open).expect("the brace closes");
            out.push(Built {
                variant: &code[head..head + len],
                at,
                fields: &code[open + 1..close],
            });
        }
        out
    }

    /// **The parameter list of the closure whose body is the
    /// construction at `at`**, or `None` if the construction is not a
    /// closure's whole body.
    ///
    /// This is what tells a refusal that was HANDED its word from one
    /// that merely spells a binding of that name. `found` being the
    /// bare identifier `found` is not enough on its own: a road can
    /// write `let found = /* its own answer */;` above the call and
    /// pass a closure that ignores the parameter, and the refusal then
    /// reads identically while answering something else. Requiring
    /// `found` to be one of the CLOSURE's parameters is the rule the
    /// door actually rests on.
    ///
    /// The walk is backwards over whitespace to a closing `|`, then
    /// back to its opener. It deliberately does not try to pair `|`s
    /// across a whole argument list: the construction must be the
    /// closure's entire body, which is both the shape every road here
    /// uses and the shape that leaves no room for a statement between
    /// the binding and the refusal.
    fn refuse_params(code: &str, at: usize) -> Option<&str> {
        let head = code[..at].trim_end();
        let close = head.strip_suffix('|')?.len();
        let open = code[..close].rfind('|')?;
        Some(&code[open + 1..close])
    }

    /// **`EntityKey::kind` is called in exactly one place in
    /// `eval/wire.rs`, and that place is inside the entity door.**
    ///
    /// That call IS `found:`. A second one is a second site deciding
    /// what an entity is, which is the shape this unit removed — three
    /// copies of `found: ent.key.kind()`, one per road.
    ///
    /// **The needle is `kind(` at a name boundary**, which catches the
    /// method form and the UFCS form alike and does NOT catch
    /// `carrier_kind(` or `kind_name(`. It is deliberately wider than
    /// `EntityKey::kind`: a textual reader cannot see a receiver's
    /// type, and the safe direction is to red on a `kind()` that turns
    /// out to be someone else's — the message says so, so a reader
    /// meeting that case is not told a falsehood about their code.
    #[test]
    fn the_entity_kind_is_read_in_one_place() {
        let code = wire_code();
        let door = door();
        let mut found = Vec::new();
        for (at, _) in code.match_indices("kind(") {
            if !boundary_before(&code, at) {
                continue;
            }
            assert!(
                door.contains(&at),
                "eval/wire.rs line {}: a `kind()` is called outside the entity door. The one \
                 this file needs is `EntityKey::kind`, which answers `found:` and is the \
                 door's to answer, not a road's; if this call is some other `kind()`, it \
                 wants a name that does not read as that question",
                line(WIRE, at)
            );
            found.push(at);
        }
        assert_eq!(
            found.len(),
            1,
            "`eval/wire.rs` calls `kind()` {} times, not once (lines {:?})",
            found.len(),
            found.iter().map(|a| line(WIRE, *a)).collect::<Vec<_>>()
        );
    }

    /// **Every entity-kind refusal is written in a door call's
    /// arguments, exactly once, and its `found` is a binding it was
    /// HANDED.**
    ///
    /// The two sets, equated by variant name and derived
    /// independently:
    ///
    /// - **declared** — `eval/mod.rs`'s `NodeErrorKind` variants with a
    ///   `found: …EntityKind` field ([`carriers`]).
    /// - **built** — the `NodeErrorKind::<Variant> { … }` expressions
    ///   written inside the argument list of a call to a door
    ///   ([`door_calls`], [`built_in`]). This side never consults the
    ///   other, so a carrier that leaves `declared` stays here and the
    ///   equality reds.
    ///
    /// Three failures a lane can actually make, all of them live:
    ///
    /// - a fifth carrier declared and refused somewhere else — in
    ///   `built` it is absent, in `declared` it is not;
    /// - a road that built its refusal outside a door call, by
    ///   computing its own answer first — absent from `built`, and the
    ///   POSITION check below names the line;
    /// - a variant with no `found: …EntityKind` handed to a door as a
    ///   refusal — present in `built`, absent from `declared`.
    ///
    /// **The `found` field must be the binding the door HANDED it**,
    /// and that is two checks rather than one, because the weaker of
    /// them is a proxy. A construction that computes its own word
    /// (`found: EntityKind::Face`) fails the first. A road that writes
    /// `let found = …;` above the call and passes a closure ignoring
    /// the parameter passes the first and reads identically — so the
    /// second requires the construction to be the whole body of a
    /// closure whose PARAMETERS include `found` ([`refuse_params`]).
    #[test]
    fn every_entity_kind_refusal_takes_found_from_the_door() {
        let mod_code = source::blanked(source::code_only, "eval/mod.rs", MOD);
        let mut declared = carriers(&mod_code);
        assert!(
            !declared.is_empty(),
            "`eval/mod.rs` declares no `found: …EntityKind` field at all — the enum moved \
             and this row is reading the wrong file"
        );
        let code = wire_code();
        let calls = door_calls(&code);
        assert!(
            !calls.is_empty(),
            "`eval/wire.rs` calls no entity door at all — the sentinels or the scan have \
             drifted from the doors they read, and every refusal below would look misplaced"
        );
        let mut built: Vec<&str> = Vec::new();
        for call in &calls {
            for b in built_in(&code, call.clone()) {
                let found = source::top_level_split(b.fields, ',')
                    .into_iter()
                    .map(|r| b.fields[r].trim())
                    .find(|f| f.split(':').next().is_some_and(|n| n.trim() == "found"));
                let Some(found) = found else { continue };
                assert!(
                    found == "found"
                        || found.split(':').nth(1).is_some_and(|v| v.trim() == "found"),
                    "eval/wire.rs line {}: `{}` sets `found` to `{found}` — the entity door \
                     answers what was found and a road takes the word it is handed",
                    line(WIRE, b.at),
                    b.variant
                );
                // …and the binding it spells must be the one the door
                // HANDED it, not a local of the same name.
                let params = refuse_params(&code, b.at);
                assert!(
                    params.is_some_and(|p| p.split(',').any(|t| t.trim() == "found")),
                    "eval/wire.rs line {}: `{}` is not the body of a closure that binds \
                     `found` (its head is `{}`) — a road that spells `found` without being \
                     handed it has answered the door's question itself",
                    line(WIRE, b.at),
                    b.variant,
                    params.unwrap_or("<not a closure body>").trim()
                );
                built.push(b.variant);
            }
        }
        assert!(
            !built.is_empty(),
            "no entity-kind refusal is written in a door call's arguments — the walk read \
             nothing and would pass over every carrier `eval/mod.rs` declares"
        );
        // The POSITION rule, in the direction the set equality cannot
        // see: a carrier built anywhere else in this file at all.
        for name in &declared {
            for b in built_in(&code, 0..code.len()) {
                assert!(
                    b.variant != *name || calls.iter().any(|c| c.contains(&b.at)),
                    "eval/wire.rs line {}: `{name}` is built outside any entity-door call — \
                     a road that writes its own refusal has somewhere to put its own answer \
                     to `found:`, which is the whole shape this door removed",
                    line(WIRE, b.at)
                );
            }
        }
        declared.sort_unstable();
        built.sort_unstable();
        assert_eq!(
            declared, built,
            "the `NodeErrorKind` variants carrying a `found: …EntityKind` and the refusals \
             written in an entity-door call's arguments are the same multiset; the list \
             above is `eval/mod.rs`'s declarations, the list below `eval/wire.rs`'s door \
             calls"
        );
    }
}
