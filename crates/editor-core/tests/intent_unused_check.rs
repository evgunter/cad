//! **What the person made and nothing uses is a finding** (D10
//! `[operations]` "The product"; `docs/INTENT-STAGE3-SPEC.md` test 26).
//! An operation they inserted none of whose outputs is used, and a
//! variable they named that nothing uses, are reported at the tip of
//! each dead chain ([`editor_core::Doc::unused`]) by the `unused` check.
//! A read, being the product and being what an assertion states are
//! uses; an output nobody named and an anonymous variable are
//! byproducts, never findings; the world is exempt.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, cap_ref, insert, len, step};
use editor_core::{
    AssertionRelation, CapEnd, CheckEvidence, CheckId, ChecksConfig, Dimension, DocEdit,
    DocumentId, EvalOptions, Formula, FreeVar, MeasurePrimitive, Node, Operand, ProfileDoc,
    RecipeNodeId, Unused, VarDecl, VarId, VarName, VarRef, run_checks,
};
use geom_core::Tol;

/// A unit block on a frame of its own, unplaced: the document and the
/// extrude.
fn block(doc: ProfileDoc, x: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = fixture::on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(x, 0.5, 0.5)],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    )
}

/// `node`'s value standing in the world, joined to the product.
fn placed(doc: ProfileDoc, node: impl Into<Operand>) -> ProfileDoc {
    insert(doc, fixture::stood(node.into())).0
}

fn doc(label: &str) -> ProfileDoc {
    ProfileDoc::empty(DocumentId::derive(label), Tol::witness())
}

fn named(name: &'static str) -> VarName {
    VarName::from_static(name)
}

fn rename(doc: ProfileDoc, var: VarId, name: Option<VarName>) -> ProfileDoc {
    step(
        doc,
        DocEdit::RenameVar {
            var: VarRef::Id(var),
            name,
        },
    )
    .0
}

fn declare(doc: ProfileDoc, name: &'static str) -> (ProfileDoc, VarId) {
    let doc = step(
        doc,
        DocEdit::DeclareVar {
            name: named(name),
            def: VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.75)),
        },
    )
    .0;
    let var = doc.var_named(name).expect("declared");
    (doc, var)
}

fn quiet() -> Unused {
    Unused::default()
}

fn ops(operations: &[RecipeNodeId]) -> Unused {
    Unused {
        operations: operations.to_vec(),
        vars: Vec::new(),
    }
}

/// **A split's unnamed half is a byproduct**; named, it is wanted for
/// itself and fires; its name cleared again, it is quiet.
///
/// *Breaks if* an output is reported by being unread alone, or a name
/// does not make it deliberate.
#[test]
fn a_split_half_fires_only_while_named() {
    let (doc, block) = block(doc("unused-split"), 0.5);
    let tool = fixture::plane_on(&doc, block, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: block.into(),
            tool,
        },
    );
    let doc = placed(doc, Operand::output(split, 0));
    assert_eq!(doc.unused(), quiet(), "the unread half is unnamed");
    let below = doc.output(split, 1).expect("the split's second half");
    let doc = rename(doc, below, Some(named("offcut")));
    assert_eq!(
        doc.unused(),
        Unused {
            operations: Vec::new(),
            vars: vec![below],
        },
        "named, the half is deliberate and nothing uses it"
    );
    let doc = rename(doc, below, None);
    assert_eq!(doc.unused(), quiet(), "un-named, a byproduct again");
}

/// **A construction's space and a revolve's axis are byproducts**: a
/// placed revolve is quiet, though neither its space nor its axis is
/// read.
#[test]
fn a_construction_space_and_a_revolve_axis_are_quiet() {
    let doc = doc("unused-revolve");
    let (doc, profile) = fixture::on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(2.0, 0.0, 0.5)],
    );
    let (doc, revolve) = insert(
        doc,
        Node::Revolve {
            profile: profile.into(),
            axis_through: [len(0.0), len(0.0)],
            axis_heading: fixture::along_y(),
            angle: editor_core::test_support::ang(std::f64::consts::TAU),
        },
    );
    assert!(
        doc.outputs(revolve).len() >= 3,
        "the body, the axis and the space"
    );
    let doc = placed(doc, Operand::output(revolve, 0));
    assert_eq!(doc.unused(), quiet());
}

/// **A fresh construction nobody placed fires**, at its tip alone: its
/// profile is read by it. Deleting the tip surfaces the profile.
///
/// *Breaks if* the walk reports a whole dead chain, or stops at the
/// first tip.
#[test]
fn a_dead_chain_reports_its_tip_and_then_the_next() {
    let doc = doc("unused-chain");
    let (doc, kept) = block(doc, 0.5);
    let doc = placed(doc, kept);
    let (doc, spare) = block(doc, 4.0);
    assert_eq!(doc.unused(), ops(&[spare]), "the unplaced construction");
    let profile = doc.upstream(spare)[0];
    let doc = step(doc, DocEdit::DeleteNode { id: spare }).0;
    assert_eq!(doc.unused(), ops(&[profile]), "its profile surfaces");
    let doc = step(doc, DocEdit::DeleteNode { id: profile }).0;
    assert_eq!(doc.unused(), quiet());
}

/// **A named parameter nothing reads fires**; read by a slot, it is
/// quiet; read only by a definition nothing reads, the definition is
/// the tip.
#[test]
fn a_named_parameter_nothing_reads_fires() {
    let doc = doc("unused-param");
    let (doc, kept) = block(doc, 0.5);
    let doc = placed(doc, kept);
    let (doc, depth) = declare(doc, "depth");
    assert_eq!(
        doc.unused(),
        Unused {
            operations: Vec::new(),
            vars: vec![depth],
        }
    );
    let doc = step(
        doc,
        DocEdit::DeclareVar {
            name: named("twice"),
            def: VarDecl::Defined(
                Formula::mul(Formula::var(depth, Dimension::Length), fixture::scl(2.0)).unwrap(),
            ),
        },
    )
    .0;
    let twice = doc.var_named("twice").unwrap();
    assert_eq!(doc.unused().vars, vec![twice], "the tip is the definition");
}

/// **A reference body read only by an assertion is quiet**, and a
/// `Measure` nothing asserts fires.
#[test]
fn an_assertion_is_a_use_and_an_unasserted_measure_fires() {
    let doc = doc("unused-measure");
    let (doc, kept) = block(doc, 0.5);
    let doc = placed(doc, kept);
    let (doc, reference) = block(doc, 4.0);
    let caps = |n| [cap_ref(n, CapEnd::Start), cap_ref(n, CapEnd::End)];
    let (doc, measure) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        caps(reference).to_vec(),
    );
    assert_eq!(
        doc.unused(),
        ops(&[measure]),
        "the measure is the tip: it reads the reference"
    );
    let value = fixture::value_of(&doc, measure);
    let (doc, _) = insert(
        doc,
        Node::Assertion {
            value,
            bound: len(0.5),
            relation: AssertionRelation::AtLeast,
        },
    );
    assert_eq!(doc.unused(), quiet(), "asserted, the chain is used");
}

/// **An indexed family is one subject**: a construction repeated over
/// `k` is one finding, and `k` is used by it.
#[test]
fn an_indexed_family_is_one_subject() {
    let doc = doc("unused-family");
    let (doc, kept) = block(doc, 0.5);
    let doc = placed(doc, kept);
    let (doc, k) = fixture::index(doc, "k", 3);
    let (doc, profile) = fixture::on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(4.0, 0.5, 0.5)],
    );
    let depth = Formula::mul(
        Formula::add(fixture::scalar_of(k), fixture::scl(1.0)).unwrap(),
        len(0.25),
    )
    .unwrap();
    let (doc, family) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: depth,
        },
    );
    assert_eq!(doc.unused(), ops(&[family]));
}

/// **The world is quiet**: an empty document reports nothing, and so
/// does one whose world nothing is placed into.
#[test]
fn the_world_is_quiet() {
    assert_eq!(doc("unused-world").unused(), quiet());
}

/// **The resident**: certified, default `Warn`, its subjects a node and
/// a variable spoken by name; `Off` skips it visibly.
#[test]
fn the_resident_reports_the_tips() {
    let doc = doc("unused-resident");
    let (doc, kept) = block(doc, 0.5);
    let doc = placed(doc, kept);
    let (doc, spare) = block(doc, 4.0);
    let (doc, _) = declare(doc, "depth");
    let ev = fixture::run(&doc, &EvalOptions::default());
    let report = run_checks(&doc, &ev, &ChecksConfig::default(), Tol::witness()).unwrap();
    let unused: Vec<_> = report
        .findings
        .iter()
        .filter(|f| f.check == CheckId::Unused)
        .collect();
    assert_eq!(unused.len(), 2, "{}", report.spoken(&doc));
    assert_eq!(unused[0].subject, editor_core::FindingSubject::Node(spare));
    assert!(matches!(unused[1].evidence, CheckEvidence::Unused));
    let spoken = report.spoken(&doc);
    assert!(
        spoken.contains(
            "check unused: depth: used by nothing: not the product, not an assertion, and \
             nothing reads it — Recourse: use it, or delete it"
        ),
        "the variable is spoken by its name: {spoken}"
    );
    assert_eq!(CheckId::Unused.kind(), editor_core::CheckKind::Certified);
    let off = ChecksConfig {
        unused: editor_core::Advisory::Off,
        ..ChecksConfig::default()
    };
    let report = run_checks(&doc, &ev, &off, Tol::witness()).unwrap();
    assert!(report.skipped.contains(&CheckId::Unused));
    assert!(report.findings.iter().all(|f| f.check != CheckId::Unused));
}

/// **A part's unused things are the part's**: a host instancing a part
/// with an unused parameter reports nothing of it.
#[test]
fn a_parts_finding_stays_in_the_part() {
    let part = doc("unused-part");
    let (part, body) = block(part, 0.5);
    let part = placed(part, body);
    let (part, spare) = declare(part, "spare");
    assert_eq!(part.unused().vars, vec![spare], "the part's own finding");
    let mut store = fixture::resolver::PartStore::default();
    let part_ref = store.insert(part, Tol::witness());
    let (host, _) = insert(
        doc("unused-host"),
        Node::instantiate_part(
            part_ref,
            Operand::Var(VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
    assert_eq!(host.unused(), quiet());
    let ev = fixture::run(&host, &fixture::resolver::with_resolver(store));
    let report = run_checks(&host, &ev, &ChecksConfig::default(), Tol::witness()).unwrap();
    assert!(
        report.findings.iter().all(|f| f.check != CheckId::Unused),
        "{}",
        report.spoken(&host)
    );
}
