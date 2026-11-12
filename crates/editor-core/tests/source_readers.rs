//! **What reads a source is asked again when the source is redefined**
//! (VARIABLES VR3, VR4): a selection of a source depends on the source's
//! body, so a cycle through it is refused and it evaluates after the
//! body; every definition and seat reading a redefined source, at any
//! depth, is held to what the load holds it to; a delete leaving a
//! source stranded writes what loads. Rows first written as the
//! re-check probes of PR 6856's fix pass (lane `placing-recheck-6856`).
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

/// What `doc` writes saves and loads back.
fn round_trips(what: &str, doc: &ProfileDoc) {
    let text = editor_core::save(doc, &[], Tol::witness())
        .unwrap_or_else(|e| panic!("{what}: save refuses: {e:?}"));
    let loaded = editor_core::load(&text, Tol::witness());
    assert!(
        loaded.is_ok(),
        "{what}: load refuses: {:?}",
        loaded.map(|_| ())
    );
}

fn evaluated(doc: &ProfileDoc) -> editor_core::Evaluation<f64> {
    editor_core::evaluate(
        doc,
        None,
        &editor_core::CancelToken::new(),
        &editor_core::EvalOptions::default(),
        Tol::witness(),
    )
}

fn top(b: editor_core::RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: b,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    }
}

/// The reader refusal a redefinition answers: the load's own refusal of
/// the definition reading the source.
fn reader_refusal(got: Result<ProfileDoc, EditError>) -> editor_core::SnapshotError {
    match got {
        Err(EditError::SourceReaderRefused { reader, .. }) => *reader,
        other => panic!("expected SourceReaderRefused: {:?}", other.map(|_| ())),
    }
}

/// `a = B`, `b = a`, `c = b`, then `a = c`: a three-long source cycle.
#[test]
fn a_three_long_source_cycle_is_refused() {
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
    assert!(
        matches!(got, Err(EditError::DefinitionCycle { .. })),
        "{:?}",
        got.map(|_| ())
    );
}

/// A source cycle through a member read: `a = row`, `b = a[0]`, then
/// `a = b`.
#[test]
fn a_cycle_through_a_member_read_is_refused() {
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
    assert!(
        matches!(got, Err(EditError::DefinitionCycle { .. })),
        "{:?}",
        got.map(|_| ())
    );
}

/// **A selection of a source depends on the source's body**: `a = B`; a
/// shell of `a.top`; `a = shell` is a cycle through the shell, refused.
#[test]
fn a_cycle_through_a_selection_of_the_source_is_refused() {
    let doc = ProfileDoc::empty_derived("rc-select-cycle", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let doc = declared(doc, "a", VarDecl::source(Operand::Node(b)));
    let (doc, shell) = insert(
        doc,
        Node::Shell {
            thickness: len(0.1),
            open: Operand::select(name("a"), vec![top(b)]),
        },
    );
    assert_eq!(doc.upstream(shell), vec![b], "the shell reads a's body");
    let got = define(&doc, "a", VarDecl::source(Operand::Node(shell)));
    assert!(
        matches!(got, Err(EditError::WouldCycle { .. })),
        "{:?}",
        got.map(|_| ())
    );
}

/// A source declared as a member of a family source: `S = row`,
/// `u = S[0]`. Redefining `S` onto one body leaves `u` reading a member
/// of no family: refused by the load's own check of `u`, placed or not.
#[test]
fn a_member_source_of_a_family_source_redefined_onto_a_body() {
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
    let u = doc.var_named("u").expect("u");
    let refused = reader_refusal(define(&doc, "S", VarDecl::source(Operand::Node(d))));
    assert!(
        matches!(&refused, editor_core::SnapshotError::SourceReadKind { var, .. } if var.id() == u),
        "{refused:?}"
    );
    let (doc, _) = insert(doc, placed(name("u")));
    let refused = reader_refusal(define(&doc, "S", VarDecl::source(Operand::Node(d))));
    assert!(
        matches!(&refused, editor_core::SnapshotError::SourceReadKind { var, .. } if var.id() == u),
        "{refused:?}"
    );
}

/// A selection of a member of a family source: a shell of `S[0].top`.
/// Redefining `S` onto one body leaves the selection reading a member of
/// no family: refused by the load's own check of the selection.
#[test]
fn a_member_selection_of_a_family_source_redefined_onto_a_body() {
    let doc = ProfileDoc::empty_derived("rc-select-member", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, d) = block(doc, (0.0, 2.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = crate::fixture::row(doc, b, ("k", 2), editor_core::TargetAxis::X, 2.0);
    let doc = declared(doc, "S", VarDecl::source(Operand::Node(row)));
    let (doc, _) = insert(
        doc,
        Node::Shell {
            thickness: len(0.1),
            open: Operand::select(
                BodyRead::indexed(name("S"), vec![Formula::count(0)]),
                vec![top(b)],
            ),
        },
    );
    let refused = reader_refusal(define(&doc, "S", VarDecl::source(Operand::Node(d))));
    assert!(
        matches!(
            refused,
            editor_core::SnapshotError::SelectionShape { .. }
                | editor_core::SnapshotError::SelectionBody { .. }
        ),
        "{refused:?}"
    );
}

/// Through a chain: `S = row`, `T = S`, `m = place(T[0])`; `S = D` is
/// refused at the member's seat.
#[test]
fn a_chain_member_placement_redefined_onto_a_body() {
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

/// `S = row`, `m = place(S[0])`, the family under it deleted.
fn member_over_a_deleted_family(
    named: bool,
) -> (
    ProfileDoc,
    editor_core::RecipeNodeId,
    editor_core::RecipeNodeId,
) {
    let doc = ProfileDoc::empty_derived("rc-delete-family", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = crate::fixture::row(doc, b, ("k", 2), editor_core::TargetAxis::X, 2.0);
    let (doc, m) = if named {
        let doc = declared(doc, "S", VarDecl::source(Operand::Node(row)));
        insert(
            doc,
            Node::place(
                BodyRead::indexed(name("S"), vec![Formula::count(0)]),
                Operand::Var(VarId::WORLD),
                editor_core::stand().bundle(),
            ),
        )
    } else {
        crate::fixture::member(doc, row, 0)
    };
    (doc, row, m)
}

/// **A delete strands a source, and writes what loads** (VR7): the
/// family under a member placement deleted, its source reads a body no
/// operation defines, which the member's seat is asked no kind and no
/// rank of at rest; the member refuses at evaluation. The delete by
/// continuation (DM6) takes the member with its family, and what it
/// writes loads too. (A source it hands over is a `DefineVar` through
/// the definition door, which asks every reader again.)
#[test]
fn deleting_the_family_under_a_member_placement() {
    for named in [true, false] {
        let (doc, row, m) = member_over_a_deleted_family(named);
        let after = doc
            .apply(&DocEdit::DeleteNode { id: row }, Tol::witness())
            .expect("a delete strands its readers")
            .doc;
        round_trips("delete row", &after);
        assert!(
            crate::blocks::failure(&evaluated(&after), m).is_some(),
            "the stranded member refuses at evaluation"
        );
        let splice = editor_core::continuation::splice(&doc, row)
            .expect("plans")
            .expect("row is live");
        assert!(
            splice.deleted.contains(&m),
            "the member goes with its family"
        );
        let mut run = editor_core::Recording::start(&doc, Tol::witness());
        splice.apply(&mut run).expect("the splice lands");
        round_trips("delete by continuation", run.doc());
    }
}

/// A hand-written file with an anonymous source cycle under a placement:
/// the placement's anonymous source re-pointed at itself.
#[test]
fn the_load_refuses_an_anonymous_self_source() {
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
    assert!(
        matches!(
            loaded,
            Err(editor_core::PersistError::Snapshot(
                editor_core::SnapshotError::DefinitionCycle { .. }
            ))
        ),
        "{:?}",
        loaded.map(|_| ())
    );
}

/// A member placement of a 1-D family's source, the source redefined onto
/// a 2-D family: the member's one index no longer matches the rank.
#[test]
fn a_member_source_redefined_onto_a_family_of_another_rank() {
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
    assert!(
        matches!(got, Err(EditError::IndexedRead { .. })),
        "{:?}",
        got.map(|_| ())
    );
}

/// A pattern of a plain copy of a body source, the source redefined onto a
/// family: the pattern would repeat a family.
#[test]
fn a_pattern_of_a_copy_whose_source_becomes_a_family() {
    let doc = ProfileDoc::empty_derived("rc-pattern-of-copy", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = crate::fixture::row(doc, b, ("k", 2), editor_core::TargetAxis::X, 2.0);
    let doc = declared(doc, "S", VarDecl::source(Operand::Node(b)));
    let (doc, c) = insert(doc, placed(name("S")));
    let (doc, _, _) = crate::fixture::row(doc, c, ("m", 2), editor_core::TargetAxis::Y, 2.0);
    let got = define(&doc, "S", VarDecl::source(Operand::Node(row)));
    assert!(
        matches!(got, Err(EditError::IndicesUnrelated { .. })),
        "{:?}",
        got.map(|_| ())
    );
}

/// **A member placement's `on` side selecting its family source**
/// (`matetool::on_read` writes `Var(source)` for members too):
/// `m = place(S[2])` into the world, its mate `on` side a plane of
/// `S.bottom` (the family source whole, as the placement editor writes
/// it), onto the floor: admitted, and it evaluates.
#[test]
fn a_member_placements_on_side_in_its_family_source() {
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
    let on = Operand::select(Operand::Var(source), vec![bottom]);
    let placed_by = squared(&doc, mate(plane(on), flip(ground)), &[]);
    let written = doc
        .apply(
            &DocEdit::SetConstraints {
                placement: copy,
                constraints: placed_by.bundle(),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect("admitted");
    let ev = evaluated(&written.doc);
    assert!(
        crate::blocks::failure(&ev, copy).is_none(),
        "the member evaluates: {:?}",
        crate::blocks::failure(&ev, copy)
    );
}

/// **A selection of a source is an edge to the source's body**: `a = B`;
/// a shell of `a.top`; a copy `c` of B inserted after the shell; `a = c`.
/// The shell now depends on `c`, and evaluates after it.
#[test]
fn a_selection_of_a_source_orders_after_the_sources_body() {
    let doc = ProfileDoc::empty_derived("rc-select-order", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let doc = declared(doc, "a", VarDecl::source(Operand::Node(b)));
    let (doc, shell) = insert(
        doc,
        Node::Shell {
            thickness: len(0.1),
            open: Operand::select(name("a"), vec![top(b)]),
        },
    );
    let (doc, c) = insert(doc, placed(Operand::Node(b)));
    let after = define(&doc, "a", VarDecl::source(Operand::Node(c))).expect("admitted");
    assert!(
        after.upstream(shell).contains(&c),
        "the shell depends on c: {:?}",
        after.upstream(shell)
    );
    assert!(
        crate::blocks::failure(&evaluated(&after), shell).is_none(),
        "the shell evaluates after c"
    );
}
