---
id: fuzz-rows-discard-trials-against-a-floor-that-counts-them
kind: issue
title: A fuzz row's continue shrinks its own sample while its coverage floor is written against the attempted trial count: twelve candidate files
status: open
opened: 2026-09-11
refs: [mesh-cert10-fold-fuzz-row-flakes-on-a-fresh-seed]
---


**Filed for S-TINT at the landing of the `cert10` fix (2026-09-11), by
the lane that hit the instance.** This program opened the same day, for
test-suite integrity, and this is that class exactly: rows that are
sound in what they assert and wrong about how much they assert it over.

## The shape

A randomized row draws `trials` cases, `continue`s past the ones its
subject refuses, accumulates evidence only from the survivors, and then
floors that evidence against **`trials`** — the count it attempted, not
the count it measured. Nothing records the discard rate, so:

- the floor silently tightens as the discard rate rises;
- the failure message names a denominator the run never reached;
- and the row's own claim about its breadth ("a sweep of N random nets")
  is false by whatever the discard rate happens to be.

It fails as a *transient* red, which is the worst way for it to fail:
it reads as a flake and gets re-run, and the re-run passes.

## The worked instance, closed

`crates/mesh/src/nurbs_cert_fuzz.rs` —
`work/mesh/mesh-cert10-fold-fuzz-row-flakes-on-a-fresh-seed` has the full
measurement. Two rows in one file discarding **44%** and **32%** of their
trials, one of them reddening CI twice (2026-09-06, 2026-09-11) and being
diagnosed as a seed both times, with a failure message reporting 300
comparisons where 120 ran.

## The candidates, unmeasured

`continue` inside a `fuzz::`-driven loop, tree-wide, **not yet checked
one by one** — this list is a grep, not a finding, and each needs the
same ten minutes of instrumentation before anything is claimed about it:

- `crates/profile/tests/review_s8_probe.rs`, `review_s2.rs`
- `crates/sweep/tests/review_d8_consumer_differential.rs`
- `crates/geom/tests/curves/review_m5_pr4_adversarial.rs`, `r2_lt_probes.rs`
- `crates/geom-core/tests/ring_interval_differential.rs`, `d8_knot_queries_adversarial.rs`
- `crates/bvh/tests/ray_r2.rs`
- `crates/viewer/tests/review_gui2_r1.rs`, `review_gui2_r2.rs`
- `crates/editor-core/tests/gui1_pick_r2.rs`

A row only belongs to this class if it BOTH discards and floors against
the attempted count; a discard with no floor is a breadth question, and a
floor with no discard is fine. `cert10`'s sibling shows the middle case:
a discard under a **max**-shaped floor, which cannot fail the arithmetic
way but still quietly narrows the sweep.

## The cheap instrument

Counting is the whole fix, and it is three lines: count what the loop
actually measured, assert it against `trials`, and spell every reported
denominator from the counter rather than from `trials`. Whether that
wants a shared helper in `test-utils` — `fuzz::Sweep` holding the
attempted/measured pair and the floor — is this program's call and is
the reason this is one issue rather than twelve.

**Fences**: the candidate files are seven programs' ground. Each fix is
test-side and local, so it announces rather than asks — but the list is
not this program's to sweep unilaterally without saying so first.
