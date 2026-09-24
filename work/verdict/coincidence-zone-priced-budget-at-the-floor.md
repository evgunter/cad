---
id: coincidence-zone-priced-budget-at-the-floor
kind: issue
title: The coincidence zone of a magnitude slot refuses DegenerateExtrusion on every sub-box yet is bisected to the depth floor and priced Budget
status: open
opened: 2026-09-05
refs: [1969, k-stats-escalation-channel-and-redo]
priority: P1
cost: H
---

## What

On the planted-flip fixture `slab(20ε, 40ε)`
(`crates/editor-core/tests/m10_3_driver_interval.rs`), a leaf whose
depth enclosure sits wholly inside the coincidence zone `(−ε, ε)`
decides `extrusion_normal_component` as `Zero` DEFINITELY and the
extrude refuses `ExtrudeError::DegenerateExtrusion`
(`crates/sweep/src/extrude.rs`) — a definite refusal, not an
escalation. Every sub-box of such a leaf refuses identically, yet
`drive::classify_replay` (`crates/editor-core/src/drive.rs`) has no
arm for it: the failure is neither an escalation nor one of the
`box_independent_measure_class` kinds, so the leaf bisects to the
depth floor and is priced `Budget`.

Measured (R2's leaf ledger on PR #1969's head, 4096 leaves): `Budget`
mass **2.499 %** lies wholly in `[−ε, ε]` and **0.011 %** within
`0.01ε` of the band's four edges; no `Budget` mass anywhere else.
`SliverTerminal` is 22.498 %, all of it wholly inside `(ε, Kε)` or
`(−Kε, −ε)`, naming `extrusion_normal_component`. The k-stats row
`a_sliver_wrapped_in_the_ops_own_error_is_priced_sliver_terminal_not_budget`
bounds the `Budget` mass by `2ε / 80ε + 1e-3` rather than claiming it.

## Why it is M10's

It is the same class M10-6 recognised for measure refusals
(`box_independent_measure_class`: a fact about the document no box
moves, priced under its own name rather than refined to the floor),
but the refusal is a KERNEL one — a magnitude slot decided zero — and
naming it is a driver decision: a `RefusalReason` for "the box lies in
a coincidence zone of a magnitude slot", terminal like a sliver, with
the degenerate-extrusion (and its siblings: a zero revolve angle, a
zero pattern step) mapped to it.

## Re-homed at M10's exit sweep (2026-09-13)

Here because the fix is a `RefusalReason` arm in `drive::classify_replay`, and
`crates/editor-core/src/drive.rs` is PROPS' at this sweep — the analysis lane its
`keep_out` said it would inherit here. The row already `refs` PROPS' own
`k-stats-escalation-channel-and-redo`.

From `work/m10/` at M10's close (`docs/DOC-LEDGER.md` sweep 13; the walk and the directory are recoverable at the SHA it names). The id is unchanged.

## What it costs a consumer (DOCM-9, 2026-09-13)

Evidence added here rather than in a second row: the defect is the
one above, and this is what it makes unreportable one level up.

`classify_replay`'s catch-all — `_ => return LeafVerdict::Bisect` for a
node that FAILED for anything but an escalation, a guided structure
flip, or a box-independent measure refusal — means a leaf where a node
definitely does not build never reaches the verdict comparison. So a
`FlipCrossing` can carry a node STANDING change by exactly one route,
`NodeErrorKind::ProfileLaneReplay { structure: Some(Flipped), .. }`
(plus the descendants that route poisons). Every other way for a
document to stop building over part of a box — a degenerate extrusion,
a profile the loops of which stop being simple, a boolean that empties
— is priced `Budget` instead.

`editor_core::range` (DOCM-9) reads a drive's leaves as a range and
reports a boundary under the class of the first leaf the driver
classified definitely otherwise. Its `NewFailure` arm — "a node that
built at the witness does not, inside this bracket", which is the
answer `viewer::bounds` exists to give — is therefore unreachable on
every fixture the unit could build: the boundary comes back
`DecisionFlip` where the far side still builds, and `Indeterminate`
(`Budget`) where it does not. `crates/editor-core/tests/docm9_range.rs`
covers the three reachable arms and states this gap; the arm stays in
the contract because the driver naming these refusals terminally is
what makes it reachable, and a vocabulary that grew the arm later
would silently reclassify existing reports.

Measured on the query's own fixtures at `d2c084d18`: a plate whose
circular hole slides out of it (`round_escape`, a 10x2 plate, a
0.2-radius hole, seed `[-1, 5.5]`, 256 leaves) reports `DecisionFlip`
at offset 3.367 and never reaches the containment inversion beyond
10.2; a 2x2 plate with a square hole whose centre is a parameter
certifies NOTHING at any seed down to `±1e-7` (that second observation
is its own row, `parametric-polygon-loop-certifies-nothing`).
