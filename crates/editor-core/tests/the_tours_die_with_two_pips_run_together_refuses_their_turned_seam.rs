//! **The tour's composed die with one pip moved into its neighbour**:
//! `die_composed_tour` with one pip centre coordinate set 0.3 further
//! along the face (0.28 to 0.58, or 0.5 to 0.8), which puts that pip's
//! ball 0.08 from a neighbour's, so the two dimples run together.
//!
//! The n-ary union of the 21 balls builds: the step joining the moved
//! pip re-charts its ball and the neighbour's it crosses
//! (`topo::BooleanNaming::recharted`), and the fold reads each turned
//! face through its input shell. The document's union then refuses to
//! name the turned seam ([`ROW`]), typed, and the cut and the fillets
//! over it are poisoned.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{DocEdit, FreeValue, FreeVar, NamingError, Node, NodeErrorKind, NodeResult};
use geom_core::Tol;

const ROW: &str =
    "work/orbit/closed-sphere-escape-is-re-charted-by-rotation-beside-the-meridian-cut.md";

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
fn each_pip_moved_into_its_neighbour_refuses_at_the_unions_turned_seam() {
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
                NodeErrorKind::Naming(NamingError::UnsupportedTurnedCell {
                    cell: topo::Cell::Edge(_),
                    ..
                })
            ),
            "{spoken} = {to}: the union refuses at a turned seam piece ({ROW}), not {kind:?}"
        );
    }
}
