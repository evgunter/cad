---
id: the-germ-loci-re-read-whether-an-edge-runs-in-a-partner-face-by-midpoint
kind: issue
title: The classification's germ loci re-read whether an edge runs in a partner's face by midpoint, beside the sweep's edge-face record
status: open
opened: 2026-11-01
priority: P3
cost: M
---

Found by PARTING's `parting/sweep-records-edge-in-face` sweep for sites
that re-derive an edge-in-face verdict from values (2026-11-01). Read,
not measured.

**What.** `boolean::sectors::germ_loci` (`crates/topo/src/boolean/sectors.rs`)
places two germs running along one another by asking whether each
edge runs inside the partner's face: `runs_in` decides an open edge's
midpoint on the face's plane and inside its trim (`on_planar_face`),
and `closed_reach` reads a closed circle's midpoint against each planar
face around the partner's site. Both rest on "the sweep splits an edge
at every crossing of the partner, so the midpoint speaks for the whole
edge", which is the sweep's own verdict: for a line edge lying in a
plane face the sweep now decides it and records an edge-face record per
piece the face holds (`ReductionContacts::a_in_b` / `b_in_a`, written by
`reduce::settle_in_plane`). The germ reading decides it again from
values, outside `[coincidence.door]`.

**What is owed.** For a line in a plane, read `runs_in` off the
reduction's edge-face records rather than the midpoint. A circle
(`closed_reach`) and an arc have no record yet: the sweep keeps a conic
lying in a plane out until the census reads that record
(`work/contacthold/an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane.md`),
so those keep their reading until then. `sectors.rs` is shared ground
(CLEAVE's PR 6177 edits it); filed here because it is the parting's
question.
