//! The five scenarios PERF-PLAN §5 names, and nothing else.
//!
//! # What this lane is for
//!
//! PERF-PLAN §2.3 gates every micro-optimization on a measurement that
//! did not exist, which made the gate a deadlock rather than a
//! deferral. These six benchmarks (the washer is tessellated at two
//! chordal tolerances, so five scenarios are six rows) are the first
//! per-kernel wall-clock numbers this repository has had.
//!
//! **Reporting, never gating** — `memories/perf-measurement-lane.md`.
//! No CI row fails on a millisecond here. The lane's product is the
//! append-only history under `docs/perf-data/criterion/`, and a
//! regression is read off the trend.
//!
//! # Read the noise floor before believing a delta
//!
//! Criterion's per-run confidence interval is a within-run statistic and
//! it UNDERSTATES what a comparison across two runs can resolve. Three
//! consecutive runs on a quiet 4-core box (2026-08-27) spread as follows,
//! against within-run intervals of ±2-3%:
//!
//! | row | run 1 | run 2 | run 3 | spread |
//! |---|---|---|---|---|
//! | `tessellate/washer/1e-4` | 10.33 ms | 10.75 ms | 9.68 ms | ~5% |
//! | `tessellate/washer/1e-6` | 614 ms | 643 ms | 680 ms | ~5% |
//! | `validate/tier23_washer` | 20.5 µs | 24.3 µs | 21.7 µs | ~9% |
//! | `mass_props/washer` | 1.59 µs | 1.52 µs | 1.49 µs | ~3% |
//! | `build/extrude` | 18.5 µs | 20.1 µs | 17.5 µs | ~7% |
//! | `boolean/two_bricks` | 127 µs | 132 µs | 121 µs | ~4% |
//!
//! A hosted 2-vCPU runner has a fatter tail than that box does. Treat a
//! move under ~10% as noise unless several consecutive entries agree,
//! which is the same instruction `docs/perf-data/rebuild-latency/`
//! carries and for the same reason.
//!
//! # Why these scenarios
//!
//! Each row is a cost center PERF-PLAN §1.3 ranks, sited where the plan
//! says the cost is:
//!
//! * `tessellate/washer/*` — the CDT insertion path (finding 7b), and
//!   the row a `spade` bulk-load adoption (§2.1) would have to move.
//!   Two tolerances because the finding is about the QUADRATIC: the
//!   1e-4 -> 1e-6 ratio is the shape, not either number alone.
//! * `validate/tier23_washer` — the commit lane's validation ladder on a
//!   revolved body (findings 4, 5, 16).
//! * `mass_props/washer` — per-face flux quadrature; §2.2's canonical
//!   idiom-2 parallelism target, unbuilt.
//! * `build/extrude` — Euler-op surgery through the sweep front door
//!   (finding 9's kill-direction arena scans).
//! * `boolean/two_bricks` — the boolean commit path: join (finding 13),
//!   `graft_solid` (14), the double tier-1 gate (4), `StableName`
//!   nesting (15).
//!
//! Authored through `pncad::prelude` alone, as a library consumer would:
//! a scenario that cannot be written that way is a finding about the
//! library (`memories/demo-purpose.md`), not a licence to reach past it.
//!
//! # The wall-clock budget
//!
//! The per-group sample counts below are set deliberately low rather
//! than left at criterion's default of 100, and the reason is the noise
//! floor above rather than parsimony: extra samples narrow a WITHIN-run
//! interval that the cross-run spread swamps anyway, so past a handful
//! they buy resolution the trend cannot use, with Actions minutes, every
//! night. One row dominates the budget — the 1e-6 tessellation, which is
//! sub-second an iteration while everything else is microseconds — so
//! `tessellate` carries its own sample count.

use criterion::{Criterion, criterion_group, criterion_main};
use pncad::prelude::*;
use pncad::tolerance::Tol;
use std::hint::black_box;
use std::time::Duration;

/// An axis-aligned rectangle, authored through the PATHS lattice.
fn rect(x: (f64, f64), y: (f64, f64)) -> ClosedLoop<f64> {
    Open.at(p2(x.0, y.0))
        .line_to(p2(x.1, y.0), Tol::witness())
        .and_then(|t| t.line_to(p2(x.1, y.1), Tol::witness()))
        .and_then(|t| t.line_to(p2(x.0, y.1), Tol::witness()))
        .and_then(|t| t.line_to(Start, Tol::witness()))
        .expect("the rectangle authors")
}

/// The washer: rectangle [1,2]x[0,1] revolved fully — genus 1, slit
/// annuli and full-2π cylinder walls. The tessellation suite's own
/// standard body, so the row is comparable with what that suite reports.
fn washer() -> Body<f64> {
    let profile = validated(
        SketchPlane::<f64>::xy(),
        vec![rect((1.0, 2.0), (0.0, 1.0)).into()],
        Tol::witness(),
    )
    .expect("the washer profile validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: v2(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the washer revolves")
    .body
}

/// An axis-aligned box [x0,x1]x[y0,y1]x[z0,z1], built by extrusion.
fn slab(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    let plane = SketchPlane::from_frame(
        p3::<f64>(0.0, 0.0, z.0),
        v3(1.0, 0.0, 0.0),
        v3(0.0, 1.0, 0.0),
    );
    let profile = validated(plane, vec![rect(x, y).into()], Tol::witness())
        .expect("the slab profile validates");
    extrude(
        &profile,
        Extrusion::Distance(real(z.1 - z.0)),
        Tol::witness(),
    )
    .expect("the slab extrudes")
    .body
}

/// The two tessellation rows. Own group, own sample count: at ~0.7 s an
/// iteration the 1e-6 row is the whole lane's wall clock, and criterion
/// warns rather than overrunning if the budget is short.
fn tessellation(c: &mut Criterion) {
    let body = washer();
    let mut group = c.benchmark_group("tessellate");
    group.sample_size(10).warm_up_time(Duration::from_secs(1));
    for (delta, measure) in [(1e-4_f64, 3), (1e-6_f64, 9)] {
        group.measurement_time(Duration::from_secs(measure));
        group.bench_function(format!("washer/{delta:e}"), |b| {
            b.iter(|| {
                tessellate(black_box(&body), black_box(delta), Tol::witness())
                    .expect("the washer tessellates")
            });
        });
    }
    group.finish();
}

/// The four sub-millisecond rows: validation, mass properties, one
/// Euler-op build, one boolean. Cheap enough to afford real samples.
fn kernel_ops(c: &mut Criterion) {
    let body = washer();
    let base = slab((0.0, 3.0), (0.0, 2.0), (0.0, 1.0));
    // Strictly interior in x and y, poking out of the base's top: the two
    // genuinely interpenetrate and NO pair of faces is coincident, so this
    // is the plain seamed path rather than a declared-contact union.
    let post = slab((0.5, 1.5), (0.5, 1.5), (0.5, 2.0));

    let mut group = c.benchmark_group("kernel");
    group
        .sample_size(50)
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_millis(1500));

    // Tier 2 + tier 3: `validate_geometric` runs the structural tiers
    // first (`validate_closed`), then the geometric ones.
    group.bench_function("validate/tier23_washer", |b| {
        b.iter(|| {
            validate_geometric(black_box(&body), Tol::witness()).expect("the washer certifies")
        });
    });
    group.bench_function("mass_props/washer", |b| {
        b.iter(|| mass_properties(black_box(&body), Tol::witness()).expect("mass properties"));
    });
    // The BUILD, not a rebuild of a cached body: profile authoring plus
    // the extrusion's Euler-op sequence, which is what finding 9 is about.
    group.bench_function("build/extrude", |b| {
        b.iter(|| black_box(slab((0.0, 3.0), (0.0, 2.0), (0.0, 1.0))));
    });
    group.bench_function("boolean/two_bricks", |b| {
        b.iter(|| union(black_box(&base), black_box(&post), Tol::witness()).expect("the union"));
    });
    group.finish();
}

criterion_group!(kernel, tessellation, kernel_ops);
criterion_main!(kernel);
