//! Review probes for PR 6856 (lane `placing-dual-6856-r1`): rows that try
//! to falsify the named-source claims. Each asserts the behaviour the
//! ruling asks for; a red row is a finding.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::block;
use crate::fixture::{insert, out, step};

use editor_core::{
    BodyRead, DocEdit, EditError, Formula, Node, Operand, ProfileDoc, VarDecl, VarId, VarName,
};
use geom_core::Tol;

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

fn cycle_doc() -> ProfileDoc {
    let doc = ProfileDoc::empty_derived("r1-source-cycle", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let doc = declared(doc, "a", VarDecl::source(Operand::Node(b)));
    declared(
        doc,
        "b",
        VarDecl::source(Operand::Name(VarName::from_static("a"))),
    )
}

/// **A cycle through two sources is refused** (VR3: a definition may not
/// reach its own variable): `a = B`, `b = a`, then `a = b`.
#[test]
fn probe_a_two_source_cycle_is_refused() {
    let doc = cycle_doc();
    let two = define(
        &doc,
        "a",
        VarDecl::source(Operand::Name(VarName::from_static("b"))),
    );
    assert!(two.is_err(), "a = b = a admitted");
}

/// **A source reading itself is refused**: `a = a`.
#[test]
fn probe_a_self_source_is_refused() {
    let doc = cycle_doc();
    let one = define(
        &doc,
        "a",
        VarDecl::source(Operand::Name(VarName::from_static("a"))),
    );
    assert!(one.is_err(), "a = a admitted");
}

/// **The load refuses a source cycle too** — a file holding `a = b`,
/// `b = a`, written by hand.
#[test]
fn probe_the_load_refuses_a_source_cycle() {
    let doc = ProfileDoc::empty_derived("r1-source-cycle-load", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let doc = declared(doc, "a", VarDecl::source(Operand::Node(b)));
    let doc = declared(
        doc,
        "b",
        VarDecl::source(Operand::Name(VarName::from_static("a"))),
    );
    let bb = doc.var_named("b").unwrap();
    let text = editor_core::save(&doc, &[], Tol::witness()).expect("saves");
    let spell = |var: VarId| serde_json::to_string(&var).expect("an id spells");
    let needle = format!("\"Source\": {}", spell(out(&doc, b)));
    assert_eq!(text.matches(&needle).count(), 1, "a's definition, once");
    let edited = text.replace(&needle, &format!("\"Source\": {}", spell(bb)));
    let loaded = editor_core::load(&edited, Tol::witness());
    assert!(loaded.is_err(), "a = b, b = a loads");
}

/// **A redefinition of a family's source as one body** leaves its member
/// placement reading `F[k]` of no family. The door should refuse what the
/// load refuses: either the DefineVar refuses, or the document it writes
/// round-trips.
#[test]
fn probe_a_family_source_redefined_as_one_body() {
    let (doc, cube) = crate::eval6_placers_over_instances::cube_doc("r1-family-source");
    let (doc, _, xs) = crate::fixture::linear(doc, cube, [1.0, 0.0, 0.0], 2.0, 3);
    let (doc, member) = insert(
        doc,
        Node::place(
            BodyRead::indexed(xs, vec![Formula::count(0)]),
            Operand::Var(VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
    let source = doc.copied_read(member).expect("a source");
    let doc = step(
        doc,
        DocEdit::RenameVar {
            var: source.into(),
            name: Some(VarName::from_static("row")),
        },
    )
    .0;
    let redefined = define(&doc, "row", VarDecl::source(Operand::Node(cube)));
    if let Ok(doc) = redefined {
        let text = editor_core::save(&doc, &[], Tol::witness()).expect("saves");
        let loaded = editor_core::load(&text, Tol::witness());
        assert!(
            loaded.is_ok(),
            "the door admitted row = cube under a member read, and the load refuses: {:?}",
            loaded.map(|_| ())
        );
    }
}

/// **`Node::place` and `Doc::place_read` agree on a body read by its
/// output variable.** `place_read(Var(out B))` mints a source; does
/// `Node::place(Var(out B))` too, or does the door refuse it?
#[test]
fn probe_node_place_of_an_output_var() {
    let doc = ProfileDoc::empty_derived("r1-output-var", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let body = Operand::Var(out(&doc, b));
    assert!(
        matches!(doc.place_read(body.clone()), Operand::Source(_)),
        "place_read wraps an output read"
    );
    let applied = doc.apply(
        &DocEdit::InsertNode {
            node: Box::new(Node::place(
                BodyRead::plain(body),
                Operand::Var(VarId::WORLD),
                editor_core::stand().bundle(),
            )),
            fresh: Vec::new(),
        },
        Tol::witness(),
    );
    assert!(
        applied.is_ok(),
        "Node::place(Var(out B)) refuses where place_read mints: {:?}",
        applied.map(|_| ())
    );
}

/// **The takeover door (the viewer's re-point, the source row, Python's
/// `take_over`) on a member placement's source**: `place(xs[0])` reads a
/// source of the family at index 0; taking it over onto one body writes
/// `source = cube` plain. The document it answers must save.
#[test]
fn probe_take_over_of_a_member_placements_source() {
    let (doc, cube) = crate::eval6_placers_over_instances::cube_doc("r1-take-over-member");
    let (doc, _, xs) = crate::fixture::linear(doc, cube, [1.0, 0.0, 0.0], 2.0, 3);
    let (doc, member) = insert(
        doc,
        Node::place(
            BodyRead::indexed(xs, vec![Formula::count(0)]),
            Operand::Var(VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
    let source = doc.copied_read(member).expect("a source");
    let done = editor_core::take_over(
        &doc,
        &[source],
        &Operand::Node(cube),
        &editor_core::EvalOptions::default(),
        &editor_core::CancelToken::new(),
        Tol::witness(),
    );
    if let Ok(done) = done {
        let saved = editor_core::save(&done.doc, &[], Tol::witness());
        assert!(
            saved.is_ok(),
            "the takeover answered a document that does not save: {:?}",
            saved.map(|_| ())
        );
    }
}

/// **The reader-count qualifier, reading (a)**: a lone placement whose
/// mate `on` side is a plane of a face selected in its own anonymous
/// source counts as one reader. Naming that `on` side's pose (`ax`) is a
/// rename, which recomputes nothing; the source is still read only by
/// its placement and its own on side, so the rename should be admitted.
#[test]
fn probe_naming_a_lone_placements_own_on_pose() {
    use crate::fixture::{bundle, flip, floor, fname, mate, plane, squared, stood, write_bundle};
    let doc = ProfileDoc::empty_derived("r1-qualifier", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, ground, _) = floor(doc, Vec::new());
    let (doc, copy) = insert(doc, stood(Operand::Node(b)));
    let source = doc.copied_read(copy).expect("a source");
    let bottom = fname(b, editor_core::RoleSeg::Cap(editor_core::CapEnd::Start));
    let m = mate(
        plane(Operand::select(Operand::Var(source), vec![bottom])),
        flip(ground),
    );
    let placed = squared(&doc, m, &[]);
    let (doc, _) = write_bundle(doc, copy, placed.bundle());
    assert_eq!(doc.var_name(source), None, "the source stays anonymous");
    let on = bundle(&doc, copy)
        .iter()
        .find_map(|c| c.reads().into_iter().next().map(|(_, &read)| read))
        .expect("the mate's on side");
    let renamed = doc.apply(
        &DocEdit::RenameVar {
            var: on.into(),
            name: Some(VarName::from_static("ax")),
        },
        Tol::witness(),
    );
    assert!(
        renamed.is_ok(),
        "naming the placement's own on-side pose refuses: {:?}",
        renamed.map(|_| ())
    );
}
