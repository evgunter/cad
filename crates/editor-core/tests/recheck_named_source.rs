//! Re-check probes for PR 6856's fix pass (lane `placing-recheck-6856`).
//! Each row asserts what the fix pass claims; a red row is a finding.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::block;
use crate::fixture::{insert, len, out, step};

use editor_core::{
    BodyRead, CapEnd, DocEdit, EditError, EntityKind, Formula, Node, Operand, ProfileDoc, RoleSeg,
    StableName, VarDecl, VarId, VarName,
};
use geom_core::Tol;

fn placed(read: Operand) -> editor_core::AuthoredNode {
    Node::place(
        BodyRead::plain(read),
        Operand::Var(VarId::WORLD),
        editor_core::stand().bundle(),
    )
}

fn declared(doc: ProfileDoc, name: &'static str, decl: VarDecl) -> ProfileDoc {
    step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::from_static(name),
            def: decl,
        },
    )
    .0
}

fn define(doc: &ProfileDoc, name: &'static str, decl: VarDecl) -> Result<ProfileDoc, EditError> {
    doc.apply(
        &DocEdit::DefineVar {
            var: VarName::from_static(name).into(),
            def: decl,
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .map(|a| a.doc)
}

fn name(n: &'static str) -> Operand {
    Operand::Name(VarName::from_static(n))
}

/// The door either refuses, or what it writes saves and loads.
fn refused_or_round_trips(what: &str, got: Result<ProfileDoc, EditError>) {
    if let Ok(doc) = got {
        let text = editor_core::save(&doc, &[], Tol::witness());
        let text = text.unwrap_or_else(|e| panic!("{what}: admitted, save refuses: {e:?}"));
        let loaded = editor_core::load(&text, Tol::witness());
        assert!(
            loaded.is_ok(),
            "{what}: admitted, load refuses: {:?}",
            loaded.map(|_| ())
        );
    }
}

/// `a = B`, `b = a`, `c = b`, then `a = c`: a three-long source cycle.
#[test]
fn recheck_a_three_long_source_cycle_is_refused() {
    let doc = ProfileDoc::empty_derived("rc-three", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let doc = declared(doc, "a", VarDecl::source(Operand::Node(b)));
    let doc = declared(doc, "b", VarDecl::source(name("a")));
    let doc = declared(doc, "c", VarDecl::source(name("b")));
    let (doc, _) = insert(doc, placed(name("c")));
    let got = define(&doc, "a", VarDecl::source(name("c")));
    assert!(
        matches!(got, Err(EditError::DefinitionCycle { .. })),
        "{:?}",
        got.map(|_| ())
    );
    let got = define(&doc, "b", VarDecl::source(name("b")));
    assert!(got.is_err(), "b = b admitted");
}

/// A source cycle through a member read: `a = row`, `b = a[0]`, then
/// `a = b`.
#[test]
fn recheck_a_cycle_through_a_member_read_is_refused() {
    let doc = ProfileDoc::empty_derived("rc-member-cycle", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = crate::fixture::row(doc, b, ("k", 2), editor_core::TargetAxis::X, 2.0);
    let doc = declared(doc, "a", VarDecl::source(Operand::Node(row)));
    let doc = declared(
        doc,
        "b",
        VarDecl::source(BodyRead::indexed(name("a"), vec![Formula::count(0)])),
    );
    let got = define(&doc, "a", VarDecl::source(name("b")));
    assert!(got.is_err(), "a = b = a[0] admitted");
}

/// A source cycle mixing a selection: `a = B`; a shell of `a.top`;
/// `a = shell`. A cycle through a node reading a selection of the source.
#[test]
fn recheck_a_cycle_through_a_selection_of_the_source_is_refused() {
    let doc = ProfileDoc::empty_derived("rc-select-cycle", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let doc = declared(doc, "a", VarDecl::source(Operand::Node(b)));
    let top = StableName {
        kind: EntityKind::Face,
        node: b,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    };
    let (doc, shell) = insert(
        doc,
        Node::Shell {
            thickness: len(0.1),
            open: Operand::select(name("a"), vec![top]),
        },
    );
    let got = define(&doc, "a", VarDecl::source(Operand::Node(shell)));
    if let Ok(after) = &got {
        let a = after.var_named("a").expect("a");
        eprintln!("a = {:?}; shell = {:?}", after.source(a), after.node(shell));
        let ev: editor_core::Evaluation<f64> = editor_core::evaluate(
            after,
            None,
            &editor_core::CancelToken::new(),
            &editor_core::EvalOptions::default(),
            Tol::witness(),
        );
        eprintln!(
            "evaluated: shell {:?}",
            ev.result(shell)
                .map(|r| format!("{r:?}").chars().take(200).collect::<String>())
        );
        let text = editor_core::save(after, &[], Tol::witness());
        eprintln!("save: {:?}", text.as_ref().map(|_| ()));
        if let Ok(text) = text {
            eprintln!(
                "load: {:?}",
                editor_core::load(&text, Tol::witness()).map(|_| ())
            );
        }
    }
    assert!(got.is_err(), "a = shell(a.top) admitted");
}

/// A source declared as a member of a family source: `S = row`,
/// `u = S[0]`, `p = place(u)`. Redefining `S` onto one body leaves `u`
/// reading a member of no family. Refuse, or write what loads.
#[test]
fn recheck_a_member_source_of_a_family_source_redefined_onto_a_body() {
    let doc = ProfileDoc::empty_derived("rc-member-of-source", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, d) = block(doc, (0.0, 2.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = crate::fixture::row(doc, b, ("k", 2), editor_core::TargetAxis::X, 2.0);
    let doc = declared(doc, "S", VarDecl::source(Operand::Node(row)));
    let doc = declared(
        doc,
        "u",
        VarDecl::source(BodyRead::indexed(name("S"), vec![Formula::count(0)])),
    );
    let unread = define(&doc, "S", VarDecl::source(Operand::Node(d)));
    refused_or_round_trips("u unread", unread);
    let (doc, _) = insert(doc, placed(name("u")));
    let read = define(&doc, "S", VarDecl::source(Operand::Node(d)));
    refused_or_round_trips("u placed", read);
}

/// A selection of a member of a family source: a shell of `S[0].top`.
/// Redefining `S` onto one body leaves the selection reading a member of no
/// family. Refuse, or write what loads.
#[test]
fn recheck_a_member_selection_of_a_family_source_redefined_onto_a_body() {
    let doc = ProfileDoc::empty_derived("rc-select-member", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, d) = block(doc, (0.0, 2.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = crate::fixture::row(doc, b, ("k", 2), editor_core::TargetAxis::X, 2.0);
    let doc = declared(doc, "S", VarDecl::source(Operand::Node(row)));
    let top = StableName {
        kind: EntityKind::Face,
        node: b,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    };
    let (doc, _) = insert(
        doc,
        Node::Shell {
            thickness: len(0.1),
            open: Operand::select(
                BodyRead::indexed(name("S"), vec![Formula::count(0)]),
                vec![top],
            ),
        },
    );
    let got = define(&doc, "S", VarDecl::source(Operand::Node(d)));
    refused_or_round_trips("shell of S[0].top", got);
}

/// Through a chain: `S = row`, `T = S`, `m = place(T[0])`; `S = D`.
#[test]
fn recheck_a_chain_member_placement_redefined_onto_a_body() {
    let doc = ProfileDoc::empty_derived("rc-chain-member", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, d) = block(doc, (0.0, 2.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = crate::fixture::row(doc, b, ("k", 2), editor_core::TargetAxis::X, 2.0);
    let doc = declared(doc, "S", VarDecl::source(Operand::Node(row)));
    let doc = declared(doc, "T", VarDecl::source(name("S")));
    let (doc, _) = insert(
        doc,
        Node::place(
            BodyRead::indexed(name("T"), vec![Formula::count(0)]),
            Operand::Var(VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
    let got = define(&doc, "S", VarDecl::source(Operand::Node(d)));
    assert!(
        matches!(got, Err(EditError::SlotVarKind { .. })),
        "{:?}",
        got.map(|_| ())
    );
}

/// The delete door hands a source's definition back (DM6): `S = row`,
/// `m = place(S[0])`, delete `row`, so `S = B` with `m` reading `S[0]`.
/// The delete either refuses or writes what loads.
#[test]
fn recheck_deleting_the_family_under_a_member_placement() {
    let doc = ProfileDoc::empty_derived("rc-delete-family", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = crate::fixture::row(doc, b, ("k", 2), editor_core::TargetAxis::X, 2.0);
    let doc = declared(doc, "S", VarDecl::source(Operand::Node(row)));
    let (doc, _) = insert(
        doc,
        Node::place(
            BodyRead::indexed(name("S"), vec![Formula::count(0)]),
            Operand::Var(VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
    let got = doc
        .apply(&DocEdit::DeleteNode { id: row }, Tol::witness())
        .map(|a| a.doc);
    if let Ok(after) = &got {
        let s = after.var_named("S").expect("S");
        eprintln!("S after delete: {:?}", after.source(s));
    }
    refused_or_round_trips("delete row", got);
}

/// Anonymous member placement (`fixture::member`): delete the family.
#[test]
fn recheck_deleting_the_family_under_an_anonymous_member_placement() {
    let doc = ProfileDoc::empty_derived("rc-delete-family-anon", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = crate::fixture::row(doc, b, ("k", 2), editor_core::TargetAxis::X, 2.0);
    let (doc, _) = crate::fixture::member(doc, row, 0);
    let got = doc
        .apply(&DocEdit::DeleteNode { id: row }, Tol::witness())
        .map(|a| a.doc);
    refused_or_round_trips("delete row (anonymous)", got);
}

/// A hand-written file with an anonymous source cycle under a placement:
/// the placement's anonymous source re-pointed at itself.
#[test]
fn recheck_the_load_refuses_an_anonymous_self_source() {
    let doc = ProfileDoc::empty_derived("rc-anon-self", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, c) = insert(doc, placed(Operand::Node(b)));
    let s = doc.copied_read(c).expect("a source");
    let text = editor_core::save(&doc, &[], Tol::witness()).expect("saves");
    let spell = |var: VarId| serde_json::to_string(&var).expect("an id spells");
    let needle = format!("\"Source\": {}", spell(out(&doc, b)));
    assert_eq!(text.matches(&needle).count(), 1);
    let edited = text.replace(&needle, &format!("\"Source\": {}", spell(s)));
    let loaded = editor_core::load(&edited, Tol::witness());
    assert!(loaded.is_err(), "an anonymous s = s loads");
}

/// A member placement of a 1-D family's source, the source redefined onto
/// a 2-D family: the member's one index no longer matches the rank.
#[test]
fn recheck_a_member_source_redefined_onto_a_family_of_another_rank() {
    use crate::fixture::{family_of, index, scalar_of};
    let doc = ProfileDoc::empty_derived("rc-rank", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = crate::fixture::row(doc, b, ("k", 2), editor_core::TargetAxis::X, 2.0);
    let (doc, i) = index(doc, "i", 2);
    let (doc, j) = crate::fixture::index_of(doc, "j", Formula::count(2), Some(i));
    let (doc, grid) = family_of(
        doc,
        b,
        vec![
            (
                editor_core::Coord::slide(editor_core::TargetAxis::X),
                Formula::mul(scalar_of(i), len(2.0)).unwrap(),
            ),
            (
                editor_core::Coord::slide(editor_core::TargetAxis::Y),
                Formula::mul(scalar_of(j), len(2.0)).unwrap(),
            ),
        ],
    );
    let doc = declared(doc, "S", VarDecl::source(Operand::Node(row)));
    let (doc, _) = insert(
        doc,
        Node::place(
            BodyRead::indexed(name("S"), vec![Formula::count(0)]),
            Operand::Var(VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
    let got = define(&doc, "S", VarDecl::source(Operand::Node(grid)));
    eprintln!("rank: {:?}", got.as_ref().map(|_| ()));
    refused_or_round_trips("S = grid under S[0]", got);
}

/// A pattern of a plain copy of a body source, the source redefined onto a
/// family: the pattern would repeat a family.
#[test]
fn recheck_a_pattern_of_a_copy_whose_source_becomes_a_family() {
    let doc = ProfileDoc::empty_derived("rc-pattern-of-copy", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = crate::fixture::row(doc, b, ("k", 2), editor_core::TargetAxis::X, 2.0);
    let doc = declared(doc, "S", VarDecl::source(Operand::Node(b)));
    let (doc, c) = insert(doc, placed(name("S")));
    let (doc, _, _) = crate::fixture::row(doc, c, ("m", 2), editor_core::TargetAxis::Y, 2.0);
    let got = define(&doc, "S", VarDecl::source(Operand::Node(row)));
    eprintln!("pattern-of-copy: {:?}", got.as_ref().map(|_| ()));
    refused_or_round_trips("S = row under a pattern of place(S)", got);
}

/// **A member placement's `on` side selecting its family source** (fix
/// item 4: `matetool::on_read` now writes `Var(source)` for members too):
/// `m = place(S[0])` into the world, its mate `on` side a plane of
/// `S.bottom` (the family source, no index), onto the floor. Admitted, and
/// it evaluates; and does it land where `S[0].bottom` would put it?
#[test]
fn recheck_a_member_placements_on_side_in_its_family_source() {
    use crate::fixture::{flip, floor, fname, mate, plane, squared};
    let doc = ProfileDoc::empty_derived("rc-member-on", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = crate::fixture::row(doc, b, ("k", 3), editor_core::TargetAxis::X, 2.0);
    let (doc, ground, _) = floor(doc, Vec::new());
    let (doc, copy) = insert(
        doc,
        Node::place(
            BodyRead::indexed(row, vec![Formula::count(2)]),
            Operand::Var(VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
    let source = doc.copied_read(copy).expect("a source");
    let bottom = fname(b, RoleSeg::Cap(CapEnd::Start));
    for (what, on) in [
        (
            "whole",
            Operand::select(Operand::Var(source), vec![bottom.clone()]),
        ),
        (
            "member",
            Operand::select(
                BodyRead::indexed(Operand::Var(source), vec![Formula::count(2)]),
                vec![bottom.clone()],
            ),
        ),
    ] {
        let m = mate(plane(on), flip(ground.clone()));
        let placed_by = std::panic::catch_unwind(|| squared(&doc, m, &[]));
        let Ok(placed_by) = placed_by else {
            eprintln!("{what}: squared panicked");
            continue;
        };
        let written = doc.apply(
            &DocEdit::SetConstraints {
                placement: copy,
                constraints: placed_by.bundle(),
                fresh: Vec::new(),
            },
            Tol::witness(),
        );
        eprintln!("{what}: write {:?}", written.as_ref().map(|_| ()));
        if let Ok(a) = written {
            let ev: editor_core::Evaluation<f64> = editor_core::evaluate(
                &a.doc,
                None,
                &editor_core::CancelToken::new(),
                &editor_core::EvalOptions::default(),
                Tol::witness(),
            );
            eprintln!(
                "{what}: eval {:?}",
                ev.result(copy)
                    .map(|r| format!("{r:?}").chars().take(300).collect::<String>())
            );
        }
    }
}

/// **A selection of a source is an edge to the source's body**: `a = B`;
/// a shell of `a.top`; a copy `c` of B inserted after the shell; `a = c`.
/// The shell now reads `c`, inserted after it: does it evaluate?
#[test]
fn recheck_a_selection_of_a_source_orders_after_the_sources_body() {
    let doc = ProfileDoc::empty_derived("rc-select-order", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let doc = declared(doc, "a", VarDecl::source(Operand::Node(b)));
    let top = StableName {
        kind: EntityKind::Face,
        node: b,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    };
    let (doc, shell) = insert(
        doc,
        Node::Shell {
            thickness: len(0.1),
            open: Operand::select(name("a"), vec![top]),
        },
    );
    let (doc, c) = insert(doc, placed(Operand::Node(b)));
    let eval = |doc: &ProfileDoc| {
        let ev: editor_core::Evaluation<f64> = editor_core::evaluate(
            doc,
            None,
            &editor_core::CancelToken::new(),
            &editor_core::EvalOptions::default(),
            Tol::witness(),
        );
        ev.result(shell)
            .map(|r| format!("{r:?}").chars().take(160).collect::<String>())
    };
    eprintln!("before: {:?}", eval(&doc));
    eprintln!("upstream of shell: {:?}", doc.upstream(shell));
    let after = define(&doc, "a", VarDecl::source(Operand::Node(c))).expect("admitted");
    eprintln!("after: {:?}", eval(&after));
    eprintln!(
        "upstream of shell after: {:?}; c = {c:?}",
        after.upstream(shell)
    );
    assert!(
        after.upstream(shell).contains(&c),
        "the shell does not depend on c"
    );
}
