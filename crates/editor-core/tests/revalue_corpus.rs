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
    /// The census's fallback for a document whose branches
    /// [`Move::Apart`] moves, never a gating row's: on a document of
    /// lengths it is a dilation, so a laundered quantity that depends
    /// only on ratios of the variables (an angle, a unit normal's
    /// component, a fraction of a domain) does not move under it.
    InStep,
}

/// What one document's comparison found.
#[derive(Default, Debug)]
struct Found {
    /// Decisions the two builds took, in step.
    compared: usize,
    /// Decisions whose margin moved between `p0` and `p1` AND re-valued
    /// — the row's power: a margin that moved but went opaque proves
    /// nothing, and neither does a document where nothing moved.
    moved: usize,
    /// Decisions whose DAG, re-valued at `p0` itself, is not the `p0`
    /// build's own bits: `revalue` has drifted from how `Sym` builds.
    drifted: usize,
    /// Decisions whose DAG reached an opaque value.
    opaque: usize,
    /// Predicates whose re-valued margin is not the `p1` build's.
    laundered: BTreeSet<&'static str>,
    /// How the variables moved.
    mode: Option<Move>,
    /// Every predicate the `p0` build decided, with its count.
    seen: BTreeMap<&'static str, usize>,
    /// Where the two builds parted, when the census compared only the
    /// decisions before it: (decisions in common, decisions at `p0`,
    /// the `p0` predicate at the first difference).
    parted: Option<(usize, usize, &'static str)>,
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

/// A session at the shipped budget, or, under `REVALUE_TIER=off`, at a
/// zero-term budget: the identity tier switched off, which re-valuation
/// does not read (it walks the DAG, which a session records at any
/// budget). The census takes it for the documents whose discharge walks
/// at `Sym<f64>`, with every variable a symbol, outrun a run.
fn session<R>(f: impl FnOnce() -> R) -> R {
    let off = std::env::var("REVALUE_TIER").is_ok_and(|v| v == "off");
    let budget = if off {
        SymBudget {
            max_terms: 0,
            max_degree: 0,
        }
    } else {
        SymBudget {
            max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
            max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
        }
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
    compare_moved(doc, Move::Apart, false)
        .or_else(|| compare_moved(doc, Move::InStep, false))
        .unwrap_or_else(|| {
            panic!(
                "{name}: the builds at p0 and p1 took different decisions under both moves — \
                 the move changed a branch"
            )
        })
}

/// The census's reading of a document whose branches both moves
/// change: the decisions before the first difference, which the two
/// builds took along one path, under whichever move keeps more of them.
/// Weaker than [`compare`] — a branch taken without a decision before
/// that point would show here as a laundering — and never a gating row.
fn compare_prefix(doc: &ProfileDoc) -> Found {
    let a = compare_moved(doc, Move::Apart, true).expect("a prefix comparison always answers");
    let b = compare_moved(doc, Move::InStep, true).expect("a prefix comparison always answers");
    if a.compared >= b.compared { a } else { b }
}

/// `None` when the two builds did not take the same decisions, unless
/// `prefix`, which compares the decisions before the first difference.
fn compare_moved(doc: &ProfileDoc, mode: Move, prefix: bool) -> Option<Found> {
    let vars = continuous_vars(doc);
    let at_p1 = offsets(&vars, mode);
    let at_p0: BTreeMap<VarId, f64> = at_p1.keys().map(|id| (*id, 0.0)).collect();
    let env_at = |at: &BTreeMap<VarId, f64>| -> BTreeMap<ParamSymbol, f64> {
        at.iter()
            .map(|(id, d)| (ParamSymbol::new(id.0.digest()), *d))
            .collect()
    };
    let (env, env0) = (env_at(&at_p1), env_at(&at_p0));
    let p1 = session(|| build(doc, &options(&at_p1)));
    session(|| {
        let p0 = build(doc, &options(&at_p0));
        let predicates = |d: &[Decision]| d.iter().map(|x| x.predicate).collect::<Vec<_>>();
        let (n0, n1) = (predicates(&p0), predicates(&p1));
        let common = n0.iter().zip(&n1).take_while(|(a, b)| a == b).count();
        if n0 != n1 && !prefix {
            return None;
        }
        let mut found = Found {
            mode: Some(mode),
            parted: (n0 != n1)
                .then(|| (common, n0.len(), n0.get(common).copied().unwrap_or("<end>"))),
            ..Found::default()
        };
        for (a, b) in p0.iter().zip(&p1).take(common) {
            found.compared += 1;
            *found.seen.entry(a.predicate).or_default() += 1;
            let (v0, v1) = (a.value.expect("Sym<f64>"), b.value.expect("Sym<f64>"));
            let same = |x: f64, y: f64| x.to_bits() == y.to_bits() || (x.is_nan() && y.is_nan());
            if revalue(a.node, &|s| env0.get(&s).copied()).is_some_and(|r| !same(r, v0)) {
                found.drifted += 1;
            }
            match revalue(a.node, &|s| env.get(&s).copied()) {
                None => found.opaque += 1,
                Some(r) if same(r, v1) => {
                    if !same(v0, v1) {
                        found.moved += 1;
                    }
                }
                Some(r) => {
                    if !same(v0, v1) {
                        found.moved += 1;
                    }
                    if std::env::var_os("REVALUE_DETAIL").is_some() {
                        eprintln!(
                            "  laundered #{} {}: p0 {v0:e}, re-valued {r:e}, p1 {v1:e}",
                            found.compared, a.predicate
                        );
                    }
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
fn holds(name: &str, floor: usize, owed: &[(&str, &str)]) {
    let cd = corpus::documents()
        .into_iter()
        .find(|cd| cd.name == name)
        .unwrap_or_else(|| panic!("{name} is not a corpus document"));
    let found = compare(name, &cd.doc);
    assert_eq!(
        found.drifted, 0,
        "{name}: a decision's DAG re-valued at p0 is not the p0 build's own bits — revalue no \
         longer computes an op as Sym builds it ({found:?})"
    );
    assert_eq!(
        found.mode,
        Some(Move::Apart),
        "{name}: the row fell back to the in-step move, under which a laundered quantity that \
         depends only on ratios of the variables does not move ({found:?})"
    );
    assert!(
        found.moved >= floor,
        "{name}: {} margins moved and re-valued, under the row's floor of {floor} — the row \
         compares less than it did ({found:?})",
        found.moved
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
// Each row's floor is half the margins it moved and re-valued when it was
// pinned, so a change that makes a row opaque or static goes red.
macro_rules! rows {
    ($($row:ident: $doc:literal, $floor:literal, [$($owed:expr),*];)*) => {$(
        #[test]
        fn $row() {
            holds($doc, $floor, &[$($owed),*]);
        }
    )*};
}

rows! {
    revalue_plate_param: "plate_param", 650, [];
    revalue_boss_union: "boss_union", 560, [];
    revalue_cut_cylinder: "cut_cylinder", 290, [];
    revalue_die_chamfer: "die_chamfer", 740, [];
    revalue_face_sketch: "face_sketch", 250, [];
    revalue_heat_sink_fins: "heat_sink_fins", 190, [];
    revalue_loft_prism: "loft_prism", 3, [];
    revalue_measured_web: "measured_web", 45, [];
    revalue_declared_tangency: "declared_tangency", 160, [];
}

/// The census over the whole corpus, one document per run with
/// `REVALUE_DOC=<name>` (`REVALUE_SEEN=1` lists every predicate decided).
#[test]
#[ignore = "evidence: REVALUE_DOC=<name> cargo nextest run -p editor-core --run-ignored only -E 'test(revalue_census)' --no-capture"]
fn revalue_census() {
    let only = std::env::var("REVALUE_DOC").ok();
    let docs = corpus::documents();
    if let Some(o) = only.as_deref() {
        let names: Vec<&str> = docs.iter().map(|cd| cd.name).collect();
        assert!(
            names.contains(&o),
            "REVALUE_DOC={o} is not a corpus document: {names:?}"
        );
    }
    for cd in docs {
        if only.as_deref().is_some_and(|o| o != cd.name) {
            continue;
        }
        let t = std::time::Instant::now();
        let found = compare_moved(&cd.doc, Move::Apart, false)
            .or_else(|| compare_moved(&cd.doc, Move::InStep, false))
            .unwrap_or_else(|| compare_prefix(&cd.doc));
        let Found {
            compared,
            moved,
            drifted,
            opaque,
            laundered,
            mode,
            seen,
            parted,
        } = &found;
        eprintln!(
            "{} ({:.1?}) {mode:?}: compared {compared}, moved {moved}, drifted {drifted}, opaque {opaque}, \
             laundered {laundered:?}, parted {parted:?}",
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

/// Every op `revalue` mirrors, re-valued at `p1` against `Sym<f64>`'s own
/// build there: one decision per op, over two parameters, each inside
/// its op's domain at both points. A drifted arm is a false laundering
/// on every corpus row that reaches the op; this row names the op.
#[test]
fn revalue_mirrors_every_op_as_sym_builds_it() {
    use geom_core::Real;
    use geom_core::predicate::{Band, Margin};
    let band = Band::new(1.0e-9, 1.0e-8).unwrap();
    let (x, y) = (ParamSymbol::new(11), ParamSymbol::new(12));
    let ops = |d: f64| -> Vec<(&'static str, Sym<f64>)> {
        let a = Sym::<f64>::from_f64(0.3) + Sym::param(x, d);
        let b = Sym::<f64>::from_f64(-1.7) + Sym::param(y, 2.0 * d);
        vec![
            ("add", a + b),
            ("sub", a - b),
            ("mul", a * b),
            ("div", a / b),
            ("neg", -a),
            ("powi", a.powi(3)),
            ("powi0", a.powi(0)),
            ("sqrt", a.sqrt()),
            ("abs", b.abs()),
            ("sin", b.sin()),
            ("cos", b.cos()),
            ("tan", a.tan()),
            ("asin", a.asin()),
            ("acos", a.acos()),
            ("atan", b.atan()),
            ("atan2", a.atan2(b)),
            ("min", a.min(b)),
            ("max", a.max(b)),
            ("floor", b.floor()),
            ("copysign", a.copysign(b)),
            ("select", b.select_le_zero(a, b)),
            ("pi", a * Sym::pi()),
            ("tau", a * Sym::tau()),
        ]
    };
    let decide_all = |d: f64| {
        record(|| {
            for (_, m) in ops(d) {
                let _ = geom_core::k_stats::decide("revalue_op", Margin::of(m), band);
            }
        })
        .1
    };
    let p1 = session(|| decide_all(0.01));
    let names: Vec<&str> = ops(0.0).iter().map(|(n, _)| *n).collect();
    let drifted: Vec<&str> = session(|| {
        let p0 = decide_all(0.0);
        let env = |s: ParamSymbol| {
            Some(if s == x {
                0.01
            } else if s == y {
                0.02
            } else {
                return None;
            })
        };
        p0.iter()
            .zip(&p1)
            .zip(&names)
            .filter(|((a, b), _)| {
                let v1 = b.value.unwrap();
                revalue(a.node, &env).is_none_or(|r| r.to_bits() != v1.to_bits())
            })
            .map(|(_, n)| *n)
            .collect()
    });
    assert_eq!(names.len(), p1.len(), "one decision per op");
    assert!(
        drifted.is_empty(),
        "ops whose re-valued margin is not Sym<f64>'s at p1: {drifted:?}"
    );
}
