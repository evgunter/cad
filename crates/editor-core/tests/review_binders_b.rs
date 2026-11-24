//! **Review B of the binder kernel PR (7506)**: rows written to falsify
//! the PR's claims about names, the splice, the doors and containment
//! (D10 `[repetition]`). Every node is inserted as written ([`put`]),
//! never through `fixture::insert`, which rewrites a family read at a
//! one-body seat to `xs[k]`, splices a bare yield and yields every node
//! reaching an index on its own.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::blocks::block;
use crate::fixture;
use editor_core::{
    Bodies, BodyRead, DocEdit, EditError, Formula, Node, Operand, ProfileDoc, RecipeNodeId, Spell,
    VarDecl, VarId, VarName, VarRef, YieldsFault,
};
use geom_core::Tol;

fn put(doc: ProfileDoc, node: editor_core::AuthoredNode) -> (ProfileDoc, RecipeNodeId) {
    let (doc, minted) = fixture::step(
        doc,
        DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
    );
    (doc, minted.expect("the insert mints the node"))
}

fn try_put(
    doc: &ProfileDoc,
    node: editor_core::AuthoredNode,
) -> Result<(ProfileDoc, RecipeNodeId), EditError> {
    doc.apply(
        &DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .map(|a| (a.doc, a.record.minted.unwrap()))
}

fn set_yields(doc: &ProfileDoc, binder: VarId, yields: &[VarId]) -> Result<ProfileDoc, EditError> {
    doc.apply(
        &DocEdit::SetYields {
            binder: binder.into(),
            yields: yields.iter().map(|&y| VarRef::from(y)).collect(),
        },
        Tol::witness(),
    )
    .map(|a| a.doc)
}

/// A plate, an index `k = index(n)` and a row of `n` copies of a small
/// block in the plate's space, member `k` slid `k` along x; `yielded`
/// says whether the binder yields the row.
struct Row {
    doc: ProfileDoc,
    plate: RecipeNodeId,
    k: VarId,
    row: RecipeNodeId,
    copies: VarId,
}

fn row_doc(label: &str, n: i64, yielded: bool) -> Row {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, plate) = block(doc, (-1.0, 10.0), (-1.0, 1.0), 0.5);
    let (doc, small) = block(doc, (0.0, 0.4), (-0.2, 0.2), 1.0);
    let (doc, k) = fixture::index(doc, "k", n);
    let target = fixture::space(&doc, plate);
    let placed = editor_core::Placed::of(
        &doc,
        Vec::new(),
        fixture::slides_along(k, [1.0, 0.0, 0.0], 1.0),
    );
    let (doc, row) = put(
        doc,
        Node::place(
            BodyRead::plain(Operand::Node(small)),
            Operand::Var(target),
            placed.bundle(),
        ),
    );
    let copies = doc.output(row, 0).unwrap();
    let doc = if yielded {
        set_yields(&doc, k, &[copies]).expect("the binder yields the row")
    } else {
        doc
    };
    Row {
        doc,
        plate,
        k,
        row,
        copies,
    }
}

fn union_of(entries: Vec<Spell<BodyRead<Formula>>>) -> editor_core::AuthoredNode {
    Node::Union {
        members: Bodies::Spelled(entries),
    }
}

fn one(id: RecipeNodeId) -> Spell<BodyRead<Formula>> {
    Spell::One(BodyRead::plain(Operand::Node(id)))
}

fn all(id: RecipeNodeId) -> Spell<BodyRead<Formula>> {
    Spell::All(BodyRead::plain(Operand::Node(id)))
}

/// The member keys a node's table names, as the debug text of each
/// `Member { at: [..] }` segment, at any depth.
fn member_keys(ev: &editor_core::Evaluation<f64>, id: RecipeNodeId) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for (name, _) in fixture::table(ev, id).iter() {
        let text = format!("{name:?}");
        for (at, _) in text.match_indices("Member { at: [") {
            let rest = &text[at..];
            let end = rest.find(']').unwrap();
            out.insert(rest[..=end].to_owned());
        }
    }
    out
}

/// The names of a node's table, its own id written out of them.
fn names(ev: &editor_core::Evaluation<f64>, id: RecipeNodeId) -> BTreeSet<String> {
    let own = format!("{id:?}");
    fixture::table(ev, id)
        .iter()
        .map(|(name, _)| format!("{name:?}").replace(&own, "SELF"))
        .collect()
}

fn run(doc: &ProfileDoc) -> editor_core::Evaluation<f64> {
    fixture::run(doc, &Default::default())
}

// ---- Claim 2: names ----

/// `Union([…xs])` and `Union(xs)` name alike (ruling 8).
#[test]
fn rb_a_splice_alone_names_as_the_family_read_whole() {
    let r = row_doc("rb-splice-whole", 3, true);
    let (a, ua) = put(r.doc.clone(), union_of(vec![all(r.row)]));
    let (b, ub) = put(
        r.doc.clone(),
        Node::Union {
            members: Bodies::Family(BodyRead::plain(Operand::Node(r.row))),
        },
    );
    let (ea, eb) = (run(&a), run(&b));
    assert!(ea.value(ua).is_some(), "{:?}", ea.result(ua));
    assert_eq!(names(&ea, ua), names(&eb, ub));
}

/// `[plate, …xs]` names each member by its read, the same as `xs` read
/// whole: the member keys of the splice are the row's.
#[test]
fn rb_a_splice_beside_a_body_names_members_by_the_binders_index() {
    let r = row_doc("rb-splice-beside", 3, true);
    let (doc, u) = put(r.doc.clone(), union_of(vec![one(r.plate), all(r.row)]));
    let ev = run(&doc);
    assert!(ev.value(u).is_some(), "{:?}", ev.result(u));
    let keys = member_keys(&ev, u);
    eprintln!("{keys:#?}");
    assert_eq!(keys.len(), 3, "each member keyed by k: {keys:#?}");
    let k = format!("{:?}", r.k);
    assert!(keys.iter().all(|key| key.contains(&k)), "keyed by k's id");
}

/// Names are stable under a count edit: members 0..3 keep their names
/// when the count goes to 4.
#[test]
fn rb_names_are_stable_under_a_count_edit() {
    let r = row_doc("rb-count", 3, true);
    let (doc, u) = put(r.doc.clone(), union_of(vec![one(r.plate), all(r.row)]));
    let before = names(&run(&doc), u);
    let doc4 = fixture::step(doc, fixture::recount_edit(&r.doc, r.k, Formula::count(4))).0;
    let ev4 = run(&doc4);
    let after = names(&ev4, u);
    // A face the plate and the copies merge into lists each member, so a
    // fourth member renames it; every other name stays.
    let lost: Vec<_> = before
        .difference(&after)
        .filter(|n| !n.contains("Merged"))
        .collect();
    assert!(lost.is_empty(), "names lost by a count edit: {lost:#?}");
    assert_eq!(member_keys(&ev4, u).len(), 4);
}

/// Names are stable under a re-ordering of the binder's yields.
#[test]
fn rb_names_are_stable_under_reordered_yields() {
    let r = row_doc("rb-reorder", 3, true);
    // A second yield: a copy of the plate placed per member.
    let target = fixture::space(&r.doc, r.plate);
    let placed = editor_core::Placed::of(
        &r.doc,
        Vec::new(),
        fixture::slides_along(r.k, [0.0, 0.0, 1.0], 2.0),
    );
    let (doc, second) = put(
        r.doc.clone(),
        Node::place(
            BodyRead::plain(Operand::Node(r.plate)),
            Operand::Var(target),
            placed.bundle(),
        ),
    );
    let second_v = doc.output(second, 0).unwrap();
    let ab = set_yields(&doc, r.k, &[r.copies, second_v]).unwrap();
    let ba = set_yields(&doc, r.k, &[second_v, r.copies]).unwrap();
    let (ab, u) = put(ab, union_of(vec![all(r.row), all(second)]));
    let (ba, u2) = put(ba, union_of(vec![all(r.row), all(second)]));
    let (e1, e2) = (run(&ab), run(&ba));
    assert!(e1.value(u).is_some(), "{:?}", e1.result(u));
    assert_eq!(u, u2, "the union is minted alike");
    assert_eq!(names(&e1, u), names(&e2, u2));
}

// ---- Claim 3: the splice ----

/// A bare yield in a spelled list refuses (ruling 7), typed.
#[test]
fn rb_a_bare_yield_in_a_list_refuses() {
    let r = row_doc("rb-bare", 3, true);
    for (label, node) in [
        ("union", union_of(vec![one(r.row), one(r.plate)])),
        (
            "intersect",
            Node::Intersect {
                members: Bodies::Spelled(vec![one(r.row), one(r.plate)]),
            },
        ),
        (
            "assemble",
            Node::Assemble {
                members: Bodies::Spelled(vec![one(r.row), one(r.plate)]),
            },
        ),
    ] {
        let refused = try_put(&r.doc, node).err();
        assert!(
            matches!(
                refused,
                Some(EditError::SlotVarKind {
                    found: editor_core::VarKind::Bodies,
                    ..
                })
            ),
            "{label}: {refused:?}"
        );
        eprintln!("{label}: {}", refused.unwrap());
    }
}

/// A splice of a single body (no yield) refuses by kind.
#[test]
fn rb_a_splice_of_one_body_refuses() {
    let r = row_doc("rb-splice-body", 3, true);
    let refused = try_put(&r.doc, union_of(vec![one(r.row), all(r.plate)])).err();
    eprintln!("{refused:?}");
    assert!(refused.is_some(), "a splice of one body is admitted");
}

/// A subtract's tool is a list seat (DESIGN `[repetition]`):
/// `subtract(plate, holes)`, the tool the family read whole, is one plate
/// with every member cut.
#[test]
fn rb_a_subtract_tool_takes_a_family_whole() {
    let r = row_doc("rb-subtract-tool", 3, true);
    let got = try_put(
        &r.doc,
        Node::Subtract {
            from: BodyRead::plain(Operand::Node(r.plate)),
            tool: editor_core::Bodies::Family(BodyRead::plain(Operand::Node(r.row))),
        },
    );
    match &got {
        Err(e) => eprintln!("subtract(plate, holes) refused: {e}\n{e:?}"),
        Ok((doc, cut)) => {
            let ev = run(doc);
            eprintln!("subtract(plate, holes) = {:?}", ev.result(*cut).map(|_| ()));
        }
    }
    assert!(got.is_ok(), "subtract(plate, holes) is refused");
}

/// The splice's edge cases: a family of zero, of one, and of a member
/// that refuses.
#[test]
fn rb_splice_edge_cases_report() {
    for n in [0, 1] {
        let r = row_doc(&format!("rb-splice-{n}"), n, true);
        let (doc, u) = put(r.doc.clone(), union_of(vec![one(r.plate), all(r.row)]));
        let ev = run(&doc);
        eprintln!(
            "[plate, …xs] with {n} members: value? {} result {:?}",
            ev.value(u).is_some(),
            ev.result(u)
                .map(|r| format!("{r:?}").chars().take(300).collect::<String>())
        );
        if n == 1 {
            assert!(ev.value(u).is_some(), "a family of one splices");
        }
    }
    // A member that refuses: member 2's slide divides by `k - 2`.
    let doc = ProfileDoc::empty_derived("rb-splice-refuse", Tol::witness());
    let (doc, plate) = block(doc, (-1.0, 10.0), (-1.0, 1.0), 0.5);
    let (doc, small) = block(doc, (0.0, 0.4), (-0.2, 0.2), 1.0);
    let (doc, k) = fixture::index(doc, "k", 3);
    let target = fixture::space(&doc, plate);
    let denom = Formula::sub(fixture::scalar_of(k), Formula::scalar(2.0).unwrap()).unwrap();
    let slide = Formula::div(fixture::len(1.0), denom).unwrap();
    let placed = editor_core::Placed::of(
        &doc,
        Vec::new(),
        vec![(editor_core::Coord::slide(editor_core::TargetAxis::X), slide)],
    );
    let (doc, row) = put(
        doc,
        Node::place(
            BodyRead::plain(Operand::Node(small)),
            Operand::Var(target),
            placed.bundle(),
        ),
    );
    let doc = set_yields(&doc, k, &[doc.output(row, 0).unwrap()]).unwrap();
    let (doc, u) = put(doc, union_of(vec![one(plate), all(row)]));
    let ev = run(&doc);
    let said = ev
        .result(u)
        .map(|r| format!("{r:?}").chars().take(400).collect::<String>());
    eprintln!("[plate, …xs] with member 2 refusing: {said:?}");
    assert!(
        ev.value(u).is_none(),
        "a refusing member refuses the splice"
    );
}

// ---- Claim 4: the doors ----

#[test]
fn rb_set_yields_refusals_are_typed_and_say_what_to_do() {
    let r = row_doc("rb-doors", 3, true);
    // A further index of the binder.
    let doc = fixture::step(
        r.doc.clone(),
        DocEdit::DeclareVar {
            name: VarName::new("j").unwrap(),
            def: VarDecl::Index {
                count: Formula::count(2),
                after: Some(Formula::var(r.k, editor_core::Dimension::Count)),
            },
        },
    )
    .0;
    let j = doc.var_named("j").unwrap();
    let cases: Vec<(&str, Result<ProfileDoc, EditError>)> = vec![
        ("not a binder (after)", set_yields(&doc, j, &[r.copies])),
        ("not a binder (no index)", set_yields(&doc, r.copies, &[])),
        ("index", set_yields(&doc, r.k, &[j])),
        ("twice", set_yields(&doc, r.k, &[r.copies, r.copies])),
    ];
    for (label, got) in cases {
        let e = got.expect_err(label);
        assert!(matches!(e, EditError::Yields { .. }), "{label}: {e:?}");
        eprintln!("{label}: {e}");
    }
    // Elsewhere: a second binder yields the row.
    let (doc2, m) = fixture::index(doc, "m", 2);
    let e = set_yields(&doc2, m, &[r.copies]).expect_err("elsewhere");
    assert!(
        matches!(
            e,
            EditError::Yields {
                fault: YieldsFault::Elsewhere { .. },
                ..
            }
        ),
        "{e:?}"
    );
    eprintln!("elsewhere: {e}");
}

/// A binder that yields nothing refuses at evaluation, typed.
#[test]
fn rb_yields_nothing_refuses_at_evaluation() {
    let r = row_doc("rb-nothing", 3, false);
    let ev = run(&r.doc);
    let res = format!("{:?}", ev.result(r.row));
    assert!(res.contains("YieldsNothing"), "{res}");
}

// ---- Containment ----

/// A k-reader that is no yield, read by a union outside the binder:
/// the union reads k through it, so it repeats. Report what it is.
#[test]
fn rb_a_non_yield_k_reader_read_outside_report() {
    let r = row_doc("rb-contain", 3, false);
    // a second, yielded placement makes the binder yield something.
    let target = fixture::space(&r.doc, r.plate);
    let placed = editor_core::Placed::of(
        &r.doc,
        Vec::new(),
        fixture::slides_along(r.k, [0.0, 0.0, 1.0], 2.0),
    );
    let (doc, other) = put(
        r.doc.clone(),
        Node::place(
            BodyRead::plain(Operand::Node(r.plate)),
            Operand::Var(target),
            placed.bundle(),
        ),
    );
    let doc = set_yields(&doc, r.k, &[doc.output(other, 0).unwrap()]).unwrap();
    let got = try_put(&doc, union_of(vec![one(r.plate), one(r.row)]));
    match got {
        Err(e) => eprintln!("union(plate, row-not-yielded) refused: {e}"),
        Ok((doc, u)) => {
            let ev = run(&doc);
            eprintln!(
                "union(plate, row-not-yielded) admitted; reach {:?}; value kind {:?}",
                doc.node_reach(u),
                ev.value(u).map(|v| v.payload.kind_name())
            );
            let product = doc.apply(
                &DocEdit::SetProduct {
                    product: Some(Operand::Node(u)),
                },
                Tol::witness(),
            );
            eprintln!("as the product: {:?}", product.err().map(|e| e.to_string()));
        }
    }
}

/// Yielding a variable that reads no index makes it a family of
/// identical members, and an existing one-body reader of it refuses.
#[test]
fn rb_yielding_a_constant_report() {
    let r = row_doc("rb-const-yield", 3, true);
    let plate_v = r.doc.outputs(r.plate)[0];
    let got = set_yields(&r.doc, r.k, &[r.copies, plate_v]);
    match got {
        Err(e) => eprintln!("yield of a constant refused: {e}"),
        Ok(doc) => {
            let ev = run(&doc);
            eprintln!(
                "yield of a constant admitted: plate kind {:?}",
                ev.value(r.plate).map(|v| v.payload.kind_name())
            );
        }
    }
}

/// Diagnostic: the corpus `heat_sink`'s per-node shape, to diff against
/// main.
#[test]
fn rb_heat_sink_shape_dump() {
    let doc = crate::corpus::documents()
        .into_iter()
        .find(|d| d.name == "heat_sink")
        .unwrap();
    let ev = crate::corpus::eval::<f64>(&doc.doc);
    for id in doc.doc.ids() {
        let node = doc
            .doc
            .node(id)
            .map(|n| format!("{n:?}").chars().take(18).collect::<String>());
        let kind = ev.value(id).map(|v| v.payload.kind_name());
        let n = ev.table_at(id).map(|t| t.iter().count());
        let faces = ev
            .value(id)
            .map(|v| format!("{:?}", v.payload).matches("FaceKey").count());
        println!("HS {node:?} {kind:?} names={n:?} facekeys={faces:?}");
    }
}
