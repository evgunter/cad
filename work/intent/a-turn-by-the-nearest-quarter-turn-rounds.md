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
