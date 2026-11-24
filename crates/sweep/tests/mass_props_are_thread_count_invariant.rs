//! **The face walk answers the same thing at any thread count as the
//! serial walk does.**
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
//! **The baseline is the serial walk, run beside the pooled ones, not
//! the other width.** Two widths of the SAME code agree whenever the
//! loss is width-independent — a walk that dropped every worker's
//! recording identically at one and four threads would pass a
//! t1-vs-t4 row while asserting nothing. So the reference reading is
//! taken with the shape report installed, under which `topo::props`'
//! `decide_faces` takes its serial arm: every face decided on the
//! caller's thread, in arena order, into the caller's own frame and
//! sink, with no worker, no detached recording and no splice. The one-
//! and four-thread walks are read against that.
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
//! and `k_stats`' module docs say the same). So these rows hold that the
//! escalation log the pooled walks record is the serial walk's — which
//! is nothing — and they would catch a composition that invented
//! escalations or moved them; they cannot demonstrate a non-empty
//! escalation log surviving a worker, because no body in this tree
//! produces one. The thin strip is here as the body whose target-level
//! reading REFUSES, which is the escalation PATH through the walk; it
//! is not a body that escalates on the channel.
//!
//! **Its `probe` row lives in `thread_count_probe_populations`**: the
//! sample population is the thing k-lint counts, and only the probe
//! sweep runs a `probe` build, so the row sits in a file the sweep can
//! select without re-running the rows here.
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
    .into_body()
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
    .into_body()
}

/// The roster: the reporting-door digest's three lanes that this walk
/// can reach from here (`quintic_prism` — composite rounds;
/// `tilted_cut_upper` — the cylinder chart's Green form, which no loft
/// verb produces; `bulged_extrusion` — the same chart from the extrude
/// door), `loft_prism` (described splines, the finding's own body), the
/// arc lofts scaled against the run's own ε so no row goes vacuous at
/// one point of the matrix, and the thin strip.
fn roster() -> Vec<(String, Body<f64>, topo::Settle<f64>)> {
    let unit = unit();
    let pinned = topo::test_support::pinned_resolution(Tol::witness());
    let mut out: Vec<(String, Body<f64>, topo::Settle<f64>)> = vec![
        ("quintic_prism".to_string(), quintic_prism(), pinned),
        ("tilted_cut_upper".to_string(), tilted_cut_upper(), pinned),
        ("bulged_extrusion".to_string(), bulged_extrusion(), pinned),
        // `loft_prism` is in this roster because it is the body the
        // finding measured
        // (`work/perf/mass-properties-are-serial-per-face.md`: 157 ms)
        // — polyline sections, so its walls are described splines on
        // the quadrature lane, which is the lane the per-face serialism
        // is about. A corpus DOCUMENT cannot stand in for it here:
        // `editor-core` sits above this crate, so the kernel-side
        // fixture is the only reachable spelling.
        ("loft_prism".to_string(), loft_prism(Tol::witness()), pinned),
    ];
    out.extend([1.0e11, 1.0e9].iter().map(|k| {
        (
            format!("arc_loft_{k:e}eps"),
            arc_loft(k * unit),
            resolution(),
        )
    }));
    out.push((
        "thin_strip".to_string(),
        strip_loft(1.0e12 * unit, 1.0e9 * unit),
        resolution(),
    ));
    out
}

/// **The roster's unit** ([`crate::common::box_unit`]): ε, or less
/// where the strip at `1e12` units, reaching `1e12` (its half width and
/// its height), would pass the session box's own extent: at ε = 1e-6
/// the unit is `1e-9`, and the bodies there are the 1e-9 row's. The
/// extent and not the far end (`common::FAR_END`) because the 1e-9
/// row's strip already reaches the box's edge. The scaled bodies are
/// measured at `1024` units, the fixed ones at the pinned `1024·ε`. The
/// names keep `eps` for the unit.
fn unit() -> f64 {
    let eps = Tol::witness().get().eps;
    crate::common::box_unit(
        1.0e12,
        geom_core::tolerance::SESSION_BOX_EXTENT,
        eps / 1024.0,
    )
}

fn resolution<T: geom_core::Real>() -> topo::Settle<T> {
    crate::common::unit_resolution(unit())
}

/// One body's line: the reading (bits, or the typed refusal — which is
/// where the refusing FACE is named) and both recorded channels.
fn line(name: &str, body: &Body<f64>, resolution: topo::Settle<f64>) -> String {
    // Finished outside the bracket, so the line is the measurement's own
    // readings. Every body here finishes at every ε, and one that did not
    // would panic here rather than reach the pinned line.
    let at_rest = topo::test_support::finished(name, topo::Body::clone(body), Tol::witness());
    let bracket = Bracket::open();
    let read = match topo::mass_properties(&at_rest, Tol::witness(), resolution) {
        Ok(m) => format!(
            "v={:016x} a={:016x} vpad={:016x} apad={:016x}",
            m.enclosure.volume.to_bits(),
            m.enclosure.surface_area.to_bits(),
            m.enclosure.volume_pad.to_bits(),
            m.enclosure.area_pad.to_bits(),
        ),
        // The refusal's sentence leaves the face's key out; the walk's
        // choice of WHICH face refused is part of what is pinned.
        Err(e @ topo::MassPropsError::Face { face, .. }) => format!(
            "REFUSED at {face:?}: {e_at}",
            e_at = e.render(geom_brep::recourse::Reading::AtRest)
        ),
        Err(e) => format!(
            "REFUSED {e_at}",
            e_at = e.render(geom_brep::recourse::Reading::AtRest)
        ),
    };
    format!("{name} {read} | {}", channels(&bracket.finish()))
}

/// The f64 block of the digest: every roster body, in a fixed order
/// (the block is compared whole, so the order is part of the pin).
fn f64_block() -> String {
    roster()
        .iter()
        .map(|(name, body, resolution)| line(name, body, *resolution))
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

/// The arc loft at `Sym<f64>`, at scale `1e9` [`unit`]s.
fn sym_arc_loft() -> Body<Sym<f64>> {
    let s = 1.0e9 * unit();
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
    .into_body()
}

/// The thin strip at `Sym<f64>`.
fn sym_thin_strip() -> Body<Sym<f64>> {
    let unit = unit();
    loft_body::<Sym<f64>>(
        &sweep::test_support::lift_sections::<Sym<f64>>(&[
            strip_section(1.0e12 * unit, 1.0e9 * unit, false),
            strip_section(1.0e12 * unit, 1.0e9 * unit, false),
        ]),
        &sweep::test_support::lift_places::<Sym<f64>>(
            &stacked(&[0.0, 1.0], 1.0e12 * unit),
            geom_core::Tol::witness(),
        ),
        1,
        Tol::witness(),
    )
    .expect("the strip lofts at Sym")
    .body
    .into_body()
}

/// `retried` is not a column: the session block opens `with_session`,
/// which installs no retry ladder, so it would read zero by construction.
fn counts_line(name: &str, door: &str, refused: bool, c: &SymCounts, shapes: usize) -> String {
    format!(
        "{name} {door} {} decisions={} sym={} rref={} num={} frozen={} shapes={}",
        if refused { "REFUSED" } else { "OK" },
        c.decisions(),
        c.symbolic,
        c.registrations_refused,
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
/// is not in its table, so the walk can only make it an unknown. So the
/// session is installed first, and the receipt is the door's alone: the counts the build
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
                    Some(finished) => {
                        topo::mass_properties(finished, Tol::witness(), resolution()).is_err()
                    }
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
        numeric,
        retried,
        frozen,
    } = end;
    SymCounts {
        symbolic: symbolic - start.symbolic,
        registrations_refused: registrations_refused - start.registrations_refused,
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

/// **One reading of the whole roster**: the `# f64` block, one line per
/// roster body ([`line`]) — the certified volume, area and their pads
/// as bits, or the typed refusal naming its face, then the verdict and
/// escalation channels' counts and hashes — and the `# sym-session`
/// block, one line per `Sym` body and door ([`counts_line`]), each the
/// DOOR's receipt over a body the same session built ([`receipts`]).
///
/// `serial` takes the f64 block with the shape report installed, which
/// is what sends `topo::props`' face walk down its serial arm
/// (`geom_core::sym::decisions_are_thread_portable`); the session block
/// is read under an installed session, where the walk is serial at
/// every width.
fn reading(serial: bool) -> String {
    let f64_block = if serial {
        geom_core::sym::report::start_shape_report();
        let block = f64_block();
        geom_core::sym::report::take_shape_report();
        block
    } else {
        f64_block()
    };
    let receipts = receipts(&sym_bodies(), budget());
    digest(&f64_block, &receipts)
}

/// The verdicts a reading's `# f64` block records, summed over its
/// bodies.
fn verdicts_recorded(reading: &str) -> usize {
    reading
        .lines()
        .filter_map(|l| l.split("verdicts n=").nth(1))
        .filter_map(|t| t.split_whitespace().next())
        .filter_map(|n| n.parse::<usize>().ok())
        .sum()
}

/// **At one thread and at four, the face walk reads what the serial
/// walk reads**: every body's bits or refusal, both recorded channels,
/// and every session receipt. The serial reading must carry verdicts,
/// or two empty channels would compare equal.
#[test]
fn the_walk_matches_the_serial_walk_at_one_and_four_threads() {
    let eps = Tol::witness().get().eps;
    let serial = reading(true);
    assert!(
        verdicts_recorded(&serial) > 0,
        "the serial walk records no verdict on any body — the comparison below would be \
         between empty channels:\n{serial}"
    );
    for threads in [1, 4] {
        let pooled = on_pool(threads, || reading(false));
        assert_eq!(
            pooled.trim(),
            serial.trim(),
            "at {threads} thread(s) the face walk's readings or its recorded channels differ \
             from what the serial walk records (eps={eps:e})"
        );
    }
}

/// A live walk records the props lanes' OWN predicates rather than
/// someone else's: the described-spline body reaches the quadrature
/// lane, so the comparison above is over a walk that does something.
#[test]
fn the_roster_records_the_props_lanes_own_verdicts() {
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
         reaches the quadrature lane"
    );
}

/// **The contradiction instrument over the session block's bodies**
/// (`geom_core::sym::contradiction`): each `Sym` body built and read
/// through both reporting doors inside one session, and the instrument
/// asking every definite decision whether the tier's algebra or a
/// registration would have claimed `Zero` there. It finds nothing, and
/// it asked: every door's decisions include definite ones, none on a node
/// the session does not hold.
#[test]
fn the_session_blocks_bodies_contradict_nothing() {
    use geom_core::sym::contradiction;
    use geom_core::sym::revalue::record;
    for (name, build) in sym_bodies() {
        for door in ["mass_properties", "validate_geometric"] {
            let (report, _) = with_session(budget(), || {
                let ((), decisions) = record(|| {
                    let body = build();
                    let _ = if door == "mass_properties" {
                        let finished =
                            topo::test_support::finished("the built body", body, Tol::witness());
                        topo::mass_properties(&finished, Tol::witness(), resolution()).is_err()
                    } else {
                        topo::validate_geometric(&body, Tol::witness()).is_err()
                    };
                });
                contradiction::check(&decisions).expect("inside the session")
            });
            println!("{name} {door}: {report:?}");
            assert!(
                report.checked > 0 && report.absent == 0,
                "{name} {door}: the instrument must have asked something it could read: {report:?}"
            );
            assert!(
                report.contradictions.is_empty(),
                "{name} {door}: the two channels contradict: {report:?}"
            );
        }
    }
}
