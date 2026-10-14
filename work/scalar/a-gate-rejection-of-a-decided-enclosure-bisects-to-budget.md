---
id: a-gate-rejection-of-a-decided-enclosure-bisects-to-budget
kind: issue
title: drive: a gate's rejection of a DECIDED enclosure reads as Bisect, though no sub-box can change its sign
status: open
opened: 2026-10-03
priority: P3
cost: E
---

Found by CLEAVE (the geom-core doors step of
`work/cleave/topo-mints-indeterminates-outside-the-funnel.md`, branch
`cleave/mints-doors`). Read from the code, not measured on a driven
document.

`geom_core::k_stats`'s gate doors (`decide_positive`, `decide_negative`,
`decide_nonzero`) escalate a DEFINITE sign the caller's question cannot
use, and since that step the escalation carries the margin the
classifier decided: at `Interval`, an enclosure wholly within the zero
band, or wholly past `±escalate`. Interval enclosures shrink
monotonically, so every sub-box decides the same sign and the gate
rejects it again.

`editor_core::drive`'s leaf read does not see that. `log_read` hands
the first escalation to `indeterminate`, which reads only
`Indeterminate::terminal_sliver`; the gate doors mint that `false`
(`k_stats::classify_gated`), so the leaf answers `Bisect`, and the
driver splits until its depth or resolution budget is spent and then
prices the region as a budget refusal rather than naming the decided
rejection. The error-enum arm (`NodeErrorKind::Escalated`) reads the
same way.

The fix is a reading at one of two homes: the gate doors set a flag
the driver reads (as `terminal_sliver` is set by the classifier), or
`indeterminate` treats a decided reading as terminal. Either one names
a refusal reason other than `SliverTerminal`, whose word is wrong for a
decided zero. Not measured: no document is known to drive a gate
rejection at a leaf; a probe through `drive` over a box whose leaf
rejects a decided zero arm (`sector_arm` on a collapsed chord) would
measure it.

## Re-homed from FLUX to SCALAR (2026-10-10)

(FLUX orchestrator) FLUX measured 125.5 budget points against 30 and was cut on its priority seam: FLUX kept the curved closed-form arms. SCALAR collects the scalar doors and the readers over them: the certified-door family, the margin and recourse readers, the unit-direction doors, and the box driver's readings. The id and the body above are unchanged; the move may have set `priority`, `cost` or `status` in the header.

## Two more doors mint onto the log (CLEAVE, branch `cleave/split-straddles-carry-reading`)

- `k_stats::gate_rejection` is now the one mint of a gate's rejection.
  The gate doors call it, and so does a site that decides a schedule
  and refuses once after it. Its first reachable caller is `topo`'s
  join frame (`split_join_frame_arm`, every member decided Zero), which
  adds a decided rejection to this row's population.
- `k_stats::straddle` records two decided bounds' enclosure with
  `terminal_sliver: false`, so the driver reads it as `Bisect`. That is
  D9 row 1's posture for a straddling enclosure. Where the two bounds
  are slack in the geometry and not in the box (an ellipse bending
  tighter than the band), no sub-box closes the gap, which is this
  row's shape again.

Not measured through `drive`.
