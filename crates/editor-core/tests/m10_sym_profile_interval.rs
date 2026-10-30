//! **Where the E12 tier's time goes** — the cost profile of the
//! symbolic normal form on the M10-3 slab and the two-hole plate
//! (`work/sym/symbolic-tier-costs-95-percent-of-the-m10-3-drive`).
//!
//! Every row here but one is EVIDENCE-ONLY and `#[ignore]`d: each
//! prints the tier's structural profile (`geom_core::sym::profile`,
//! behind the test-only `sym-profile-testing` feature the
//! dev-dependency edge turns on) for one replay or one drive, and
//! asserts nothing about the numbers — they are the record, read into
//! the item body and the tier's `# Cost` section, and re-taken by
//! re-running the row. The fixtures are the ones the M10-3 suite
//! drives (`bounded_chamber`, the row S-TCOST bisected on) and the
//! plate the tier was built for, through the same doors.
//!
//! The one gating row,
//! [`the_forms_the_walks_build_are_pinned_per_eps_row`], reads the
//! same instrument for the one thing in it that is not a clock: the
//! walk ledger — per walk and origin the calls, the forms, the frozen,
//! and the digest chain of the forms built — so what the tier BUILDS
//! on both documents is a pinned number, over and above what the
//! pins say it DECIDES.
//!
//! The one row without the profile installed,
//! [`sym_profile_callgrind_replay`], is the callgrind target: it
//! replays one box a chosen number of times and nothing else, so an
//! instruction count over it is the tier's and not the instrument's.
//! The command that takes it is recorded in the item body.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// Gated to the code it profiles: the normal form and its rules, the
// driver that replays leaves through it, and the fixtures — the shared
// tree and the four sibling suites whose doors build the documents and
// run the replay.
test_utils::gated_to![
    "crates/geom-core/src/sym.rs",
    "crates/geom-core/src/sym/",
    "crates/editor-core/src/drive.rs",
    "crates/sweep/src/swept.rs",
    "crates/editor-core/tests/fixture/",
    "crates/editor-core/tests/m10_3_r1_probes_interval.rs",
    "crates/editor-core/tests/m10_7_plate.rs",
    "crates/editor-core/tests/m10_8_arc_family_interval.rs",
    "crates/editor-core/tests/m10_8_harness.rs",
    "crates/editor-core/src/test_support.rs",
];

use std::collections::BTreeMap;
use std::time::Instant;

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, drive};
use geom_core::sym::profile::{start_profile, take_profile};
use geom_core::sym::report::DecisionShape;
use geom_core::{SymRules, Tol};

use crate::m10_3_r1_probes_interval::{CHAMBER_LEAVES, bounded_chamber};
use crate::m10_7_plate::plate;
use crate::m10_8_arc_family_interval::replay;
use crate::m10_8_harness::nominal_box;
use editor_core::drive::{DEFAULT_SYM_MAX_DEGREE, DEFAULT_SYM_MAX_TERMS};
use editor_core::{CancelToken, EvalOptions, evaluate};
use geom_core::SymBudget;
use geom_core::sym::with_session_rules;
use std::sync::Arc;

/// One replay at `Sym<Interval>` over `box_` with NOTHING installed —
/// no shape report (which renders every blocked residual through the
/// walks, ~1 % of a replay's instructions) — so a callgrind count over
/// it is the tier's and the numeric channel's alone.
fn bare_replay(doc: &editor_core::ProfileDoc, box_: &ParamBox, tol: Tol) -> geom_core::SymCounts {
    let opts = EvalOptions {
        param_box: Some(Arc::new(box_.clone())),
        ..EvalOptions::default()
    };
    let budget = SymBudget {
        max_terms: DEFAULT_SYM_MAX_TERMS,
        max_degree: DEFAULT_SYM_MAX_DEGREE,
    };
    let (_, counts) = with_session_rules(budget, SymRules::shipped(), || {
        let ev: editor_core::Evaluation<geom_core::Sym<geom_core::Interval>> =
            evaluate(doc, None, &CancelToken::new(), &opts, tol);
        ev.order.len()
    });
    counts
}

fn eps() -> f64 {
    Tol::witness().eps()
}

/// **The M10-3 slab**: the bounded chamber at the dimensions S-TCOST
/// bisected the tier's cost on.
fn slab() -> ProfileDoc {
    bounded_chamber(60.0 * eps(), 30.0 * eps(), 100.0 * eps())
}

/// **The two-hole plate at its real study** (the M10-8 nominal that
/// froze 1,056 forms).
fn the_plate(tol: Tol) -> ProfileDoc {
    plate(5.0e-5, 1.0e-5, tol).0
}

/// The three scales a replay is read at: the degenerate box at the
/// nominal (every form built); a LEAF-sized box — every axis `± ε`
/// about its nominal, the width at which the M10-3 drive's leaves
/// certify, so the evaluation runs to the end and the forms are
/// built only where the numeric channel does not answer; and the
/// whole analyzed box, which on the slab refuses at its second node
/// (the extrusion vector straddles zero over it) and is kept as the
/// record of that.
fn boxes(doc: &ProfileDoc) -> [(&'static str, ParamBox); 3] {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    let root = ParamBox::of(&analyzed);
    let leaf = ParamBox::from_axes(
        root.axes()
            .iter()
            .map(|(n, a)| {
                let m = a.midpoint();
                (
                    *n,
                    BoxAxis::Varying {
                        lo: m - eps(),
                        hi: m + eps(),
                    },
                )
            })
            .collect(),
    );
    [
        ("nominal", nominal_box(&analyzed)),
        ("leaf", leaf),
        ("root", root),
    ]
}

fn outcomes(shapes: &[DecisionShape]) -> BTreeMap<String, usize> {
    let mut out = BTreeMap::new();
    for s in shapes {
        *out.entry(format!("{:?}", s.outcome)).or_default() += 1;
    }
    out
}

/// One replay over `box_` with the profile installed; prints the
/// decision split, the counts and the profile.
fn profiled_replay(label: &str, doc: &ProfileDoc, box_: &ParamBox, tol: Tol) {
    start_profile();
    let t0 = Instant::now();
    let (shapes, refusal, counts) = replay(doc, box_, SymRules::shipped(), tol);
    let wall = t0.elapsed();
    let profile = take_profile();
    println!("=== {label}: wall {wall:?}");
    println!("decisions {:?}", outcomes(&shapes));
    println!("counts {counts:?}");
    if let Some(r) = refusal {
        println!("first refusal: {r}");
    }
    assert_eq!(profile.unnoted(), 0, "a refusal site without a note");
    print!("{}", profile.render());
}

/// **The slab, one replay at each scale** — the nominal (every form
/// built), a leaf-sized box, and the whole analyzed box ([`boxes`]).
#[test]
#[ignore = "evidence-only: prints the tier's cost profile of one slab replay at three scales"]
fn sym_profile_slab_replays() {
    let tol = Tol::witness();
    let doc = slab();
    for (scale, box_) in boxes(&doc) {
        profiled_replay(&format!("slab {scale}"), &doc, &box_, tol);
    }
}

/// **The slab, driven whole** at the M10-3 row's own leaf budget,
/// sequentially, with the profile on for every leaf — the population
/// over the entire row S-TCOST measured rather than one replay of it.
#[test]
#[ignore = "evidence-only: prints the tier's cost profile summed over the M10-3 chamber drive"]
fn sym_profile_slab_drive() {
    let tol = Tol::witness();
    let doc = slab();
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let config = DriveConfig {
        max_leaves: CHAMBER_LEAVES,
        ..DriveConfig::default()
    };
    start_profile();
    let t0 = Instant::now();
    let v = drive(&doc, &analyzed, &config, tol).unwrap();
    let wall = t0.elapsed();
    let profile = take_profile();
    println!(
        "=== slab drive at {CHAMBER_LEAVES} leaves: wall {wall:?} receipt {:?} decisions {:?}",
        v.receipt(),
        v.decisions()
    );
    assert_eq!(profile.unnoted(), 0, "a refusal site without a note");
    print!("{}", profile.render());
}

/// **The plate at its nominal**, the freeze population only — the
/// comparison point on the document the tier was built for.
#[test]
#[ignore = "evidence-only: prints the tier's cost profile of one plate replay at the nominal"]
fn sym_profile_plate_nominal() {
    let tol = Tol::witness();
    let doc = the_plate(tol);
    let (_, nominal) = boxes(&doc).into_iter().next().unwrap();
    profiled_replay("plate nominal", &doc, &nominal, tol);
}

/// **The callgrind target**: `CAD_SYM_PROFILE_DOC` (`slab` | `plate`,
/// default `slab`) replayed over `CAD_SYM_PROFILE_BOX` (`nominal` |
/// `leaf` | `root`, default `nominal`) `CAD_SYM_PROFILE_REPEATS` times
/// (default 1), neither the profile nor the shape report installed
/// ([`bare_replay`]), so an instruction count over the process is the
/// tier's own. Prints the counts per replay.
#[test]
#[ignore = "evidence-only: the callgrind target — replays one box, nothing else"]
fn sym_profile_callgrind_replay() {
    let tol = Tol::witness();
    let doc = match std::env::var("CAD_SYM_PROFILE_DOC").as_deref() {
        Ok("plate") => the_plate(tol),
        _ => slab(),
    };
    let which = std::env::var("CAD_SYM_PROFILE_BOX").unwrap_or_else(|_| "nominal".into());
    let repeats: usize = std::env::var("CAD_SYM_PROFILE_REPEATS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);
    let (scale, box_) = boxes(&doc)
        .into_iter()
        .find(|(s, _)| *s == which)
        .expect("CAD_SYM_PROFILE_BOX is `nominal`, `leaf` or `root`");
    for i in 0..repeats {
        let t0 = Instant::now();
        let counts = bare_replay(&doc, &box_, tol);
        println!(
            "replay {i} {scale}: wall {:?} counts {counts:?}",
            t0.elapsed()
        );
    }
}

/// The ε row this run is on, as the index into a three-row table
/// (`1e-6`, `1e-9`, `1e-12`); any other ε has no captured row and
/// fails loud rather than reading a neighbour's.
fn eps_row(eps: f64) -> usize {
    [1.0e-6, 1.0e-9, 1.0e-12]
        .iter()
        .position(|&e| (eps / e - 1.0).abs() < 1.0e-3)
        .unwrap_or_else(|| panic!("no captured row at eps = {eps:e}: capture one and add it"))
}

/// The slab's walk ledger at its nominal, per ε row.
///
/// Re-captured when the certification schedule began assigning its end
/// samples (`geom_brep::schedule_param`: `t₀` and `t₁` themselves): the
/// `Plain/Decision` walk builds 260 fewer forms (9686 → 9426) and its
/// digest moves at every row; calls, every other line and the receipt
/// are unchanged. Measured by restoring the old end samples on a probe,
/// which restores the old line.
///
/// Re-captured again when a variable's symbol became its minted id
/// (INTENT-VARS-1 PR 2) rather than a hash of its name: the slab's
/// variables sort differently inside the forms, so three digests move
/// at every row while every count — calls, forms, frozen — holds, and
/// `Early/Decision`, which reads no variable, holds its digest.
///
/// Re-captured when an intrinsic edge description's surfaces became a
/// set (`geom_brep::SurfacePair`): the certificate reads the
/// transversality wedge (`geom_brep::dihedral::wedge_decided`'s
/// `n1.cross(n2)`) in key order where the extrude reads it in builder
/// order, and the cross product is symmetric in value up to sign but not
/// in form, so mixed orientations build distinct hash-consed forms.
/// Measured composed with the cap orientation (`cap_plane_orientation`,
/// [`PLATE_LEDGER`]'s note: +4 `Assertion` calls per walk here): the
/// `*/Assertion` walks build 188 more forms (998 → 1186, 2041 → 2229)
/// and the `Plain/*` and `Early/Assertion` digests move at every row;
/// calls, frozen counts and `Early/Decision` hold.
///
/// What moves it is what moves [`PLATE_LEDGER`]; on the slab the
/// edges' mid-parameter points are the lever — the witness an edge is
/// minted with, the certificate's midpoint check and its schedule's
/// middle station are one [`geom::mid_param`] evaluation, and a second
/// spelling of that point builds a second chain of forms, which reads
/// here as `Plain/Decision` forms alone.
///
/// INTENT-LITERALS PR C moves the `Plain/Decision` line as it moves
/// [`PLATE_LEDGER`]'s: one more call, the non-finite door's theorem on
/// the anonymous definition a slot's formula lowers to.
///
/// The faces' extent levering the wedge moves the `*/Assertion` lines
/// as it moves [`PLATE_LEDGER`]'s: `Plain/Assertion` 1186 → 2690 and
/// `Early/Assertion` 2229 → 4121 forms, and their digests, at every
/// row; calls and the `*/Decision` lines hold.
///
/// The final non-finite check asking boundedness rather than deciding
/// `value · 0` (`expr-non-finite-asks-value-times-zero`) takes its
/// calls out of `Plain/Decision` (981 → 928, forms 9426 → 9415); the
/// forms it used to build first are now built under `Plain/Assertion`
/// (2690 → 2697). Its digests move at every row; the `Early/*` lines
/// and the largest form hold.
///
/// A chord's director spelled as its components, prescaled by the
/// largest and normalized (`profile`'s `path::chord_director`), where
/// it was the `sin_cos` of an `atan2`, builds more forms under the
/// assertions for the slab's straight legs: `Plain/Assertion`
/// 2697 → 2733 and `Early/Assertion` 4121 → 4157, and their digests, at
/// every row; calls, frozen counts, both `*/Decision` lines and the
/// largest form hold.
///
/// The joint's turn and the door's second junction read, as on the
/// plate: `Plain/Assertion` and `Early/Assertion` +14 calls
/// (514 → 528) and +30 forms, with their digests, at every row; both
/// `*/Decision` lines and the largest form hold.
///
/// INTENT stage 3 D (Transform retires into a placement) re-spells the
/// slab's fixture (`bounded_chamber`) without a datum frame: its
/// profiles are 2-D and its bodies reach the world through `Place`s. Four
/// `Assertion` calls fewer per walk (514 → 510), `Plain/Decision` 9415 →
/// 9296 forms, `Plain/Assertion` 2697 → 2658 and `Early/Assertion`
/// 4121 → 4082, and those lines' digests at every row; `Early/Decision`,
/// the frozen counts and the largest form hold.
///
/// Re-taken on INTENT stage 3 D after FORK-REST: the slab's ledger is
/// what D's head read before it (`434139ef1c`, measured there), so
/// neither ruling moves it. It moved on D's lane of 2026-10-19/20 with
/// main's merge at `0a1d95321e`: `Plain/Assertion` and
/// `Early/Assertion` 510 -> 524 calls, 2658 -> 2724 and 4082 -> 4148
/// forms, and those lines' digests at every row; the `*/Decision`
/// lines, the freezes and the largest form hold.
///
/// **Pinned at what main measures** (2026-10-29), not explained. It
/// first moved with #5542's merge (`fa901e4308`; `42af7c0b48` before it
/// holds the old pin), where `segment_gap` joined the profile's
/// validation: `Plain/Decision` 928 → 944 calls, both `*/Assertion`
/// lines 524 → 532, and both walks gained `*/Report` lines. Then
/// `Plain/Decision` 944 → 992, unbisected; then PR 5952 took the
/// `*/Report` lines away and `Plain/Decision` 992 → 976. Every digest
/// moved, at every row
/// (`work/cpair/main-red-slow-set-receipts-moved-at-5542.md`).
const SLAB_LEDGER: [&str; 3] = [
    "\
     Plain/Decision calls 976 forms 9789 frozen 0 digest 8858c21e7c09110b4a4a85bbf14ec889\n\
     Plain/Assertion calls 532 forms 2807 frozen 0 digest b1292a7d0849233b0dc62e91e26d70ac\n\
     Early/Decision calls 16 forms 30 frozen 0 digest c109ddd3b905b5672a4454b753a3ed8e\n\
     Early/Assertion calls 532 forms 4231 frozen 0 digest 22e1ea40284774ef624fde19d7ba0f06",
    "\
     Plain/Decision calls 976 forms 9789 frozen 0 digest 08650e2e51b39ab8a7719732f4e2430a\n\
     Plain/Assertion calls 532 forms 2807 frozen 0 digest ed4fab55a50737de9fa0d4bba53f57ec\n\
     Early/Decision calls 16 forms 30 frozen 0 digest c109ddd3b905b5672a4454b753a3ed8e\n\
     Early/Assertion calls 532 forms 4231 frozen 0 digest f84140de6ef76c9c89ac18d75feab99a",
    "\
     Plain/Decision calls 976 forms 9789 frozen 0 digest 4b7556244d138ea5c634de0f542b7f0a\n\
     Plain/Assertion calls 532 forms 2807 frozen 0 digest 4ba81d361ee96c59e00226ced255f112\n\
     Early/Decision calls 16 forms 30 frozen 0 digest c109ddd3b905b5672a4454b753a3ed8e\n\
     Early/Assertion calls 532 forms 4231 frozen 0 digest fc61ebc3d4a97dba3fec18a676708db6",
];

/// The largest form (numerator plus denominator terms) any op built
/// on each document at its nominal — the growth guard: a
/// representation change that grows forms reds here in seconds,
/// where the ledger alone would first run for minutes.
///
/// **Both moved with SYM-5's rule E** (the quotient's common factor),
/// and in opposite directions, which is the rule's shape: the slab's
/// largest 10 → 6, because the rule divides the shared factor out of
/// every form it is given and can only SHRINK that form; the plate's
/// 90 → 288, because a form the rule brings back under the budget is
/// one that no longer FREEZES to a one-term indeterminate, and its
/// consumers then build what the freeze used to cut off (the plate's
/// `Early/Decision` frozen falls 48 → 8 in the ledger below). The rule
/// never grows a form; the WALK's largest form grows because fewer
/// forms are cut short.
///
/// **The plate's 288 → 252 is NOT this unit's**, and the row above is
/// what says so: the plain walk consults no dial of the early walk,
/// the plate's largest form is 252 under `shipped`, under
/// `without_canonical_root`, under `without_the_reads` and under both
/// shut, and the two `Plain/*` digests are identical across all four.
/// Both numbers were stale against this pin before DECIDE-3's branch
/// was cut — its own Phase 1 baseline records this row RED on the base
/// with "every count identical, every digest moved" — so what changed
/// them is a base change between the pin's last capture and the cut,
/// and DECIDE-3 re-pins them without claiming them.
///
/// **What IS rule G's, by the same differential**: the plate's
/// `Early/Assertion` and `Door/Decision` freezes fall 104 → 0, and the
/// `Early/*` and `Door/*` digests move. **What A0's new fold is**: the
/// slab's `Early/*` digests, and the `*/Report` rows absent on both
/// documents — the sites the shape report used to be called on are
/// comparisons of two rational CONSTANTS, and A0 decides those exactly
/// now (`work/decide/a0-leaves-max-and-min-of-constants-opaque`), so
/// there is nothing left for the instrument to report.
///
/// **What rule G's exact quotient moves** (`SymRules::root_quotient`,
/// the only change between the two captures): the plate's
/// `Early/Decision` digest, and nothing else — every count on every
/// line identical, the slab's ledger and both largest forms unmoved.
/// Its reach on the plate's nominal replay is 108 of the 208 root mints
/// that reach it (ten distinct arguments, asked again across walks):
/// on 76 of them the split would have keyed the root differently, on
/// 32 it declined, and on each the quotient's root is a different form
/// (counted by a local probe, reverted, and matching review r1's count
/// at the rule's first head). No decision moves: the plate's split and
/// receipt are unmoved (`sym_9_retry_interval`'s pin reads
/// `[811, 0, 140, 462]` either way). One digest and not more is the
/// ledger's charging rule read on this document: a memoized form is
/// chained into the line of the walk and origin that first builds it,
/// and these roots are first built by the decision walk — the pin
/// holds that no other line moves, and that reading of it is not a
/// separate measurement.
///
/// **With the span read as the stored sweep signed by the decided turn**
/// (`sweep`'s `arc_span`) and the pushforward and a cap's apex reading
/// that same stored sweep, every form count is the one the doc above
/// the ledger names (`Plain/Decision` 15046, `Door/Decision` 11864),
/// and the `Early/Assertion` and `Door/Decision` lines freeze 104 nodes
/// each again where rule G had taken them to 0. The cause is the cap
/// apex: `sweep::swept::arc_apex` reads `tan(|Δθ|/4)` off the sweep,
/// so its sagitta carries a `tan(1·atan(1))` atom rule D does not fold
/// (rule D folds `sin` and `cos` of `q·atan X`, not `tan`), and every
/// one of the 104 is a `Powi` of a form over that atom refused on the
/// COEFFICIENT bound (`2^186`-scale coefficients). Folding `tan` the way
/// rule D folds `sin`/`cos` takes both lines back to 0, measured on a
/// probe and reverted
/// (`work/rules/rule-d-leaves-tan-of-atan-opaque-and-the-cap-apex-mints-it`).
/// The plate's receipt is `[811, 0, 140, 462]` either way, and the
/// `*/Report` rows stay absent.
///
/// **When the certification schedule began assigning its end samples**
/// (`geom_brep::schedule_param`: `t₀` and `t₁` themselves) the decision
/// walks build fewer forms — `Plain/Decision` 15046 → 14609,
/// `Early/Decision` 7995 → 7741, `Door/Decision` 11864 → 11548 — and
/// those three digests move; calls, frozen counts, the assertion lines
/// and the receipt are unchanged, and the slab's `Plain/Decision` line
/// moves the same way. Measured by restoring the old end samples on a
/// probe, which restores both old ledgers.
///
/// **When `geom::mid_param` was respelled `(t0 + t1)·½`** (from
/// `t0 + (t1 − t0)·½`, reading each end once), every walk builds
/// different forms. The slab's `Plain/Decision` goes 9426 → 9852 at
/// every row. On the plate, every line's forms rise
/// (`Plain/Decision` 14609 → 15086, `Plain/Assertion` 2593 → 2941,
/// `Early/Decision` 7741 → 7973, `Early/Assertion` 3405 → 3753,
/// `Door/Decision` 11548 → 12012), and so do the freezes:
/// `Plain/Decision` 672 → 720, `Plain/Assertion` 372 → 420, and the
/// apex lines 104 → 112. Calls are unchanged. Measured by restoring the
/// old spelling on a probe, which restores both ledgers.
///
/// **With the one apex spelling** (`Arc2::apex`, whose sagitta reads
/// `Arc2::quarter_tan`, the quarter sweep's `sin / cos` that rule D
/// folds, in place of `tan`), the `Early/Assertion` and `Door/Decision`
/// lines freeze nothing: the 104 nodes each that `tan(1·atan(1))` froze
/// are gone, as folding `tan` measured above. A probe spelling
/// `quarter_tan` as `tan` restores 104 on both lines.
///
/// **When the circles began storing their authored carrier** (centre
/// and `|r|`, `store-constructed-carriers`), the 148 decisions the door
/// answered on the plate became theorems: `Door/Decision` calls
/// 596 → 448 (forms 13624 → 6768), and the walks build smaller forms —
/// the plate's largest 252 → 28 terms, `Plain/Decision` forms
/// 16596 → 16228 and frozen 696 → 252, `Plain/Assertion` 3944 → 3915
/// and 372 → 360, `Early/Decision` 9399 → 9117 and 8 → 0,
/// `Early/Assertion` 4860 → 4831; every digest but `Door/Assertion`'s
/// and the `Report` lines' moves. The slab, a polygon, does not move.
const SLAB_MAX_TERMS: usize = 6;
const PLATE_MAX_TERMS: usize = 28;

/// The plate's walk ledger at its nominal — one row, because the
/// plate's nominal reads no ε (its dimensions are literals, not
/// multiples of ε) and the captures at the three rows agree.
///
/// What moves it: any change to the forms the walks meet, or the order
/// they meet them in — a loop's start vertex, how a sketch segment or
/// an arc apex is pushed forward, where a curve is sampled, how an
/// edge's mid parameter is spelled ([`geom::mid_param`]). A change
/// that moves only digests reorders the forms; one that moves counts
/// changes what is built.
///
/// The door's calls include the lowering's endpoint registrations
/// (`Arc2::register_endpoints`): 16 decisions and 16 assertions. The
/// extrude's closing pcurve mint certifies the plate's wall rows, so
/// the certificate's decisions are walked too, and the residuals it
/// leaves blocked are rendered by the shape report (the `Report`
/// lines, which build no form).
///
/// **When check 4 was restated as frame, incidence and fidelity, check 3
/// left the box, and the loop walk's branch became a literal** (PCERT,
/// PR 3812), every line moved. Against the pin it replaced, in three
/// steps, each measured on its own head:
///
/// | line | base | parts 1 + 2 | literal branch | frame twin | escape (PR 3981) |
/// |---|---|---|---|---|---|
/// | `Plain/Decision` calls / forms / frozen | 1255 / 21366 / 1176 | 1141 / 16247 / 696 | 1127 / 16018 / 696 | 1127 / 16429 / 696 | 1143 / 16623 / 696 |
/// | `Plain/Assertion` calls / forms | 578 / 3233 | 612 / 3237 | 666 / 3474 | 666 / 4051 | 650 / 3921 |
/// | `Plain/Report` calls | 176 | 62 | 32 | 32 | 40 |
/// | `Early/Decision` calls / forms | 552 / 14290 | 426 / 9125 | 416 / 8899 | 416 / 9188 | 432 / 9430 |
/// | `Early/Assertion` calls / forms | 578 / 4144 | 612 / 4166 | 666 / 4404 | 666 / 5015 | 650 / 4837 |
/// | `Door/Decision` calls / forms | 662 / 19170 | 558 / 13722 | 590 / 13212 | 590 / 13928 | 582 / 13610 |
/// | `Door/Assertion` calls | 322 | 356 | 410 | 410 | 394 |
///
/// - **Parts 1 and 2.** Check 3's samples on the wall rows are no longer
///   decided over the box (they cross-check at the point witness,
///   unlogged), so the map-residual forms leave every walk, and the
///   step's forms and `Plain/Decision` freezes fall (1176 → 696; which
///   share is check 3's and which check 4's was not split). Check 4
///   re-derives each row's image, so the
///   derivation's assertions are taken twice (`Assertion` calls up).
/// - **The literal branch.** The stored azimuths carry no `floor`
///   node; the walk asks each branch as sign decisions at the
///   half-period marks (`Assertion` and `Door` calls up). Check 4's
///   fidelity is a theorem on every walked row, so the blocked
///   residuals the `Report` lines render halve (62 → 32).
/// - **The frame twin.** Check 4 derives on the chart's Gram–Schmidt
///   twin (`n̂ = axis/‖axis‖`, `ê₁` normalised) and adds the `Frame`
///   term, bounded in the frame's invariants: the twin's
///   normalisations and the invariants are new forms in every walk.
///   Calls and freezes do not move: the twin is the same frame on this
///   plate's literal charts, so no decision is added or lost.
/// - **The escape (PR 3981).** Check 5 decides only an escape's
///   positive part (`geom_brep::pcurve_cache::escape`), so every
///   trim-containment margin carries a `Max` node: new forms in every
///   walk, and the calls move between lines (`Plain` and `Early`
///   `Decision` +16 and `Assertion` −16, `Door/Decision` −8,
///   `Door/Assertion` −16). Eight trim margins that read a definite
///   clearance now read Zero numerically, and the form cannot discharge
///   through the `Max`, so the blocked residuals the `Report` lines
///   render rise 32 → 40 (40 → 48 without the canonical root). Every
///   verdict passes as before: Zero and Negative are both contained.
/// - **The variable table (INTENT-VARS-1 PR 2).** A variable's symbol
///   is its minted id rather than a hash of its name, so the plate's
///   variables sort differently inside the forms: the five digests of
///   lines that build forms move, and every count — calls, forms,
///   frozen — holds.
/// - **The joint elements (the re-anchor ruling, PR 4024).** A loop's
///   walk decides its closure joint as every other joint, between two
///   images, and reads the winding off the integer elements, so the
///   closure margins go and the closure joints' branch, pole-lever and
///   continuity margins come: `Assertion` calls +14 on every walk and
///   `Door/Decision` +14, `Plain` and `Early` `Decision` forms −27 and
///   −31. Every digest moves; the freezes hold.
/// - **The chart-angle unit (PCERT,
///   `pcurve-loop-decisions-state-a-3d-identity-plus-a-branch-margin`).**
///   The loop's chart-space angle comparisons and check 5 retire, so
///   the walks lose exactly those decisions' forms and nothing they
///   shared: `Plain/Decision` 1143 → 1047 calls (16228 → 15720 forms),
///   `Early/Decision` 432 → 352 (9117 → 8689), `Door/Decision`
///   448 → 392 (6768 → 6474), each `Assertion` line 16 calls down with
///   its forms unmoved, and every freeze unmoved. The two `Report` lines
///   go: the 40 residuals they rendered were among the retired
///   decisions', and nothing else blocks on the plate under the shipped
///   rules (8 still do with the canonical root off, against 48 before).
///   The digests of the three `Decision` lines move; the
///   `Assertion` lines' hold.
/// - **The cap orientation (CARVE).** Each extruded cap decides
///   `cap_plane_orientation` once: `Assertion` calls +6 on both walks
///   and `Door` +4 on each line here, +4 per walk on the slab at every
///   ε row. `Decision` calls hold; the plate's `Plain` and `Early`
///   `Decision` forms fall 10 and 56, every digest of a line that
///   builds forms moves, and the freezes hold. `Door/Decision`'s forms
///   rise 6768 → 7465 with it alone, and to 7393 once composed with the
///   one-segment loop (#4169): measured with this change applied at
///   that merge's first parent and at the merge, where #4169 alone
///   leaves the line at 6768.
/// - **The two composed** (the chart-angle unit merged over the cap
///   orientation): the calls add, `Decision` 1047 / 352 / 396 and every
///   `Assertion` line 654 / 654 / 396; `Plain/Decision`'s forms fall
///   15720 → 15710 and `Early/Decision`'s 8689 → 8633 with the cap's own
///   10 and 56, the `Assertion` forms are the cap's, and
///   `Door/Decision` builds 7099 forms. No `Report` line.
/// - **A slot holds a variable (INTENT-LITERALS PR C).** A formula
///   written at a slot lowers to an anonymous defined variable, which
///   the environment binds through the non-finite door: one more
///   `Plain/Decision` call, a theorem, and that line's digest. Every
///   other line holds; the untoleranced variables bind as constants
///   (VR8), as the literals did.
/// - **The pcurve envelope sums two more terms (`pcert/projected-image`).**
///   `EnvelopeTerm` gained `Incidence` and `Fidelity`, the projected
///   image's, and the closed-form lanes' envelope still folds every slot,
///   so each `pcurve_envelope` decision adds two zero-valued summands:
///   `Plain/Decision`'s and `Early/Decision`'s forms rise 24 each
///   (15710 → 15734, 8633 → 8657, every one an `Add`), their digests
///   move, and the calls, the freezes and every other line hold.
/// - **A measure is an operation (INTENT stage 2 unit D).** The web is
///   the assertion's definition over the distance measure's output, so
///   the measured value enters the arithmetic as a variable the
///   definition reads: one more `Plain/Decision` form (15734 → 15735),
///   and that line's digest. Calls, freezes and every other line hold.
///
/// An intrinsic edge description's surfaces becoming a set
/// (`geom_brep::SurfacePair`), measured on the tree holding every change
/// above: the certificate reads the transversality wedge's
/// `n1.cross(n2)` in key order, a form distinct from the extrude's
/// builder-order one, so `Plain/Assertion` 4173/360 → 4349/389 (+176
/// forms, +29 frozen), `Early/Assertion` 5136 → 5312 (+176) and
/// `Door/Decision` 7099 → 7181 (+82); calls, `*/Decision` forms and
/// every decision tally hold (`m10_10_pins_interval` reads the pair's
/// sum unchanged).
///
/// Check 4 reading one vector at its norm where it summed its parts,
/// each measured by reverting it alone on the tree holding the rest:
/// - the turning cylinder's class circle (one `σ` for the radius and
///   orientation terms): `Plain/Decision` +267 forms and +72 frozen,
///   `Early/Decision` +267;
/// - the cylinder class selector's radial circle at its exact sup
///   (`radial_amplitude`, read at the door and on both walks):
///   `Plain/Decision` +20, `Plain/Assertion` +193 forms and +16 frozen,
///   `Early/Decision` +20, `Early/Assertion` +193, `Door/Decision`
///   +196;
/// - the two fidelity channels read as one displacement's legs (`√(u² +
///   v²)` in the envelope's total): `Plain/Decision` and
///   `Early/Decision` +36 each;
/// - `EnvelopeTerm::Tilt` and `EnvelopeTerm::Orientation` retired, one
///   zero summand fewer per envelope each: `Plain/Decision` and
///   `Early/Decision` −12 per slot, −24 in all.
///
/// Together `Plain/Decision` 15749/252 → 16048/324, `Plain/Assertion`
/// 4349/389 → 4542/405, `Early/Decision` 8671 → 8970, `Early/Assertion`
/// 5312 → 5505, `Door/Decision` 7181 → 7377, and every digest of those
/// lines; calls, every decision tally and the largest form hold, and
/// the slab does not move.
///
/// Check 4 reading a `cos t`/`sin t` pair at its exact sup
/// (`√(m + √(q² + (a·b)²))`, `√(a² + b²)`) instead of the sum of its
/// coefficients' norms: a longer expression per envelope term, so
/// `Plain/Decision` 15735 → 15749 and `Early/Decision` 8657 → 8671
/// (+14 forms each) and those lines' digests; calls, freezes and every
/// other line hold.
///
/// A form's digest no longer carries a read's gate (DECIDE-10), so the
/// `Early/Assertion` and `Door/Decision` digests move on every form the
/// decision read built, with every count held; and the contradiction
/// check asks the tier's algebra without the door, so the
/// `Door/Assertion` line, 396 calls that built nothing, is gone. The
/// slab's `Early/Assertion` digests move the same way at each ε row.
///
/// The pcurve envelope's orthonormal chart squaring its frame by cross
/// products (`geom::square_frame`) builds its frame terms in another
/// form: `Plain/Decision`, `Early/Assertion` and `Door/Decision` +14
/// forms each, and the `Plain/Assertion` digest; every call and freeze
/// holds.
///
/// The wedge between an edge's faces levered at the faces' extent
/// (`topo::wedge_reach`) rather than the edge's: each certification and
/// each check-4 read folds the squared distances of the faces' boundary
/// vertices and their maximum, so `Plain/Assertion` 4542 → 5753 forms
/// (its 405 frozen held), `Early/Assertion` 5519 → 6972 and
/// `Door/Decision` 7391 → 7987; `Early/Decision` 8970 → 8955, the arm
/// the edge's chord no longer builds; every call, every decision tally
/// and the largest form hold.
///
/// The final non-finite check asking boundedness
/// (`expr-non-finite-asks-value-times-zero`): `Plain/Decision`
/// 1048 → 1003 calls and 16062 → 16013 forms, `Plain/Assertion`
/// 5753 → 5790 forms, which it now builds where the check's walk built
/// them first, and both digests; the frozen counts, the `Early/*` and
/// `Door/*` lines and the largest form hold.
///
/// Placing each row over its edge's span in the chart's principal
/// window (C4) reads two half-period marks per placed azimuth, each
/// decided numerically, so each is checked against the tier: 36
/// placements, `Plain/Assertion` and `Early/Assertion` 654 → 726 calls
/// and `Door/Decision` 396 → 468, `Plain/Assertion` frozen 405 → 417.
/// A cylinder's meridian azimuth is read only on a meridian, so the
/// rim circles centred on the axis no longer take the frame read of
/// their zero radial offset: `Plain/Decision` 1048 → 1032 calls. Forms
/// and digests move on every line; the largest form holds.
/// The placement's lever, the direction's length, squares by
/// `powi(2)`: the `Plain/Assertion` digest, with every count held.
/// With the wedge levered at its faces' extent as well, the calls and
/// frozen counts are the placement's and the forms take both: Plain/Assertion
/// 6081, Early/Decision 8925, Early/Assertion 7244, Door/Decision 8231;
/// the largest form holds. The placement's azimuth levered at the
/// carrier's farthest reach from the axis at the span's ends and midpoint
/// (PR 5225's one lever rule) builds each mark's form from three points:
/// every call holds; forms Plain/Decision 15888, Plain/Assertion 6909
/// (frozen 417 → 405), Early/Decision 8889, Early/Assertion 8108,
/// Door/Decision 9469. Main's final non-finite check and the placement
/// add: `Plain/Decision` 1032 → 987 calls (main's −45) and forms
/// 15888 → 15839, `Plain/Assertion` forms 6909 → 6946, both digests;
/// every other line is the placement's. The frame decided across the
/// window's cut builds the cut-side margin's form in place of `x`'s:
/// forms Plain/Assertion 6946 → 7020, Early/Assertion 8108 → 8182,
/// Door/Decision 9469 → 9545, and their digests; every call and frozen
/// count holds.
///
/// A harmonic row's windings are structure it stores and certification
/// compares exactly (PCERT B2a, `[ev]` PR 5113's first decision), so
/// the decisions that read a winding off a slope are gone: the angle
/// channel's two affine decisions, the winding walk and the harmonic
/// lane's period gate, `Plain/Decision` 987 → 939 calls and
/// `Plain/Assertion`, `Early/Assertion` 726 → 686, `Door/Decision`
/// 468 → 428; forms and digests move with them, and freezes and the
/// largest form hold.
///
/// A winding read as its slope through the chart channel's `Period`
/// (`Period::rate`: `times(k)` over the carrier's period, PCERT B2a's
/// fix pass, H1) rather than as a literal: each angle channel's slope
/// is a quotient of two forms, so `Plain/Decision`, `Plain/Assertion`,
/// `Early/Decision` and `Early/Assertion` +3 forms and `Door/Decision`
/// +10, with every digest; every call, every freeze and the largest
/// form hold.
///
/// A chord's director spelled as its components, as on the slab:
/// forms `Plain/Assertion` 7003 → 7043 and `Early/Assertion`
/// 8165 → 8205, and their digests; every call, frozen count, the three
/// `*/Decision` lines and the largest form hold.
///
/// The profile's joint pass reading each joint's turn, levered by the
/// shorter leg's arm, and the path door reading each junction a
/// departing leg follows again once that leg is stored (#5354's ruling
/// and #5377's fix pass): `Plain/Assertion` and `Early/Assertion` +7
/// calls (686 → 693) and +30 forms, with their digests; the
/// `*/Decision` lines, the freezes and the largest form hold.
///
/// Check 4 deciding no whole period (PCERT B2b): fidelity's 32 branch
/// decisions are gone, each a numeric decision checked against the
/// tier, so `Plain/Assertion` and `Early/Assertion` 693 → 661 calls and
/// `Door/Decision` 428 → 396; forms and digests move with them, and
/// `Plain/Decision`'s calls, every freeze and the largest form hold.
/// Measured on the tree with every move above.
///
/// The arc's diameter clearance spelled `2r·cos²(|Δθ|/4)/(1 + sin(|Δθ|/4))`
/// (`profile`'s `seg::classify`), where `1 − sin(|Δθ|/4)` cancelled near
/// a full turn: forms `Plain/Assertion` 7013 → 7017, `Early/Assertion`
/// 8175 → 8179 and `Door/Decision` 9465 → 9471, with their digests;
/// every call, every freeze and the largest form hold.
///
/// INTENT stage 3 D (Transform retires into a placement) re-spells the
/// plate without datum frames, each construction placed into the world
/// by a `Place`, and the box lanes' solved map stays tight (a spin set
/// from the two charts' frames, a direction exactly on an axis read as
/// that axis, Sym snapping at rest where its lane does). Together:
/// `Plain/Decision` 1003 → 1009 calls, 16013 → 16400 forms and 324 → 24
/// frozen; `Plain/Assertion` 654 → 662 calls, 5790 → 5800 forms and 405
/// → 137 frozen; `Early/Decision` 352 → 254 calls and 8955 → 8496
/// forms; `Early/Assertion` 654 → 662 calls and 6972 → 7204 forms;
/// `Door/Decision` 396 → 406 calls and 7987 → 8218 forms; every digest.
/// The largest form holds.
///
/// Re-taken under FORK-REST (only the identity bundle as written places
/// by no map): the plate's two holes are placed by plane mates, which
/// the narrowing's walk let snap, and now run their maps, so their
/// transforms and the copies' re-certification are walked too. Every
/// line rose, identically at all three ε rows: `Plain/Decision` 945 ->
/// 1253 calls, 16224 -> 23528 forms, 24 -> 969 frozen;
/// `Plain/Assertion` 701 -> 1043 calls, 7084 -> 11606 forms, 137 -> 588
/// frozen; `Early/Decision` 254 -> 554 calls, 8434 -> 15715 forms;
/// `Early/Assertion` 701 -> 1043 calls, 8468 -> 12990 forms;
/// `Door/Decision` 438 -> 780 calls, 9760 -> 17979 forms; every digest
/// (the "before" is D's head before FORK-REST, `434139ef1c`, after
/// main's merge, which had moved the plate from the line above). The
/// largest form holds. Without the canonical root the plain walk
/// renders 16 blocked residuals where it rendered 8, the copies' beside
/// the construction's.
///
/// Merging main at `557e581a64` (check 4 deciding no whole period, PCERT
/// B2b) takes fidelity's numeric branch decisions out of the plate's
/// walks, twice main's own count because D walks each hole twice:
/// `Plain/Assertion` and `Early/Assertion` 1043 -> 979 calls and
/// `Door/Decision` 780 -> 716, forms 16 to 116 fewer on every line, with
/// their digests; every freeze and the largest form hold. A spin written
/// as a fraction of a turn (the holes' constant `0`) now enters the
/// walks as its exact sine and cosine (`geom_core::Angle`), not as the
/// sine and cosine of a radian, which moves the two `Plain` digests and
/// no count (measured both ways on D's tree).
///
/// **Pinned at what main measures** (2026-10-29), not explained: every
/// line moved. The plate is the two-hole plate that PR 5728 and PR 5811
/// moved (`work/offset/main-red-three-m10-interval-pins-moved-at-5728`);
/// #5542's `segment_gap` moved it first, and PR 5952 took the
/// `*/Report` lines #5542 brought away again
/// (`work/cpair/main-red-slow-set-receipts-moved-at-5542.md`).
/// `Early/Decision`'s digest is pinned at main's, not explained
/// (`work/green/g-moves-four-main-reds-messages.md`).
const PLATE_LEDGER: &str = "\
     Plain/Decision calls 1149 forms 20618 frozen 36 digest 5b68ab11cb6a6488e4a842bd68fd8b54\n\
     Plain/Assertion calls 824 forms 9331 frozen 193 digest 3b9e5da836f18dc5a35a3a4ece119754\n\
     Early/Decision calls 342 forms 11649 frozen 0 digest 38b61e2e3311ab58939c93cd540bd52b\n\
     Early/Assertion calls 824 forms 10745 frozen 0 digest bfcde6cb8486b98883080e5e22c9cf20\n\
     Door/Decision calls 557 forms 13569 frozen 0 digest 07819a3c990e37a0b06cfef06cbc955d";

/// **What the walks BUILD is pinned, not only what the tier decides.**
/// For the slab and the plate at their nominals, every (walk, origin)
/// line of the profile's walk ledger — calls, forms, frozen, and the
/// digest chain of the forms themselves (`WalkProfile::digest`: each
/// memoized form's canonical digest, the key an atom over it is minted
/// under, chained in build order). The ledger was captured on the tree
/// that held a form as a `BTreeMap` of terms and is asserted since: a
/// form's storage may change, and what it SAYS — every coefficient,
/// every term in the map's order, hence every atom key and every
/// decision — may not. The pins hold the decisions; this row holds
/// the forms behind them, so a swapped term order or a coefficient
/// normalised differently reads here even where the outcome survives
/// it. The slab's ledger is per ε row, because its literals are
/// multiples of ε.
#[test]
fn the_forms_the_walks_build_are_pinned_per_eps_row() {
    let tol = Tol::witness();
    let row = eps_row(tol.eps());
    let slab_doc = slab();
    let plate_doc = the_plate(tol);
    // Both ledgers are taken before either is read, so a red on the
    // slab still shows the plate's.
    let taken: Vec<_> = [
        ("slab", &slab_doc, SLAB_LEDGER[row], SLAB_MAX_TERMS),
        ("plate", &plate_doc, PLATE_LEDGER, PLATE_MAX_TERMS),
    ]
    .into_iter()
    .map(|(name, doc, expected, max_terms)| {
        let (_, nominal) = boxes(doc).into_iter().next().unwrap();
        start_profile();
        let (_, _, counts) = replay(doc, &nominal, SymRules::shipped(), tol);
        let profile = take_profile();
        let ledger = profile.walk_ledger();
        let largest = profile
            .ops
            .values()
            .map(|o| o.max_terms_out)
            .max()
            .unwrap_or(0);
        println!(
            "\nLEDGER {name} eps {:e} largest form {largest}\n{ledger}",
            tol.eps()
        );
        (name, ledger, expected, counts, largest, max_terms)
    })
    .collect();
    for (name, ledger, expected, counts, largest, max_terms) in taken {
        assert_eq!(
            largest,
            max_terms,
            "{name} at eps {:e}: the largest form built has {largest} terms, pinned {max_terms}",
            tol.eps()
        );
        assert_eq!(
            ledger.trim(),
            expected.trim(),
            "{name} at eps {:e}: the walks built different forms (counts {counts:?})",
            tol.eps()
        );
    }
}

/// **The walk ledger on the documents the pinned row does not
/// measure** — the M10-10 evidence set at scale 1 (the plate, the two
/// brackets, the annulus, the pad, the link), at the nominal and over
/// a leaf-sized box — printed, not asserted, so a merge-base build and
/// a head build of one row can be diffed: every counter and every
/// digest chain must agree. From SYM-4's reviews (both reviewers ran
/// this differential: `origin/sym/4-review-r2`'s row, with
/// `origin/sym/4-review-r1`'s four-document twin), which is the
/// coverage the unit's own record did not claim.
#[test]
#[ignore = "evidence-only: the walk ledger on six documents, for a merge-base differential"]
fn the_walk_ledger_on_the_unmeasured_documents() {
    let tol = Tol::witness();
    let docs: Vec<(&str, ProfileDoc)> = vec![
        ("two_hole_plate", the_plate(tol)),
        (
            "r2_filleted_bracket",
            crate::m10_7_r2_probes_interval::bracket(1.0, tol).0,
        ),
        (
            "r1_bracket",
            crate::m10_8_arc_family_interval::documents(tol)
                .pop()
                .map(|(_, d)| d)
                .unwrap(),
        ),
        (
            "r1_annulus",
            crate::m10_8_r1_probes_interval::annulus(1.0, tol).0,
        ),
        (
            "r2_rounded_pad",
            crate::m10_8_r2_probes_interval::pad(1.0, tol).0,
        ),
        ("r2_link", crate::m10_9_r2_probes_interval::link(1.0, tol).0),
    ];
    for (name, doc) in &docs {
        for (scale, box_) in boxes(doc) {
            if scale == "root" {
                continue;
            }
            start_profile();
            let t0 = Instant::now();
            let (shapes, refusal, counts) = replay(doc, &box_, SymRules::shipped(), tol);
            let wall = t0.elapsed();
            let prof = take_profile();
            println!(
                "LEDGER {name} {scale} eps {:e} counts {counts:?} refusal {refusal:?} outcomes {:?} nodes {} atoms {} freezes {} rat_ops {} big_ops {} promotions {} widest {} refused {} (wall {wall:?}, local)\n{}",
                tol.eps(),
                outcomes(&shapes),
                prof.nodes,
                prof.atoms,
                prof.freezes.len(),
                prof.rat_ops,
                prof.big_ops,
                prof.promotions,
                prof.widest_bits,
                prof.widest_refused_bits,
                prof.walk_ledger()
            );
        }
    }
}

/// **Which dial moves which line of the plate's ledger.** The unit
/// that shipped rule G re-baselined the plate's `PLATE_MAX_TERMS` and
/// both of its `Plain/*` digests and said rule G had moved them; it
/// had not — the plain walk consults neither new dial, and the two
/// numbers were stale against the pin before this branch was cut. This
/// row is what keeps the attribution honest: it replays the plate under
/// each dial set and asserts that the PLAIN lines are identical across
/// all of them, so a future claim that a rule of the early walk moved
/// a plain form reds here first.
#[test]
fn the_plains_ledger_lines_are_the_same_under_every_dial_set() {
    let tol = Tol::witness();
    let doc = the_plate(tol);
    let (_, nominal) = boxes(&doc).into_iter().next().unwrap();
    // `Plain/Report` is not a form the plain walk builds: it counts the
    // residuals rendered because they BLOCKED, and which ones block is
    // the early walk's rules' to decide. So it is the one plain line a
    // dial may move, and its count is pinned per dial set instead of
    // compared across them: a dial that starts moving it moves a pin.
    let is_report = |l: &str| l.trim_start().starts_with("Plain/Report");
    let plain_lines = |ledger: &str| {
        ledger
            .lines()
            .filter(|l| l.trim_start().starts_with("Plain/") && !is_report(l))
            .map(|l| l.trim().to_owned())
            .collect::<Vec<_>>()
    };
    let report_calls = |ledger: &str| {
        ledger
            .lines()
            .find(|l| is_report(l))
            .and_then(|l| l.split_whitespace().nth(2))
            .and_then(|n| n.parse::<u64>().ok())
    };
    let sets: [(&str, SymRules, Option<u64>); 4] = [
        // Pinned at what main measures (2026-10-29): the two sets that
        // render blocked residuals render 12 where they rendered 16.
        // #5542's merge moved every set's count (to 8 and 20) and PR 5952
        // brought the shipped sets back to none
        // (`work/cpair/main-red-slow-set-receipts-moved-at-5542.md`).
        ("shipped", SymRules::shipped(), None),
        (
            "without_canonical_root",
            SymRules::without_canonical_root(),
            Some(12),
        ),
        ("without_the_reads", SymRules::without_the_reads(), None),
        (
            "both new dials off",
            SymRules {
                decision_read: false,
                ..SymRules::without_canonical_root()
            },
            Some(12),
        ),
    ];
    let mut seen: Option<(&str, Vec<String>)> = None;
    for (name, rules, blocked) in sets {
        start_profile();
        let _ = replay(&doc, &nominal, rules, tol);
        let p = take_profile();
        let largest = p.ops.values().map(|o| o.max_terms_out).max().unwrap_or(0);
        let ledger = p.walk_ledger();
        let lines = plain_lines(&ledger);
        println!(
            "  {name}: largest form {largest}, Plain/Report {:?}\n    {}",
            report_calls(&ledger),
            lines.join("\n    ")
        );
        assert_eq!(
            report_calls(&ledger),
            blocked,
            "{name}: the plain walk renders a different number of blocked residuals"
        );
        match &seen {
            None => seen = Some((name, lines)),
            Some((first, want)) => assert_eq!(
                &lines, want,
                "{name} builds a different PLAIN walk from {first}: the plain form reads no \
                 dial of the early walk, so a difference here is a defect and not a \
                 re-baseline"
            ),
        }
    }
}
