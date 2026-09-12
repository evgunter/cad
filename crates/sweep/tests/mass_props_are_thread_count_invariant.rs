//! **The face walk answers the same thing at any thread count.**
//!
//! `topo::props` runs each face's flux in an indexed parallel map
//! (PERF-PLAN §2.2 idiom 1) and sums the slots in a sequential
//! arena-order fold (idiom 2), so the arithmetic is schedule-free by
//! construction. What is NOT free — and what this suite pins — is the
//! K-funnel's recording: the verdict log, the escalation log and the
//! `probe` sample population are thread-local, so a face decided on a
//! worker records into that worker's frame and sink unless the walk
//! composes them back. These rows read the composition from outside,
//! through the public doors, at an explicit 1-thread pool (one face at a
//! time in arena order — the serial walk's own decision order) and an
//! explicit 4-thread pool.
//!
//! **Why a pool and not `RAYON_NUM_THREADS`.** The environment variable
//! configures the global pool once per process, so it cannot vary
//! between two rows of one binary; a `ThreadPool` can, and
//! `ThreadPool::install` runs the closure on that pool's worker, so
//! everything below — the bracket, the sink, the walk — sits on one
//! thread of the pool being measured.
//!
//! The bodies are PERF-6's roster: rational-walled arc lofts, every wall
//! face a certified quadrature, plus the thin strip whose sign the
//! schedule cannot decide — the body that escalates and whose
//! target-level reading refuses.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::{arc_section, stacked, strip_section};
use geom_core::Tol;
use geom_core::k_stats::{Bracket, Recorded};
use sweep::loft_body;
use topo::{Body, MassProperties};

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

/// The roster, scaled against the run's own ε so no row goes vacuous at
/// one point of the eps matrix: two arc lofts spanning the schedule's
/// reach (PERF-6's digest roster) and the strip that cannot decide.
fn roster() -> Vec<(String, Body<f64>)> {
    let eps = Tol::witness().get().eps;
    let mut out: Vec<(String, Body<f64>)> = [1.0e11, 1.0e9]
        .iter()
        .map(|k| (format!("arc loft @ {k:e}·eps"), arc_loft(k * eps)))
        .collect();
    out.push((
        "thin strip @ 1e12·eps".to_string(),
        strip_loft(1.0e12 * eps, 1.0e9 * eps),
    ));
    out
}

/// Runs `run` on a pool of exactly `threads` threads, inside a bracket
/// opened on the pool thread that runs it, and answers its value beside
/// everything the funnel recorded.
fn on_pool<R: Send>(threads: usize, run: impl Fn() -> R + Send + Sync) -> (R, Recorded) {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("the pool builds");
    pool.install(|| {
        let bracket = Bracket::open();
        let out = run();
        (out, bracket.finish())
    })
}

/// The four numbers a mass-property reading carries, as bits — the
/// comparison "bit-identical" names, and the one a tolerance would
/// quietly stop making.
fn bits(m: &MassProperties<f64>) -> [u64; 4] {
    [
        m.volume.to_bits(),
        m.surface_area.to_bits(),
        m.volume_pad.to_bits(),
        m.area_pad.to_bits(),
    ]
}

/// A reading reduced to something comparable: the bits on success, the
/// rendered refusal on failure — which is where the refusing FACE is
/// named, so a walk that named a different face fails this row.
fn reading(body: &Body<f64>) -> Result<[u64; 4], String> {
    topo::mass_properties(body, Tol::witness())
        .map(|m| bits(&m))
        .map_err(|e| e.to_string())
}

/// How many of the recorded verdicts are the props lanes' own — the
/// non-vacuity read: a roster that recorded no `props_quad_*` decision
/// would compare two empty logs and pass while asserting nothing.
fn props_quad_verdicts(r: &Recorded) -> usize {
    r.verdicts
        .iter()
        .filter(|v| v.predicate.starts_with("props_quad"))
        .count()
}

#[test]
fn mass_properties_reads_the_same_bits_and_log_at_one_and_four_threads() {
    let mut quad_total = 0usize;
    for (name, body) in roster() {
        let (one, log_one) = on_pool(1, || reading(&body));
        let (four, log_four) = on_pool(4, || reading(&body));
        assert_eq!(one, four, "{name}: the reading moved with the thread count");
        assert_eq!(
            log_one.verdicts, log_four.verdicts,
            "{name}: the verdict log moved with the thread count"
        );
        assert_eq!(
            log_one.escalations, log_four.escalations,
            "{name}: the escalation log moved with the thread count"
        );
        quad_total += props_quad_verdicts(&log_one);
    }
    assert!(
        quad_total > 0,
        "no roster body recorded a props_quad verdict — the rows above compared empty logs"
    );
}

#[test]
fn tier_three_reads_the_same_verdict_and_log_at_one_and_four_threads() {
    let mut quad_total = 0usize;
    for (name, body) in roster() {
        let gate = |b: &Body<f64>| {
            topo::validate_geometric(b, Tol::witness())
                .map(|_| ())
                .map_err(|e| format!("{e:?}"))
        };
        let (one, log_one) = on_pool(1, || gate(&body));
        let (four, log_four) = on_pool(4, || gate(&body));
        assert_eq!(
            one, four,
            "{name}: tier 3's verdict moved with the thread count"
        );
        assert_eq!(
            log_one.verdicts, log_four.verdicts,
            "{name}: tier 3's verdict log moved with the thread count"
        );
        assert_eq!(
            log_one.escalations, log_four.escalations,
            "{name}: tier 3's escalation log moved with the thread count"
        );
        quad_total += props_quad_verdicts(&log_one);
    }
    assert!(
        quad_total > 0,
        "tier 3 recorded no props_quad verdict on any roster body — the rows above compared empty logs"
    );
}

/// The refusing body names the SAME face at either thread count, and
/// the log it refuses with is the same log.
///
/// The strip's target-level reading is refused with the first face in
/// arena order whose quadrature ran out of schedule; a walk that folded
/// the slots in any other order, or that dropped a worker's recording,
/// would name another face or carry another log.
#[test]
fn the_refusing_body_names_the_same_face_at_one_and_four_threads() {
    let eps = Tol::witness().get().eps;
    let body = strip_loft(1.0e12 * eps, 1.0e9 * eps);
    let (one, log_one) = on_pool(1, || reading(&body));
    let (four, log_four) = on_pool(4, || reading(&body));
    let refusal = one
        .clone()
        .expect_err("the thin strip's target reading refuses");
    assert!(
        refusal.contains("face"),
        "the refusal does not name a face: {refusal}"
    );
    assert_eq!(one, four, "the refusal moved with the thread count");
    assert_eq!(log_one.verdicts, log_four.verdicts);
    assert_eq!(log_one.escalations, log_four.escalations);
}

/// **The `probe` sample population does not shrink with the thread
/// count** — what k-lint counts, read at 1 and 4 threads.
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
        .0
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
