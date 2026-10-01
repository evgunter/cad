---
id: viewer-plate-suites-index-at-a-display-tolerance-nothing-asserts
kind: issue
title: Six viewer suites index at a display tolerance no row can see move, measured over 5000x
status: open
opened: 2026-09-20
priority: P3
cost: D
---


## Finding

- **Where**: `crates/viewer/tests/common/mod.rs`'s `plate_delta`, and
  through it `blend_authoring`, `debug_dumps`, `edge_pick`,
  `eval_seam`, `frame_policy` and `select_pick` — the six suites that
  used to write `DisplayTolerance::new(2.0e-4)` out privately and now
  share one door. Three of those private copies carried prose saying
  the value is *"fine enough that the hole is a ring of facets rather
  than a polygon that misses the ray"*.
- **The measurement**: three plants in the shared door, with the
  direction argued before the result was read (S-DUP plan, method
  item 16). Baseline `cargo test -p viewer --test all`, default lane,
  **626 passed / 0 failed / 1 ignored**.

  | plant | direction | result |
  | --- | --- | --- |
  | δ ×50, 2×10⁻⁴ → 1×10⁻² | **coarsen** — harder for every "this ray meets the hole's rim" row | 626 / 0 |
  | δ ×0.1, 2×10⁻⁴ → 2×10⁻⁵ | **refine** — harder only for a row keyed on the δ value itself | 626 / 0 |
  | δ ×5000, 2×10⁻⁴ → **1.0 m**, wider than the plate | **coarsen, maximal** | 626 / 0 |
  | the door's body → `panic!` | **the divergent control** (method item 19): it separates *unasserted* from *never called*, and is the only plant here whose direction is not a direction | **551 / 75** |

  **This row is the single home for that argument.** The δ row
  (`work/dup/viewer-tests-spell-one-display-tolerance-in-seven-places`)
  and PR #2929 point here rather than restating it; why, below.

  The control's red set is **exactly `plate_delta`'s caller set, with
  nothing missing**: `select_pick` 21, `edge_pick` 17,
  `blend_authoring` 16, `frame_policy` 16, `eval_seam` 4,
  `debug_dumps` 1 — six suites, 75 rows, summing with the 551 to the
  626 baseline. So the folded sites are executed and 75 rows reach
  this value, and not one of them can see it move three orders of
  magnitude. *Called and wholly unasserted*, not dark.

  Every row sums to the baseline's 626. The third plant sets the
  display tolerance an order of magnitude wider than the whole
  fixture, and the crate is still green.

- **What that is NOT evidence of.** The suites are live: the same
  tree reds 52 rows when the shared downward pick ray is aimed under
  the fixture, and 10 when the shared index door is keyed on a
  coarser δ *than the one its caller asked for* (a mismatch, which is
  a different claim from the value being watched). So this is
  *nothing asserts anything about this δ*, not *nothing here asserts
  anything*.
- **Why it is S-TINT's and not S-DUP's**: the duplication is closed —
  seven spellings became one door, `work/dup/viewer-tests-spell-one-
  display-tolerance-in-seven-places`. What is left is a coverage
  question: three suites' prose states a property of the value
  (the hole tessellates as a ring) that no row can see fail. Either a
  row should hold it, or the prose is a claim with nothing behind it
  and should go.
- **The other folded sites with no red, sorted by what the measurement
  actually shows.** Six suites appear in NO red set across the lane's
  thirteen plants: `debug_dumps`, `focus_highlight`, `eval_seam`,
  `pick_windows`, `cascade_delete`, `path_authoring`. The reviewer's
  divergent controls then split them, and the split is the finding:
  - **`focus_highlight` and `pick_windows` are live**, proved by a
    `panic!` planted in `common::index_of` (495 / 131). They are in
    the same state as the δ above — *called, and wholly unasserted on
    the value they pass* — not dark.
  - **`path_authoring` is explained and fine**: its row asserts on the
    NOTATION a literal is written in, not the value, so a value plant
    cannot reach it by construction.
  - **`debug_dumps` and `eval_seam` are live too**, and saying
    otherwise here was wrong: both are `plate_delta` callers and both
    are IN the control's red set above (1 and 4). They are in the same
    state as `focus_highlight` and `pick_windows`.
  - **`cascade_delete` alone remains unprobed in either direction.**
    It is not a δ caller at all — its folded site is an `ang(0.0)` —
    so no control run so far has shown that site executes, which is a
    weaker statement than the one above and a different thing to
    check.

### A second instance: `common::ring_delta`

Folded in the merge pass from the GUI-2 suites' two spellings of the
gallery ring's δ. On the merged tree (baseline **685 / 0 / 1
ignored**):

| plant in `common::ring_delta` | direction | total |
| --- | --- | --- |
| ×25, 2×10⁻³ → 5×10⁻² | **coarsen** | 685 / 0 |
| body → `panic!` | **the divergent control** | 683 / 2 — `review_gui2_r1` 1, `review_gui2_r2` 1 |

Same state as `plate_delta`: called by both suites, and nothing either
suite asserts can see it move 25×. It is a cost choice by its own
doc, so this is the expected result, and it is recorded because it is
the same kind of fact, not because it needs a separate fix.

### How that sentence got written, which is this program's own subject

The false sentence sat **three bullets below the table that
contradicts it**, in the same commit, because the control's argument
had been written out three times — here, in the δ row, and in the PR
body — and the correction reached one copy. The numbers agreed in all
three; the prose around them did not.

Two things worth keeping. **Duplication became drift inside a single
commit**, in a unit whose whole subject is duplication, written by the
lane that had just disclosed the trap. And **the wrong half is the
actionable half**: "75 rows red, none missing" is a receipt a reader
checks and moves past, while "these three are unprobed" is a work
order a later lane would act on — it would have gone looking for
coverage that the table above it already proves exists. A restated
argument does not drift evenly; it drifts into the sentence that
tells someone what to do.

The repair is the one a sibling unit set for
`orient-module-prose-accumulation`: **one home for the argument, and
pointers from everywhere else.**
- **Instrument, and its blind spot**: mutation, over the whole `all`
  binary. A plant in a shared door reaches every consumer of that
  door, so a green result is a statement about the whole crate; it
  says nothing about the same δ in another crate's suites, and it
  cannot see a row that is `#[ignore]`d (one is) or feature-gated out
  of the default lane (`docm9_range_vs_probe` is).
- **Raised by**: the S-DUP lane closing the four viewer-suite door
  rows, 2026-09-20, measured at `cd9fdfd6b`.

## Why this sits on S-TINT's slate

A suite that cannot go red on a value it names is S-TINT's charter by
the S-DUP plan's own split (*"a suite with no assertions is a coverage
defect, and a fixture built from scratch in six crates is a vocabulary
defect"*). The vocabulary half is closed on S-DUP's slate; this is the
other half, and the lane that measured it does not own it.
