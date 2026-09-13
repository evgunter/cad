//! **The per-shell census answers the same thing at any thread count,
//! and the same thing the serial walk answered.**
//!
//! `topo::classify_shells_of` restricts the REPORTING face walk to one
//! shell's faces and sums that shell's contributions in the shell's own
//! face order. The per-face lanes are an indexed parallel map into
//! per-face slots (PERF-PLAN §2.2 idiom 1) and every combination after
//! it is sequential in that order (idiom 2), so the arithmetic is
//! schedule-free by construction — the same pair of idioms, through the
//! same `decide_faces`, as the whole-body walk
//! (`mass_props_are_thread_count_invariant`).
//!
//! What is NOT free is the K-funnel's recording: the verdict log, the
//! escalation log and the `probe` sample population are thread-local, so
//! a face decided on a worker records into that worker's frame and sink
//! unless the walk composes them back. This suite is that pin for the
//! census door.
//!
//! **The baseline is a committed golden, not the other width.** Two
//! widths of the SAME code agree whenever the loss is width-independent
//! — a walk that dropped every worker's recording identically at one
//! and four threads would pass a t1-vs-t4 row while asserting nothing.
//! So `shell-census-digest/eps-*.txt` is cut on the MERGE BASE, where
//! the census's face loop is serial, and both widths are read against
//! it. Same instrument as `mass_props_are_thread_count_invariant` and
//! `reporting_door_bit_digest`, through the shared fold in
//! `common::channels`.
//!
//! **The roster is the census's own shapes**, not the whole-body walk's:
//! a body with more than one shell whose CAVITY is on the certified
//! quadrature lane (so the map has rounds to record), a hollow body
//! whose every face is closed form (the shell door's commonest body,
//! and the one whose channels are the sign reads alone), and an
//! inside-out shell whose census REFUSES — the failure path, where the
//! serial walk stopped at a shell and the map must say the same thing.
//!
//! **One row is `probe`-gated and it is rostered as EXECUTED**
//! (`scripts/gates/probe-suite-census.sh`'s `RUN_FLOOR`): the sample
//! population is the thing k-lint counts, so a row asserting it does not
//! move with the thread count is worth nothing if it only compiles.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::cavity::{brick, rod};
use crate::common::{arc_section, channels, on_pool, stacked, strip_section};
use geom_core::k_stats::Bracket;
use geom_core::{Point2, Point3, Tol};
use sweep::loft_body;
use topo::{Body, BooleanResult, BooleanResultKind};

/// **A brick with a rod-shaped cavity strictly inside it**: one solid,
/// two shells, and the void shell is the rod's boundary reverted — a
/// cylinder wall and two discs, every one of them a CURVED face whose
/// closed form decides predicates of its own. That is what makes it
/// this roster's multi-shell recording case: a walk that mapped the
/// cavity's faces onto workers and dropped what they recorded loses
/// most of this line's verdict channel.
fn voided_rod() -> Body<f64> {
    let a = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(3.0, 3.0, 3.0));
    let b = rod(Point2::new(1.5, 1.5), 0.5, 1.0, 2.0);
    let BooleanResult::Body(bb) = topo::subtract(&a, &b, Tol::witness()).expect("the cut runs")
    else {
        panic!("a rod strictly inside the brick leaves a voided body")
    };
    assert_eq!(bb.kind, BooleanResultKind::Voided);
    bb.body
}

/// **The arc loft**: three rational-walled stations, one shell, and the
/// body this roster has that puts a SHELL's faces on the certified
/// quadrature lane and still answers. The boolean engine cannot carve a
/// cavity out of one (`CurvedEdgeUnsupported`), and no cavity this
/// corpus can carve reaches that lane — an extruded bulge's wall is
/// iso-trimmed and a rod's is too — so the quadrature arm of the
/// census's map is pinned on a one-shell body rather than on a void.
///
/// Scaled against the run's own ε so the row says the same thing at
/// every point of the matrix.
fn arc_loft() -> Body<f64> {
    let s = 1.0e9 * Tol::witness().get().eps;
    loft_body::<f64>(
        &[arc_section(s), arc_section(s), arc_section(s)],
        &stacked(&[0.0, 1.0, 2.0], s),
        2,
        Tol::witness(),
    )
    .expect("the arc loft lofts")
    .body
}

/// **The hollow box**: the shell verb's sealed body — two shells in one
/// solid, every face planar, so its census is the closed-form arm and
/// its channels are the two sign reads.
fn hollow_box() -> Body<f64> {
    topo::shell(
        &brick(Point3::new(0.0, 0.0, 0.0), Point3::new(2.0, 3.0, 4.0)),
        0.25,
        Tol::witness(),
    )
    .expect("a box thicker than twice the wall shells")
    .body
}

/// **The inside-out strip**: `common::strip_section`'s thin curved
/// strip traversed the other way, lofted two stations high at a scale
/// whose schedule is exhausted after round 0 — so its shell's volume
/// bracket straddles zero and the census refuses rather than guessing a
/// role. Scaled against the run's own ε so the row says the same thing
/// at every point of the matrix.
fn inside_out_strip() -> Body<f64> {
    let eps = Tol::witness().get().eps;
    let s = 1.0e12 * eps;
    let delta = 1.0e9 * eps;
    loft_body::<f64>(
        &[strip_section(s, delta, true), strip_section(s, delta, true)],
        &stacked(&[0.0, 1.0], s),
        1,
        Tol::witness(),
    )
    .expect("the reversed strip lofts")
    .body
}

fn roster() -> Vec<(String, Body<f64>)> {
    vec![
        ("voided_rod".to_string(), voided_rod()),
        ("hollow_box".to_string(), hollow_box()),
        ("arc_loft".to_string(), arc_loft()),
        ("inside_out_strip".to_string(), inside_out_strip()),
    ]
}

/// One body's census line: every class in the order the door answers
/// them (role and the four numbers by bits), or the typed refusal —
/// which is where the refusing SHELL is named — and both recorded
/// channels.
fn line(name: &str, body: &Body<f64>) -> String {
    let bracket = Bracket::open();
    let read = match topo::classify_shells(body, Tol::witness()) {
        Ok(classes) => classes
            .iter()
            .map(|c| {
                format!(
                    "{:?}/v={:016x}/vpad={:016x}/a={:016x}/apad={:016x}",
                    c.role,
                    c.volume.to_bits(),
                    c.volume_pad.to_bits(),
                    c.surface_area.to_bits(),
                    c.area_pad.to_bits(),
                )
            })
            .collect::<Vec<_>>()
            .join(" "),
        Err(e) => format!("REFUSED {e}"),
    };
    format!("{name} {read} | {}", channels(&bracket.finish()))
}

/// The whole digest: every roster body in a fixed order (the block is
/// compared whole, so the order is part of the pin).
fn digest() -> String {
    roster()
        .iter()
        .map(|(name, body)| line(name, body))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The committed digest for each ε row the matrix gates. Cut on the
/// MERGE BASE (see the module docs); an ε with no entry prints its
/// block and fails, which is how a new row gets cut.
fn expected(eps: f64) -> Option<&'static str> {
    match eps {
        1e-6 => Some(include_str!("shell-census-digest/eps-1e-6.txt")),
        1e-9 => Some(include_str!("shell-census-digest/eps-1e-9.txt")),
        1e-12 => Some(include_str!("shell-census-digest/eps-1e-12.txt")),
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
        "at {threads} thread(s) the shell census's readings or its recorded channels differ \
         from what the SERIAL census recorded at the merge base (eps={eps:e}). This is not a \
         baseline to preserve: if the new behaviour is right, re-cut the table on the merge \
         base and say in the PR what moved and why."
    );
}

#[test]
fn the_census_matches_the_serial_golden_at_one_thread() {
    check_against_golden(1);
}

#[test]
fn the_census_matches_the_serial_golden_at_four_threads() {
    check_against_golden(4);
}

/// Non-vacuity, and it is about the GOLDEN as much as about the code: a
/// table of empty channels would compare equal to a re-cut table of
/// empty channels forever. The committed digest must carry verdicts at
/// all, it must carry the census's OWN sign read, and a live census must
/// reach the quadrature lane — otherwise the rows above pin a walk with
/// nothing in its map.
#[test]
fn the_roster_records_the_censuss_own_verdicts() {
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
    let _ = topo::classify_shells(&arc_loft(), Tol::witness());
    let _ = topo::classify_shells(&voided_rod(), Tol::witness());
    let log = bracket.finish();
    let quad = log
        .verdicts
        .iter()
        .filter(|v| v.predicate.starts_with("props_quad"))
        .count();
    let sign = log
        .verdicts
        .iter()
        .filter(|v| v.predicate == "chk_shell_volume_sign")
        .count();
    assert!(
        quad > 0,
        "the arc loft's census recorded no props_quad verdict — no roster shell reaches \
         the quadrature lane any more, so the golden above pins a map with no rounds in it"
    );
    assert!(
        sign > 0,
        "the census recorded no chk_shell_volume_sign verdict — the door's own sign read is \
         not in the channels the golden compares"
    );
}

/// **The `probe` sample population does not shrink with the thread
/// count** — what k-lint counts, read at 1 and 4 threads over the
/// census door. The golden above cannot carry this: the sink only
/// exists in a `probe` build.
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
            let _ = topo::classify_shells(&body, Tol::witness());
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
