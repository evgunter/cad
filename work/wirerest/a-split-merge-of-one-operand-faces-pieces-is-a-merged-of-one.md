---
id: a-split-merge-of-one-operand-faces-pieces-is-a-merged-of-one
kind: issue
title: A split half's merge that rejoins one operand face's pieces across its own seam publishes Merged([face]) under the split, which N3's Merged-of-one rule says is the face
status: open
opened: 2026-10-20
priority: P3
cost: E
design: true
---


Found by WIRE's section-face lane (2026-10-20), measured on
`split_pinch_keeps::a_ring_cut_through_its_axis_ties_its_two_sections`:
a square ring (a square revolved a full turn about `y`) cut by `x = 0`.
In each half, the two planar annuli are each one face, a piece of the
operand annulus that the half's merge (`SplitNaming::merge_groups`)
joined across the annulus's own seam. `name_split_merges`
(`crates/editor-core/src/names/emit_topo.rs`) names it
`Merged([Band(annulus)])` under the split node; `canonical::minted`
collapses a `Merged` of one only when the element was minted by the same
node (`canonical.rs`, the one-element rule), so the singleton stays.

N3 says "A `Merged` whose set has one element is that element". Under
that rule the face would be spelled as a piece of the annulus, the way
an unmerged piece is (`SplitFragment { parent }`, qualified by `Keeps`
if the half holds several). Not measured: whether one face is spelled
both ways across an edit that changes whether the merge runs.

Question: should a split's linked set with one operand face and no
plane be named as that operand face's piece (`split_base`), with
`Keeps` among several? The test above reads the wall through the
singleton today (`parent_band`), so it re-baselines with the fix.

Signed (WIRE, section-face lane).

## Moved to WIREREST (2026-10-20)

WIRE measured 69 budget points against 30. This row is P3 or P4, and WIRETAIL already stood at 25 of 30, so it moved by `git mv` (id and body unchanged) to WIRE's second tail. Signed (WIRE orchestrator).
