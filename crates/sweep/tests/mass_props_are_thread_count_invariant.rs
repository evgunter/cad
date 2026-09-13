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
//! the walk is serial, and both widths are read against it. Same
//! instrument as `reporting_door_bit_digest`, and cut the same way: a
//! row cut on this branch would record what this branch does, which is
//! the thing under test.
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
//! **One row is `probe`-gated and it is rostered as EXECUTED**
//! (`scripts/gates/probe-suite-census.sh`'s `RUN_FLOOR`, which
//! `k_probe_sweep.sh` derives its default-selection loop from). The
//! sample population is the thing k-lint counts, so a row asserting it
//! does not move with the thread count is worth nothing if it only
//! compiles: an inert pin reports the same green as one that ran. The
//! other rows are ungated and run on every merge as usual.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::{
    arc_section, bulged_extrusion, channels, on_pool, quad, quintic_prism, stacked, strip_section,
    tilted_cut_upper,
};
use geom_core::k_stats::Bracket;
use geom_core::sym::{SymBudget, SymCounts, with_session};
use geom_core::{Sym, Tol};
use sweep::loft_body;
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

/// The thin curved strip, two stations high — the body whose volume
/// enclosure straddles zero after the schedule has run out, so its
/// target-level reading refuses and names a face.
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

/// `loft_prism`, rebuilt from the corpus document's own sections
/// (`editor-core/tests/corpus/loft_prism.rs`: squares at z = 0 and
/// z = 2, a trapezoid at z = 1, v-degree 2). Polyline sections, so the
/// walls are described splines on the quadrature lane — and it is the
/// body the finding measured (`work/perf/mass-properties-are-serial-per-face.md`:
/// 157 ms). A corpus DOCUMENT cannot come here: `editor-core` sits
/// above this crate.
fn loft_prism() -> Body<f64> {
    let sq = quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]);
    let d = 0.375;
    let tr = quad([(-1.0 - d, -1.0), (1.0 + d, -1.0), (1.0, 1.0), (-1.0, 1.0)]);
    loft_body::<f64>(
        &[sq.clone(), tr, sq],
        &stacked(&[0.0, 1.0, 2.0], 1.0),
        2,
        Tol::witness(),
    )
    .expect("the prism lofts")
    .body
}

/// The roster: the reporting-door digest's three lanes that this walk
/// can reach from here (`quintic_prism` — composite rounds;
/// `tilted_cut_upper` — the cylinder chart's Green form, which no loft
/// verb produces; `bulged_extrusion` — the same chart from the extrude
/// door), `loft_prism` (described splines, the finding's own body), the
/// arc lofts scaled against the run's own ε so no row goes vacuous at
/// one point of the matrix, and the thin strip whose target reading
/// refuses.
fn roster() -> Vec<(String, Body<f64>)> {
    let eps = Tol::witness().get().eps;
    let mut out: Vec<(String, Body<f64>)> = vec![
        ("quintic_prism".to_string(), quintic_prism()),
        ("tilted_cut_upper".to_string(), tilted_cut_upper()),
        ("bulged_extrusion".to_string(), bulged_extrusion()),
        ("loft_prism".to_string(), loft_prism()),
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
    let bracket = Bracket::open();
    let read = match topo::mass_properties(body, Tol::witness()) {
        Ok(m) => format!(
            "v={:016x} a={:016x} vpad={:016x} apad={:016x}",
            m.volume.to_bits(),
            m.surface_area.to_bits(),
            m.volume_pad.to_bits(),
            m.area_pad.to_bits(),
        ),
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

/// The same two bodies at `Sym<f64>` — one that answers and one whose
/// target reading refuses.
fn sym_bodies() -> Vec<(String, Body<Sym<f64>>)> {
    let eps = Tol::witness().get().eps;
    let arc = |s: f64| {
        loft_body::<Sym<f64>>(
            &[arc_section(s), arc_section(s), arc_section(s)],
            &stacked(&[0.0, 1.0, 2.0], s),
            2,
            Tol::witness(),
        )
        .expect("the arc loft lofts at Sym")
        .body
    };
    let strip = loft_body::<Sym<f64>>(
        &[
            strip_section(1.0e12 * eps, 1.0e9 * eps, false),
            strip_section(1.0e12 * eps, 1.0e9 * eps, false),
        ],
        &stacked(&[0.0, 1.0], 1.0e12 * eps),
        1,
        Tol::witness(),
    )
    .expect("the strip lofts at Sym")
    .body;
    vec![
        ("sym_arc_loft".to_string(), arc(1.0e9 * eps)),
        ("sym_thin_strip".to_string(), strip),
    ]
}

fn counts_line(name: &str, door: &str, refused: bool, c: &SymCounts, shapes: usize) -> String {
    format!(
        "{name} {door} {} decisions={} sz={} sg={} reg={} rref={} rcon={} num={} frozen={} shapes={}",
        if refused { "REFUSED" } else { "OK" },
        c.decisions(),
        c.symbolic_zero,
        c.sign_gated,
        c.registered,
        c.registrations_refused,
        c.registrations_contradicted,
        c.numeric,
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
fn session_block() -> String {
    let mut out = Vec::new();
    for (name, body) in sym_bodies() {
        for door in ["mass_properties", "validate_geometric"] {
            geom_core::sym::report::start_shape_report();
            let (refused, counts) = with_session(budget(), || {
                if door == "mass_properties" {
                    topo::mass_properties(&body, Tol::witness()).is_err()
                } else {
                    topo::validate_geometric(&body, Tol::witness()).is_err()
                }
            });
            let shapes = geom_core::sym::report::take_shape_report().len();
            out.push(counts_line(&name, door, refused, &counts, shapes));
        }
    }
    out.join("\n")
}

/// The whole digest, both blocks.
fn digest() -> String {
    format!("# f64\n{}\n# sym-session\n{}", f64_block(), session_block())
}

/// The committed digest for each ε row the matrix gates. Cut on the
/// MERGE BASE (see the module docs); an ε with no entry prints its
/// block and fails, which is how a new row gets cut.
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
    let got = on_pool(threads, digest);
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
fn the_walk_matches_the_serial_golden_at_one_thread() {
    check_against_golden(1);
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
    let bracket = Bracket::open();
    let _ = topo::mass_properties(&loft_prism(), Tol::witness());
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

/// **The `probe` sample population does not shrink with the thread
/// count** — what k-lint counts, read at 1 and 4 threads. The golden
/// above cannot carry this: the sink only exists in a `probe` build.
#[cfg(feature = "probe")]
#[test]
fn the_sample_population_is_identical_at_one_and_four_threads() {
    use geom_core::Probe;
    use geom_core::k_stats::{start_recording, take_samples};

    let eps = Tol::witness().get().eps;
    let body: Body<Probe> = loft_body::<Probe>(
        &[arc_section(1.0e9 * eps), arc_section(1.0e9 * eps)],
        &stacked(&[0.0, 1.0], 1.0e9 * eps),
        1,
        Tol::witness(),
    )
    .expect("the probe arc loft lofts")
    .body;
    let population = |threads: usize| {
        on_pool(threads, || {
            start_recording();
            let _ = topo::mass_properties(&body, Tol::witness());
            take_samples()
                .iter()
                .map(|s| (s.predicate, s.margin.to_bits(), s.outcome.token()))
                .collect::<Vec<_>>()
        })
    };
    let one = population(1);
    let four = population(4);
    assert!(
        !one.is_empty(),
        "the probe lane recorded no sample — the comparison below is vacuous"
    );
    assert_eq!(
        one, four,
        "the sample population moved with the thread count"
    );
}
