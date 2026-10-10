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

/// The relative move a variable takes at `p1`: small enough that the
/// two builds take the same decisions (asserted).
const MOVE: f64 = 1.0e-7;

/// How the variables move from `p0` to `p1`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Move {
    /// Each by its own multiple of [`MOVE`], so no two move in step.
    Apart,
    /// All by the same relative [`MOVE`], a zero one not at all: two
    /// variables of equal value stay equal, so a coincidence the
    /// document holds by value (a kiss, a flush seat) holds at `p1` too.
    /// The fallback for a document whose branches [`Move::Apart`] moves.
    InStep,
}

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
    /// How the variables moved.
    mode: Option<Move>,
    /// Every predicate the `p0` build decided, with its count.
    seen: BTreeMap<&'static str, usize>,
}

fn continuous_vars(doc: &ProfileDoc) -> Vec<(VarId, f64)> {
    doc.free_vars()
        .filter_map(|(id, p)| match *p {
            FreeVar::Continuous { value, .. } => Some((id, value)),
            _ => None,
        })
        .collect()
}

fn offsets(vars: &[(VarId, f64)], mode: Move) -> BTreeMap<VarId, f64> {
    vars.iter()
        .enumerate()
        .map(|(i, &(id, value))| {
            let d = match mode {
                Move::Apart if value == 0.0 => MOVE,
                Move::Apart => MOVE * value.abs() * (1.0 + 0.37 * i as f64),
                Move::InStep => MOVE * value,
            };
            (id, d)
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

/// The comparison under [`Move::Apart`], or under [`Move::InStep`]
/// where the first moves a branch.
fn compare(name: &str, doc: &ProfileDoc) -> Found {
    compare_moved(doc, Move::Apart)
        .or_else(|| compare_moved(doc, Move::InStep))
        .unwrap_or_else(|| {
            panic!(
                "{name}: the builds at p0 and p1 took different decisions under both moves — \
                 the move changed a branch"
            )
        })
}

/// `None` when the two builds did not take the same decisions.
fn compare_moved(doc: &ProfileDoc, mode: Move) -> Option<Found> {
    let vars = continuous_vars(doc);
    let at_p1 = offsets(&vars, mode);
    let at_p0: BTreeMap<VarId, f64> = at_p1.keys().map(|id| (*id, 0.0)).collect();
    let env: BTreeMap<ParamSymbol, f64> = at_p1
        .iter()
        .map(|(id, d)| (ParamSymbol::new(id.0.digest()), *d))
        .collect();
    let p1 = session(|| build(doc, &options(&at_p1)));
    session(|| {
        let p0 = build(doc, &options(&at_p0));
        let predicates = |d: &[Decision]| d.iter().map(|x| x.predicate).collect::<Vec<_>>();
        if predicates(&p0) != predicates(&p1) {
            return None;
        }
        let mut found = Found {
            mode: Some(mode),
            ..Found::default()
        };
        for (a, b) in p0.iter().zip(&p1) {
            found.compared += 1;
            *found.seen.entry(a.predicate).or_default() += 1;
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
        Some(found)
    })
}

/// One gating row: the document's decisions re-value to the `p1`
/// build's except at the predicates `owed` names, each with the work
/// item that owes its fix — a laundering found and not yet fixed is a
/// pinned red, never a skip. A fix moves the pin.
fn holds(name: &str, owed: &[(&str, &str)]) {
    let cd = corpus::documents()
        .into_iter()
        .find(|cd| cd.name == name)
        .unwrap_or_else(|| panic!("{name} is not a corpus document"));
    let found = compare(name, &cd.doc);
    assert!(
        found.moved > 0,
        "{name}: no margin moved between p0 and p1, so the row compared nothing that could launder"
    );
    let want: BTreeSet<&str> = owed.iter().map(|(p, _)| *p).collect();
    let got: BTreeSet<&str> = found.laundered.iter().copied().collect();
    assert_eq!(
        got, want,
        "{name}: the predicates whose margin re-values away from the p1 build (laundered: a \
         computed value re-entered as a constant) — {found:?}"
    );
}

// The corpus documents this row runs: those a debug build replays twice
// at `Sym<f64>` in seconds. The others (`die`, `cup`, `tube_ring`, …) run
// through `revalue_census`; `kiss_carry` changes branch under both moves
// and is not compared.
macro_rules! rows {
    ($($row:ident: $doc:literal, [$($owed:expr),*];)*) => {$(
        #[test]
        fn $row() {
            holds($doc, &[$($owed),*]);
        }
    )*};
}

rows! {
    revalue_plate_param: "plate_param", [];
    revalue_boss_union: "boss_union", [];
    revalue_cut_cylinder: "cut_cylinder", [];
    revalue_die_chamfer: "die_chamfer", [];
    revalue_face_sketch: "face_sketch", [];
    revalue_heat_sink_fins: "heat_sink_fins", [];
    revalue_loft_prism: "loft_prism", [];
    revalue_measured_web: "measured_web", [];
    revalue_declared_tangency: "declared_tangency", [];
}

/// The census over the whole corpus, one document per run with
/// `REVALUE_DOC=<name>` (`REVALUE_SEEN=1` lists every predicate decided).
#[test]
#[ignore = "evidence: REVALUE_DOC=<name> cargo nextest run -p editor-core --run-ignored only -E 'test(revalue_census)' --no-capture"]
fn revalue_census() {
    let only = std::env::var("REVALUE_DOC").ok();
    for cd in corpus::documents() {
        if only.as_deref().is_some_and(|o| o != cd.name) {
            continue;
        }
        let t = std::time::Instant::now();
        let found = compare(cd.name, &cd.doc);
        let Found {
            compared,
            moved,
            opaque,
            laundered,
            mode,
            seen,
        } = &found;
        eprintln!(
            "{} ({:.1?}) {mode:?}: compared {compared}, moved {moved}, opaque {opaque}, \
             laundered {laundered:?}",
            cd.name,
            t.elapsed()
        );
        if std::env::var_os("REVALUE_SEEN").is_some() {
            eprintln!("  seen {seen:?}");
        }
    }
}

/// The instrument against a planted laundering, so the corpus row's
/// silence is evidence: a margin `x − lit(x)`, where `lit(x)` is `x`
/// read out to `f64` and re-entered as a constant, is re-valued as
/// laundered; the same value kept in the scalar is not; and entered
/// through `from_computed` it is opaque, never compared.
#[test]
fn a_planted_laundering_is_seen_and_from_computed_is_opaque() {
    use geom_core::Real;
    use geom_core::predicate::{Band, Margin};
    let band = Band::new(1.0e-9, 1.0e-8).unwrap();
    let w = ParamSymbol::new(7);
    // `p0`: w = 0.25 over a nominal of 2; `p1` moves it.
    let at = |d: f64| Sym::<f64>::from_f64(2.0) + Sym::param(w, d);
    let decide = |m: Sym<f64>| geom_core::k_stats::decide("revalue_plant", Margin::of(m), band);
    let margins = |d: f64| {
        let x = at(d) * at(d);
        [
            x - Sym::from_f64(x.value),
            x - at(d) * at(d),
            x - Sym::from_computed(x.value),
        ]
    };
    let p1: Vec<Decision> = session(|| {
        record(|| {
            for m in margins(0.5) {
                let _ = decide(m);
            }
        })
        .1
    });
    let rows: Vec<Option<bool>> = session(|| {
        let p0 = record(|| {
            for m in margins(0.25) {
                let _ = decide(m);
            }
        })
        .1;
        p0.iter()
            .zip(&p1)
            .map(|(a, b)| {
                revalue(a.node, &|s| (s == w).then_some(0.5))
                    .map(|r| r.to_bits() == b.value.unwrap().to_bits())
            })
            .collect()
    });
    assert_eq!(
        rows,
        [Some(false), Some(true), None],
        "laundered / kept in the scalar / from_computed, each re-valued at p1 against the p1 build"
    );
}
