---
id: viewer-plate-suites-index-at-a-display-tolerance-nothing-asserts
kind: issue
title: Six viewer suites index at a display tolerance no row can see move, measured over 5000x
status: open
opened: 2026-09-20
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
  | the door's body → `panic!` | **the divergent control**, method item 19: it separates *unasserted* from *never called* | **551 / 75** |

  The control is what makes the reading safe. 75 rows across every
  calling suite reach this value and none is missing from the red set,
  so the folded sites are executed — and not one of them can see the
  value move three orders of magnitude.

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
  - **`debug_dumps`, `eval_seam` and `cascade_delete` remain unprobed
    in either direction** — no control has yet shown their folded
    sites execute at all, which is a weaker statement than the one
    above and a different thing to check.
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
