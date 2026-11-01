//! **The K-funnel survives the per-face parallel map** — the composing
//! half of `tessellate`'s D9 idiom 1, and the row that holds it.
//!
//! The funnel's frame stack and its `probe` sink are thread-locals
//! (`geom_core::k_stats`), so a lane running on a rayon worker decides
//! its predicates into a frame nobody reads: a `Bracket` around
//! `mesh::tessellate` would come back empty from a pass that decided
//! dozens of them. The lanes run under
//! `geom_core::k_stats::detached` and the arena-order fold splices each
//! recording back, which is `topo::props`' shape one crate down.
//!
//! What this row asserts is the composition, not merely that something
//! was recorded: the verdict and escalation vectors must be **the same
//! sequence** at an explicit 1-thread pool, at an explicit 4-thread
//! pool, and on a pass that ran on the bracket's own thread — the last
//! being the reference, because a `par_iter` issued from INSIDE a
//! one-worker pool runs every lane on that worker, which is the
//! bracket's, so its recordings reach the caller's frame whether or not
//! anything composes them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::k_stats::{Bracket, Recorded};
use topo::Body;

use crate::common;
use common::*;

/// The recording a bracketed tessellation of `body` produces, as two
/// comparable sequences: `predicate@sign` per verdict, and the
/// escalations' predicates. The bracket is opened and finished on
/// whatever thread this runs on, which is the point of every case
/// below.
fn bracketed(body: &Body<f64>, delta: f64) -> (Vec<String>, Vec<String>) {
    let bracket = Bracket::open();
    let _ = mesh::tessellate(body, delta, Tol::witness());
    render(bracket.finish())
}

fn render(recorded: Recorded) -> (Vec<String>, Vec<String>) {
    (
        recorded
            .verdicts
            .iter()
            .map(|v| format!("{}@{:?}", v.predicate, v.sign))
            .collect(),
        recorded
            .escalations
            .iter()
            .map(|e| e.predicate().to_string())
            .collect(),
    )
}

fn pool(threads: usize) -> rayon::ThreadPool {
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("a rayon pool of the requested width")
}

#[test]
fn the_k_funnel_records_the_same_sequence_at_every_thread_count() {
    // A body whose lanes decide predicates: the rounded prism's walls
    // take the curved lane, whose chart doors and sizing classify
    // through `k_stats::decide`.
    let body = rounded_prism();
    let delta = 0.05;

    // THE REFERENCE: the whole pass on the bracket's own thread. A
    // one-worker pool's `install` puts `tessellate` on that worker, and
    // an indexed `par_iter` issued from inside a pool runs its work
    // there too — so every lane decides on the thread the bracket was
    // opened on, and its recording would reach that bracket with or
    // without anything composing it. This is what the serial loop
    // recorded, and what every other case has to reproduce.
    let want = pool(1).install(|| bracketed(&body, delta));
    assert!(
        !want.0.is_empty(),
        "the fixture decides no predicates at all, so this row would pass over a \
         funnel that recorded nothing — pick a body whose lanes decide"
    );

    // Same shape, four workers: now the map spreads the lanes over
    // threads the bracket was not opened on, and only the detached
    // frame each lane runs under plus the fold's splice bring them
    // back.
    let wide = pool(4).install(|| bracketed(&body, delta));
    assert_eq!(
        wide, want,
        "at four workers the bracket did not record what a pass on its own thread \
         records: the map's recordings are not reaching the caller's frame in arena \
         order"
    );

    // And the shape a real caller has: the bracket on a thread that is
    // NOT a pool worker, with the map injecting every lane into the
    // global pool. Nothing runs on this thread but the fold, so before
    // the lanes were detached this case recorded NOTHING at all.
    let ambient = bracketed(&body, delta);
    assert_eq!(
        ambient,
        want,
        "a bracket on a non-worker caller recorded {} verdicts where a pass on the \
         bracket's own thread records {}",
        ambient.0.len(),
        want.0.len()
    );
}

/// The `probe` sink is the funnel's second channel, and
/// `k_stats::detached` swaps it per lane the same way it swaps the
/// frame. **This crate cannot fill it**, and the row says so by
/// measuring rather than by arguing: samples are pushed by
/// `k_stats::Probe`'s `Real` impl alone, `mesh` takes `&Body<f64>` and
/// instantiates no other scalar, so a tessellation decides its
/// predicates at `f64` and records no margin. A row asserting a
/// composed sample population here would be asserting `0 == 0`.
///
/// What this row holds instead is that the claim stays true: the day a
/// lane reaches a `Probe`-instantiated predicate, this turns red and
/// the assertion above it becomes worth writing.
#[cfg(feature = "probe")]
#[test]
fn the_probe_sink_is_unreachable_from_this_crate() {
    let body = rounded_prism();
    geom_core::k_stats::start_recording();
    let bracket = Bracket::open();
    let _ = mesh::tessellate(&body, 0.05, Tol::witness());
    let recorded = bracket.finish();
    let samples = geom_core::k_stats::take_samples();
    assert!(
        !recorded.verdicts.is_empty(),
        "the fixture decided nothing, so the sample count below says nothing either"
    );
    assert!(
        samples.is_empty(),
        "{} margin samples reached the sink from an f64-only crate — a lane now \
         decides at `Probe`, so the composition row above owes the sample \
         population the same equality it asserts for the verdicts",
        samples.len()
    );
}
