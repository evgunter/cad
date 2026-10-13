---
id: a-sliver-arc-beside-a-joinable-vertex-refuses-the-boolean
kind: issue
title: A curved arc a few ε long beside a joinable vertex refuses the whole boolean JoinUndecided, where main built the body
status: closed
opened: 2026-10-07
priority: P3
cost: E
closed: 2026-10-13
---

## The finding

`joinable` reads the surface pair's dihedral at a valence-2 vertex
levered over the shorter of its two edges' extents
(`crates/topo/src/boolean/edge_join.rs:353`, `classify_dihedral` at
`:367`), so the reading does not depend on which edge the arena lists
first. A curved arc only a few ε long beside an otherwise joinable
vertex shrinks that lever until the wedge reads in the band, and the
whole boolean refuses `BooleanError::JoinUndecided`, where main, which
did not join curved vertices, built the body and left the vertex. The
ruling allows it (a reading in the band refuses typed), and the offer
site says so (`offer_rows::EDGE_JOIN_SITE`). No `ci` row reaches it.

## What it needs

A lever that does not shrink with the sliver: the longer edge's
extent, or the locus's own curvature scale at the vertex, so a short
arc beside a transverse pair still decides. Whichever is chosen must
stay independent of which edge the arena lists first.

## 2026-10-13: closed, the lever is the joined edge (FUSE, `fuse/join-lever-is-the-joined-edge`)

The premise does not hold on certified input: a piece a few ε long
refuses its own certification where it would be minted, so the join
never meets one. Pinned by `crates/sweep/tests/join_lever_is_the_joined_edge.rs`
on the D prism ∩ `brick((c, 4), (−4, 4), (−3, 3))`, `c = √(1 − sin²θ)`,
whose vertical line edge where `x = c` meets the cylinder has the
pair's wedge `sin θ`.

- **The line edge, at `sin θ` = 0.5, 0.1 and 0.02.** A station `s`
  from the edge's end, with `sin θ · s = 0.95 Kε`, refuses at the cut:
  `Certification { Escalated { Transversality } }`. One at
  `1.05 Kε` builds, `join_edges` joins 1, and tiers 3 and 3′ pass.
  Calibrated before pinning: the cut refuses up to `sin θ · s = Kε`
  and builds from `1.01 Kε` at all three angles.
- **The cap arc, at `sin θ = 1`.** A station at `9.5ε` refuses at the
  split (`SplitParamEscalated`, the parameter in the band of the arc's
  end). One at `10.5ε` joins 1 and passes tiers 3 and 3′. It builds from
  `10ε` on.

`joinable`'s lever (`crates/topo/src/boolean/edge_join.rs`) is now the
scale of the edge the join makes: the larger of the two edges'
extents, which bounds the joined edge's extent from below and does not
depend on which edge the arena lists first. Before, it was the smaller
extent. The exact joined extent was not cheap here: the two edges carry
separately certified curves with no shared parameter range, so their
union's extent would need the joined edge's own spec.

`offer_rows::EDGE_JOIN_SITE`'s comment no longer claims the few-ε-arc
scenario. The reachable in-band case is a vertex near a crossing of the
locus, where the pair itself nearly turns tangent.

**Mutant.** Reverting `max` → `min` keeps every row green: the new two,
and 323 join, edge-join and station rows across topo and sweep `ci`.
Past the certification line, the short piece's own margin already
reads transverse, so the lever changes no outcome on certified input.
This is a cleanup of meaning, not of outcome.
