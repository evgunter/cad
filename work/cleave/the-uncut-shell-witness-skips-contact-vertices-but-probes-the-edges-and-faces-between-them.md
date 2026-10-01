---
id: the-uncut-shell-witness-skips-contact-vertices-but-probes-the-edges-and-faces-between-them
kind: issue
title: The uncut-shell witness skips contact vertices at tier 1 but probes the edges and faces between them, so a declared contact the reduction recorded ON the boundary can be re-asked of the geometry
status: open
opened: 2026-10-01
priority: P2
cost: M
---



## What

`crates/topo/src/boolean/shell_witness.rs`, `shell_side`: tier 1 skips
every vertex in `contact_skip_set`. That set holds the reduction's
contact records (`ContactRecords::vv`, `a_on_b` / `b_on_a`), which are
the vertices the reduction recorded ON the other operand's boundary,
by geometry or by declaration. Tiers 2 and 3 (edge midpoints and
certified face-interior points) have no record to read, so they probe
the edges and faces that run between contact vertices.

The skip has been in place since the boolean's first core
(`898f114c6`). No commit, doc or comment gives a reason for it. The
reading in the module doc is that a recorded-ON point must not be asked
of the geometry again. The case where that matters is a declared Rest
pair whose carriers differ by less than the band. The reduction treats
those carriers as one, but `point_in_solid` knows nothing of the
declaration. On such a pair a tier-2 or tier-3 point on the declared
face would answer `Escalated`, and that error propagates before a
decisive point elsewhere on the shell is reached.

## Measured (CLEAVE, 2026-10-01, PR 3655)

On the suite's flush fixtures (`contained_flush_witness`,
`probe_subtract_notch_rests_on_b`), tier 1 met 534 skipped vertices and
probed 11. In `r4tri`, every vertex of `b` is a contact vertex. So the
same proxy at tiers 2 and 3 (skip an edge or face built only from
contact vertices) would skip `b`'s end faces and undo the fix. The local
answer is not a vertex proxy. The in-band declared-pair case has not
been built.

## What a fix decides

What record says that an edge or face is ON the other boundary.
Candidates are the declared face pairs (`BooleanDeclarations::coincident_faces`,
which the fallback has and `setopfinish` does not), and the curve and
patch contact records (`ContactRecords::curves` / `patches`, the latter
not certifiable yet). The other option is to treat an in-band
`point_in_solid` answer at tiers 2 and 3 as inconclusive and move to the
next witness, the way `certified_in_face` treats an in-band certificate.
