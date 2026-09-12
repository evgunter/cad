---
id: coincidence-zone-priced-budget-at-the-floor
kind: issue
title: The coincidence zone of a magnitude slot refuses DegenerateExtrusion on every sub-box yet is bisected to the depth floor and priced Budget
status: open
opened: 2026-09-05
refs: [1969, k-stats-escalation-channel-and-redo]
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
