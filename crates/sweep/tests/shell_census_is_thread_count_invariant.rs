//! **The per-shell census answers the same thing at any thread count,
//! and the same thing the serial walk answered.**
//!
//! `topo::classify_shells_of` restricts the REPORTING face walk to one
//! shell's faces and sums that shell's contributions in the shell's own
//! face order. That walk is SERIAL, and deliberately so: every shell
//! the census meets is below the per-face map's break-even
//! (`work/perf/parallel-map-costs-a-fixed-price-on-a-cheap-body.md`
//! carries the numbers, and `topo::props`' `decide_faces_serially`
//! states the reason at the door).
//!
//! **So what do these rows pin, on a walk that is not parallel?** That
//! the census's readings and its RECORDED CHANNELS do not depend on the
//! pool at all. The verdict log, the escalation log and the `probe`
//! sample population are thread-local, so any face decided off the
//! caller's thread records into some other frame and sink: these rows
//! are what turns red the day this walk grows a map without a splice,
//! and they are also what the `decide_faces_serially`/`decide_faces`
//! choice is checked against if the census's grain is ever revisited —
//! the map's answer would have to be these bytes.
//!
//! **The baseline is a committed golden, not the other width.** Two
//! widths of the SAME code agree whenever the loss is width-independent
//! — a walk that dropped every worker's recording identically at one
//! and four threads would pass a t1-vs-t4 row while asserting nothing.
//! So `shell-census-digest/eps-*.txt` is cut on the MERGE BASE and both
//! widths are read against it. Same instrument as
//! `mass_props_are_thread_count_invariant` and
//! `reporting_door_bit_digest`, through the shared fold in
//! `common::channels`.
//!
//! **The roster, and what it is not.** Four bodies, all authored in
//! this file — none is a corpus document, because `editor-core` sits
//! above this crate:
//!
//! * `voided_rod` — a brick with a rod-shaped cavity: two shells, and
//!   the cavity's faces are curved, which is what makes it this
//!   roster's richest verdict channel. Its cavity is NOT on the
//!   quadrature lane, and no cavity this corpus can carve is (the
//!   fixture doc says why);
//! * `hollow_box` — the shell verb's sealed box: two shells, every face
//!   planar, the shell door's commonest body and the one whose channels
//!   are the two sign reads alone;
//! * `arc_loft` — one shell whose faces ARE on the certified quadrature
//!   lane and answer, which is the quadrature arm of this pin;
//! * `inside_out_strip` — the reversed thin strip, whose census
//!   REFUSES: the failure path, where the walk stops at a shell.
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
/// this roster's multi-shell recording case: a walk that decided the
/// cavity's faces anywhere but the caller's thread would lose most of
/// this line's verdict channel.
///
/// **Its cavity answers in CLOSED FORM, not on the quadrature lane**,
/// and no cavity this corpus can carve does: this wall is iso-trimmed,
/// an extruded bulge's is too, and the boolean engine refuses a lofted
/// operand outright (`CurvedEdgeUnsupported`). `arc_loft` below is the
/// quadrature arm instead, on one shell.
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
/// quadrature lane and still answers — the quadrature arm of this pin,
/// on one shell for [`voided_rod`]'s reason.
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
///
/// **When a lane may re-cut here instead, and what licenses it.** The
/// merge-base rule exists to stop a branch recording its own
/// regression as the baseline — not to make a pinned number a
/// contract. A branch re-cuts on itself exactly when its own change is
/// what moved the table AND the new reading is the right answer, with
/// the cause named at the cut: `work/scalar/H5.md` ruling 2 (a
/// certified bound that gets tighter re-baselines like any other move)
/// and `memories/output-stability-as-justification.md`. Anything else
/// — a move the branch cannot explain, or one in the wrong direction —
/// is a finding, and the table stays where it is.
///
/// **Re-cut at all three ε when the C9 ring became a newtype over
/// `interval-transcendentals`' `DInterval`.** That is the other repair
/// the assertion below names: the ring padded one representable step
/// outward on every operation and the backend pads only where the
/// operation is inexact, so `arc_loft`'s outer pads shrank and none
/// grew; the two closed-form bodies and the refusing strip are
/// unmoved. Every verdict hash in the block is unchanged — nothing
/// certified that refused, or refused that certified — and the pads
/// are the whole of what moved, downward.
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
         the quadrature lane any more, so the golden above pins a census with no rounds \
         in it"
    );
    assert!(
        sign > 0,
        "the census recorded no chk_shell_volume_sign verdict — the door's own sign read is \
         not in the channels the golden compares"
    );
}

/// **`voided_rod`'s verdicts as a SORTED multiset** — the row the
/// golden above cannot be. `common::channels` hashes the verdict
/// stream in DECISION order, so a verdict whose sign changed and a
/// verdict that merely moved are the same kind of miss there; this
/// row pins each `(predicate, sign)` with its count, order-free, so
/// the two are told apart when the golden moves.
///
/// ONE of the twelve predicates is ANCHOR-RELATIVE by construction,
/// and its sign is a fact about cycle order rather than about the
/// body: `props_rim_side` is the sign of `lo + hi − 2·level` on
/// whichever rim the loop walk from `Cycle::first` meets FIRST
/// (`geom_brep`'s `props/curved.rs`, `linear_rim_side`'s `side`). The
/// flux compensates (`Positive ⇒ d_u_sign`, `Negative ⇒ flip`), so
/// the readings do not depend on the anchor while that sign does —
/// the void shell here is the rod REVERTED, and `Body::revert` moves
/// every loop's anchor to its source predecessor, which is why it
/// reads `Positive` on this tree and `Negative` on one whose reversal
/// kept the anchor. The other eleven are per-rim, per-meridian or
/// per-face facts and count the same whichever rim comes first.
///
/// It was two. `props_rim_dir_group` compared each rim's traversal
/// direction against that same first rim's, through a `Margin` over
/// two values that are `±1` by construction; the direction is a
/// discrete sign now and is compared as one, so that predicate
/// records nothing and the multiset below is one row shorter.
#[test]
fn voided_rods_verdicts_as_a_sorted_multiset() {
    let body = voided_rod();
    let bracket = Bracket::open();
    let _ = topo::classify_shells(&body, Tol::witness());
    let log = bracket.finish();
    let mut got: Vec<(String, usize)> = Vec::new();
    for v in &log.verdicts {
        let key = format!("{} {:?}", v.predicate, v.sign);
        match got.iter_mut().find(|(k, _)| *k == key) {
            Some((_, n)) => *n += 1,
            None => got.push((key, 1)),
        }
    }
    got.sort();
    let want: Vec<(String, usize)> = [
        ("chk_shell_volume_sign Negative", 1),
        ("chk_shell_volume_sign Positive", 1),
        ("props_circle_axis_class Positive", 4),
        ("props_du_consistent Zero", 2),
        ("props_face_extent Positive", 2),
        ("props_meridian_axial Zero", 4),
        ("props_meridian_on_surface Zero", 4),
        ("props_rim_axis_parallel Zero", 4),
        ("props_rim_center_on_axis Zero", 4),
        ("props_rim_fit Zero", 4),
        ("props_rim_level Zero", 4),
        ("props_rim_level_group Positive", 2),
        ("props_rim_side Positive", 2),
    ]
    .into_iter()
    .map(|(k, n)| (k.to_string(), n))
    .collect();
    assert_eq!(
        got, want,
        "voided_rod's verdict multiset moved: a sign changed or a predicate came or went \
         (an ORDER change alone does not reach this row — that is the golden's)"
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
