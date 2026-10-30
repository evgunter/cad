//! **The face walk answers the same thing at any thread count, and the
//! same thing `main` answered.**
//!
//! `topo::props` runs each face's flux in an indexed parallel map
//! (PERF-PLAN §2.2 idiom 1) and sums the slots in a sequential
//! arena-order fold (idiom 2), so the arithmetic is schedule-free by
//! construction. What is NOT free — and what this suite pins — is the
//! K-funnel's recording: the verdict log, the escalation log and the
//! `probe` sample population are thread-local, so a face decided on a
//! worker records into that worker's frame and sink unless the walk
//! composes them back.
//!
//! **The baseline is a committed golden, not the other width.** Two
//! widths of the SAME code agree whenever the loss is width-independent
//! — a walk that dropped every worker's recording identically at one
//! and four threads would pass a t1-vs-t4 row while asserting nothing.
//! So `thread-count-digest/eps-*.txt` is cut on the MERGE BASE, where
//! the walk is serial, and the four-thread walk is read against it: a
//! row cut on this branch would record what this branch does, which is
//! the thing under test. The one case where a lane re-cuts here
//! anyway, and what licenses it, is at `expected`.
//!
//! **What the recorded channels here can and cannot see.** The verdict
//! channel is full — every `props_quad_*` and check-7 decision the
//! walk takes lands in it. The ESCALATION channel is empty on every
//! body of this roster, at every ε the matrix gates, and that is a
//! property of the funnel rather than of the fixtures: the props lanes
//! ask the funnel, get a definite sign, and then mint an
//! `Indeterminate` of their OWN for a budget that ran out, which the
//! frame never sees
//! (`work/props/escalation-channel-misses-op-minted-indeterminates.md`,
//! and `k_stats`' module docs say the same). So these rows pin that the
//! escalation log is still exactly what `main` recorded — which is
//! nothing — and they would catch a composition that invented
//! escalations or moved them; they cannot demonstrate a non-empty
//! escalation log surviving a worker, because no body in this tree
//! produces one. The thin strip is here as the body whose target-level
//! reading REFUSES, which is the escalation PATH through the walk; it
//! is not a body that escalates on the channel.
//!
//! **Its `probe` row lives in `thread_count_probe_populations`**: the
//! sample population is the thing k-lint counts, and only the probe
//! sweep runs a `probe` build, so the row sits in a file the sweep can
//! select without re-running the goldens here.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::{
    arc_section, bulged_extrusion, channels, on_pool, quintic_prism, stacked, strip_section,
    tilted_cut_upper,
};
use geom_core::k_stats::Bracket;
use geom_core::sym::{SymBudget, SymCounts, with_session};
use geom_core::{Sym, Tol};
use sweep::loft_body;
use sweep::test_support::loft_prism;
use topo::Body;

/// A three-station arc loft at scale `s` — rational walls, so every
/// wall face is a certified quadrature and none of them is a closed
/// form.
fn arc_loft(s: f64) -> Body<f64> {
    loft_body::<f64>(
        &[arc_section(s), arc_section(s), arc_section(s)],
        &stacked(&[0.0, 1.0, 2.0], s),
        2,
        Tol::witness(),
    )
    .expect("the arc loft lofts")
    .body
}

/// The thin curved strip, two stations high: two large rational walls
/// whose fluxes nearly cancel. It refused its number at round 0 while
/// a lane refused on a predicted last round; its walls now refine past
/// that, and at `1e12·ε` its number certifies (`sign_walk_plus_v`
/// carries the strip that still refuses, at `1e14·ε`).
fn strip_loft(s: f64, delta: f64) -> Body<f64> {
    loft_body::<f64>(
        &[
            strip_section(s, delta, false),
            strip_section(s, delta, false),
        ],
        &stacked(&[0.0, 1.0], s),
        1,
        Tol::witness(),
    )
    .expect("the strip lofts")
    .body
}

/// The roster: the reporting-door digest's three lanes that this walk
/// can reach from here (`quintic_prism` — composite rounds;
/// `tilted_cut_upper` — the cylinder chart's Green form, which no loft
/// verb produces; `bulged_extrusion` — the same chart from the extrude
/// door), `loft_prism` (described splines, the finding's own body), the
/// arc lofts scaled against the run's own ε so no row goes vacuous at
/// one point of the matrix, and the thin strip.
fn roster() -> Vec<(String, Body<f64>)> {
    let eps = Tol::witness().get().eps;
    let mut out: Vec<(String, Body<f64>)> = vec![
        ("quintic_prism".to_string(), quintic_prism()),
        ("tilted_cut_upper".to_string(), tilted_cut_upper()),
        ("bulged_extrusion".to_string(), bulged_extrusion()),
        // `loft_prism` is in this roster because it is the body the
        // finding measured
        // (`work/perf/mass-properties-are-serial-per-face.md`: 157 ms)
        // — polyline sections, so its walls are described splines on
        // the quadrature lane, which is the lane the per-face serialism
        // is about. A corpus DOCUMENT cannot stand in for it here:
        // `editor-core` sits above this crate, so the kernel-side
        // fixture is the only reachable spelling.
        ("loft_prism".to_string(), loft_prism(Tol::witness())),
    ];
    out.extend(
        [1.0e11, 1.0e9]
            .iter()
            .map(|k| (format!("arc_loft_{k:e}eps"), arc_loft(k * eps))),
    );
    out.push((
        "thin_strip".to_string(),
        strip_loft(1.0e12 * eps, 1.0e9 * eps),
    ));
    out
}

/// One body's line: the reading (bits, or the typed refusal — which is
/// where the refusing FACE is named) and both recorded channels.
fn line(name: &str, body: &Body<f64>) -> String {
    // Finished outside the bracket, so the line is the measurement's own
    // readings. Every body here finishes at every ε, and one that did not
    // would panic here rather than reach the pinned line.
    let at_rest = topo::test_support::finished(name, topo::Body::clone(body), Tol::witness());
    let bracket = Bracket::open();
    let read = match topo::mass_properties(
        &at_rest,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    ) {
        Ok(m) => format!(
            "v={:016x} a={:016x} vpad={:016x} apad={:016x}",
            m.enclosure.volume.to_bits(),
            m.enclosure.surface_area.to_bits(),
            m.enclosure.volume_pad.to_bits(),
            m.enclosure.area_pad.to_bits(),
        ),
        // The refusal's sentence leaves the face's key out; the walk's
        // choice of WHICH face refused is part of what is pinned.
        Err(e @ topo::MassPropsError::Face { face, .. }) => format!("REFUSED at {face:?}: {e}"),
        Err(e) => format!("REFUSED {e}"),
    };
    format!("{name} {read} | {}", channels(&bracket.finish()))
}

/// The f64 block of the digest: every roster body, in a fixed order
/// (the block is compared whole, so the order is part of the pin).
fn f64_block() -> String {
    roster()
        .iter()
        .map(|(name, body)| line(name, body))
        .collect::<Vec<_>>()
        .join("\n")
}

// ---------------------------------------------------------------- //
// The session block: what a decision writes that NO splice can undo.  //
// ---------------------------------------------------------------- //

/// A symbolic session's budget for these rows. Small: the point is that
/// the receipt COUNTS the walk's decisions, not what the tier proves.
fn budget() -> SymBudget {
    SymBudget {
        max_terms: 32,
        max_degree: 4,
    }
}

/// A `Sym` body's builder, run inside the session that reads it.
type SymBody = fn() -> Body<Sym<f64>>;

/// The same two bodies at `Sym<f64>`, the arc loft and the thin
/// strip, as builders, so a session can be installed
/// before their nodes are minted ([`receipts`]).
fn sym_bodies() -> [(&'static str, SymBody); 2] {
    [
        ("sym_arc_loft", sym_arc_loft),
        ("sym_thin_strip", sym_thin_strip),
    ]
}

/// The arc loft at `Sym<f64>`, at scale `1e9·ε`.
fn sym_arc_loft() -> Body<Sym<f64>> {
    let s = 1.0e9 * Tol::witness().get().eps;
    loft_body::<Sym<f64>>(
        &sweep::test_support::lift_sections::<Sym<f64>>(&[
            arc_section(s),
            arc_section(s),
            arc_section(s),
        ]),
        &sweep::test_support::lift_places::<Sym<f64>>(
            &stacked(&[0.0, 1.0, 2.0], s),
            geom_core::Tol::witness(),
        ),
        2,
        Tol::witness(),
    )
    .expect("the arc loft lofts at Sym")
    .body
}

/// The thin strip at `Sym<f64>`.
fn sym_thin_strip() -> Body<Sym<f64>> {
    let eps = Tol::witness().get().eps;
    loft_body::<Sym<f64>>(
        &sweep::test_support::lift_sections::<Sym<f64>>(&[
            strip_section(1.0e12 * eps, 1.0e9 * eps, false),
            strip_section(1.0e12 * eps, 1.0e9 * eps, false),
        ]),
        &sweep::test_support::lift_places::<Sym<f64>>(
            &stacked(&[0.0, 1.0], 1.0e12 * eps),
            geom_core::Tol::witness(),
        ),
        1,
        Tol::witness(),
    )
    .expect("the strip lofts at Sym")
    .body
}

/// `retried` is not a column: the session block opens `with_session`,
/// which installs no retry ladder, so it would read zero by construction.
fn counts_line(name: &str, door: &str, refused: bool, c: &SymCounts, shapes: usize) -> String {
    format!(
        "{name} {door} {} decisions={} sym={} rref={} rcon={} num={} disp={} chk={}/{} frozen={} shapes={}",
        if refused { "REFUSED" } else { "OK" },
        c.decisions(),
        c.symbolic,
        c.registrations_refused,
        c.registrations_contradicted,
        c.numeric,
        c.theorems_disputed,
        c.checks_expanded,
        c.checks_expanded + c.checks_frozen,
        c.frozen,
        shapes,
    )
}

/// The session block: for each body, the receipt the session collected
/// and the shape report's length, for both reporting doors.
///
/// **Why these two are a pin at all.** Every other channel this suite
/// reads is a value the walk hands back, so a parallel walk can compose
/// it. These are not: `count_decision` mutates the INSTALLED session in
/// place and `sym::report::record` pushes onto a thread-local, so a face
/// decided past the point the serial walk stopped inflates both with no
/// way to take it back. The walk's answer is to stay on the caller's
/// thread whenever a session is installed and to short-circuit there
/// exactly as `main` did (`topo::props`' `decide_faces`), and these
/// lines are `main`'s numbers.
fn session_block(receipts: &[Receipt]) -> String {
    receipts
        .iter()
        .map(|r| counts_line(r.name, r.door, r.refused, &r.counts, r.shapes))
        .collect::<Vec<_>>()
        .join("\n")
}

/// One body read through one door inside a session.
struct Receipt {
    name: &'static str,
    door: &'static str,
    refused: bool,
    counts: SymCounts,
    shapes: usize,
}

/// Each body through both reporting doors, each in a session of its
/// own under `budget` that BUILDS the body before reading it.
///
/// **Built inside, receipted after.** A node minted before the session
/// is not in its table, so the walk can only make it an unknown, and a
/// contradiction check over the body's arithmetic would be reading
/// unknowns (`SymCounts::checks_expanded`). So the session is installed
/// first, and the receipt is the door's alone: the counts the build
/// left ([`session_counts`](geom_core::sym::session_counts)) are taken
/// off the end, and the shape report opens after the build.
fn receipts(bodies: &[(&'static str, SymBody)], budget: SymBudget) -> Vec<Receipt> {
    let mut out = Vec::new();
    for &(name, build) in bodies {
        for door in ["mass_properties", "validate_geometric"] {
            let ((refused, built), end) = with_session(budget, || {
                let body = build();
                // The measurement door's body is finished before the build's
                // counts are taken, so its receipt is the door's own, off the
                // end. The validation door reads the body as built: a gate run
                // first would freeze the forms its own receipt counts.
                let finished = (door == "mass_properties").then(|| {
                    topo::test_support::finished("the built body", body.clone(), Tol::witness())
                });
                let built = geom_core::sym::session_counts().expect("a session is installed");
                geom_core::sym::report::start_shape_report();
                let refused = match &finished {
                    Some(finished) => topo::mass_properties(
                        finished,
                        Tol::witness(),
                        topo::test_support::pinned_resolution(Tol::witness()),
                    )
                    .is_err(),
                    None => topo::validate_geometric(&body, Tol::witness()).is_err(),
                };
                (refused, built)
            });
            let shapes = geom_core::sym::report::take_shape_report().len();
            out.push(Receipt {
                name,
                door,
                refused,
                counts: since(end, built),
                shapes,
            });
        }
    }
    out
}

/// The counts `end` took on after `start`, column by column — every
/// column of a session outside a drive counts events, `frozen` the
/// plain walk's freezes included.
fn since(end: SymCounts, start: SymCounts) -> SymCounts {
    let SymCounts {
        symbolic,
        registrations_refused,
        registrations_contradicted,
        theorems_disputed,
        checks_expanded,
        checks_frozen,
        numeric,
        retried,
        frozen,
    } = end;
    SymCounts {
        symbolic: symbolic - start.symbolic,
        registrations_refused: registrations_refused - start.registrations_refused,
        registrations_contradicted: registrations_contradicted - start.registrations_contradicted,
        theorems_disputed: theorems_disputed - start.theorems_disputed,
        checks_expanded: checks_expanded - start.checks_expanded,
        checks_frozen: checks_frozen - start.checks_frozen,
        numeric: numeric - start.numeric,
        retried: retried - start.retried,
        frozen: frozen - start.frozen,
    }
}

/// The whole digest, both blocks.
fn digest(f64_block: &str, receipts: &[Receipt]) -> String {
    format!(
        "# f64\n{f64_block}\n# sym-session\n{}",
        session_block(receipts)
    )
}

/// The committed digest for each ε row the matrix gates. Cut on the
/// MERGE BASE (see the module docs); an ε with no entry prints its
/// block and fails, which is how a new row gets cut.
///
/// **What a golden holds.** The `# f64` block is one line per roster
/// body ([`line`]): the certified volume, area and their pads as bits,
/// or the typed refusal naming its face, then the verdict and
/// escalation channels' counts and hashes. The `# sym-session` block is
/// one line per `Sym` body and door ([`counts_line`]), each the DOOR's
/// receipt over a body the same session built ([`receipts`]): by
/// outcome (`sym`, `rref`, `rcon`, `num`), its disputes (`disp`), its
/// contradiction checks as the ones that saw a whole form over all of
/// them (`chk`), the nodes its plain walks froze (`frozen`) and the
/// shape report's length. Every receipt also clears
/// [`dispute_coverage`] before the table is read.
///
/// **When a lane may re-cut here instead, and what licenses it.** The
/// merge-base rule exists to stop a branch recording its own
/// regression as the baseline — not to make a pinned number a
/// contract. A branch re-cuts on itself exactly when its own change is
/// what moved the table AND the new reading is the right answer, with
/// the cause named at the cut: H5's ratified ruling 2 (PR 2701) (a
/// certified bound that gets tighter re-baselines like any other move)
/// and `memories/output-stability-as-justification.md`. A move the
/// branch cannot explain is a finding, and the table stays where it is.
/// A move in the wrong direction that is the named consequence of a
/// change that makes the code right re-cuts, with the cause named at the
/// cut and an item filed for the looseness (the INLANE orchestrator's
/// ruling on C6 S3, PR 5290: one 90° rational sub-arc in place of two
/// 45° ones, filed as
/// `work/quad/a-ninety-degree-rational-span-encloses-looser-than-two-forty-five-degree-spans.md`).
///
/// **Each re-cut names its cause in its PR body**, and the history of
/// what moved is `git log -p -- crates/sweep/tests/thread-count-digest/`.
///
/// **Pinned at what main measures** (2026-10-30), not explained: the
/// 1e-12 table's `sym_arc_loft validate_geometric` reads `chk=97/104`
/// where it read `99/104`, every other column holding, since INTENT
/// stage 3 G (PR 5954, bisected at 1e-12). Which two checks stopped
/// seeing a whole form is owed on
/// `work/green/g-moves-four-main-reds-messages.md`.
fn expected(eps: f64) -> Option<&'static str> {
    match eps {
        1e-6 => Some(include_str!("thread-count-digest/eps-1e-6.txt")),
        1e-9 => Some(include_str!("thread-count-digest/eps-1e-9.txt")),
        1e-12 => Some(include_str!("thread-count-digest/eps-1e-12.txt")),
        _ => None,
    }
}

fn check_against_golden(threads: usize) {
    let eps = Tol::witness().get().eps;
    let (f64_block, receipts) =
        on_pool(threads, || (f64_block(), receipts(&sym_bodies(), budget())));
    for r in &receipts {
        if let Err(why) = dispute_coverage(&r.counts) {
            panic!("{} {}: {why}: {:?}", r.name, r.door, r.counts);
        }
    }
    let got = digest(&f64_block, &receipts);
    let Some(want) = expected(eps) else {
        panic!("no committed digest for eps={eps:e}; this run produced\n{got}");
    };
    assert_eq!(
        got.trim(),
        want.trim(),
        "at {threads} thread(s) the face walk's readings or its recorded channels differ \
         from what the SERIAL walk recorded at the merge base (eps={eps:e}). This is not a \
         baseline to preserve: if the new behaviour is right, re-cut the table on the merge \
         base and say in the PR what moved and why."
    );
}

#[test]
fn the_walk_matches_the_serial_golden_at_four_threads() {
    check_against_golden(4);
}

/// Non-vacuity, and it is about the GOLDEN as much as about the code:
/// a table of empty channels would compare equal to a re-cut table of
/// empty channels forever. Two reads, one cheap body each way — the
/// committed digest must carry verdicts at all, and a live walk must
/// record the props lanes' OWN predicates rather than someone else's.
#[test]
fn the_roster_records_the_props_lanes_own_verdicts() {
    let eps = Tol::witness().get().eps;
    let want = expected(eps).expect("a committed digest for this eps");
    let recorded: usize = want
        .lines()
        .filter_map(|l| l.split("verdicts n=").nth(1))
        .filter_map(|t| t.split_whitespace().next())
        .filter_map(|n| n.parse::<usize>().ok())
        .sum();
    assert!(
        recorded > 0,
        "the committed digest records no verdict on any body — it is a table of empty channels"
    );
    let prism =
        topo::test_support::finished("the loft prism", loft_prism(Tol::witness()), Tol::witness());
    let bracket = Bracket::open();
    let _ = topo::mass_properties(
        &prism,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    );
    let log = bracket.finish();
    let quad = log
        .verdicts
        .iter()
        .filter(|v| v.predicate.starts_with("props_quad"))
        .count();
    assert!(
        quad > 0,
        "the described-spline body recorded no props_quad verdict — the roster no longer \
         reaches the quadrature lane, so the golden above pins a walk that does nothing"
    );
}

// ---------------------------------------------------------------- //
// Dispute coverage: what a `disp=0` in the session block speaks for.  //
// ---------------------------------------------------------------- //

/// **The least share of a receipt's contradiction checks that must
/// have seen a whole form**, as `(numerator, denominator)`.
///
/// Measured at all three ε with the bodies built inside the session:
/// 103/110 (0.936) on the arc loft's `validate_geometric` and 104/116
/// (0.897) on the strip's — the lowest share, ten points over this
/// floor, eleven checks of its 116. Built outside the session the same
/// arc door reads 14/110 (0.127).
const COVERAGE_FLOOR: (u64, u64) = (4, 5);

/// **The fewest checks the floor is read over.** A share of a handful
/// is noise — on the 2-check `mass_properties` receipt one freeze is
/// half of it — so below this the guard asks only that a check ran,
/// and the golden table pins those receipts' `chk` exactly.
const COVERAGE_MIN_CHECKS: u64 = 20;

/// **The guard on `disp=0`.** A contradiction check over a form it
/// cannot see whole cannot dispute (`SymCounts::checks_expanded`), so a
/// receipt's zero disputes are evidence only for the checks that saw
/// their form. The guard is a floor on that share, not on a count: it
/// holds when the walk takes more or fewer decisions for reasons of its
/// own, and fails when the checks it does take go blind. A receipt with
/// no check at all fails too — its `disp=0` is about nothing.
fn dispute_coverage(c: &SymCounts) -> Result<(), String> {
    let checks = c.checks_expanded + c.checks_frozen;
    let (n, d) = COVERAGE_FLOOR;
    if checks == 0 {
        return Err("no contradiction check ran, so disp=0 speaks for nothing".to_owned());
    }
    if checks >= COVERAGE_MIN_CHECKS && c.checks_expanded * d < checks * n {
        return Err(format!(
            "{} of {checks} contradiction checks saw a whole form, under the floor of \
             {n}/{d}: disp={} speaks for too few of them",
            c.checks_expanded, c.theorems_disputed
        ));
    }
    Ok(())
}

/// **The guard trips where the checks go blind, and `disp=0` alone
/// does not.** The arc loft built BEFORE the session is installed, then
/// read through both doors inside one: every node of the body is
/// unknown to the session, the checks of `validate_geometric` read
/// unknowns, and the guard fails there while both doors still read no
/// dispute. Built inside, the same door clears the guard in the golden
/// row.
#[test]
fn bodies_built_before_the_session_trip_the_guard_while_disp_still_reads_zero() {
    let body = sym_arc_loft();
    // The measurement door's body is finished outside the session too, so
    // its receipt is the door's own; the validation door reads it as built.
    let finished = topo::test_support::finished("the arc loft", body.clone(), Tol::witness());
    let all: Vec<_> = ["mass_properties", "validate_geometric"]
        .into_iter()
        .map(|door| {
            let (_, c) = with_session(budget(), || match door {
                "mass_properties" => topo::mass_properties(
                    &finished,
                    Tol::witness(),
                    topo::test_support::pinned_resolution(Tol::witness()),
                )
                .is_err(),
                _ => topo::validate_geometric(&body, Tol::witness()).is_err(),
            });
            (door, c)
        })
        .collect();
    for (door, c) in &all {
        println!("sym_arc_loft {door} built outside: {c:?}");
        assert_eq!(
            c.theorems_disputed, 0,
            "{door}: the blind run disputes nothing, so disp=0 cannot be the guard: {c:?}"
        );
    }
    let tripped: Vec<_> = all
        .iter()
        .filter(|(_, c)| dispute_coverage(c).is_err())
        .map(|(door, _)| *door)
        .collect();
    assert_eq!(
        tripped,
        ["validate_geometric"],
        "the doors whose checks a body built outside the session blinds: {all:?}"
    );
}

/// The guard's three branches at their edges: no check, a small
/// receipt the floor is not read over, and a large one either side of
/// the floor.
#[test]
fn the_guard_reads_its_floor_over_enough_checks() {
    let at = |expanded, frozen| SymCounts {
        checks_expanded: expanded,
        checks_frozen: frozen,
        ..SymCounts::default()
    };
    for (expanded, frozen, holds) in [(0, 0, false), (1, 1, true), (16, 4, true), (15, 5, false)] {
        assert_eq!(
            dispute_coverage(&at(expanded, frozen)).is_ok(),
            holds,
            "{expanded} of {} checks on whole forms",
            expanded + frozen
        );
    }
}
