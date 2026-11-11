---
id: the-in-plane-settle-reads-a-short-piece-by-its-midpoint-where-no-end-reads-at-a-corner
kind: issue
title: The in-plane settle reads a piece shorter than 2Kε by its midpoint, in band of its own ends, where no end reads at a corner
status: open
opened: 2026-11-10
priority: P3
cost: M
refs: [main-red-a-needle-cavity-12eps-channel-escalates-at-bool-contact-vertex, the-germ-loci-re-read-whether-an-edge-runs-in-a-partner-face-by-midpoint]
---

Found by FLUSH's `flush/needle-12eps` lane (2026-11-10) while fixing
`work/flush/main-red-a-needle-cavity-12eps-channel-escalates-at-bool-contact-vertex.md`.
Read, not measured: no pose on main is known to reach it.

**What.** `boolean::reduce::settle_in_plane`
(`crates/topo/src/boolean/reduce.rs`) decides whether a planar face
holds a piece of a line edge lying in its plane. An end recorded inside
the face holds the piece. Failing that, a piece whose midpoint lies in
band of its own end (`bool_contact_vertex`) reads its germ at an end
tied to exactly one corner of the face (`held_at_corners`, the corner's
shape from `geom_brep::sector_shape`). Failing both, the piece's
midpoint goes to `contfp_decided`. A piece between `2ε` and `2Kε` long
has its midpoint in band of its ends, so where it reaches the midpoint
read the boundary pre-pass's `bool_contact_vertex` refuses on the
piece's own length, a thinness read the pairs do not make
(`[coincidence.booleans]`).

The corner read returns nothing, and the midpoint is read, where:
- no end is tied to a corner of the face;
- an end is tied to two of the face's vertices (a slit);
- a corner has more than one boundary edge in or out (a pinch);
- an edge at the corner is curved;
- the boundary folds back on itself at the corner, which
  `sector_shape` refuses as a spike;
- a sign falls in band;
- the two ends disagree (a crossing within the band the sweep did not
  split).

A piece shorter than about `2ε` has its midpoint read Zero against its
end, so the guard passes it to the midpoint read too. Its two ends are
then an in-band pair, which no operand at rest holds unless a split
minted one, so this case is the split's question as much as the
settle's.

**What is owed.** Read the remaining cases off the records or a germ,
as `held_at_corners` does for a line corner, or show that the sweep
leaves none of them a piece that short.
