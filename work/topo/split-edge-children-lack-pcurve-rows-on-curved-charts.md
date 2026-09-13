---
id: split-edge-children-lack-pcurve-rows-on-curved-charts
kind: issue
title: Body::split_edge mints no pcurve cache rows for its children — a split on a curved chart leaves the body tier-3 invalid until mint_pcurves runs
status: open
opened: 2026-09-08
---


Found by SHELL-7's implementer lane (PR #2200) and confirmed by both
its reviewers by execution; placed here by the SHELL orchestrator
(`crates/topo/src/split.rs` is TOPO's).

`Body::split_edge` splits an edge at a parameter and mints the two
children with the parent's carrier, but writes no pcurve cache rows
for the children's half-edges. On a CURVED chart the body is then
tier-3 invalid — `validate_geometric` reports `Pcurve MissingCache`
for each child — until a caller runs `topo::mint_pcurves`; measured on
the drum's cylinder seam split at mid-height (`shell` on the split
operand refuses `NotValid` with the two missing caches). The gap is
conditional on a child half-edge landing on a curved chart: the
wedge's AXIS edge between two planar meridian caps splits and stays
tier-3 valid (R2's measurement), which is why SHELL-7's own
no-profile-constraint row can split that edge and never mint.

`split.rs`'s module docs carry a "Tier-3 caveat (review F2)" section
naming the iso-rectangle case and not this one, which has the same
shape and an obvious home beside it. What closing this needs is either
the op minting the children's rows (it has the parent's, so the
parameter split of a cached pcurve is the natural mint) or the caveat
stated at the op with `mint_pcurves` named as the caller's step.
Signed (SHELL orchestrator).
