//! **The tour's composed die with one pip moved into its neighbour**:
//! `die_composed_tour` with one pip centre coordinate set 0.3 further
//! along the face (0.28 to 0.58, or 0.5 to 0.8), which puts that pip's
//! ball 0.08 from a neighbour's, so the two dimples run together.
//!
//! The n-ary union of the 21 balls builds: the step joining the moved
//! pip cuts its ball and the neighbour's it crosses along their own
//! charts' meridians, and its output stage closes both cuts again
//! (`topo::boolean::slit`, closing a cut), so each ball keeps its face
//! and holds the circle the two balls meet in as a ring (the step's
//! kernel body at its closed form:
//! `sweep`'s `the_nary_door_reads_a_cut_in_accumulation_through_its_pieces`).
//! That circle keeps one conventional vertex, which no name reads yet:
//! the document's union refuses there, and the fillets over it are
//! poisoned ([`CONVENTIONAL`]).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::emit_sphere_cut_in_cap::CONVENTIONAL;
use editor_core::{DocEdit, FreeValue, FreeVar, NamingError, Node, NodeErrorKind, NodeResult};
use geom_core::Tol;

/// Each moved variable, as the document speaks it, with its nominal.
const MOVED: [(&str, f64); 8] = [
    ("#129:767741f704e8ebe6", 0.28),
    ("#357:2810486fa916a8a9", 0.28),
    ("#433:4871340191d62d6a", 0.5),
    ("#509:b22228a4e6a704b8", 0.5),
    ("#744:9f2157e91944fa82", 0.28),
    ("#972:77de0c7836d1da04", 0.28),
    ("#1352:3d7e3060f60b5a4b", 0.28),
    ("#1504:5e9b7e9fd63def62", 0.28),
];

#[test]
fn each_pip_moved_into_its_neighbour_refuses_at_the_circles_conventional_vertex() {
    let d = crate::corpus::die_composed_tour::document();
    for (spoken, nominal) in MOVED {
        let (var, value) = d
            .doc
            .free_vars()
            .find(|&(id, _)| editor_core::test_support::var_text(&d.doc.spoken_var(id)) == spoken)
            .unwrap_or_else(|| panic!("{spoken}: a variable of the document"));
        let FreeVar::Continuous { value, .. } = value else {
            panic!("{spoken}: a continuous variable, not {value:?}");
        };
        assert_eq!(*value, nominal, "{spoken}: its nominal");
        let to = nominal + 0.3;
        let doc = editor_core::apply(
            &d.doc,
            &DocEdit::SetVarValue {
                var: editor_core::VarRef::Id(var),
                value: FreeValue::Continuous(to),
            },
            Tol::witness(),
        )
        .unwrap_or_else(|e| panic!("{spoken} = {to}: the edit applies: {e:?}"))
        .doc;
        let ev = editor_core::evaluate::<f64>(
            &doc,
            None,
            &editor_core::CancelToken::new(),
            &editor_core::EvalOptions::default(),
            Tol::witness(),
        );
        let failed: Vec<_> = ev
            .order
            .iter()
            .filter_map(|n| match ev.nodes.get(n) {
                Some(NodeResult::Failed(e)) => Some((*n, &e.kind)),
                _ => None,
            })
            .collect();
        let [(node, kind)] = failed[..] else {
            panic!("{spoken} = {to}: one node refuses, not {failed:?}");
        };
        assert!(
            matches!(doc.node(node), Some(Node::Union { .. })),
            "{spoken} = {to}: the pips' union refuses, not {}",
            editor_core::test_support::node_text(&doc.spoken(node))
        );
        assert!(
            matches!(
                kind,
                NodeErrorKind::Naming(NamingError::ConventionalVertex { .. })
            ),
            "{spoken} = {to}: the union refuses at the circle's conventional vertex \
             ({CONVENTIONAL}), not {kind:?}"
        );
        die_meshes(spoken, &doc, &ev, node);
    }
}

/// **The die's kernel bodies under `union`, the node naming refuses,
/// build and mesh**: the n-ary union of its pips, whose two moved balls
/// each hold the circle they meet in as a ring once the cut is closed,
/// and the die's cube less it. The kernel answers the user's drag; only
/// the circle's names wait ([`CONVENTIONAL`]).
fn die_meshes(
    spoken: &str,
    doc: &editor_core::ProfileDoc,
    ev: &editor_core::Evaluation<f64>,
    union: editor_core::RecipeNodeId,
) {
    let tol = Tol::witness();
    let Some(Node::Union { members, .. }) = doc.node(union) else {
        panic!("{spoken}: the union is a union")
    };
    let pips: Vec<topo::AtRestBody<f64>> = members
        .reads()
        .map(|m| {
            topo::test_support::finished(
                "a pip",
                crate::corpus::read_body::<f64>(ev, doc, m.read),
                tol,
            )
        })
        .collect();
    let inputs = topo::MemberInputs::default();
    let run = topo::nary_boolean::<f64>(
        topo::NaryOp::Union,
        &pips
            .iter()
            .map(|body| topo::NaryMember {
                body,
                inputs: &inputs,
            })
            .collect::<Vec<topo::NaryMember<'_, f64>>>(),
        topo::SweepStrategy::Realized,
        tol,
    );
    assert!(
        matches!(run.end, topo::NaryEnd::Built),
        "{spoken}: the pips' union builds: {:?}",
        run.end
    );
    let fused = &run.steps.last().expect("a step").body.body;
    let meshes = |what: &str, body: &topo::Body<f64>| {
        let m = mesh::tessellate(body, 1e-3, tol)
            .unwrap_or_else(|e| panic!("{spoken}: {what} meshes: {e:?}"));
        assert_eq!(
            mesh::validate::check_mesh(&m),
            Ok(()),
            "{spoken}: {what} meshes closed and manifold"
        );
    };
    meshes("the pips' union", fused);
    let cube = doc
        .ids()
        .iter()
        .find_map(|&id| match doc.node(id) {
            Some(Node::Subtract { from, tool })
                if tool
                    .reads()
                    .any(|t| doc.operation_of(t.read) == Some(union)) =>
            {
                Some(crate::corpus::read_body::<f64>(ev, doc, from.read))
            }
            _ => None,
        })
        .expect("the die cuts its cube by the union");
    let cube = topo::test_support::finished("the cube", cube, tol);
    let fused = topo::test_support::finished("the union", fused.clone(), tol);
    let out = topo::subtract(&cube, &fused, tol);
    let Ok(topo::BooleanResult::Body(die)) = &out else {
        panic!("{spoken}: the cube less the union builds: {out:?}")
    };
    meshes("the cube less the union", &die.body);
}
