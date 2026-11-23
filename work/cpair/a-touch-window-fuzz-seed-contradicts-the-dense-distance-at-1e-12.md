---
id: a-touch-window-fuzz-seed-contradicts-the-dense-distance-at-1e-12
kind: issue
title: curve_pair touch-window oracle fuzz row reds at eps 1e-12 on seed 0x810fa45f837dd426
status: open
opened: 2026-10-23
priority: P2
cost: M
---


## Found (QUAD, measure door unit A lane, 2026-10-23)

`crates/geom-brep/src/curve_pair/pose/window/oracle.rs:329`
`each_window_reading_holds_on_a_dense_distance_at_a_touch` failed in
the geom-brep suite run at `CAD_TOLERANCE_EPS=1e-12` on seed
`0x810fa45f837dd426`. That was the only geom-brep failure at that ε on
PR 5589's head. The PR does not touch `curve_pair`. Not root-caused
here.

**To reproduce:** run the row at ε = 1e-12 with that seed, replaying it
through `fuzz::replay()`'s documented environment variable. Then check
whether the same seed reds on main. A contradiction against the dense
distance is a soundness question until shown otherwise. The fuzz draws
at 1e-12 more rarely than the nightly reaches, so a seed is the
evidence.

## 2026-10-24 — a second seed (INTENT nightly triage)

Nightly run 40066431545 (main `3dfb96b91c`), full-suite job, eps=1e-6
pass: `ε 0.000000000001: 2 readings contradicted (reproduce with
CAD_FUZZ_SEED=0x9ac83e19a6f5b5a3 CAD_FUZZ_EFFORT=1)`. Draw 57 is a
pair of circles of radius ≈553 whose `Recedes` reading on
`(-3.909e-7, -3.908e-7)` (away false) and `(1.338e-7, 1.346e-7)`
(away true) contradicts the dense distance. The default and 1e-12
passes drew other seeds and passed. No merge in the window touched
`curve_pair/pose/window/`, so this is the fuzzer drawing into the
same defect, not a regression. Pin both seeds as deterministic rows
beside the fix.

## 2026-10-27 — another seed (SPLICE, PR 5868's slow-set run)

Default suite run (the row loops its own ε rows), PR 5868 at `833da0303`,
which does not touch `curve_pair`: `ε 0.000000000001: 2 readings
contradicted`, reproduce with `CAD_FUZZ_SEED=0xdcf91059414eafd4
CAD_FUZZ_EFFORT=1`. Draw 7, f64: `Recedes` read on two windows
`(-2.142e-7, -2.117e-7)` (away false) and `(1.139e-6, 1.141e-6)` (away
true), against a Circle of radius 477.40 centred at
`(-160.017, -381.167, 1.259)`, axis `(0.00135, 0.00227, 0.99999)`, and the
point b `(105.656, 48.066)`.

## 2026-10-28 — pinned on main (GREEN, `green/full-suite-pins`)

Nightly run 40637814390 (main `a1cfd1cfdb`), `full suite`; re-run on main `91c83c38`. A fourth seed: the 1e-6 pass drew `CAD_FUZZ_SEED=0xf440002aa745ad47`
(`CAD_FUZZ_EFFORT=1`), 3 readings contradicted at ε 1e-12, draw 31:
`Recedes` on `(-7.715e-7, -7.710e-7)`, `(-5.076e-7, -5.031e-7)` and
`(-1.380e-6, -1.379e-6)` (away false) against a Circle of radius 489.76.

The search is a counterexample fuzz, so no output can be pinned. The
1e-12 band is out of the row's loop
(`each_window_reading_holds_on_a_dense_distance_at_a_touch` prints a
`SKIPPED` line naming this row); the default band still searches.
Closing this row puts the 1e-12 band back, with the four seeds as
deterministic rows beside the fix.

## 2026-10-28 — another seed (SPLICE structural-rim lane)

A local default-ε run of the whole `geom-brep` suite on
`splice/structural-rim` (PR 5959, which does not touch `curve_pair`)
red this row on `CAD_FUZZ_SEED=0xc8d7ad5a8a57245d CAD_FUZZ_EFFORT=1`,
at the oracle's ε 1e-12 reading: draw 19 reads `Recedes` on
`(1.05e-6, 1.20e-6)` for a circle of radius 727 m centred about 675 m
from the origin. The seed replays deterministically.

## 2026-11-16 — the same family off a touch (onecopy/ringed-near-end)

The non-touch row `each_window_reading_holds_on_a_dense_distance`, run at the needle's band (1e-12/1e-11), drew one contradiction on seed `CAD_FUZZ_SEED=0x57955e51f786ba3a CAD_FUZZ_EFFORT=1`. Draw 51 is a circle of radius ≈662 against an ellipse with semi-axes (72.3, 48.3). Its ringed `Recedes` reading on `(1.926e-7, 2.016e-7)` (away true) contradicts the dense distance. That row now holds only the unringed reading at 1e-12 and leaves the ringed one to this item. Fixing this item should restore the ringed reading there.

## 2026-11-16: from PR 7029's review

- **The 1e-12 fence is wider than the defect.** The non-touch oracle row drops the whole ringed lane at 1e-12, `Off` and `On` included (`window/oracle.rs:306-307, 222-227`). Every seed this row lists is a ringed `Recedes`. When this row is worked, narrow the fence to skip only `Recedes`. The chain restart (`Window::chained`) leans on ringed `Recedes` plus a ringed `off` past the zero, and neither is held at 1e-12 while the fence stands.
- **The contradiction may be oracle noise (unsure).** The oracle's own rounding is about `1e-13·scale` (`oracle.rs:243-247`), and `tol` is capped at `z/8`. At 1e-12 on circles of radius ~500 that is about `5e-11` of rounding against a `1.25e-13` tolerance. So the `Recedes` monotonicity check may be measuring the oracle, not the kernel. Check this first.
- **The restart has no row that goes red without it.** Open a `k_stats::Bracket` around `w.chained(..)` on one of the 15 real windows, e.g. `curve_pair::plane_rows::two_circles_at_a_shallow_tilt_meet_twice_near_one_plane_root`. Assert that no verdict names `curve_pair_window_off_span` or `curve_pair_window_width` (locus.rs:1308 is the precedent).
- **"A receding piece is off where its near end is" is spelled twice:** in `read`'s unringed arm (`window.rs:316-326`) and in `chained` (`window.rs:453`). One helper should map direction to near end for both. Alternatively, `read` could return `Off` for a ringed `Recedes` whose near end is off, leaving `chained` unchanged.

## 2026-11-17: the unringed reading at 1e-12 too (ONEHOME, fork 275 step 3b)

The non-touch row `each_window_reading_holds_on_a_dense_distance`, which since 2026-11-16 holds only the unringed reading at the needle's band, reds about 3 seeds in 25 there. Each is an **unringed** `Recedes` (`ring None`), so the family reaches the reading the fence kept. The three seeds, each one contradiction at ε 1e-12, `CAD_FUZZ_EFFORT=1`:
- `0xe8ff061a1fba18f3`: draw 51, `Recedes` on `(-3.527e-9, -3.486e-9)`, an ellipse of semi-axes (667.5, 1092.9) centred about 755 m from the origin;
- `0x45acb5a5d7ddd040`;
- `0xd5a0f0c0b139ce05`;
- `0x04f4dcf753f68827` (2026-11-18): draw 37, `Recedes` on `(2.803e-8, 2.806e-8)` on a circle centred about 610 m from the origin, red identically on the merge of PR 7048 (`0c2b761e91`).

All three red identically on `onehome/pair-read` at e4f142b6b2 (PR 7048), which does not touch `curve_pair/pose/window/`, and on `onehome/span-foot` (step 3b), so neither branch is their cause. The scale, ~10³ m at ε 1e-12, matches the oracle-noise question above.

## 2026-11-18: the dense-distance row at 1e-6 (importgate/build)

The sibling row `curve_pair::pose::window::oracle::each_window_reading_holds_on_a_dense_distance`
(not `_at_a_touch`) reds the same way at ε = 1e-6 on
`CAD_FUZZ_SEED=0x362191a627c797f4` (`CAD_FUZZ_EFFORT=1`): two `Recedes`
readings contradicted on a circle of radius 569.93 m whose axis tilts
~1.6e-9 off −z, against the point (152.48, 92.76), at windows
(−2.509e-9, −2.367e-9) and (1.0218e-9, 1.0227e-9). Found on
importgate/build's local run (2026-11-18), which does not touch the
pose's window code.

## At effort 20 it reds most runs (ONEHOME, PR 7137, 2026-11-19)
At the default ε, `CAD_FUZZ_EFFORT=20` reds
`each_window_reading_holds_on_a_dense_distance` in 9 runs out of 10 on
PR 7137's head. Each red is an unringed `Recedes` on a conic of radius
280–930 m. Three seeds replayed red on the merge base `3bddaf8a60` too:
`0xf0018fd9fa4abc02`, `0x9fc6cc9735038399`, `0xc1ab1587225e7a95`.
PR 7137 changes only a comment in `curve_pair/pose/window.rs`.

## 2026-11-19 — a third seed (ONEROUND, `oneround/rounded-room` lane)

`each_window_reading_holds_on_a_dense_distance` at the default ε drew
`CAD_FUZZ_SEED=0x089b1926d015d5b1` (`CAD_FUZZ_EFFORT=1`) and failed its
1e-12 pass: `1 readings contradicted`, draw 54, a circle of radius
≈523.1 whose `Recedes` reading on `(-2.742e-8, -2.562e-8)`, ring `None`,
contradicts the dense distance. Replayed on `origin/main` at `b80352cd`
with the same seed it fails the same way, so the lane's diff
(`half_angle`, `chart_singularity`) is not its cause. Two of five runs
of the row here drew a failing seed (only this one was captured).

## 2026-11-20 — a fourth seed (ONEROUND, the review of PR 7145)

The same row drew `CAD_FUZZ_SEED=0xd5b1cddf2406ff5f` (`CAD_FUZZ_EFFORT=1`)
in the reviewer's run and failed its 1e-12 pass, `1 readings
contradicted`. Replayed alone it fails and reaches no `half_angle` walk.

## 2026-11-21 — two more seeds (ONEHOME, the reviews of PRs 7208 and 7277)

The same row (`curve_pair::pose::window::oracle::each_window_reading_holds_on_a_dense_distance`)
drew two more seeds, and both are red on main too:
- `CAD_FUZZ_SEED=0xa67275b2cf5bde2e` (`CAD_FUZZ_EFFORT=1`), in 7208's
  run: `3 readings contradicted`, a `Recedes` on a far ellipse. It
  reproduces on `c8c00a2a33`.
- `CAD_FUZZ_SEED=0x917ed9c53b80de98` (`CAD_FUZZ_EFFORT=10`), in the
  review of PR 7277: red on main.

## 2026-11-21 — a third seed (GERM U3a, PR 7226's CI)

The slow-set run of `each_window_reading_holds_on_a_dense_distance` on
PR 7226 drew `CAD_FUZZ_SEED=0x1b1bbf1c5dca1efb CAD_FUZZ_EFFORT=1`:
`ε 0.000000000001: 1 readings contradicted`, draw 54, a circle of
radius ≈548.6 whose `Recedes` reading on
`(5.121e-10, 5.139e-10)` contradicts the dense distance. It reproduces
locally with `crates/geom/src/curves/conic.rs` (the one shared file
the PR changes on that path) put back to main's, and the PR does not
touch `curve_pair/pose/window/`: the same defect, a new seed.

A second draw on the next run: `CAD_FUZZ_SEED=0xb5c0e6c80ba9b4dc`, draw
48, an ellipse of semi-axes ≈634 and ≈1640 whose `Recedes` reading on
`(1.6457e-10, 1.6461e-10)` contradicts the dense distance; it too
reproduces with main's `conic.rs`. Over twelve fixed seeds the row reds
on none, with either `conic.rs`: the rate is low and the PR does not
move it.

## 2026-11-23 — the plain row too, two seeds, reproduced on main (ONEROUND, PR 7221)

The sibling row without the touch, `curve_pair::pose::window::oracle::each_window_reading_holds_on_a_dense_distance`,
reds at ε = 1e-12 the same way: one `Recedes` reading on a pair of circles of radius 550 to 600 contradicted by the dense distance.

- Seed `0x7a797c5d224361de`: "draw 16 f64: Recedes on (-6.450266237113439e-10, -6.414448355365588e-10) ring None", a circle of radius 555.67 against `b (134.196…, 134.196…)`.
- Seed `0x9e4e8974246bf757`: "draw 54 f64: Recedes on (8.334309359444234e-10, 8.349085180644984e-10) ring None", a circle of radius 596.49 against `b (54.349…, 19.914…)`.

Both red on main `c76360d3d4` (run here, `CAD_TOLERANCE_EPS=1e-12 CAD_FUZZ_SEED=<seed> CAD_FUZZ_EFFORT=1 cargo nextest run -p geom-brep -E 'test(=curve_pair::pose::window::oracle::each_window_reading_holds_on_a_dense_distance)'`), and the first reds on PR 7221's head `6862af6efe` with the draw, the interval and every number identical to main's, so the PR's conversion of the rounded pairs in `window.rs` moved nothing in it.

**How it arrives:** the row is in the slow set, so a PR runs it only when its diff edits `window/oracle.rs`'s file set, and then at a seed the run draws. PR 7221's hosted run 45088390543 drew the first seed and went red in `test (kernel)` on it; its earlier run on the same code passed that step on another seed. Until this is fixed, any PR that edits `curve_pair/pose/window/` may red by the draw. Pin these two seeds beside the others at the fix.
