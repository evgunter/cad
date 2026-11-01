---
id: pair-classes-falls-back-to-per-pair-rows-beside-a-partner-that-reads-none
kind: issue
title: A vertex in pairs alone beside a partner whose polygon cone reads None keeps main's per-pair rows, silently
status: closed
opened: 2026-10-08
priority: P2
cost: M
refs: [pairs-beside-an-unread-partner-keep-mains-rows]
closed: 2026-10-08
---



Filed by PR 4289's second review (m3), as a silent path the PR discloses.

## What

`crates/topo/src/boolean/vtxfac.rs` `pair_classes` layers a vertex's
partners where every one reads. A partner's polygon cone
(`sectors::cone_read`) reads `None` wherever some edge of the vertex
has no reference whose every reading is decided, and no reference
escalated. Beside such a partner, `pair_classes` keeps each pair's own
rows, as main did for every non-convex partner. Those rows read each
partner alone, so they can be wrong where the unread partner holds the
edge. Nothing refuses, and naming takes the rows as given.

`None` is reachable by valid geometry. The exact-oracle fuzz of PR 4289
reads about 710 `None` per seed of 918 cones at ε = 1e-9 (about 2 090
at 1e-6). Almost all are cones with two faces folded onto two others
within the band (a dart notched to within 1e-8 of its tip), but some
are near-flat saddles and random stars.

The symmetric saddle loses a decidable reading this way (review 2's
probe 34: the fuzz's `saddle0.4` cone, the direction opposite a bound
across its plane, 0.65 rad from the link, exactly `Out`). The
direction lies on the planes of the two faces it faces away from, so
only the other two faces give references; and each of those
references lies on the opposite face's plane, so its arc runs along
that plane and no crossing point is located. Both references are
passed over. The arc misses that face's sector, so a reading of an
arc lying in a face's plane would decide it.

No row of the topo suite reaches a pair beside a `None` partner: an
instrumented run counts none.

## The shape to give

Either refuse typed where a partner reads nothing in pairs alone (as a
touch does, `touch_classes`), or read more references so that fewer
cones end at `None`. The first is the K-funnel's answer, but it widens
refusals where main built, so measure first which cells reach it.

## Closed

Both shapes, in that order. **Fewer cones read nothing**: where a
rule of `sectors::cone_side` would pass a reference over, a face is
read as not crossed if the arc and its sector lie decidedly apart in
its plane (`sectors::apart`). Wherever the arc meets the plane, it
meets it in the wedge its ends span there, so a line through a bound
of the sector or of the arc that has the other wedge strictly on its
far side decides it. Each reading is a sine levered at the least joint
deviation of its two points. The symmetric saddle's probe 34 reads
`Out`. The exact-oracle fuzz (seeds 1 and 4, effort 10) reads
`cone_side` `None` 0 and 0 times at ε 1e-9, where main read 244 and
262, and 7 and 2 at 1e-6, where main read 1 649 and 1 591. It still
reads 0 wrong.

**What still reads nothing refuses**: `vtxfac::pair_classes` refuses
`VertexReadTwice`, naming two partners, where one reads nothing beside
another, against the vertex's edges or against the other's. One pair
alone keeps its rows, as before.

Measured on main: no row of the topo, sweep or editor-core suite
reaches a pair beside an unread partner, so the refusal refuses no cell
main built there. The germ oracle's two new scenes, a pyramid opposite
the saddle's valley, bare and beside an arch, reach it on main in 30
cells, and 60 of their 300 rows are wrong. They now read 0 wrong and 0
missing. `sectors::cone_fuzz`'s pair oracle asserts that no row is kept
beside a partner that reads nothing.

The layering's other fallbacks (an edge left undecided, the outermost
disagreeing) are filed as
`pair-classes-keeps-per-pair-rows-where-its-layering-is-undecided`.
