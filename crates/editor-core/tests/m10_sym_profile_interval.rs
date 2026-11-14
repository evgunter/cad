//! **Where the E12 tier's time goes** — the cost profile of the
//! symbolic normal form on the M10-3 slab and the two-hole plate
//! (`work/sym/symbolic-tier-costs-95-percent-of-the-m10-3-drive`).
//!
//! The `#[ignore]`d rows are EVIDENCE-ONLY: each
//! prints the tier's structural profile (`geom_core::sym::profile`,
//! behind the test-only `sym-profile-testing` feature the
//! dev-dependency edge turns on) for one replay or one drive, and
//! asserts nothing about the numbers — they are the record, read into
//! the item body and the tier's `# Cost` section, and re-taken by
//! re-running the row. The fixtures are the ones the M10-3 suite
//! drives (`bounded_chamber`, the row S-TCOST bisected on) and the
//! plate the tier was built for, through the same doors.
//!
//! The gating rows read the same instrument for the one thing in it
//! that is not a clock: the walk ledger — per walk and origin the
//! calls, the forms, the frozen, and the digest chain of the forms
//! built — so what the tier BUILDS on both documents is a pinned
//! number, over and above what the pins say it DECIDES
//! ([`assert_walk_ledger`]).
//!
//! The one row without the profile installed,
//! [`sym_profile_callgrind_replay`], is the callgrind target: it
//! replays one box a chosen number of times and nothing else, so an
//! instruction count over it is the tier's and not the instrument's.
//! The command that takes it is recorded in the item body.
//!
//! **This file is ungated**: it carries no `gated_to!` marker, so every
//! row here outside the `ci` slow set runs on every PR whose closure
//! reaches editor-core. A new fast row inherits that. The walk ledger
//! reads every kernel change the documents meet, so no path list could
//! name what moves it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

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
    let v = drive(&doc, &analyzed, &config, &editor_core::Stop::default(), tol).unwrap();
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

/// The slab's walk ledger at its nominal, per ε row: its literals are
/// multiples of ε.
///
/// What moves it is what moves [`PLATE_LEDGER`]; on the slab the
/// edges' mid-parameter points are the lever — the witness an edge is
/// minted with, the certificate's midpoint check and its schedule's
/// middle station are one [`geom::mid_param`] evaluation, and a second
/// spelling of that point builds a second chain of forms, which reads
/// here as `Plain/Decision` forms alone.
///
/// **Moved with PR 6594** (the profile's chord reads onto
/// `geom_core::Line2`), with the calls unchanged:
/// - `Line2::side` and `Line2::height` divide by `|dir|` (the A′
///   metric reading), where the stored unit was read bare. That adds
///   all of `Plain/Decision` 9789 → 9815 and `Early/Decision` 30 → 38,
///   and 22 of each `Assertion` row's growth.
/// - The other 12 of each (`Plain/Assertion` 2827 → 2861,
///   `Early/Assertion` 4251 → 4295) are the line×arc pair and the ray
///   cast: each reading now forms its own `C − a`, where the old code
///   shared one `to_center`. The values are the same.
///
/// Measured by undoing each in turn on the PR's first commit: with both
/// undone, the ε 1e-9 row's counts and digests are the old pin's.
///
/// **When a refused direction began reading its norm once**
/// (`Vec3::checked_direction_and_length`, which the direction door
/// also takes its direction from rather than normalizing again):
/// `Plain/Decision` 9815 →
/// 9809, `Plain/Assertion` 2861 → 2807, `Early/Assertion` 4295 → 4235,
/// with the same calls; `Early/Decision` does not move.
///
/// **Moved with the constructors' whole-wedge reads**: the extrude and
/// revolve read each edge they write by its whole wedge, the arm, the
/// wedge and a corner's sign at each of its 7 interior stations, where
/// they read the class at one witness. `Plain/Assertion` and
/// `Early/Assertion` 532 → 988 calls (2807 → 3503 and 4235 → 4931
/// forms), `Plain/Decision` 9809 → 9709 forms over the same calls (an
/// edge's witness is its carrier's middle, which the walks spell with
/// fewer forms than a strut's chord midpoint).
///
/// **Moved with a line rim's span read from its sketch segment** (PR
/// 6953): a placed line's span is `‖b − a‖` over the sketch and its
/// direction the sketch chord's unit placed, where both were read off
/// the placed vertices, and its carrier end is registered against the
/// placed far vertex, as an arc's is (`register_placed_line_end`).
/// - The span spelling alone takes `Plain/Decision` 9709 → 9663 and
///   `Plain/Assertion` 3503 → 3391 forms, same calls (PR 6953's review,
///   on its first head): the 3-space difference `q_to − q_from`, its
///   three squares, its norm and its normalization are formed no more
///   for each line rim, and the two-coordinate sketch chord and its one
///   norm, over sketch values other forms already hold, replace them.
/// - The registration adds the `Door/Decision` walk, 940 calls and 5923
///   forms: the door now proves each line rim's far-end decisions (check
///   3) by the registered alias (the carrier's end against its placed
///   sketch end, as an arc's is), and those decisions build no plain
///   forms. With it `Plain/Decision` is 9559 forms, `Plain/Assertion`
///   3375 and `Early/Assertion` 4931 → 4854, calls unchanged.
const SLAB_LEDGER: [&str; 3] = [
    "\
     Plain/Decision calls 976 forms 9559 frozen 0 digest cb0330eb6422ff0b3787f555a7c6bd20\n\
     Plain/Assertion calls 988 forms 3375 frozen 0 digest 9d51ee8e5d27c08f84d2c19d6ada93a1\n\
     Early/Decision calls 16 forms 38 frozen 0 digest 9da516ca1875052288aafee2b397dd61\n\
     Early/Assertion calls 988 forms 4854 frozen 0 digest afe8371cf57350ba09777217386d7224\n\
     Door/Decision calls 940 forms 5923 frozen 0 digest 02135644926066d0be3dbbdccb0a7961",
    "\
     Plain/Decision calls 976 forms 9559 frozen 0 digest 1e76b0ed45fb1ecc9dbf35db192b2b8f\n\
     Plain/Assertion calls 988 forms 3375 frozen 0 digest 2617924e0a82d92b5de4344b48457546\n\
     Early/Decision calls 16 forms 38 frozen 0 digest 9da516ca1875052288aafee2b397dd61\n\
     Early/Assertion calls 988 forms 4854 frozen 0 digest cba48293163310adb85acb8777a4e600\n\
     Door/Decision calls 940 forms 5923 frozen 0 digest a89f86054bc776e1b538a64d3ec5e4dd",
    "\
     Plain/Decision calls 976 forms 9559 frozen 0 digest 2ea5e2fe9a0970fe0e4699410cb53451\n\
     Plain/Assertion calls 988 forms 3375 frozen 0 digest 6e29316b4194cf0b9f3a5a71e3d022b4\n\
     Early/Decision calls 16 forms 38 frozen 0 digest 9da516ca1875052288aafee2b397dd61\n\
     Early/Assertion calls 988 forms 4854 frozen 0 digest 1ac419697f3342ab925a63c087641c79\n\
     Door/Decision calls 940 forms 5923 frozen 0 digest ba28753dc636952bb25af72793dec4f9",
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
/// (`work/sym/rule-d-leaves-tan-of-atan-opaque-and-the-cap-apex-mints-it`).
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
const PLATE_MAX_TERMS: usize = 38;

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
/// the certificate's decisions — its frame, incidence and fidelity
/// checks — are walked too, and a change to how any of them reads moves
/// this ledger. The residuals the certificate leaves blocked are
/// rendered by the shape report (the `Report` lines, which build no
/// form).
///
/// The holes' plane mates place by a reach read off the faces' cover
/// (`topo::cover`): its norms, its maxima over corners and its arc
/// windows' sines and cosines are `Plain/Decision` forms, and the
/// largest form ([`PLATE_MAX_TERMS`]) is one of them, so a change to how
/// a cover is read can move that line and that bound with no decision
/// moving on the plate at these ε. That is the plate's fact, not a rule:
/// freezing is not monotone in reach, so a form frozen past the degree
/// budget can flip a verdict elsewhere.
///
/// **Moved with ONEHOME's composed placements** (`mate::solve::admitted`
/// removed: a solved placement is composed as motions and decides no
/// rigidity margin): the two holes' placements no longer decide their seven
/// rigidity margins, so `Plain/Decision` loses those 14 calls (1203 →
/// 1189) and the nine forms they built (55907 → 55898). No other line
/// moves.
///
/// **Moved with ONEHOME's moved copy** (a move carries its verdict,
/// `Held::moved`): the cut reads each hole into the plate's frame with
/// its certificates and rows carried, so neither hole's rims are
/// certified again there. Every walk loses that copy's share: `Plain`
/// 1189 → 1015 decisions and 824 → 669 assertions, `Early` 412 → 308
/// and 824 → 669, `Door` 557 → 402. The largest form holds at 38.
///
/// **Moved with the move's session-box ask** (`Replay::moved_by`: every
/// move asks the box of its image): one assertion per walk more, the
/// box's margin, a form over every vertex's and edge's reach of the
/// moved hole: `Plain` 669 → 670 assertions (forms 6963 → 9047),
/// `Early` 669 → 670 (8611 → 10422), `Door` 402 → 403 decisions (9906 →
/// 11719). `Plain/Decision`'s forms fall 42005 → 41501 with its
/// calls unchanged, and that is the memo, not a decision: the Plain walk
/// keeps one memo for both origins, and a form is charged to the origin
/// that first builds it (`sym::profile`'s per-`(walk, origin)` entry).
/// The box ask's assertion runs at the move, before the decisions over
/// the moved hole, and builds 504 of the vertex- and edge-reach forms
/// those decisions built before; they now hit the memo. So
/// `Plain/Assertion` gains 2084 forms, those 504 and the 1580 the
/// assertion alone needs, and Decision plus Assertion grows by 1580.
///
/// PCERT's closed-form bound (a stated harmonic image bounded by its
/// residual's coefficients, main's `4121c920`), and its slack (`closed_bound.rs`
/// `Residual::bound`'s `+ slack`, one more form per bound, main's
/// `b8f04c55`), move none of these counts: the stated images sit on moved
/// copies whose certificates are carried, so no bound is built again in
/// the measuring frame.
///
/// **Moved with the constructors' whole-wedge reads**: the extrude and
/// revolve read each edge they write by its whole wedge, the arm, the
/// wedge and a corner's sign at each of its 7 interior stations, where
/// they read the class at one witness. The assertions 670 → 1050 calls
/// on both walks (`Plain` 9047 → 13400 forms, frozen 137 → 245; `Early`
/// 10422 → 14775), `Door/Decision` 403 → 555 calls (11719 → 15037
/// forms), and `Plain/Decision` and `Early/Decision` keep their calls
/// over fewer forms (41501 → 40761, 28904 → 28232).
///
/// **Moved with a line rim's span read from its sketch segment** (PR
/// 6953, [`SLAB_LEDGER`]'s note): the span spelling alone takes
/// `Plain/Assertion` 13400 → 13293 (PR 6953's review); with the far-end
/// registration the door proves each line rim's far-end decisions,
/// `Door/Decision` 555 → 1002 calls (15037 → 18053 forms), and
/// `Plain/Assertion` reads 13305, `Plain/Decision` 40761 → 40813,
/// `Early/Decision` 28232 → 28205, `Early/Assertion` 14775 → 14807.
const PLATE_LEDGER: &str = "\
     Plain/Decision calls 1015 forms 40813 frozen 54 digest 485731a745d32488b10cc82b2ba6460d\n\
     Plain/Assertion calls 1050 forms 13305 frozen 245 digest 4c5517586a973b448659d89edfc0593a\n\
     Early/Decision calls 308 forms 28205 frozen 0 digest 26327ae7eaa72dca689e4aa27ad071c4\n\
     Early/Assertion calls 1050 forms 14807 frozen 0 digest 2925bd5da0c54a8d53a602781a6be0b3\n\
     Door/Decision calls 1002 forms 18053 frozen 0 digest e4240acf1c23fd2839211cad92c0c778";

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
/// it.
///
/// One replay of `doc` at its nominal; panics unless its largest form
/// and its ledger are the pinned ones.
fn assert_walk_ledger(name: &str, doc: &ProfileDoc, expected: &str, max_terms: usize, tol: Tol) {
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

/// The slab's walk ledger ([`assert_walk_ledger`]), per ε row. One
/// replay, cheap enough for the per-PR gate, so a PR that moves what
/// the slab's walks build sees it there: it stands in, on the gate, for
/// [`the_forms_the_plates_walks_build_are_pinned`] and
/// [`the_plains_ledger_lines_are_the_same_under_every_dial_set`], which
/// replay the plate and run nightly. It does not stand in for the
/// plate's own lines: the slab has no curved wall, so a change to how
/// the certificate reads a cylinder's pcurve moves the plate alone.
#[test]
fn the_forms_the_slabs_walks_build_are_pinned_per_eps_row() {
    let tol = Tol::witness();
    let row = eps_row(tol.eps());
    assert_walk_ledger("slab", &slab(), SLAB_LEDGER[row], SLAB_MAX_TERMS, tol);
}

/// The plate's walk ledger ([`assert_walk_ledger`]); its nominal reads
/// no ε, so one pin serves every row.
#[test]
fn the_forms_the_plates_walks_build_are_pinned() {
    let tol = Tol::witness();
    assert_walk_ledger("plate", &the_plate(tol), PLATE_LEDGER, PLATE_MAX_TERMS, tol);
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
        // The two sets without the canonical root render the blocked
        // residuals of the certificate's reads; the shipped sets none.
        // ONEHOME's moved copy (`Held::moved`): the holes are not
        // re-certified in the plate's frame, so 94 -> 64 of them.
        ("shipped", SymRules::shipped(), None),
        (
            "without_canonical_root",
            SymRules::without_canonical_root(),
            Some(64),
        ),
        ("without_the_reads", SymRules::without_the_reads(), None),
        (
            "both new dials off",
            SymRules {
                decision_read: false,
                ..SymRules::without_canonical_root()
            },
            Some(64),
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
