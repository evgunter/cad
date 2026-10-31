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
//!
//! Every build runs in the coincidence door's own replay session
//! ([`editor_core::coincide::with_replay_session`]: deferring, at the
//! door's budget), so the instrument reads exactly what
//! the door replays
//! (`the_instrument_and_the_door_replay_in_one_session`).
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
use geom_core::{ParamSymbol, Sym, Tol};

use crate::corpus;

/// The relative move a variable takes at `p1`: small enough that the
/// two builds take the same decisions (asserted). It is `100·ε`, `1e-7`
/// at the default ε, so it stands where it does against the predicates'
/// zero bands at every ε the CI runs: those bands scale with ε, and a
/// move fixed in meters would sit inside a wide band at a coarse ε (an
/// exact zero moved by it no longer flips) and outside a narrow one at a
/// fine ε, so what a row pins would hold at one ε only.
fn step() -> f64 {
    1.0e-7 * (Tol::witness().eps() / geom_core::tolerance::DEFAULT_EPS)
}

/// How the variables move from `p0` to `p1`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Move {
    /// Each by its own multiple of [`step`], so no two move in step.
    Apart,
    /// All by the same relative [`step`], a zero one not at all: two
    /// variables of equal value stay equal, so a coincidence the
    /// document holds by value (a kiss, a flush seat) holds at `p1` too.
    /// The fallback for a document whose branches [`Move::Apart`]
    /// moves, and the pinned move of two gating rows (`plate_param`,
    /// `boss_union`), each saying why. On a document of
    /// lengths it is a dilation — of every length, the profile's
    /// included — so a
    /// laundered quantity that depends only on ratios of the variables
    /// (an angle, a unit normal's component, a fraction of a domain)
    /// does not move under it: a row pinned in step has no coverage of
    /// those past its apart move's parting, an open item, and before it
    /// an apart row holds the document ([`holds_until_parting`]).
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
    /// Decisions the `p0` build's numeric channel left indeterminate
    /// over a real margin: where a session that discharges as it
    /// decides could prove `Zero` ([`session`]).
    escalated: usize,
    /// Predicates whose re-valued margin is not the `p1` build's.
    laundered: BTreeSet<&'static str>,
    /// The same, after a decision the builds answered with different
    /// signs: possibly two margins under one name ([`tally`]).
    after_flip: BTreeSet<&'static str>,
    /// Every decision the builds answered with different signs: its
    /// index (from 1), predicate, and margin at `p0` and at `p1`.
    flips: Vec<(usize, &'static str, f64, f64)>,
    /// How the variables moved.
    mode: Option<Move>,
    /// Every predicate the `p0` build decided, with its count.
    seen: BTreeMap<&'static str, usize>,
    /// Where the two builds parted, when the census compared only the
    /// decisions before it: (decisions in common, decisions at `p0`,
    /// the `p0` predicate at the first difference).
    parted: Option<(usize, usize, &'static str)>,
    /// The side reader's decisions paired and re-valued
    /// ([`compare_decisions`]): they align apart from the document's,
    /// and their launderings count in `laundered` with the rest.
    family: usize,
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
            let step = step();
            let d = match mode {
                Move::Apart if value == 0.0 => step,
                Move::Apart => step * value.abs() * (1.0 + 0.37 * i as f64),
                Move::InStep => step * value,
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

/// A unit row's session: the door's replay session
/// ([`editor_core::coincide::with_replay_session`]), opened for its
/// tier and recorder alone. A unit row evaluates no document, so the
/// options the session hands over are not read.
fn session<R>(f: impl FnOnce() -> R) -> R {
    editor_core::coincide::with_replay_session(EvalOptions::default(), |_| f()).0
}

/// `doc`'s decisions under the box `at`, in the door's replay session
/// ([`editor_core::coincide::with_replay_session`]): the
/// instrument reads exactly what the door replays.
fn replayed<R>(
    at: &BTreeMap<VarId, f64>,
    f: impl FnOnce(&EvalOptions) -> R,
) -> (R, geom_core::SymCounts) {
    editor_core::coincide::with_replay_session(options(at), f)
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

/// `decisions` split into the document's own and the side reader's ray
/// caster's (`topo::SIDE_READER_FAMILY`, the family `k_stats::in_family`
/// tags; the reader's volume fallback is the document's): a flip among
/// the caster's changes whether a side is read, never which
/// (`[verdicts]`), so where it aims cannot part or flip a document's row.
fn streams(decisions: &[Decision]) -> (Vec<Decision>, Vec<Decision>) {
    decisions
        .iter()
        .cloned()
        .partition(|d| d.family != Some(topo::SIDE_READER_FAMILY))
}

/// The two builds' decisions compared: the document's own aligned
/// ([`in_step`]) and read ([`tally`]); `None` where they part, unless
/// `prefix`. The ray caster's are paired among themselves, up to their
/// own first difference, and re-valued as the document's are: a
/// laundering among them counts in `laundered`, while one of their flips,
/// and what follows it, parts and flips nothing. Past that difference a
/// build may have aimed a ray the other did not, and nothing on the
/// record says which ray a decision belongs to, so no later pair is known
/// to be one decision
/// (`work/flushtail/revalue-pairs-the-ray-casters-decisions-only-to-their-first-difference.md`).
fn compare_decisions(
    p0: &[Decision],
    p1: &[Decision],
    prefix: bool,
    env0: &dyn Fn(ParamSymbol) -> Option<f64>,
    env: &dyn Fn(ParamSymbol) -> Option<f64>,
) -> Option<Found> {
    let ((d0, f0), (d1, f1)) = (streams(p0), streams(p1));
    let common = in_step(&d0, &d1);
    let parts = common < d0.len().max(d1.len());
    if parts && !prefix {
        return None;
    }
    let mut found = Found {
        parted: parts.then(|| {
            let at = d0.get(common).map_or("<end>", |d| d.predicate);
            (common, d0.len(), at)
        }),
        ..Found::default()
    };
    tally(&mut found, &d0, &d1, common, env0, env);
    let mut reader = Found::default();
    tally(&mut reader, &f0, &f1, in_step(&f0, &f1), env0, env);
    found.family = reader.compared;
    found.drifted += reader.drifted;
    found.laundered.extend(reader.laundered);
    Some(found)
}

/// How many decisions the two builds took by the same predicate.
fn in_step(p0: &[Decision], p1: &[Decision]) -> usize {
    p0.iter()
        .zip(p1)
        .take_while(|(a, b)| a.predicate == b.predicate)
        .count()
}

/// Reads the first `common` decisions of the two builds into `found`:
/// each `p0` margin re-valued at `p0` (`env0`, the drift self-check)
/// and at `p1` (`env`) against the `p1` build's. A mismatch is a
/// laundering, or, once an EARLIER decision was answered with another
/// sign at `p1`, kept apart in `after_flip`: past a flip the builds may
/// decide one predicate over two margins. A flip parts nothing itself.
fn tally(
    found: &mut Found,
    p0: &[Decision],
    p1: &[Decision],
    common: usize,
    env0: &dyn Fn(ParamSymbol) -> Option<f64>,
    env: &dyn Fn(ParamSymbol) -> Option<f64>,
) {
    let mut flipped = None;
    for (k, (a, b)) in p0.iter().zip(p1).take(common).enumerate() {
        found.compared += 1;
        // A NaN margin is a domain violation, which no session
        // discharges.
        if a.sign.is_none() && !a.value.is_some_and(f64::is_nan) {
            found.escalated += 1;
        }
        *found.seen.entry(a.predicate).or_default() += 1;
        let (v0, v1) = (a.value.expect("Sym<f64>"), b.value.expect("Sym<f64>"));
        let same = |x: f64, y: f64| x.to_bits() == y.to_bits() || (x.is_nan() && y.is_nan());
        if revalue(a.node, env0).is_some_and(|r| !same(r, v0)) {
            found.drifted += 1;
        }
        match revalue(a.node, env) {
            None => found.opaque += 1,
            Some(r) => {
                if !same(v0, v1) {
                    found.moved += 1;
                }
                if !same(r, v1) {
                    let after = flipped.is_some();
                    if std::env::var_os("REVALUE_DETAIL").is_some() {
                        eprintln!(
                            "  {} #{} {}: p0 {v0:e}, re-valued {r:e}, p1 {v1:e}{}",
                            if after { "after_flip" } else { "laundered" },
                            k + 1,
                            a.predicate,
                            flipped.map_or(String::new(), |(f, p)| format!(" (flip #{f} {p})"))
                        );
                    }
                    if after {
                        found.after_flip.insert(a.predicate);
                    } else {
                        found.laundered.insert(a.predicate);
                    }
                }
            }
        }
        if a.sign != b.sign {
            found.flips.push((k + 1, a.predicate, v0, v1));
            if flipped.is_none() {
                flipped = Some((k + 1, a.predicate));
            }
        }
    }
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
    let p1 = replayed(&at_p1, |opts| build(doc, opts)).0;
    replayed(&at_p0, |opts| {
        let p0 = build(doc, opts);
        compare_decisions(&p0, &p1, prefix, &|s| env0.get(&s).copied(), &|s| {
            env.get(&s).copied()
        })
        .map(|found| Found {
            mode: Some(mode),
            ..found
        })
    })
    .0
}

/// One gating row: the document's decisions re-value to the `p1`
/// build's except at the predicates `owed` names, each with the work
/// item that owes its fix — a laundering found and not yet fixed is a
/// pinned red, never a skip. A fix moves the pin.
///
/// `flips` is the same for a decision the builds answer with different
/// signs: the flipping decisions' predicates IN ORDER, each with its
/// item. They are held exactly, given or not: the row reds when an owed
/// flip goes and when any other decision flips, under any name, so a row
/// that names none owes none.
///
/// `after` names the predicates that may re-value away from the `p1`
/// build after an owed flip. It is an upper bound: a named after-flip
/// predicate may be absent, because its re-valued bits depend on the step
/// (past a flip `p1` builds on the other branch, and a rounding-level
/// margin there matches `p1`'s bits or not as [`step`], and so ε, moves);
/// an unnamed one fails.
fn holds(
    name: &str,
    mode: Move,
    floor: usize,
    owed: &[(&str, &str)],
    flips: &[(&str, &str)],
    after: &[&str],
) {
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
        Some(mode),
        "{name}: the row compared under another move than its pin — under the in-step move a \
         laundered quantity that depends only on ratios of the variables does not move, so a \
         row's move is pinned and a change of it is read ({found:?})"
    );
    assert_eq!(
        found.escalated, 0,
        "{name}: a decision the f64 channel escalated, where a session that discharges as it \
         decides could prove Zero and take another branch than this deferring one ({found:?})"
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
    for (_, item) in flips {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(item);
        assert!(
            path.is_file(),
            "{name}: the owed flip's item {item} is no file"
        );
    }
    let got: Vec<&str> = found.flips.iter().map(|(_, p, _, _)| *p).collect();
    let want: Vec<&str> = flips.iter().map(|(p, _)| *p).collect();
    assert_eq!(
        got, want,
        "{name}: the decisions the builds answered with different signs are not the owed \
         ones, in order ({flips:?}) — a row owes none unless it names them — {found:?}"
    );
    let want_after: BTreeSet<&str> = after.iter().copied().collect();
    let unnamed: BTreeSet<&str> = found.after_flip.difference(&want_after).copied().collect();
    assert!(
        unnamed.is_empty(),
        "{name}: a margin re-values away from the p1 build after a decision the builds answered \
         with different signs, under {unnamed:?}, which the row does not name — a laundering, or \
         two margins decided under one name; read which with REVALUE_DETAIL=1 ({found:?})"
    );
}

/// **A gating row under [`Move::Apart`] for a document whose apart move
/// parts**, read over the decisions before the parting: the coverage a
/// row pinned [`Move::InStep`] lacks (a laundered quantity that depends
/// only on ratios of the variables, a plane's normal or an angle, does
/// not move under a dilation), up to where the builds part. The parting
/// is held exactly: it falls at `parting`, the work item `item` owes it,
/// and no decision before it flips. Past it the row says nothing, which
/// the document's in-step row covers.
fn holds_until_parting(name: &str, floor: usize, owed: &[(&str, &str)], parting: &str, item: &str) {
    let cd = corpus::documents()
        .into_iter()
        .find(|cd| cd.name == name)
        .unwrap_or_else(|| panic!("{name} is not a corpus document"));
    let found = compare_moved(&cd.doc, Move::Apart, true).expect("a prefix comparison answers");
    for item in owed.iter().map(|(_, i)| *i).chain([item]) {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(item);
        assert!(path.is_file(), "{name}: the owed item {item} is no file");
    }
    assert_eq!(
        found.drifted, 0,
        "{name}: a DAG re-valued at p0 is not the p0 build's own bits ({found:?})"
    );
    assert_eq!(
        found.escalated, 0,
        "{name}: a decision the f64 channel escalated ({found:?})"
    );
    assert!(
        found.moved >= floor,
        "{name}: {} margins moved and re-valued before the parting, under the floor of {floor} \
         ({found:?})",
        found.moved
    );
    let at = found.parted.map(|(_, _, at)| at);
    assert_eq!(
        at,
        Some(parting),
        "{name}: the apart move parts elsewhere than its owed parting ({found:?})"
    );
    let early: Vec<&str> = (found.flips.iter())
        .filter(|(k, p, _, _)| (*k, *p) != (found.compared, parting))
        .map(|(_, p, _, _)| *p)
        .collect();
    assert!(
        early.is_empty(),
        "{name}: a decision before the parting flipped: {early:?} ({found:?})"
    );
    let want: BTreeSet<&str> = owed.iter().map(|(p, _)| *p).collect();
    let got: BTreeSet<&str> = found.laundered.iter().copied().collect();
    assert_eq!(
        got, want,
        "{name}: the predicates re-valued away from the p1 build before the parting ({found:?})"
    );
}

/// `plate_param` under the apart move, to the ray-schedule gate's exact
/// zero it parts at.
#[test]
fn revalue_plate_param_apart_to_its_parting() {
    holds_until_parting(
        "plate_param",
        2539,
        &[],
        "point_in_loop_arm",
        "work/inlane/under-the-guided-lift-plate-params-apart-move-parts-at-a-point-exactly-on-its-loop.md",
    );
}

/// `boss_union` under the apart move, to where it parts. Nothing is
/// read as laundered before the parting: the boss's copy is placed by a
/// mate, not the identity bundle, so it runs its map at `p0` as at `p1`
/// (FORK-REST), and the two builds visit the separating axes in one
/// order where `p0`'s exactly axis-aligned faces used to merge them
/// (the owed item's `bool_pair_gap` swap).
#[test]
fn revalue_boss_union_apart_to_its_parting() {
    holds_until_parting(
        "boss_union",
        1619,
        &[],
        "pcurve_chart_radial_moving",
        "work/inlane/under-the-guided-lift-boss-unions-bool-pair-gap-decisions-swap-order-under-the-apart-move.md",
    );
}

// The corpus documents this row runs: those a debug build replays twice
// at `Sym<f64>` in seconds. The others (`die`, `cup`, `tube_ring`, …) run
// through `revalue_census`; `kiss_carry` changes branch under both moves
// and is not compared, and `face_sketch` is not compared yet: it parts
// under the apart move (`work/inlane/a-rest-contacts-volume-backstop-decides-its-equality-by-rounding.md`).
// Each row's move is pinned, and its floor is half the margins it moved
// and re-valued when it was pinned, so a change that makes a row opaque or
// static, or sends it to the weaker move, goes red.
macro_rules! rows {
    ($($row:ident: $doc:literal, $mode:ident, $floor:literal, [$($owed:expr),*]
        $(, flips [$($flip:expr),*], after [$($after:expr),*])?;)*) => {$(
        #[test]
        fn $row() {
            holds(
                $doc,
                Move::$mode,
                $floor,
                &[$($owed),*],
                &[$($($flip),*)?],
                &[$($($after),*)?],
            );
        }
    )*};
}

rows! {
    // In step: moved apart, the ray-schedule gate's margin is exactly 0
    // at the nominal (`point_in_loop_arm`, an axis-aligned face normal on
    // a schedule member, decision 6035), so the builds
    // part after it, from decision 6036 of 15539
    // (`work/inlane/under-the-guided-lift-plate-params-apart-move-parts-at-a-point-exactly-on-its-loop.md`).
    revalue_plate_param: "plate_param", InStep, 3938, [];
    // In step: moved apart, its `bool_pair_gap` margins are decided in
    // another order and the builds part after decision 3996, the first
    // difference at #3997 (then `pcurve_chart_azimuth_frame`)
    // (`work/inlane/under-the-guided-lift-boss-unions-bool-pair-gap-decisions-swap-order-under-the-apart-move.md`,
    // which NOTHING here pins).
    revalue_boss_union: "boss_union", InStep, 3236, [];
    // Apart: the cutter's placement reads a variable, so both builds run
    // its map and decide its transform and unit norms alike (were `p0`
    // to snap it at rest, the builds would part at `pose_unit_norm`).
    revalue_cut_cylinder: "cut_cylinder", Apart, 1302, [];
    revalue_die_chamfer: "die_chamfer", Apart, 1690, [];
    revalue_loft_prism: "loft_prism", Apart, 259, [];
    revalue_measured_web: "measured_web", Apart, 716, [];
    revalue_declared_tangency: "declared_tangency", Apart, 1217, [];
}

/// Apart, a pinned red: a point-in-solid plane test the union of the
/// family decides reads a margin of exactly 0 at the nominal, so the
/// builds part after decision 6956 of 22250 and the row compares in step
/// at every ε
/// (`work/inlane/under-the-apart-move-heat-sink-fins-parts-at-a-point-exactly-on-a-plane.md`).
#[test]
fn revalue_heat_sink_fins() {
    test_utils::pinned_red::row(
        Tol::witness().eps(),
        &test_utils::pinned_red::EVERY_ROW,
        "heat_sink_fins under the apart move",
        &[
            "heat_sink_fins: the row compared under another move than its pin",
            "after_flip: {}, flips: [], mode: Some(InStep), seen: {",
            "left: Some(InStep)\n right: Some(Apart)",
        ],
        || holds("heat_sink_fins", Move::Apart, 1353, &[], &[], &[]),
    );
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
        // `REVALUE_APART=1`: the apart move only, read to where it parts.
        let found = if std::env::var_os("REVALUE_APART").is_some() {
            compare_moved(&cd.doc, Move::Apart, true).expect("a prefix comparison always answers")
        } else {
            compare_moved(&cd.doc, Move::Apart, false)
                .or_else(|| compare_moved(&cd.doc, Move::InStep, false))
                .unwrap_or_else(|| compare_prefix(&cd.doc))
        };
        let Found {
            compared,
            moved,
            drifted,
            opaque,
            escalated,
            laundered,
            after_flip,
            flips,
            mode,
            seen,
            parted,
            family,
        } = &found;
        eprintln!(
            "{} ({:.1?}) {mode:?}: compared {compared}, moved {moved}, drifted {drifted}, opaque {opaque}, escalated {escalated}, \
             laundered {laundered:?}, after_flip {after_flip:?}, flips {flips:?}, parted {parted:?}, \
             side reader {family}",
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

/// **The side reader's decisions align apart, and a laundering among
/// them is still caught.** Each build decides a document's predicate, then
/// under the reader's family (`topo::SIDE_READER_FAMILY`) a margin
/// laundered through `from_f64`, a tie exact at `p0` that `p1` answers
/// `Positive`, and, at `p1` only, one more; then the document's next.
/// The document's decisions align and nothing flips or parts, while the
/// family's laundering, paired before its flip, is reported as any other.
#[test]
fn a_side_reader_laundering_is_caught_and_its_flips_part_nothing() {
    use geom_core::Real;
    use geom_core::predicate::{Band, Margin};
    let band = Band::new(1.0e-9, 1.0e-8).unwrap();
    let w = ParamSymbol::new(11);
    let decide =
        |name: &'static str, m: Sym<f64>| geom_core::k_stats::decide(name, Margin::of(m), band);
    let build = |d: f64| {
        let x = Sym::<f64>::from_f64(2.0) + Sym::param(w, d);
        record(|| {
            let _ = decide("revalue_doc", x);
            geom_core::k_stats::in_family(topo::SIDE_READER_FAMILY, || {
                let xx = x * x;
                let _ = decide("revalue_reader_laundered", xx - Sym::from_f64(xx.value));
                let _ = decide("revalue_reader_tie", Sym::param(w, d) - Sym::from_f64(0.25));
                if d > 0.3 {
                    let _ = decide("revalue_reader_extra", x);
                }
            });
            let _ = decide("revalue_doc_after", x * x);
        })
        .1
    };
    let p1 = session(|| build(0.5));
    let found = session(|| {
        let p0 = build(0.25);
        assert!(
            p0.iter()
                .any(|d| d.family == Some(topo::SIDE_READER_FAMILY)),
            "the recorder tags the family's decisions"
        );
        compare_decisions(&p0, &p1, false, &|s| (s == w).then_some(0.25), &|s| {
            (s == w).then_some(0.5)
        })
    })
    .expect("the document's decisions align: the family's extra one parts nothing");
    assert_eq!(found.compared, 2, "the document's two decisions");
    assert!(
        found.flips.is_empty(),
        "the family's tie flips nothing: {found:?}"
    );
    assert_eq!(found.parted, None);
    assert_eq!(
        found.family, 2,
        "the family's two decisions both builds took; the third is p1's alone"
    );
    assert_eq!(
        found.laundered,
        BTreeSet::from(["revalue_reader_laundered"]),
        "the family's laundering is caught"
    );
}

/// A decision the two builds answer with different signs: a later
/// mismatch is kept apart from the launderings, and a later match still
/// counts. The branching build's split answers `Zero` on a margin that
/// cancels exactly at `p0` and `Negative` on its rounding at `p1`, and
/// its next decision is the same either way, over two margins. The
/// other build's split steers nothing, as `join.rs`'s `nearer` reads
/// `Zero` and `Positive` alike.
#[test]
fn a_decision_answered_with_another_sign_keeps_what_follows_apart() {
    use geom_core::Real;
    use geom_core::predicate::{Band, Margin, Sign};
    let band = Band::new(1.0e-9, 1.0e-8).unwrap();
    let w = ParamSymbol::new(9);
    let decide =
        |name: &'static str, m: Sym<f64>| geom_core::k_stats::decide(name, Margin::of(m), band);
    let build = |d: f64, branches: bool, laundered_split: bool| {
        let x = Sym::param(w, d);
        record(|| {
            let split = decide(
                "revalue_split",
                if laundered_split {
                    x + (x - Sym::from_f64(x.value))
                } else {
                    x
                },
            );
            let arm = if branches && !matches!(split, Ok(Sign::Zero)) {
                x * Sym::from_f64(3.0)
            } else {
                x
            };
            let _ = decide("revalue_arm", arm);
        })
        .1
    };
    let tallied = |branches: bool, laundered_split: bool| {
        let p1 = session(|| build(1.0e-6, branches, laundered_split));
        session(|| {
            let p0 = build(0.0, branches, laundered_split);
            assert_ne!(
                p0[0].sign, p1[0].sign,
                "the split answers differently at p0 and p1"
            );
            let mut found = Found::default();
            tally(
                &mut found,
                &p0,
                &p1,
                in_step(&p0, &p1),
                &|s| (s == w).then_some(0.0),
                &|s| (s == w).then_some(1.0e-6),
            );
            found
        })
    };
    let branching = tallied(true, false);
    assert!(
        branching.laundered.is_empty() && branching.after_flip == BTreeSet::from(["revalue_arm"]),
        "the arm after the split is two margins under one name: kept apart, never a laundering \
         ({branching:?})"
    );
    let steering_nothing = tallied(false, false);
    assert!(
        steering_nothing.laundered.is_empty()
            && steering_nothing.after_flip.is_empty()
            && steering_nothing.moved == 2,
        "a split that steers nothing leaves the arm paired and compared ({steering_nothing:?})"
    );
    let self_flip = tallied(false, true);
    assert!(
        self_flip.laundered == BTreeSet::from(["revalue_split"]),
        "the decision that flips is itself paired: its own mismatch is a laundering, never \
         after_flip ({self_flip:?})"
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

/// **The instrument and the door replay in one session.** `plate_param`'s
/// decisions at the nominal, built through the instrument's own door
/// ([`replayed`], the session every row opens) and as the coincidence
/// door replays it ([`editor_core::coincide::with_replay_session`] with
/// every variable a symbol), are one list, predicate and sign, with one
/// set of session counts. The instrument's session is the door's in
/// what it is: DEFERRING — no margin's form is built unless a reader
/// asks, so nothing is discharged eagerly. An eager session or another
/// budget's freezes in [`replayed`] parts the two readings.
#[test]
fn the_instrument_and_the_door_replay_in_one_session() {
    let cd = corpus::documents()
        .into_iter()
        .find(|d| d.name == "plate_param")
        .expect("plate_param is a corpus document");
    let doc = &cd.doc;
    let at_p0: BTreeMap<VarId, f64> = continuous_vars(doc)
        .iter()
        .map(|(id, _)| (*id, 0.0))
        .collect();
    let key = |d: &[Decision]| -> Vec<_> { d.iter().map(|x| (x.predicate, x.sign)).collect() };
    let (instrument, instrument_counts) = replayed(&at_p0, |opts| key(&build(doc, opts)));
    let (door, door_counts) = editor_core::coincide::with_replay_session(
        EvalOptions {
            symbols: true,
            ..EvalOptions::default()
        },
        |opts| key(&build(doc, opts)),
    );
    assert!(!instrument.is_empty(), "the replay decides");
    assert_eq!(
        (
            instrument_counts.symbolic,
            instrument_counts.checks_expanded
        ),
        (0, 0),
        "a deferring session discharges nothing eagerly: {instrument_counts:?}"
    );
    assert_eq!(
        (instrument, instrument_counts),
        (door, door_counts),
        "one session, one reading"
    );
}

/// **The side reader's exact ties at a document's nominal**
/// (`work/flush/the-side-readers-aims-decide-exact-ties-at-a-documents-nominal.md`):
/// every decision taken under the reader's family whose margin is
/// exactly 0 in a build at the nominal, for `die_chamfer` and
/// `cut_cylinder`. Evidence, not a gate: run with
/// `cargo nextest run -p editor-core --run-ignored only -E 'test(reader_ties_at_the_nominal)' --no-capture`.
#[test]
#[ignore = "evidence: the side reader's exact ties at a nominal"]
fn reader_ties_at_the_nominal() {
    for cd in corpus::documents() {
        if !["die_chamfer", "cut_cylinder"].contains(&cd.name) {
            continue;
        }
        let (decisions, _) = replayed(&BTreeMap::new(), |opts| build(&cd.doc, opts));
        let family: Vec<(usize, &Decision)> = decisions
            .iter()
            .enumerate()
            .filter(|(_, d)| d.family == Some(topo::SIDE_READER_FAMILY))
            .collect();
        let ties: Vec<(usize, &'static str)> = family
            .iter()
            .filter(|(_, d)| d.value == Some(0.0))
            .map(|&(k, d)| (k + 1, d.predicate))
            .collect();
        println!(
            "TIES {}: {} decisions, {} the reader's, exact ties {ties:?}",
            cd.name,
            decisions.len(),
            family.len()
        );
    }
}
