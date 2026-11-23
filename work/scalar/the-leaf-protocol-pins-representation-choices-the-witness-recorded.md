---
id: the-leaf-protocol-pins-representation-choices-the-witness-recorded
kind: issue
title: a certified leaf holds the witness's recorded representation choices, so a plain build inside it can record a canonical_order verdict differently
status: open
opened: 2026-11-22
---


Found by the review of PR 7369 (the certified range search), and filed from OFFER's fix pass. The leaf protocol is `drive::classify`; the range search certifies its intervals through it, so `drive` certifies the same leaves.

## What was seen

A unit square whose first corner's x is a parameter `h`, extruded: the loop `[(h, 0), (1, 0), (1, 1), (0, 1)]`, with `h = 0` (the reviewers' probe `corner_x`, `review_7369_probes::p5_corner_x_vector_diff` on branch `offer/review-7369-probes`).

- The plain `f64` build at `h = 0` records verdict #69 of the profile's certifying vector as `canonical_order_x`, sign `Zero`, role `Choice`: which of two points comes first along x (`crates/profile/src/gate.rs`, `CanonicalOrderX`).
- The plain build at every other `h` tried, `1e-12` to `0.98` and `-1e-12` to `-0.98`, records it `Negative` above zero and `Positive` below.
- The range search, through `classify`, nonetheless proves about `[-0.9999995, 0.9988]` around `h = 0` (probe `p4`). So the certificate covers values whose own `f64` build records a choice the witness did not.

## Why it matters

A certified leaf is read as "every value here builds what the witness built". What it actually asserts is narrower: the guided replay follows the witness's recorded REPRESENTATION choices (`Role::Choice` verdicts, which `resolve::vdiff`'s `Path::steps` reads as the run's path) rather than deciding them afresh, and certifies the decisions under those choices. The shape built is the same, stated from another start. `range::CertifiedRange::proven_offsets` now says so, and `docm9_range::every_value_inside_the_proof_decides_as_the_witness_did` carves `canonical_order_*` out for this reason.

## The question for this program

Whether that is the E6 certificate's intended meaning, and if so where the leaf protocol's own docs state it (`drive::classify`, the E6 clause). Or whether a `Choice` verdict decided `Zero` at the witness should refuse or split a leaf that spans both of its signs, as a `Check` verdict would.
