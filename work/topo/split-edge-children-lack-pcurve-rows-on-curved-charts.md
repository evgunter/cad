---
id: split-edge-children-lack-pcurve-rows-on-curved-charts
kind: unit
title: Body::split_edge mints no pcurve cache rows for its children — a split on a curved chart leaves the body tier-3 invalid until mint_pcurves runs
status: closed
opened: 2026-09-08
pr: 2531
branch: topo/split-edge-pcurve-rows
closed: 2026-09-14
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

## Brief (TOPO, 2026-09-13) — block TOPO-B2 slot 1, dual at review

**The answer to give.** After `Body::split_edge` on an edge whose
half-edges carry pcurve cache rows, the two children carry rows too,
and a body that was tier-3 valid before the split is tier-3 valid
after it. Phase 1 decides between the two closings the row names and
says why: (a) the op mints the children's rows from the parent's — a
cached pcurve split at the parameter is the natural mint (the parent's
row IS the description; the split of a fitted pcurve at `t` is
exact where the pcurve is a line in the chart and a re-fit where it
is not — say which, per pcurve kind); or (b) the op cannot honestly
mint (it lacks the tolerance or the lane bound `mint_pcurves_of`
needs — `split_edge<T: Real>` takes `Tol` today; check what
`PcurveFittedLane` requires) and the caveat is stated at the op with
`mint_pcurves_of(body, &[the two faces], tol)` named as the caller's
step, beside the existing "Tier-3 caveat (review F2)" section. (a) is
the answer the row wants; (b) is acceptable only with the bound
mismatch shown.

**Red-first row.** SHELL-7's measurement rebuilt as a fixture: the
drum's cylinder seam split at mid-height — `validate_geometric`
reports `Pcurve MissingCache` for each child on the merge base (assert
it), and the head is tier-3 valid after the split with no caller
mint. Control: the wedge's AXIS edge between two planar meridian caps
(R2's measurement) — split, tier-3 valid on both trees. A third row
pins the minted rows' content: the children's pcurves re-certify
against the same band the parent's did.

**Seams.** `crates/topo/src/pcurves.rs` is TRIM's: read `mint_pcurves`,
`mint_pcurves_of` and the row types end to end; if (a) needs a
helper that splits one cached row at a parameter, it lands in
`pcurves.rs` by announced seam (one function, its doc, its rows),
and the PR says so. `split.rs`'s module docs gain the closing's
statement; the "Tier-3 caveat" section names this case beside the
iso-rectangle one either way.

**Class receipt.** Every operator that mints half-edges from a parent
carrying rows: `split_edge` is the row's subject; list the others
(`mev`/`mef` on a described edge? the merge door? `ring_move`?) with
whether each mints, inherits, or leaves rows missing, and file what
you find on the owning slate per discipline §6.

Branch `topo/split-edge-pcurve-rows`. PR title: "TOPO: split_edge's
children carry their pcurve rows". Do not close the item; the dual
runs at review.
