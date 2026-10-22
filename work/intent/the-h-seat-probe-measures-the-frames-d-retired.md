---
id: the-h-seat-probe-measures-the-frames-d-retired
kind: issue
title: intent_s4_h_placed::measure_a_mated_seat_at_sym builds the six-symbol datum frames stage 3 D retired, so it no longer measures a mated seat
status: closed
opened: 2026-10-21
closed: 2026-10-22
priority: P3
cost: E
---


## What

`crates/editor-core/tests/intent_s4_h_placed.rs`
`measure_a_mated_seat_at_sym` (line 129, `#[ignore]`d) models a seat's
cap normal as `normalize(u × v)` over a `Datum::Frame`'s six component
symbols. Stage 3 D retired datum frames. A seated cap's symbols are now
its part's own variables, so the probe measures a construction the
kernel no longer builds. The seat re-measure (PR #5553) read the two
copies' carriers at `Sym` directly instead.

## Owed

Retire the probe, or rewrite it over a D-era seat. Which one follows
from the ruling on `a-mate-placed-seat-is-unproven-at-the-doors-budget`:
if a mated seat is read by its mate, the probe's question is moot.

## Closed (2026-10-22)

Retired. A mated seat is read by its mate, so the probe's question is
moot (`the-door-reads-a-mated-seat-by-its-mate`). The file keeps its
frame row, `a_poses_frame_is_orthonormal_as_a_polynomial_identity`,
since the replay still folds bundles at `Sym` for rows no mate states.

