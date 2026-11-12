//! Re-check round 2 probes for PR 6856 (lane `placing-recheck-6856`).
//! A red row is a finding.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::block;
use crate::fixture::{insert, len, step};

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

fn top(b: editor_core::RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: b,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    }
}

pub fn refused_or_round_trips(what: &str, got: Result<ProfileDoc, EditError>) {
    match got {
        Err(e) => eprintln!("{what}: refused {e:?}"),
        Ok(doc) => {
            let text = editor_core::save(&doc, &[], Tol::witness());
            let text = text.unwrap_or_else(|e| panic!("{what}: admitted, save refuses: {e:?}"));
            let loaded = editor_core::load(&text, Tol::witness());
            assert!(
                loaded.is_ok(),
                "{what}: admitted, load refuses: {:?}",
                loaded.map(|_| ())
            );
            eprintln!("{what}: admitted and round-trips");
        }
    }
}

fn grid(doc: ProfileDoc, b: editor_core::RecipeNodeId) -> (ProfileDoc, editor_core::RecipeNodeId) {
    use crate::fixture::{family_of, index, index_of, scalar_of};
    let (doc, i) = index(doc, "i", 2);
    let (doc, j) = index_of(doc, "j", Formula::count(2), Some(i));
    family_of(
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
    )
}

/// `S = row`, `u = S[0]` (a member source), `S = grid` (rank 2): `u`'s
/// one index no longer matches. Refuse, or write what loads and evaluates.
#[test]
fn r2_a_member_source_redefined_onto_another_rank() {
    let doc = ProfileDoc::empty_derived("r2-member-rank", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = crate::fixture::row(doc, b, ("k", 2), editor_core::TargetAxis::X, 2.0);
    let (doc, grid) = grid(doc, b);
    let doc = declared(doc, "S", VarDecl::source(Operand::Node(row)));
    let doc = declared(
        doc,
        "u",
        VarDecl::source(BodyRead::indexed(name("S"), vec![Formula::count(0)])),
    );
    let direct = define(
        &doc,
        "u",
        VarDecl::source(BodyRead::indexed(Operand::Node(grid), vec![Formula::count(0)])),
    );
    eprintln!("u = grid[0] directly: {:?}", direct.as_ref().map(|_| ()));
    let (doc, c) = insert(doc, placed(name("u")));
    let got = define(&doc, "S", VarDecl::source(Operand::Node(grid)));
    if let Ok(after) = &got {
        let ev: editor_core::Evaluation<f64> = editor_core::evaluate(
            after,
            None,
            &editor_core::CancelToken::new(),
            &editor_core::EvalOptions::default(),
            Tol::witness(),
        );
        eprintln!(
            "rank: eval c {:?}",
            ev.result(c).map(|r| format!("{r:?}").chars().take(200).collect::<String>())
        );
    }
    refused_or_round_trips("S = grid under u = S[0]", got);
}

/// A selection through a chain: `S = row`, `T = S`, shell of `T[0].top`;
/// `S = D`.
#[test]
fn r2_a_member_selection_through_a_chain() {
    let doc = ProfileDoc::empty_derived("r2-select-chain", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, d) = block(doc, (0.0, 2.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = crate::fixture::row(doc, b, ("k", 2), editor_core::TargetAxis::X, 2.0);
    let doc = declared(doc, "S", VarDecl::source(Operand::Node(row)));
    let doc = declared(doc, "T", VarDecl::source(name("S")));
    let (doc, _) = insert(
        doc,
        Node::Shell {
            thickness: len(0.1),
            open: Operand::select(
                BodyRead::indexed(name("T"), vec![Formula::count(0)]),
                vec![top(b)],
            ),
        },
    );
    let got = define(&doc, "S", VarDecl::source(Operand::Node(d)));
    assert!(got.is_err(), "admitted");
    refused_or_round_trips("shell of T[0].top, S = D", got);
}

/// A member of a member: `S = row`, `u = S[0]`, `w = u[0]`, and with the
/// converse `S = grid` so `u` is a family and `w` a member of a member.
#[test]
fn r2_a_member_of_a_member_source() {
    let doc = ProfileDoc::empty_derived("r2-member-member", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = crate::fixture::row(doc, b, ("k", 2), editor_core::TargetAxis::X, 2.0);
    let (doc, grid) = grid(doc, b);
    let doc = declared(doc, "S", VarDecl::source(Operand::Node(grid)));
    let doc = declared(
        doc,
        "u",
        VarDecl::source(BodyRead::indexed(name("S"), vec![Formula::count(0)])),
    );
    let w = doc.apply(
        &DocEdit::DeclareVar {
            name: VarName::from_static("w"),
            def: VarDecl::source(BodyRead::indexed(name("u"), vec![Formula::count(1)])),
        },
        Tol::witness(),
    );
    eprintln!("w = u[1] with u = grid[0]: {:?}", w.as_ref().map(|_| ()));
    if let Ok(a) = w {
        let doc = a.doc;
        refused_or_round_trips("w = u[1] declared", Ok(doc.clone()));
        let got = define(&doc, "S", VarDecl::source(Operand::Node(row)));
        refused_or_round_trips("S = row under w = S[0][1]", got);
    }
}

/// **The scratch door the offer asks per frame**: a redefinition's cost
/// as the document grows (N blocks, each a named source, a placement and a
/// shell of its top).
#[test]
fn r2_the_redefinition_door_cost_scales() {
    for n in [10usize, 40, 160] {
        let mut doc = ProfileDoc::empty_derived("r2-cost", Tol::witness());
        let names: Vec<&'static str> = (0..n)
            .map(|i| &*Box::leak(format!("s{i}").into_boxed_str()))
            .collect();
        let mut blocks = Vec::new();
        for (i, nm) in names.iter().enumerate() {
            let x = i as f64 * 3.0;
            let (d, b) = block(doc, (x, x + 1.0), (0.0, 1.0), 1.0);
            let d = declared(d, nm, VarDecl::source(Operand::Node(b)));
            let (d, _) = insert(d, placed(name(nm)));
            let (d, _) = insert(
                d,
                Node::Shell {
                    thickness: len(0.1),
                    open: Operand::select(name(nm), vec![top(b)]),
                },
            );
            doc = d;
            blocks.push(b);
        }
        let t = std::time::Instant::now();
        let reps = 5;
        for _ in 0..reps {
            let _ = define(&doc, names[0], VarDecl::source(Operand::Node(blocks[0]))).unwrap();
        }
        eprintln!(
            "N = {n} ({} nodes): one DefineVar door run {:?}",
            doc.ids().len(),
            t.elapsed() / reps
        );
    }
}
