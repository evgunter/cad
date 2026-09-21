---
id: m10-3-chamber-probes-gated-away-from-the-symbolic-tier
kind: issue
title: the M10-3 chamber probes are gated to the driver's paths and not the symbolic tier's, so a change to the tier does not run the row S-TCOST bisected the tier's cost on
status: open
opened: 2026-09-14
priority: P3
cost: D
---


## Finding

`crates/editor-core/tests/m10_3_r1_probes_interval.rs`'s `gated_to!`
list names `editor-core/src/drive.rs`, `analysis.rs`,
`distribution.rs`, `measure.rs`, `node.rs`, `resolve/`, `eval/`,
`sweep/src/extrude.rs`, `geom-core/src/tolerance.rs`,
`geom-core/src/interval.rs` and the fixtures — and not
`crates/geom-core/src/sym.rs` or `crates/geom-core/src/sym/`. So on a
PR whose diff is the symbolic tier alone (SYM-4, PR 2565, run
34823472408) the gated-suite filter excluded every row of that suite
in all six interval shards (grep of the four shards' full logs for
`the_driven_chamber_replays_bit_identically…`: no hit; the filter
line names the excluded suites), including the chamber drive —
`the_driven_chamber_replays_bit_identically_names_both_wall_flips_and_reports_containment`,
the row S-TCOST bisected the tier's cost on and the one the item
`work/sym/symbolic-tier-costs-95-percent-of-the-m10-3-drive` measures
against (95 % of that row's wall is the tier). The row also repeats
the PARALLEL schedule against itself (D9 across rayon schedules with
the tier's memos in play), which no other row does, so a tier change
that broke schedule-independence would pass this suite's gate.

On the nearest base run (34812106311, a TRIM change) the row ran and
cost 121.96 / 130.98 cpu-s in the `default` / `1e-12` shards; on
SYM-4's run it did not run, so the hosted before/after the item's
charter clause asks for has no after-value for the chamber row, and
the unit's before/after of that row is local only (368 → 225 s).

## What it asks

Add `crates/geom-core/src/sym.rs` and `crates/geom-core/src/sym/` to
the suite's `gated_to!` list — the tier is what the M10-3 drive's
cost and its D9-across-schedules claim rest on — or state why the
driver-side rows (`m10_3_driver_interval`, which did run) are judged
to cover it. The same question applies to the sibling
`m10_3_r2_probes_interval` (its `my_own_drive_is_bit_identical…` row
DID run on SYM-4's PR, so its list differs; the two lists should say
the same thing about the tier).
