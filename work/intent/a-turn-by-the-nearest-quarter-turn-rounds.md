---
id: a-turn-by-the-nearest-quarter-turn-rounds
kind: issue
title: A spin by the f64 nearest a quarter turn rounds, so a frame written by turns is off its axes by an ulp
status: open
opened: 2026-10-19
priority: P2
cost: M
parent: a-placement-pins-by-written-values-without-a-mate
---

A spin's rotation is built by Rodrigues from `angle.sin_cos()` (`crates/geom-core/src/linalg/mat.rs:203-212`). The f64 nearest π/2 gives `cos ≈ 6.1e-17`, not 0. So a frame, plane or placement spelled by turns of ±π/2 or π leaves the world's axes by about an ulp. That covers the `frame_at`/`moved_pose` frames in `crates/editor-core/src/mate/pin.rs` and the pin helpers' `moved_by`, and so `test_support::plane_on`, `axis_on` and every placement `place_at` writes. A `Datum` used to give those axes exactly.

Found under INTENT stage 3 D:
- `split_pinch_keeps` reads its tool `y = 1` off the body, and the notch tip at `y = 1` then splits by rounding (a naming refusal, or tables that differ under a flip). Its rows now build the plane from a face's plane and a slide, with no turn.
- `intent_s3_a_poses::a_meets_representative_moves_no_readers_outcome` allows rounding in its untilted case for the same reason.
- `p6_` in the D diagnosis lane found the same spins in `frame_at`.

D′ makes this worse: an unmated placement's rotation is a written spin chain, so a part stood upright is a quarter turn written as a number.

**Done when** a turn by a multiple of a quarter turn is exact wherever it is written. Either the rotation's sine and cosine are taken exactly at the f64 nearest a multiple of π/2, in every lane, a ruled change of geom-core's rotation, or the helpers spell axis-aligned frames by exact roles instead of turns. Add a row: a block placed upright by a written π/2 has its faces on the world's planes bit for bit.

**Evidence, the kitchen sink's census (2026-10-20, merging main into D).** The sink's revolve is placed in the world at `(-2, 0, 0)` with its own `y` along the world's `z` (`corpus/sink.rs`, `in_world` over `frame_at`), and `place_at`'s axis path writes that as spins taken by `atan2`: `-π/2` about `x`, then `-6.1e-17` about `z` where the turn is exactly zero. The composed map carries `cos(π/2)·sin(6.1e-17) ≈ 3.7e-33` where the datum's axes were exact, so the revolve's corner at `(0, 0, 1)` lands at `y = -3.7e-33`. In `perf12_census_goldens` (every ε row) four of the sink's contacts with the union's edge along the `z` axis read margins of `2.3e-33`, `3.7e-33` and `2.3e-49` where they read `0.0`, and one curved-face pair (the revolve's face against a block face) drops out of reach. The gallery ring's placement shows the same `-6.1e-17` spin (`crates/viewer/tests/creation_ops.rs`).
