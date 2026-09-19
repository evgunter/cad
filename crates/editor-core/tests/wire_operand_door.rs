//! **Every document-reachable operand refusal, both halves, byte-exact.**
//!
//! Every typed operand mismatch in `eval::wire` is built by one door
//! (`operand`, or `node_operand` for the road that holds no value) out
//! of two independent parts: the `expected:` phrase the door was asked
//! for, and the `found:` family the input actually carries, which the
//! door reads for itself.
//!
//! The document below is the reviewer of PR 2480's — a 20-miswiring
//! instrument written to RENDER `(expected, found, input)` for every
//! such refusal, so the same file could be compiled unchanged on two
//! trees and diffed. Adopted here with its rows intact and its verdict
//! turned into assertions, because a test that prints is evidence for
//! whoever is reading that day and a gate for nobody
//! (`memories/test-suite-cost.md`).
//!
//! Eight distinct `expected:` phrases over four distinct `found:`
//! families, so two independent things are pinned and a door that lost
//! either goes red:
//!
//! - **`expected:` varies with the caller.** A door that hard-coded any
//!   one phrase fails every row that asks for a different one.
//! - **`found:` varies with the value, under a FIXED `expected:`.** The
//!   `"datum frame"` rows differ only in what was wired in, the
//!   `"datum plane"` rows likewise, and the `"profile"` rows do the
//!   same on the node road — so a door that answered a constant, or the
//!   negation of its own `expected:` (*"not a datum frame"*), fails
//!   while the phrase beside it stays right.
//!
//! **Three rows never reach evaluation**, and that is the finding they
//! carry: the edit door refuses an assertion over a non-measure and a
//! declare reference that is not a `Declare`, so `wire_assertion`'s and
//! `declared_pairs`' kind refusals are defences behind a door rather
//! than sentences a document author can read. They are asserted as
//! edit-door refusals, so a door that stopped refusing them — and
//! started shipping those refusals to users — reds here.
//!
//! These refusals are DOCUMENT-REACHABLE: the strings here are what an
//! author reads. The SOURCE rules behind them (one construction site,
//! no phrase literal at a call site) are guarded separately, by
//! `eval::mod`'s `operand_vocabulary_census`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    AssertionDir, Datum, DocEdit, EvalOptions, Expr, Node, NodeErrorKind, PartSelect, PatternKind,
    ProfileDoc, ProfileProgram, RecipeNodeId, SplitHalf, TubeWindow,
};
use fixture::{ang, desc, insert, len, on_frame_keeping, scl, square};
use geom_core::Tol;

/// What a miswired node owes.
enum Owes {
    /// `WrongOperand`, with these two halves and the wired node as its
    /// `input`.
    Refusal(&'static str, &'static str),
    /// The edit door refuses the node, so no evaluation happens and the
    /// operand door behind it is unreachable from a document.
    EditDoor,
}

/// A miswired node: what it is, what it owes, and the operand the
/// refusal must name.
struct Row {
    what: &'static str,
    owes: Owes,
    input: RecipeNodeId,
    /// What the EDIT door did: the minted id, or the refusal.
    ///
    /// `Ok(None)` — an edit that succeeded and minted nothing — is a
    /// third answer, and it is why this is not an `Option`: an
    /// `is_none()` test would read it as "the edit door refused" and
    /// pass over a door that had stopped refusing.
    edit: Result<Option<RecipeNodeId>, String>,
}

/// One document holding every miswiring, plus the well-formed profile,
/// body and pattern the miswirings borrow.
///
/// One document and one evaluation rather than twenty: nextest is
/// process-per-test, so a test per row would pay twenty document builds
/// for one claim, and every row labels itself well enough to read a
/// failure off the message alone.
fn wired() -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    Vec<Row>,
) {
    let doc = ProfileDoc::empty_derived("wire_operand_door", Tol::witness());
    let (doc, sketch, profile) = on_frame_keeping(
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
    let (doc, body2) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(2.0),
        },
    );
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(0.0), len(0.0)],
            normal: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, axis3) = insert(
        doc,
        Node::Datum(Datum::Axis {
            origin: [len(0.0), len(0.0), len(0.0)],
            direction: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    );
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: body,
            count: Expr::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(3.0),
            },
        },
    );

    let mut rows: Vec<Row> = Vec::new();
    // Inserts the miswired node, recording whether the EDIT door took
    // it — which is itself a row's answer, not a reason to stop.
    let add = |d: ProfileDoc,
               rows: &mut Vec<Row>,
               what: &'static str,
               owes: Owes,
               node: Node<ProfileProgram>,
               input: RecipeNodeId|
     -> ProfileDoc {
        match d.apply(
            &DocEdit::InsertNode { node },
            Tol::witness(),
            &editor_core::RefusingReach,
        ) {
            Ok(applied) => {
                rows.push(Row {
                    what,
                    owes,
                    input,
                    edit: Ok(applied.record.minted),
                });
                applied.doc
            }
            Err(e) => {
                rows.push(Row {
                    what,
                    owes,
                    input,
                    edit: Err(format!("{e:?}")),
                });
                d
            }
        }
    };

    // ---- the value road: `found:` is the payload's family ----

    let mut doc = add(
        doc,
        &mut rows,
        "body_operand over a plane datum (Shell)",
        Owes::Refusal("body", "datum"),
        Node::Shell {
            target: plane,
            thickness: len(0.1),
            open: vec![],
        },
        plane,
    );
    doc = add(
        doc,
        &mut rows,
        "body_operand over instances (Shell of a pattern)",
        Owes::Refusal("body", "instances"),
        Node::Shell {
            target: pattern,
            thickness: len(0.1),
            open: vec![],
        },
        pattern,
    );
    doc = add(
        doc,
        &mut rows,
        "placeable_operand over a profile (Pattern)",
        Owes::Refusal("body or instances", "profile"),
        Node::Pattern {
            input: profile,
            count: Expr::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(3.0),
            },
        },
        profile,
    );
    doc = add(
        doc,
        &mut rows,
        "profile_plane_f64 (a profile drawn on a plane datum)",
        Owes::Refusal("datum frame", "datum"),
        Node::Profile(desc(plane, vec![square(0.0, 0.0, 1.0)])),
        plane,
    );
    doc = add(
        doc,
        &mut rows,
        "frame_value (an in-plane axis written against a body)",
        Owes::Refusal("datum frame", "body"),
        fixture::axis_in_plane(body, (0.0, 0.0), (1.0, 0.0)),
        body,
    );
    doc = add(
        doc,
        &mut rows,
        "wire_swept (an Extrude of a plane datum)",
        Owes::Refusal("profile", "datum"),
        Node::Extrude {
            profile: plane,
            distance: len(1.0),
        },
        plane,
    );
    doc = add(
        doc,
        &mut rows,
        "wire_revolve's profile pre-check (a Revolve of a plane datum)",
        Owes::Refusal("profile", "datum"),
        Node::Revolve {
            profile: plane,
            axis: axis3,
            angle: ang(1.0),
        },
        plane,
    );
    doc = add(
        doc,
        &mut rows,
        "wire_revolve's axis (a Revolve about a 3-D axis)",
        Owes::Refusal("an axis in a sketch frame (Datum::AxisInPlane)", "datum"),
        Node::Revolve {
            profile,
            axis: axis3,
            angle: ang(1.0),
        },
        axis3,
    );
    doc = add(
        doc,
        &mut rows,
        "tube_args (a Tube spined on a profile)",
        Owes::Refusal("datum axis", "profile"),
        Node::Tube {
            spine: profile,
            u_ref: [scl(1.0), scl(0.0), scl(0.0)],
            major_radius: len(2.0),
            window: TubeWindow::Full,
            minor_radius: len(0.5),
        },
        profile,
    );
    doc = add(
        doc,
        &mut rows,
        "wire_assertion's measure operand — behind the edit door",
        Owes::EditDoor,
        Node::Assertion {
            measure: plane,
            bound: len(1.0),
            dir: AssertionDir::AtMost,
        },
        plane,
    );
    doc = add(
        doc,
        &mut rows,
        "wire_split's tool (a Split tooled by a profile)",
        Owes::Refusal("datum plane", "profile"),
        Node::Split {
            target: body,
            tool: profile,
        },
        profile,
    );
    doc = add(
        doc,
        &mut rows,
        "wire_split's tool (a Split tooled by a 3-D axis datum)",
        Owes::Refusal("datum plane", "datum"),
        Node::Split {
            target: body,
            tool: axis3,
        },
        axis3,
    );
    doc = add(
        doc,
        &mut rows,
        "wire_part's SplitHalf arm (a half of a plain body)",
        Owes::Refusal("split", "body"),
        Node::Part {
            of: body,
            select: PartSelect::SplitHalf(SplitHalf::Above),
        },
        body,
    );
    doc = add(
        doc,
        &mut rows,
        "wire_part's Instance arm (an instance of a plain body)",
        Owes::Refusal("instances", "body"),
        Node::Part {
            of: body,
            select: PartSelect::Instance(Expr::count(0)),
        },
        body,
    );
    doc = add(
        doc,
        &mut rows,
        "declared_pairs on the union road — behind the edit door",
        Owes::EditDoor,
        Node::Union {
            members: vec![body, body2],
            declare: Some(plane),
        },
        plane,
    );
    doc = add(
        doc,
        &mut rows,
        "declared_pairs on the boolean road — behind the edit door",
        Owes::EditDoor,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a: body,
            b: body2,
            declare: Some(plane),
        },
        plane,
    );
    doc = add(
        doc,
        &mut rows,
        "stepped_map's circular axis (a Pattern about a plane datum)",
        Owes::Refusal("datum axis", "datum"),
        Node::Pattern {
            input: body,
            count: Expr::count(3),
            kind: PatternKind::Circular {
                axis: plane,
                step: ang(0.5),
            },
        },
        plane,
    );

    // ---- the node road: `found:` is the NODE's family ----

    doc = add(
        doc,
        &mut rows,
        "section_of (a Loft over a body)",
        Owes::Refusal("profile", "body"),
        Node::Loft {
            profiles: vec![profile, body],
            v_degree: Expr::count(1),
        },
        body,
    );
    doc = add(
        doc,
        &mut rows,
        "section_of (a Loft over a frame datum)",
        Owes::Refusal("profile", "datum"),
        Node::Loft {
            profiles: vec![profile, sketch],
            v_degree: Expr::count(1),
        },
        sketch,
    );
    doc = add(
        doc,
        &mut rows,
        "section_of on the sweep road (a Sweep whose profile is a body)",
        Owes::Refusal("profile", "body"),
        Node::Sweep {
            profile: body,
            path: profile,
            stations: Expr::count(3),
            v_degree: Expr::count(1),
        },
        body,
    );

    (doc, profile, body, pattern, rows)
}

/// Both halves of every refusal, byte-exact, over one evaluation.
#[test]
fn every_operand_refusal_names_the_phrase_asked_for_and_the_family_found() {
    let (doc, profile, body, pattern, rows) = wired();
    let ev = fixture::run(&doc, &EvalOptions::default());
    // The rows are evidence about the door only if the well-formed
    // nodes they borrow actually evaluated: a poisoned body, profile or
    // pattern would refuse for a reason that is not this test's.
    for (name, id) in [("profile", profile), ("body", body), ("pattern", pattern)] {
        assert!(
            ev.node_error(id).is_none(),
            "the {name} fixture must evaluate: {:?}",
            ev.node_error(id)
        );
    }
    let mut phrases: Vec<&'static str> = Vec::new();
    let mut families: Vec<&'static str> = Vec::new();
    for row in &rows {
        match (&row.owes, &row.edit) {
            // REFUSED, not merely "no node exists": an edit that
            // succeeded and minted nothing would satisfy the weaker
            // test while the door it stands in front of had stopped
            // refusing.
            (Owes::EditDoor, edit) => assert!(
                edit.is_err(),
                "{}: the edit door took a node it used to refuse ({edit:?}) — the operand door \
                 behind it is now document-reachable and owes its refusal a row here",
                row.what
            ),
            (Owes::Refusal(expected, found), Err(e)) => panic!(
                "{}: the edit door refused the insert ({e}), so nothing reaches the door that \
                 owes ({expected:?}, {found:?})",
                row.what
            ),
            (Owes::Refusal(expected, found), Ok(None)) => panic!(
                "{}: the edit succeeded and minted no node, so nothing reaches the door that \
                 owes ({expected:?}, {found:?})",
                row.what
            ),
            (Owes::Refusal(expected, found), Ok(Some(node))) => {
                let got = match ev.node_error(*node).map(|e| &e.kind) {
                    Some(NodeErrorKind::WrongOperand {
                        input,
                        expected,
                        found,
                    }) => (*expected, *found, *input),
                    other => panic!("{}: not a WrongOperand: {other:?}", row.what),
                };
                assert_eq!(got, (*expected, *found, row.input), "{}", row.what);
                phrases.push(expected);
                families.push(found);
            }
        }
    }
    // A census that read nothing would pass vacuously, and one that
    // reached a single phrase or a single family would pin neither half
    // against a door that answers a constant.
    assert_eq!(phrases.len(), 17, "the document-reachable rows");
    phrases.sort_unstable();
    phrases.dedup();
    families.sort_unstable();
    families.dedup();
    assert!(
        phrases.len() >= 8 && families.len() >= 4,
        "the rows must vary both halves: {} phrases over {} families",
        phrases.len(),
        families.len()
    );
}

// =====================================================================
// The SOURCE rules behind the refusals above.
// =====================================================================

/// **Two source rules with nothing else to red them.**
///
/// `eval::wire` gives "what kind is this operand, and refuse if it is
/// not" one home, and `eval::phrase` states the rule that an
/// `expected:` phrase comes from `eval::family` or `eval::phrase` and
/// is never written at a call site. Both are predicates over SOURCE
/// TEXT: a second place that builds the refusal, or a phrase spelled
/// at a call site, compiles, passes every behavioural row above, and
/// ships.
///
/// That is not hypothetical. The family literals were swept onto
/// consts once, and came back as the composed phrases these rows
/// guard; closing that a second time with nothing watching is how it
/// returns a third.
///
/// **Every row here is an EQUALITY, not a floor.** A count floor over
/// a hand-written roster is the same defect one level up: it cannot
/// see one arm of its own scan go to zero, and its slack hides drift.
/// `crates/test-utils/tests/reader_census.rs` states the rule this
/// module follows — *"a walk that matched nothing is not a pass, and
/// the equality is what says so"* — and each row below names the two
/// sets it equates.
///
/// **What these rows cannot see**, stated so the receipt is honest: a
/// refusal built in another module (`mate/member.rs` builds one —
/// `work/docm/the-third-datum-axis-phrase-lives-in-mate-member.md`),
/// a phrase reached through a helper that takes it as an argument from
/// elsewhere, the other kind-mismatch vocabularies in this crate
/// (`work/wire/the-entity-kind-door-has-six-spellings.md` enumerates
/// them, so this doc does not), and the direction-role words
/// (`work/wire/direction-role-words-respell-the-operand-phrases.md`).
/// Those files are the lists; a copy here would be a second one.
mod source_rules {
    use test_utils::source;
    use test_utils::source::{boundary_before, line};

    const WIRE: &str = include_str!("../src/eval/wire.rs");
    const SPLIT: &str = include_str!("../src/verbs/split.rs");
    const MOD: &str = include_str!("../src/eval/mod.rs");

    /// The byte range of the operand door, located once.
    fn door() -> std::ops::Range<usize> {
        source::sentinel_region(
            WIRE,
            "eval/wire.rs",
            "OPERAND-DOOR BEGIN",
            "OPERAND-DOOR END",
        )
    }

    /// Every `fn` declared in the door region, with the position of its
    /// `expected` parameter.
    ///
    /// **Derived, never listed.** A door added inside the sentinels is
    /// measured the moment it is typed, and a door renamed cannot fall
    /// out of a roster that does not exist — which is exactly what a
    /// hand-written list of door names could not promise.
    fn doors(code: &str) -> Vec<(&str, usize)> {
        let door = door();
        let mut out = Vec::new();
        for (at, _) in code[door.clone()].match_indices("fn ") {
            let at = door.start + at;
            if !boundary_before(code, at) {
                continue;
            }
            let head = at + "fn ".len();
            let name_end = head
                + code[head..]
                    .find(|c: char| !c.is_alphanumeric() && c != '_')
                    .expect("a fn name ends");
            let open = name_end + code[name_end..].find('(').expect("a fn takes parameters");
            let close = source::balanced_end(code, open).expect("the parameter list closes");
            let params = &code[open + 1..close];
            let slot = source::top_level_split(params, ',')
                .into_iter()
                .position(|r| {
                    params[r]
                        .split(':')
                        .next()
                        .is_some_and(|n| n.trim() == "expected")
                });
            if let Some(slot) = slot {
                out.push((&code[head..name_end], slot));
            }
        }
        out
    }

    /// **`WrongOperand` is constructed in exactly one place**, and that
    /// place is inside the operand door.
    ///
    /// The two sets: every brace-form `WrongOperand` in `eval/wire.rs`
    /// that names `expected` (a construction; the file's one pattern is
    /// `WrongOperand { .. }` and names nothing), against the one site
    /// the rule allows. `built.len() == 1` is the whole guard — it
    /// fails on a second construction anywhere, INCLUDING one inside
    /// the sentinels, which a containment test alone would miss.
    ///
    /// The variant token is matched BARE, so a braced or plain `use`
    /// that drops the `NodeErrorKind::` qualifier does not walk past
    /// this; the `{` is required to follow immediately, so a `use` line
    /// or a type position is not read as a construction.
    ///
    /// A future pattern that DID bind `expected` would be counted as a
    /// construction and red this row — the safe direction, and the only
    /// one a textual reader has.
    #[test]
    fn wrong_operand_is_built_in_one_place() {
        let code = source::blanked(source::code_only, "eval/wire.rs", WIRE);
        let door = door();
        let mut built = Vec::new();
        for (at, _) in code.match_indices("WrongOperand") {
            if !boundary_before(&code, at) {
                continue;
            }
            let rest = &code[at + "WrongOperand".len()..];
            let brace = rest.len() - rest.trim_start().len();
            if !rest[brace..].starts_with('{') {
                continue; // a `use`, a type position, a doc mention
            }
            let open = at + "WrongOperand".len() + brace;
            let close = source::balanced_end(&code, open).expect("the brace closes");
            if !code[open..=close].contains("expected") {
                continue; // a pattern, not a construction
            }
            assert!(
                door.contains(&at),
                "eval/wire.rs line {}: a `WrongOperand` is built outside the operand door — \
                 the refusal has one home and `found:` is not a caller's to write",
                line(WIRE, at)
            );
            built.push(at);
        }
        assert_eq!(
            built.len(),
            1,
            "`eval/wire.rs` builds `WrongOperand` {} times, not once (lines {:?})",
            built.len(),
            built.iter().map(|a| line(WIRE, *a)).collect::<Vec<_>>()
        );
    }

    /// **Every `expected:` phrase comes from `eval::family` or
    /// `eval::phrase`** — the rule as stated, not a proxy for it.
    ///
    /// The two sets, equated by byte offset: every `super::family::` /
    /// `super::phrase::` mention in `eval/wire.rs`, and every door
    /// call's `expected` argument that names one. Both directions
    /// carry a failure a lane can actually make:
    ///
    /// - an `expected` argument that is a literal, or a local const, or
    ///   any other expression, is not in the vocabulary set and reds.
    ///   A `plain_string_literal` test would pass a phrase hoisted into
    ///   a const at the call site; this does not.
    /// - a vocabulary const mentioned anywhere that is NOT a door
    ///   call's `expected` argument reds too. The words exist to be
    ///   said in this one refusal, and a second reader of them is a
    ///   second place the vocabulary is decided.
    /// - a scan that stopped matching reds, because the vocabulary set
    ///   is non-empty and would no longer be covered. That is what
    ///   replaces a count floor here.
    ///
    /// Two arguments are admitted without naming the vocabulary, and
    /// both are FORWARDED LABELS rather than fresh words — the final
    /// path segment is `expected` or `<x>_expected`, and the spelling
    /// is pinned where the thing is declared. A door's own `expected`
    /// parameter (the two calls inside the sentinels, and only there)
    /// and `verb.tool_expected`, whose value the row below reads out of
    /// `fn split` itself.
    #[test]
    fn every_expected_phrase_comes_from_the_vocabulary() {
        let code = source::blanked(source::code_and_literals, "eval/wire.rs", WIRE);
        let door = door();
        // Set A: every mention of the vocabulary.
        let mut vocabulary: Vec<usize> = Vec::new();
        for prefix in ["super::family::", "super::phrase::"] {
            for (at, _) in code.match_indices(prefix) {
                vocabulary.push(at);
            }
        }
        assert!(
            !vocabulary.is_empty(),
            "eval/wire.rs mentions no `family::` or `phrase::` const at all — the vocabulary \
             moved and this row is reading the wrong file"
        );
        // Set B: every door call's `expected` argument that names one.
        let doors = doors(&code);
        assert!(
            doors.len() >= 2,
            "the door region declares {} function(s) taking an `expected` — the sentinels or \
             the scan have drifted from the doors they read",
            doors.len()
        );
        let mut said: Vec<usize> = Vec::new();
        for (name, slot) in &doors {
            for (at, _) in code.match_indices(&format!("{name}(")) {
                if !boundary_before(&code, at) || code[..at].trim_end().ends_with("fn") {
                    continue;
                }
                let open = at + name.len();
                let close = source::balanced_end(&code, open).expect("the call's paren closes");
                let args = &code[open + 1..close];
                let Some(arg) = source::top_level_split(args, ',').into_iter().nth(*slot) else {
                    continue;
                };
                let whole = &args[arg.clone()];
                let text = whole.trim();
                // A FORWARDED LABEL is not a fresh word: its final
                // path segment is `expected` or `<x>_expected`, and
                // its own spelling is pinned where it is DECLARED —
                // the door's own parameter needs no pin, and
                // `verb.tool_expected` has the split row below. A
                // hoisted const (`const R2: &str = "split";`) has no
                // such segment and fails the form test, which is the
                // move a `plain_string_literal` test would have
                // passed.
                let last = text.rsplit('.').next().unwrap_or(text);
                if last == "expected" || last.ends_with("_expected") {
                    assert!(
                        last != "expected" || door.contains(&at),
                        "eval/wire.rs line {}: `{name}` forwards a bare `expected` from \
                         outside the operand door",
                        line(WIRE, at)
                    );
                    continue;
                }
                // Where the argument's first non-space byte sits in the
                // file: the two sets are equated by position, so a
                // vocabulary mention that is not an argument, and an
                // argument that is not a mention, are both visible.
                let lead = whole.len() - whole.trim_start().len();
                let offset = open + 1 + arg.start + lead;
                assert!(
                    text.starts_with("super::family::") || text.starts_with("super::phrase::"),
                    "eval/wire.rs line {}: `{name}` is handed `{text}` — an `expected:` phrase \
                     comes from `eval::family` or `eval::phrase`, and a const declared anywhere \
                     else is a second home for the same word",
                    line(WIRE, at)
                );
                said.push(offset);
            }
        }
        vocabulary.sort_unstable();
        said.sort_unstable();
        assert_eq!(
            vocabulary
                .iter()
                .map(|a| line(WIRE, *a))
                .collect::<Vec<_>>(),
            said.iter().map(|a| line(WIRE, *a)).collect::<Vec<_>>(),
            "every `family::`/`phrase::` const in eval/wire.rs is a door call's `expected` \
             argument and every such argument is one of them; the lines above are the \
             vocabulary's, the lines below are the door calls'"
        );
    }

    /// **The split verb's `tool_expected` is a vocabulary const too.**
    ///
    /// It is the one `expected:` this crate carries as correspondence
    /// DATA rather than as a door argument, so the door census above
    /// cannot see it. Read out of `fn split`'s own `SplitVerb` literal
    /// — not by searching the file for `tool_expected:`, which also
    /// matches the field DECLARATION and would leave the row non-empty
    /// while the real initializer moved.
    #[test]
    fn the_split_tools_label_comes_from_the_vocabulary() {
        let code = source::blanked(source::code_and_literals, "verbs/split.rs", SPLIT);
        let head = code
            .find("pub(crate) fn split<")
            .expect("`fn split` is declared");
        let source::ItemBody::Body(body) = source::item_body(&code, head) else {
            panic!("`fn split` has a body");
        };
        let open = body.start
            + code[body.clone()]
                .find("SplitVerb {")
                .expect("`fn split` returns a `SplitVerb` literal")
            + "SplitVerb ".len();
        let close = source::balanced_end(&code, open).expect("the literal closes");
        let fields = &code[open + 1..close];
        let mut found: Vec<&str> = source::top_level_split(fields, ',')
            .into_iter()
            .filter_map(|r| fields[r].trim().strip_prefix("tool_expected:"))
            .collect();
        assert_eq!(
            found.len(),
            1,
            "`fn split`'s `SplitVerb` literal sets `tool_expected` {} times, not once",
            found.len()
        );
        let value = found.remove(0).trim();
        assert!(
            value.starts_with("crate::eval::phrase::")
                || value.starts_with("crate::eval::family::"),
            "verbs/split.rs: `tool_expected` is `{value}` — the correspondence owns the FIELD, \
             and `eval::phrase` owns the word"
        );
    }

    /// **The family vocabulary is closed both ways.**
    ///
    /// A `family::` const with no `family_word!` arm is already a
    /// compile error. The silent direction is the other one: a macro
    /// arm no const uses is dead text no lint reads, so the macro would
    /// be a second hand-written list kept in step by hand — the defect
    /// it exists to remove. The two sets are the arm heads and the
    /// words the consts are defined from, and they are equated, not
    /// counted.
    ///
    /// It also refuses a family word spelled anywhere but the macro:
    /// `ValuePayload::kind_name` and `eval::node_value_kind` answer the
    /// vocabulary over a value and over a node, and a new variant whose
    /// arm returned a fresh literal would mint a word nothing knows
    /// about.
    #[test]
    fn every_family_word_has_exactly_one_const() {
        // The LITERAL-bearing view: the macro's arms ARE literals, and
        // `code_only` would blank exactly the text this row reads.
        let text = source::blanked(source::code_and_literals, "eval/mod.rs", MOD);
        let vocab = source::sentinel_region(
            MOD,
            "eval/mod.rs",
            "OPERAND-VOCABULARY BEGIN",
            "OPERAND-VOCABULARY END",
        );
        let mut arms: Vec<(&str, &str)> = Vec::new();
        let mut used: Vec<&str> = Vec::new();
        let mut pending: Option<&str> = None;
        for line in text[vocab].lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix('(')
                && let Some((word, tail)) = rest.split_once(')')
                && tail.trim_start().starts_with("=>")
            {
                pending = Some(word);
                continue;
            }
            if let Some(word) = pending
                && let Some(lit) = source::plain_string_literal(line)
            {
                arms.push((word, lit));
                pending = None;
            }
            if let Some((_, rest)) = line.split_once("family_word!(")
                && let Some((word, _)) = rest.split_once(')')
            {
                used.push(word);
            }
        }
        for (word, lit) in &arms {
            assert_eq!(
                word, lit,
                "a `family_word!` arm's head names the word it expands to, so a reader of one \
                 `family::` const does not have to open the macro to learn what it says"
            );
        }
        let mut heads: Vec<&str> = arms.iter().map(|(w, _)| *w).collect();
        heads.sort_unstable();
        let mut sorted_used = used.clone();
        sorted_used.sort_unstable();
        assert_eq!(
            heads, sorted_used,
            "every `family_word!` arm owes exactly one `family::` const and every const owes \
             an arm; an arm with no const is dead text no lint reads. An empty pair here is a \
             scan that has drifted off the macro, and reds for the same reason"
        );
        assert!(
            !heads.is_empty(),
            "the vocabulary census read no macro arm at all — the sentinels or the scan have \
             drifted from the macro they read"
        );
        // No arm of either kind function spells a family word itself.
        let code = source::blanked(source::code_only, "eval/mod.rs", MOD);
        for head in ["pub fn kind_name", "pub(crate) fn node_value_kind"] {
            let at = code
                .find(head)
                .unwrap_or_else(|| panic!("`{head}` is declared"));
            let source::ItemBody::Body(body) = source::item_body(&code, at) else {
                panic!("`{head}` has a body");
            };
            let body = &text[body];
            for (_, lit) in &arms {
                assert!(
                    !body.contains(&format!("\"{lit}\"")),
                    "`{head}` spells the family word {lit:?} as a literal — the vocabulary is \
                     `family`'s, and a variant added here must take a word from it"
                );
            }
        }
    }
}
