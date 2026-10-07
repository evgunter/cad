---
id: blend-reach-skips-every-face-at-a-chain-vertex
kind: issue
title: blend: the reach meter skips every face at a chain vertex, so an end face that folds back into the band away from the vertex is never metered
status: open
opened: 2026-10-06
priority: P2
cost: M
---


Found by the lane that built predicate 2's reach
(`blend-material-is-never-checked-against-faces-that-are-not-its-supports`).

`band_reach` in `crates/sweep/src/blend/reach.rs` (`excluded`) skips, for
each chain, its supports and every face meeting one of its vertices. A
face at a chain end is the face the band runs into (a cut-off's end face,
a mitre's other support, a ruled cap), and it touches the band's region
at the vertex by construction, so metering it whole would refuse every
open chain. But the skip is by face, not by neighbourhood: an end face
that is not convex — an L-shaped end plate whose other arm overhangs the
band further along, or a curved end face that comes back — can meet the
band's material away from the vertex, and nothing meters that.

The fix is to meter such a face against the reach with the vertex's own
neighbourhood (the end section, which the end face's plane already bounds
in the reach — `straight_reach`'s caps) taken out, rather than not at all.
No body on main is known to reach it; a witness should be built first.
