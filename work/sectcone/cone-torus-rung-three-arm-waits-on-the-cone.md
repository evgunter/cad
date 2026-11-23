---
id: cone-torus-rung-three-arm-waits-on-the-cone
kind: issue
title: Cone × torus routes to rung 3 and refuses: the torus half is built, the cone's meters conversion is not
status: closed
opened: 2026-10-27
priority: P0
cost: M
refs: [torus-certificate-runs-root-free-through-its-quartic]
closed: 2026-11-23
pr: 7395
---

## Closed (2026-11-23, PR 7395)

Cone × torus off the axis has its rung-3 door: `the-cone-torus-pair-door`, built on the cone's limb-2 reading at the search (PR 7350) and the tube's cone arm (PR 7358). The apex node and touch still refuse, because the ℝ³ march has no node end (`work/ssiedge/an-r3-tube-chain-has-no-node-end.md`).

Filed by SECT's `sect-toruscert` lane (branch `sect/torus-certificate`),
which retired sphere × torus at rung 3 and left this pair's arm unretired.

## What already exists

The torus's halves of the ℝ³ certificate are built and pair-independent:
`crates/geom-brep/src/ssi/enclose.rs` `implicit_enclosure` and
`implicit_gradient_enclosure` (the mean-value form of `g = F/(4rA)` and
its exact gradient, `Quartic`), limb 2's one reader
(`pcurve_cache::limb2_offset_sup` at `Limb2Locus::Carrier`, whose
`projected::Meters::Torus` converts per part), and
`has_implicit_enclosure` lists the torus. `ssi::sphere_torus_ssi` is the pattern: a pair gate, the pair's
own tangency decision before any rung, then `implicit_pair_r3`.

## Measured

`crates/geom-brep/src/intersect.rs` `route(Cone, Torus)` answers
`Rung::General`, `implemented: false`, its note naming the uniqueness
tube's cone arm: `implicit_enclosure` and `implicit_gradient_enclosure`
still refuse the cone. Limb 2 reads it (2026-11-22, the search reads
through the edge certificate's reader, the double cone at the carrier
locus).

## What the taker owes

The cone's own units first (the tube's cone arm, and what the cone
stores), then a `cone_torus_ssi` door with the pair's
tangency decision and a measured certified pose, then `route` flipped.

Moved from SECTRUNG to SECT on 2026-11-02, by `git mv`, when SECTRUNG merged back into SECT: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so SECT and the three tracks cut from it (SECTTAIL, SECTRUNG, SECTSITE) came in under one budget.

## Re-banded P0 and parked (2026-11-03)

Lily wall 2 (the lantern welded onto the arch) stops on this pair:
`demos/tour/src/lily.rs` `wall_probes` pins `CurvedPairUnsupported {
site: InteriorLoopGuard, kind: Cone, other_kind: Torus }`. The section
certificate cannot clear the neck cone against the arch's torus wall,
which is not coaxial with it, and this row's `route(Cone, Torus)` arm
is what it lacks. Welding a cone onto a curved tube is a union on
everyday geometry, so the join is P0 with the other demo joins
(`work/splice/curved-demo-joins-build.md`), and this row inherits the
band. "The cone's own unit first" is now its own row,
`the-cone-has-no-implicit-enclosure-or-metres-conversion`, and this row
is parked on it.
