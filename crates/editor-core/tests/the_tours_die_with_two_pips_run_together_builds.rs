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
//! That circle keeps one conventional vertex, named for the circle
//! alone (`names/README.md`, "A conventional vertex"), so the union
//! publishes its table.
//!
//! The cube less the union builds and names too, and both mesh closed.
//! Past them, each edit meets a fillet's own door, not a naming
//! one: a pip moved to 0.58 cuts its rim circle and its neighbour's
//! into arcs, so the pip-rim fillet's selection, recorded on the
//! nominal die, names a whole rim no longer there (N5, `Vanished`,
//! recourse a rebind); a pip moved to 0.8 runs its dimple into the
//! band the box-edge fillet cuts away (`fillet3_ring_clearance`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::body_of;
use editor_core::{DocEdit, FreeValue, FreeVar, Node, NodeErrorKind, NodeResult, RoleSeg};
use geom_core::Tol;

/// Each moved variable, as the document speaks it, with its nominal,
/// and how far the moved pip's centre then stands from each neighbour
/// whose ball its own meets: a 0.58 pip between two neighbours meets
/// the one 0.08 off and the one 0.14 off, or at a face's end only the
/// one 0.14 off.
const MOVED: [(&str, f64, &[f64]); 8] = [
    ("#129:767741f704e8ebe6", 0.28, &[0.08, 0.14]),
    ("#357:2810486fa916a8a9", 0.28, &[0.08, 0.14]),
    ("#433:4871340191d62d6a", 0.5, &[0.08]),
    ("#509:b22228a4e6a704b8", 0.5, &[0.08]),
    ("#744:9f2157e91944fa82", 0.28, &[0.14]),
    ("#972:77de0c7836d1da04", 0.28, &[0.14]),
    ("#1352:3d7e3060f60b5a4b", 0.28, &[0.14]),
    ("#1504:5e9b7e9fd63def62", 0.28, &[0.14]),
];

/// The pip ball's radius (`demos/tour/src/diefillet.rs`, `PIP_R`).
const R: f64 = 0.09;

/// The union of the 21 balls, the moved one at each of `apart` from a
/// neighbour: 21 balls less the lens it shares with each, which lie in
/// no third ball.
fn closed_form(apart: &[f64]) -> f64 {
    let ball = 4.0 / 3.0 * std::f64::consts::PI * R.powi(3);
    let lens = |d: f64| std::f64::consts::PI * (4.0 * R + d) * (2.0 * R - d).powi(2) / 12.0;
    21.0 * ball - apart.iter().map(|&d| lens(d)).sum::<f64>()
}

fn volume(ev: &editor_core::Evaluation<f64>, id: editor_core::RecipeNodeId) -> f64 {
    topo::test_support::pinned_volume(
        "the pips' union",
        &topo::test_support::finished(
            "the pips' union",
            topo::Body::clone(&body_of(ev, id)),
            Tol::witness(),
        ),
        Tol::witness(),
    )
    .volume
}

#[test]
fn each_pip_moved_into_its_neighbour_builds_its_union_and_meets_a_fillets_door() {
    let d = crate::corpus::die_composed_tour::document();
    for (spoken, nominal, apart) in MOVED {
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
        let union = ev
            .order
            .iter()
            .copied()
            .find(|&n| matches!(doc.node(n), Some(Node::Union { .. })))
            .expect("the pips' union");
        if let Some(NodeResult::Failed(e)) = ev.nodes.get(&union) {
            panic!(
                "{spoken} = {to}: the pips' union builds and names: {}",
                e.kind
            );
        }
        let t = crate::fixture::table(&ev, union);
        let circles: Vec<_> = t
            .iter()
            .filter(|(n, _)| matches!(n.path.as_slice(), [RoleSeg::ConventionalVertex(_)]))
            .map(|(n, _)| {
                editor_core::test_support::said_text(&editor_core::test_support::spoken_name(
                    n.clone(),
                    doc.spoken(union),
                ))
            })
            .collect();
        assert_eq!(
            circles.len(),
            apart.len(),
            "{spoken} = {to}: one vertex on each circle two balls meet in: {circles:#?}"
        );
        for circle in &circles {
            assert!(
                circle.starts_with("the conventional vertex of the seam edge of ")
                    && circle.matches("Union").count() == 3,
                "{spoken} = {to}: the vertex reads as its circle's, the union said once \
                 after each side's read and once after the circle: {circle}"
            );
        }
        let v = volume(&ev, union);
        assert!(
            (v - closed_form(apart)).abs() < 1e-9,
            "{spoken} = {to}: the union is 21 balls less the lens of each neighbour \
             {apart:?} off: {v} against {}",
            closed_form(apart)
        );
        die_meshes(spoken, &doc, &ev, union);
        let failed: Vec<_> = ev
            .order
            .iter()
            .filter_map(|n| match ev.nodes.get(n) {
                Some(NodeResult::Failed(e)) => Some((*n, &e.kind)),
                _ => None,
            })
            .collect();
        let [(node, kind)] = failed.as_slice() else {
            panic!("{spoken} = {to}: one fillet refuses, not {failed:#?}");
        };
        let said = kind.to_string();
        assert!(
            matches!(doc.node(*node), Some(Node::Fillet { .. })),
            "{spoken} = {to}: a fillet refuses: {said}"
        );
        if to > 0.7 {
            assert!(
                said.starts_with(
                    "the fillet op refused: a ring or edge lies in the part of a face the blend \
                     cuts away"
                ),
                "{spoken} = {to}: the box-edge fillet's clearance: {said}"
            );
        } else {
            assert!(
                matches!(
                    kind,
                    NodeErrorKind::SelectResolve { error, .. }
                        if matches!(**error, editor_core::ResolveError::Vanished { .. })
                ),
                "{spoken} = {to}: the pip-rim fillet's selection names a rim the cut \
                 divided, which vanished (N5): {said}"
            );
        }
    }
}

/// **The die's bodies under its fillets build and mesh** closed and
/// manifold: the pips' union, each moved ball holding the circle it
/// meets its neighbour in as a ring, and the cube less it.
fn die_meshes(
    spoken: &str,
    doc: &editor_core::ProfileDoc,
    ev: &editor_core::Evaluation<f64>,
    union: editor_core::RecipeNodeId,
) {
    let meshes = |what: &str, id: editor_core::RecipeNodeId| {
        let body = body_of(ev, id);
        let m = mesh::tessellate(&body, 1e-3, Tol::witness())
            .unwrap_or_else(|e| panic!("{spoken}: {what} meshes: {e:?}"));
        assert_eq!(
            mesh::validate::check_mesh(&m),
            Ok(()),
            "{spoken}: {what} meshes closed and manifold"
        );
    };
    meshes("the pips' union", union);
    let die = doc
        .ids()
        .iter()
        .copied()
        .find(|&id| match doc.node(id) {
            Some(Node::Subtract { tool, .. }) => tool
                .reads()
                .any(|t| doc.operation_of(t.read) == Some(union)),
            _ => false,
        })
        .expect("the die cuts its cube by the union");
    assert!(
        matches!(ev.nodes.get(&die), Some(NodeResult::Ok(_))),
        "{spoken}: the cube less the union builds and names"
    );
    meshes("the cube less the union", die);
}
