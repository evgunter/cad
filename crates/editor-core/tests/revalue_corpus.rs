//! **No computed value re-enters the symbolic lane as a constant**
//! (FORK-S4F; ERROR-DESIGN E12; `work/intent/a-computed-value-re-enters-as-a-constant.md`).
//!
//! A symbolic `Zero` is a theorem about the DAG its decision saw, and
//! that DAG is the computation only while every computed value stays in
//! the scalar. A value read out to `f64` and re-entered through
//! `Real::from_f64` is a constant where the computation had a function
//! of the variables, and a `Zero` over it is a theorem about the wrong
//! function.
//!
//! The row: every corpus document is evaluated at `Sym<f64>` with EVERY
//! continuous free variable a symbol, twice — at its nominal (`p0`) and
//! with each variable moved by its own small offset (`p1`). Each
//! decision the `p0` build took has its margin's DAG re-valued at `p1`
//! (`geom_core::sym::revalue`), op by op as `f64` performs it, and
//! compared bit for bit with the margin the `p1` build decided. Where
//! the DAG is the computation the two are the same bits; a constant
//! that should have moved stays where `p0` put it, and they differ.
//!
//! A DAG that reaches an opaque value (`Real::from_computed`, an unnamed
//! axis) re-values to nothing and is counted, not compared: no identity
//! is proved over it either.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-core/src/sym.rs",
    "crates/geom-core/src/sym/",
    "crates/geom-core/src/real.rs",
    "crates/geom/src/",
    "crates/geom-brep/src/",
    "crates/topo/src/",
    "crates/sweep/src/",
    "crates/editor-core/src/",
    "crates/editor-core/tests/corpus/",
];

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use editor_core::analysis::{BoxAxis, ParamBox};
use editor_core::{CancelToken, EvalOptions, Evaluation, FreeVar, ProfileDoc, VarId, evaluate};
use geom_core::sym::revalue::{Decision, record, revalue};
use geom_core::sym::with_session_rules;
use geom_core::{ParamSymbol, Sym, SymBudget, SymRules, Tol};

use crate::corpus;

/// The relative move each variable takes at `p1`, scaled per variable
/// so no two move in step. Small enough that no corpus decision
/// changes branch (asserted: the two builds take the same decisions).
const MOVE: f64 = 1.0e-7;

/// What one document's comparison found.
#[derive(Default, Debug)]
struct Found {
    /// Decisions the two builds took, in step.
    compared: usize,
    /// Decisions whose margin moved between `p0` and `p1` — the row's
    /// power: a document where nothing moved proves nothing.
    moved: usize,
    /// Decisions whose DAG reached an opaque value.
    opaque: usize,
    /// Predicates whose re-valued margin is not the `p1` build's.
    laundered: BTreeSet<&'static str>,
}

fn continuous_vars(doc: &ProfileDoc) -> Vec<(VarId, f64)> {
    doc.free_vars()
        .filter_map(|(id, p)| match *p {
            FreeVar::Continuous { value, .. } => Some((id, value)),
            _ => None,
        })
        .collect()
}

fn offsets(vars: &[(VarId, f64)]) -> BTreeMap<VarId, f64> {
    vars.iter()
        .enumerate()
        .map(|(i, &(id, value))| {
            let scale = if value == 0.0 { 1.0 } else { value.abs() };
            (id, MOVE * scale * (1.0 + 0.37 * i as f64))
        })
        .collect()
}

fn options(offsets: &BTreeMap<VarId, f64>) -> EvalOptions {
    let axes = offsets
        .iter()
        .map(|(id, &d)| (*id, BoxAxis::Varying { lo: d, hi: d }))
        .collect();
    EvalOptions {
        param_box: Some(Arc::new(ParamBox::from_axes(axes))),
        ..EvalOptions::default()
    }
}

fn session<R>(f: impl FnOnce() -> R) -> R {
    let budget = SymBudget {
        max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
        max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
    };
    with_session_rules(budget, SymRules::shipped(), f).0
}

fn build(doc: &ProfileDoc, opts: &EvalOptions) -> Vec<Decision> {
    record(|| {
        let ev: Evaluation<Sym<f64>> =
            evaluate(doc, None, &CancelToken::new(), opts, Tol::witness());
        ev.order.len()
    })
    .1
}

fn compare(name: &str, doc: &ProfileDoc) -> Found {
    let vars = continuous_vars(doc);
    let at_p1 = offsets(&vars);
    let at_p0: BTreeMap<VarId, f64> = at_p1.keys().map(|id| (*id, 0.0)).collect();
    let env: BTreeMap<ParamSymbol, f64> = at_p1
        .iter()
        .map(|(id, d)| (ParamSymbol::new(id.0.digest()), *d))
        .collect();
    let p1 = session(|| build(doc, &options(&at_p1)));
    session(|| {
        let p0 = build(doc, &options(&at_p0));
        let predicates = |d: &[Decision]| d.iter().map(|x| x.predicate).collect::<Vec<_>>();
        assert_eq!(
            predicates(&p0),
            predicates(&p1),
            "{name}: the builds at p0 and p1 took different decisions — the move changed a branch"
        );
        let mut found = Found::default();
        for (a, b) in p0.iter().zip(&p1) {
            found.compared += 1;
            let (v0, v1) = (a.value.expect("Sym<f64>"), b.value.expect("Sym<f64>"));
            if v0.to_bits() != v1.to_bits() {
                found.moved += 1;
            }
            match revalue(a.node, &|s| env.get(&s).copied()) {
                None => found.opaque += 1,
                Some(r) if r.to_bits() == v1.to_bits() || (r.is_nan() && v1.is_nan()) => {}
                Some(_) => {
                    found.laundered.insert(a.predicate);
                }
            }
        }
        found
    })
}

/// The census row while the audit is in flight: prints what each
/// document shows.
#[test]
#[ignore = "evidence: cargo nextest run -p editor-core --run-ignored only revalue_corpus"]
fn revalue_census() {
    for cd in corpus::documents() {
        let found = compare(cd.name, &cd.doc);
        println!("{}: {found:?}", cd.name);
    }
}
