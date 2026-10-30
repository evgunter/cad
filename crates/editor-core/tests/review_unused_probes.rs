//! Review probes for PR 6081 (`Doc::unused`): each states what the
//! ratified rule says; a red one is a falsified claim.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, cap_ref, insert, len, step};
use editor_core::{
    AssertionRelation, CapEnd, CheckId, ChecksConfig, Dimension, DocEdit, DocumentId, FreeVar,
    MeasurePrimitive, Node, Operand, ProfileDoc, RecipeNodeId, Unused, VarDecl, VarId, VarName,
    VarRef, run_checks,
};
use geom_core::Tol;

fn doc(label: &str) -> ProfileDoc {
    ProfileDoc::empty(DocumentId::derive(label), Tol::witness())
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

fn block(doc: ProfileDoc, x: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = profile(doc, x);
    extrude(doc, p)
}

fn kept(doc: ProfileDoc) -> ProfileDoc {
    let (doc, b) = block(doc, 0.5);
    insert(doc, fixture::stood(Operand::from(b))).0
}

fn delete(doc: ProfileDoc, id: RecipeNodeId) -> ProfileDoc {
    step(doc, DocEdit::DeleteNode { id }).0
}

fn ops(o: &[RecipeNodeId]) -> Unused {
    Unused {
        operations: o.to_vec(),
        vars: Vec::new(),
    }
}

fn sorted(mut v: Vec<RecipeNodeId>) -> Vec<RecipeNodeId> {
    v.sort();
    v
}

/// Diamond: one profile read by two dead extrudes. Both extrudes are
/// tips; the profile surfaces only once both are gone.
#[test]
fn probe_diamond() {
    let d = kept(doc("probe-diamond"));
    let (d, p) = profile(d, 4.0);
    let (d, a) = extrude(d, p);
    let (d, b) = extrude(d, p);
    assert_eq!(d.unused(), ops(&sorted(vec![a, b])));
    let d = delete(d, a);
    assert_eq!(d.unused(), ops(&[b]));
    let d = delete(d, b);
    assert_eq!(d.unused(), ops(&[p]));
}

/// A chain through a byproduct: a block placed into another block's
/// (unnamed) `space`. The placement is the one tip; the host block is
/// used through its space, the placed block through the placement.
/// Deleting the tip surfaces two tips at once.
#[test]
fn probe_chain_through_a_byproduct_and_two_tips() {
    let d = kept(doc("probe-byproduct"));
    let (d, host) = block(d, 4.0);
    let (d, guest) = block(d, 8.0);
    let (d, pl) = fixture::stood_in(d, guest, host);
    assert_eq!(d.unused(), ops(&[pl]));
    let d = delete(d, pl);
    assert_eq!(d.unused(), ops(&sorted(vec![host, guest])));
}

/// An assertion reading a mid-chain body (through a measure): the
/// body stays used once the dead tip beyond it is deleted.
#[test]
fn probe_assertion_on_a_mid_chain_body() {
    let d = kept(doc("probe-midchain"));
    let (d, host) = block(d, 4.0);
    let (d, mid) = block(d, 8.0);
    let (d, pl) = fixture::stood_in(d, mid, host);
    let caps = [cap_ref(mid, CapEnd::Start), cap_ref(mid, CapEnd::End)];
    let (d, m) =
        fixture::measure_node(&d, MeasurePrimitive::Distance { a: 0, b: 1 }, caps.to_vec());
    let value = fixture::value_of(&d, m);
    let (d, _) = insert(
        d,
        Node::Assertion {
            value,
            bound: len(0.5),
            relation: AssertionRelation::AtLeast,
        },
    );
    assert_eq!(d.unused(), ops(&[pl]));
    let d = delete(d, pl);
    assert_eq!(
        d.unused(),
        ops(&[host]),
        "mid is asserted on; host surfaces"
    );
}

/// Folding: a split with both halves named and neither used is one
/// node finding; once one half is used, the other named half surfaces
/// as a variable.
#[test]
fn probe_folded_named_output_is_not_lost() {
    let d = kept(doc("probe-fold"));
    let (d, b) = block(d, 4.0);
    let tool = fixture::plane_on(&d, b, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (d, split) = insert(
        d,
        Node::Split {
            target: b.into(),
            tool,
        },
    );
    let above = d.output(split, 0).unwrap();
    let below = d.output(split, 1).unwrap();
    let rn = |d, v, n: &'static str| {
        step(
            d,
            DocEdit::RenameVar {
                var: VarRef::Id(v),
                name: Some(VarName::from_static(n)),
            },
        )
        .0
    };
    let d = rn(d, above, "top");
    let d = rn(d, below, "offcut");
    assert_eq!(d.unused(), ops(&[split]));
    let (d, _) = insert(d, fixture::stood(Operand::output(split, 0)));
    assert_eq!(
        d.unused(),
        Unused {
            operations: Vec::new(),
            vars: vec![below],
        }
    );
}

/// The world is exempt even when the person names it, in a document
/// that places nothing into it yet.
#[test]
fn probe_a_named_world_is_quiet() {
    let d = doc("probe-world");
    let edit = DocEdit::RenameVar {
        var: VarRef::Id(VarId::WORLD),
        name: Some(VarName::from_static("ground")),
    };
    let d = match d.apply(&edit, Tol::witness()) {
        Ok(applied) => applied.doc,
        Err(e) => {
            eprintln!("the door refuses naming the world: {e}");
            return;
        }
    };
    eprintln!("world named: {:?}", d.var_name(VarId::WORLD));
    assert_eq!(d.unused(), Unused::default());
}

/// An instance whose copy nothing uses fires in the host (the copy is
/// not joined to the product here).
#[test]
fn probe_an_unused_instance_fires_in_the_host() {
    let part = kept(doc("probe-part"));
    let mut store = fixture::resolver::PartStore::default();
    let part_ref = store.insert(part, Tol::witness());
    let host = kept(doc("probe-host"));
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
    let inst = minted.unwrap();
    assert_eq!(host.unused(), ops(&[inst]));
}

/// The resident answers when evaluation refuses, under the default
/// configuration: a document whose product does not gather still
/// reports its unused named parameter.
#[test]
fn probe_the_resident_answers_when_the_product_refuses() {
    let (d, opts) = failed_placement_world("probe-refuse");
    let d = step(
        d,
        DocEdit::DeclareVar {
            name: VarName::from_static("spare"),
            def: VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.75)),
        },
    )
    .0;
    let spare = d.var_named("spare").unwrap();
    assert!(d.unused().vars.contains(&spare), "the premise");
    let ev = fixture::run(&d, &opts);
    let only = ChecksConfig {
        connectedness: editor_core::Severity::Off,
        unproven_coincidence: editor_core::Advisory::Off,
        at_rest: editor_core::Severity::Off,
        ..ChecksConfig::default()
    };
    let alone = run_checks(&d, &ev, &only, Tol::witness());
    assert!(
        alone.is_ok_and(|r| r.findings.iter().any(|f| f.check == CheckId::Unused)),
        "alone, it answers"
    );
    let report = run_checks(&d, &ev, &ChecksConfig::default(), Tol::witness());
    match report {
        Ok(r) => assert!(r.findings.iter().any(|f| f.check == CheckId::Unused)),
        Err(e) => panic!("default config: the unused finding is lost in `{e}`"),
    }
}

/// `docm5_subject`'s failed placement world, copied: a mate naming no
/// face fails the placement, so the product refuses.
fn failed_placement_world(id: &str) -> (ProfileDoc, editor_core::EvalOptions) {
    use editor_core::{EntityKind, RoleSeg, StableName};
    let mut store = fixture::resolver::PartStore::default();
    let part = ProfileDoc::empty_derived(&format!("{id}-part"), Tol::witness());
    let (part, body) = block(part, 0.0);
    let (part_ref, body) = store.insert_part((part, body), Tol::witness());
    let opts = fixture::resolver::with_resolver(store);
    let d = ProfileDoc::empty_derived(id, Tol::witness());
    let (d, base) = fixture::insert_pinned(d, fixture::mated_instance(part_ref), &opts);
    let (d, placed) = insert(d, fixture::mated_instance(part_ref));
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
    (fixture::constrain_at(d, placed, mate).0, opts)
}

/// The PR body's count: `Doc::unused` over the corpus.
#[test]
fn probe_corpus_count() {
    let mut total = 0;
    let mut n = 0;
    for entry in crate::corpus::documents()
        .into_iter()
        .chain(crate::corpus::stage6_beside_the_registry())
    {
        n += 1;
        let u = entry.doc.unused();
        let k = u.operations.len() + u.vars.len();
        if k > 0 {
            let kinds: Vec<String> = u
                .operations
                .iter()
                .map(|id| format!("{:?}", entry.doc.node(*id).map(std::mem::discriminant)))
                .collect();
            eprintln!("{}: {k} {kinds:?} vars={:?}", entry.name, u.vars);
        }
        total += k;
    }
    eprintln!("corpus: {total} findings over {n} documents");
}
