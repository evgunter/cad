//! Reviewer r2 probe: the viewer's parameter text door after PR A.
use crate::common;
use pncad::document::{Distribution, DocEdit, FreeVar, VarName};
use pncad::geom_core::Tol;
use pncad::prelude::MM;
use pncad::quantity::WrittenLength;
use viewer::session::{DocSession, SessionOp};

#[test]
fn r2_constant_text_over_a_toleranced_parameter() {
    let tol = Tol::witness();
    let name = VarName::from_static("base_r");
    let toleranced = FreeVar::written_length(WrittenLength::in_unit(50.0, MM))
        .with_distribution(Some(Distribution::Normal { sigma: 0.0001 }))
        .unwrap();
    let mut session = DocSession::inline(common::declared("r2-viewer", &name, toleranced, tol), tol);
    let var = common::var_of(session.committed_doc(), name.as_str());
    let before = session.history().len();
    for text in ["2 mm + 1 mm", "3", "-(5 mm)"] {
        let outcome = session.perform(SessionOp::SetParamText { var, text: text.to_owned() });
        let doc = session.committed_doc();
        eprintln!(
            "{text:?}: refusal={:?} committed={:?} free={:?} defined={:?}",
            outcome.refusal.as_ref().map(|r| r.to_string()),
            outcome.committed.iter().map(|e| matches!(e, DocEdit::DefineVar { .. })).collect::<Vec<_>>(),
            doc.free(var).map(|f| f.distribution().is_some()),
            doc.var(var).and_then(|v| v.def().defined()).is_some(),
        );
    }
    eprintln!("history {} -> {}", before, session.history().len());
}
