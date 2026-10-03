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
