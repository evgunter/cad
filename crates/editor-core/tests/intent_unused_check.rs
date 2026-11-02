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
    AssertionRelation, CapEnd, CheckId, ChecksConfig, Dimension, DocEdit, DocumentId, EvalOptions,
    Formula, FreeVar, MeasurePrimitive, Node, Operand, ProfileDoc, RecipeNodeId, Unused, VarDecl,
    VarId, VarName, VarRef, run_checks,
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
        Node::Assertion(editor_core::Claim::Value {
            value,
            bound: len(0.5),
            relation: AssertionRelation::AtLeast,
        }),
    );
    assert_eq!(doc.unused(), quiet(), "asserted, the chain is used");
}

/// **An indexed family is one subject**: a construction repeated over
/// `k` is one node, so the resident reports it once however many
/// members it evaluates to, and `k` is used by it and not reported.
///
/// *Breaks if* the resident reports per member, or a read of an index
/// through a definition is not a use.
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
    let ev = fixture::run(&doc, &EvalOptions::default());
    let report = run_checks(&doc, &ev, &ChecksConfig::default(), Tol::witness()).unwrap();
    let subjects: Vec<_> = report
        .findings
        .iter()
        .filter(|f| f.check == CheckId::Unused)
        .map(|f| f.subject)
        .collect();
    assert_eq!(
        subjects,
        [editor_core::FindingSubject::Node(family)],
        "{}",
        report.spoken(&doc)
    );
}

/// **The world is quiet**: a document whose world nothing is placed
/// into reports nothing, and so does one whose world the person named.
///
/// *Breaks if* the exemption holds on one pass and not the other.
#[test]
fn the_world_is_quiet() {
    let doc = doc("unused-world");
    assert_eq!(doc.unused(), quiet());
    let named = rename(doc, VarId::WORLD, Some(named("ground")));
    assert_eq!(
        named.var_name(VarId::WORLD).map(VarName::as_str),
        Some("ground")
    );
    assert_eq!(named.unused(), quiet());
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

/// A block placed in the world, joined to the product: what keeps the
/// rows below from reporting their own scaffolding.
fn kept(doc: ProfileDoc) -> ProfileDoc {
    let (doc, b) = block(doc, 0.5);
    placed(doc, b)
}

fn profile(doc: ProfileDoc, x: f64) -> (ProfileDoc, RecipeNodeId) {
    fixture::on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(x, 0.5, 0.5)],
    )
}

fn extrude(doc: ProfileDoc, profile: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    )
}

fn delete(doc: ProfileDoc, id: RecipeNodeId) -> ProfileDoc {
    step(doc, DocEdit::DeleteNode { id }).0
}

/// **A diamond's two dead readers are both tips**, and what they share
/// surfaces only once both are gone.
#[test]
fn a_shared_read_surfaces_when_its_last_reader_goes() {
    let doc = kept(doc("unused-diamond"));
    let (doc, p) = profile(doc, 4.0);
    let (doc, a) = extrude(doc, p);
    let (doc, b) = extrude(doc, p);
    assert_eq!(doc.unused(), ops(&[a, b]));
    let doc = delete(doc, a);
    assert_eq!(doc.unused(), ops(&[b]));
    let doc = delete(doc, b);
    assert_eq!(doc.unused(), ops(&[p]));
}

/// **A chain through a byproduct**: a block placed into another
/// block's unnamed space. The placement is the one tip, the host used
/// through its space; deleting the tip surfaces two tips at once.
#[test]
fn a_space_read_is_a_use_of_its_construction() {
    let doc = kept(doc("unused-byproduct"));
    let (doc, host) = block(doc, 4.0);
    let (doc, guest) = block(doc, 8.0);
    let (doc, placement) = fixture::stood_in(doc, guest, host);
    assert_eq!(doc.unused(), ops(&[placement]));
    let doc = delete(doc, placement);
    assert_eq!(doc.unused(), ops(&[host, guest]));
}

/// **An assertion keeps a mid-chain body used**: deleting the dead tip
/// beyond it surfaces the next tip, never the asserted body.
#[test]
fn an_asserted_mid_chain_body_stays_used() {
    let doc = kept(doc("unused-midchain"));
    let (doc, host) = block(doc, 4.0);
    let (doc, mid) = block(doc, 8.0);
    let (doc, placement) = fixture::stood_in(doc, mid, host);
    let caps = [cap_ref(mid, CapEnd::Start), cap_ref(mid, CapEnd::End)];
    let (doc, measure) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        caps.to_vec(),
    );
    let value = fixture::value_of(&doc, measure);
    let (doc, _) = insert(
        doc,
        Node::Assertion(editor_core::Claim::Value {
            value,
            bound: len(0.5),
            relation: AssertionRelation::AtLeast,
        }),
    );
    assert_eq!(doc.unused(), ops(&[placement]));
    let doc = delete(doc, placement);
    assert_eq!(
        doc.unused(),
        ops(&[host]),
        "mid is asserted on; host surfaces"
    );
}

/// **A reported operation speaks for its named outputs**: a split with
/// both halves named and neither used is one finding naming both;
/// once one half is used, the other named half is reported as itself.
#[test]
fn a_folded_named_output_is_said_and_not_lost() {
    let doc = kept(doc("unused-fold"));
    let (doc, b) = block(doc, 4.0);
    let tool = fixture::plane_on(&doc, b, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: b.into(),
            tool,
        },
    );
    let above = doc.output(split, 0).unwrap();
    let below = doc.output(split, 1).unwrap();
    let doc = rename(doc, above, Some(named("top")));
    let doc = rename(doc, below, Some(named("offcut")));
    assert_eq!(doc.unused(), ops(&[split]));
    let ev = fixture::run(&doc, &EvalOptions::default());
    let report = run_checks(&doc, &ev, &ChecksConfig::default(), Tol::witness()).unwrap();
    let spoken = report.spoken(&doc);
    assert!(
        spoken.contains("used by nothing, its outputs named top, offcut with it: not the product"),
        "{spoken}"
    );
    let doc = placed(doc, Operand::output(split, 0));
    assert_eq!(
        doc.unused(),
        Unused {
            operations: Vec::new(),
            vars: vec![below],
        }
    );
}

/// **An instance nothing uses fires in the host**: the copy is not
/// joined to the product, so the instance is the host's own tip.
#[test]
fn an_unused_instance_fires_in_the_host() {
    let mut store = fixture::resolver::PartStore::default();
    let part_ref = store.insert(kept(doc("unused-instance-part")), Tol::witness());
    let host = kept(doc("unused-instance-host"));
    let (host, minted) = step(
        host,
        DocEdit::InsertNode {
            node: Box::new(Node::instantiate_part(
                part_ref,
                Operand::Var(VarId::WORLD),
                editor_core::stand().bundle(),
            )),
            fresh: Vec::new(),
        },
    );
    let instance = minted.unwrap();
    assert_eq!(host.unused(), ops(&[instance]));
}

/// **A refusal discards the unused findings** (today's registry, filed
/// as `work/coincide/a-checks-refusal-discards-every-residents-findings.md`):
/// a document whose placement fails reports its unused parameter when
/// the `unused` check runs alone, and nothing under the default
/// configuration, which refuses the whole report.
#[test]
fn a_product_refusal_discards_the_unused_findings() {
    let (doc, opts) = failed_placement_world("unused-refuse");
    let (doc, spare) = declare(doc, "spare");
    assert!(doc.unused().vars.contains(&spare), "the premise");
    let ev = fixture::run(&doc, &opts);
    let alone = ChecksConfig {
        connectedness: editor_core::Severity::Off,
        unproven_coincidence: editor_core::Advisory::Off,
        at_rest: editor_core::Severity::Off,
        ..ChecksConfig::default()
    };
    let report = run_checks(&doc, &ev, &alone, Tol::witness()).expect("alone, it runs");
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.subject == editor_core::FindingSubject::Var(spare)),
        "{}",
        report.spoken(&doc)
    );
    assert!(
        run_checks(&doc, &ev, &ChecksConfig::default(), Tol::witness()).is_err(),
        "the default configuration refuses, its unused finding with it"
    );
}

/// `docm5_subject`'s failed placement world: a mate naming no face
/// fails the placement, so the product refuses.
fn failed_placement_world(id: &str) -> (ProfileDoc, EvalOptions) {
    use editor_core::{EntityKind, RoleSeg, StableName};
    let mut store = fixture::resolver::PartStore::default();
    let part = ProfileDoc::empty_derived(&format!("{id}-part"), Tol::witness());
    let (part, body) = block(part, 0.0);
    let (part_ref, body) = store.insert_part((part, body), Tol::witness());
    let opts = fixture::resolver::with_resolver(store);
    let doc = ProfileDoc::empty_derived(id, Tol::witness());
    let (doc, base) = fixture::insert_pinned(doc, fixture::mated_instance(part_ref), &opts);
    let (doc, placed) = insert(doc, fixture::mated_instance(part_ref));
    let nowhere = StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId::new(0, 99),
        path: vec![RoleSeg::Cap(CapEnd::End)],
    };
    let mate = fixture::rest(
        fixture::head_at(
            placed,
            fixture::resolver::in_part(placed, body, CapEnd::Start),
        ),
        fixture::head_at(base, nowhere),
    );
    (fixture::constrain_at(doc, placed, mate).0, opts)
}
