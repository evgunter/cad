---
id: rest-zip-frontier-refusals-reached-by-no-row
kind: issue
title: "zip: none of the rest zip's fifteen typed sub-frontier refusals is reached by any row, so which are gates and which are dead is unmeasured"
status: open
opened: 2026-10-01
priority: P3
cost: M
---

## Finding

`topo::RestZipFrontier` (`crates/topo/src/boolean/refusal_routes.rs`)
names fifteen sub-frontiers the declared-REST zip
(`crates/topo/src/boolean/rest.rs`) refuses at. One is reached by an
end-to-end fixture: `ChordBetweenIsolatedPierces`
(`crates/topo/tests/seat3_flush_detector.rs`). The other fourteen are
constructed only as values, by the refusal-text rows in
`boolean/mod.rs` (`every_rest_zip_frontier_ends_in_its_own_lever_and_no_declaration`
and the display table), which assert their wording and reach none of
their sites:

- seam realization (`realize_seam`, `mint_chord`):
  `ChordMefRefused`, `ChordMekrRefused`,
  `PierceRingMekrRefused`, `ChordEndpointAbsent`,
  `ChordEndpointRevisited`;
- patch pairing (`pair_patches`): `PatchVertexUnmatched`,
  `PatchCyclesIncongruent`;
- rings (`glue_pair`): `HoleVertexUnmatched`, `HoleCyclesIncongruent`;
- the slit zip (`slit_zip`, `zip_folded`): `SlitFaceHoles`,
  `WholeBoundaryShared`, `RunVertexBranches`, `BandRunOffLoops`
  (also `work/zip/slit-zip-band-run-across-two-loops-is-reached-by-no-row.md`),
  `FoldVertexFused`.

The TANG m9-3 residues unit (PR 3747) measured one family of these. The
ring-count refusal (`HoleCountsDiffer`) was shadowed by
`ChordBetweenIsolatedPierces` on the natural mismatch (a hole on one side
only leaves isolated pierce points on the other) and is folded into
`HoleCyclesIncongruent`. The ring-vertex refusal (`HoleVertexUnmatched`)
cannot fire on a plane (the patch flood stops only at seam edges, so a
ring vertex is a segment end or, by Jordan, a paired patch face's outer
vertex), but can on a periodic carrier, where a cylinder band's outer and
ring circles are designations only: two stacked bands mated against a
band split at another height leave a ring bordering a neighbour's RING.
It stays typed; that fixture is its first owed row (`glue_pair`'s doc).

Some are plausibly reachable (`PatchVertexUnmatched`: two operands whose
contact faces are split by internal edges at different interior
vertices, equal patch counts); some may be dead by an invariant, as
`HoleVertexUnmatched` is on a plane; the Euler-operator ones wrap refusals the
operators may never give on a well-formed seam
(`work/zip/rest-zip-drops-the-euler-operators-refusal.md`).

**The ring pairing is a copy that has diverged.** `glue_pair`'s loop
that pairs the promoted rings says it runs "the same test the patch
pairing ran on the outers" (`pair_patches`), and is a hand copy of it:
the same mapped-cycle walk, the same antiparallel index arithmetic.
Two copies of one rule drift: they already refuse through different
variants and differ in what they check before the walk. One helper
both call would keep the congruence test one rule. Recorded from
PR 3747's style review (S2).

**No row has a glue consume a seam segment.** `try_rest_union` keeps
the seam edges a glue kills as interior (`settle_glue`), for "an
already-fused run the glue consumed", whose example used to be the
meridian seams of a closed cosurface band. Instrumented over all 3826
topo and sweep rows (PR 3747), no real glue killed a seam segment edge;
the two-peg union's cylinder bands do not. A fixture that does belongs
with the rows above.

**What would close it.** Per variant: a fixture that reaches it,
asserted end to end; or the invariant that makes it unreachable, stated
at the site, with the refusal turned into a desync; or, where an
earlier refusal shadows it on the natural fixture, its text folded into
the one a user actually meets.

Filed from the TANG m9-3 residues unit.

## Now sixteen (JOIN-1, PR 3790)

`ChordBetweenIsolatedPierces` lost its fixture: the stepped bricks
(`seat3_flush_detector`) build in the chord join since JOIN-1, so the
zip never runs on them, and the row now pins the build
(`a_declared_report_is_a_set_and_the_whole_set_builds`). Its
`NOT_YET_ENDING` text check went with it; the display table in
`boolean/mod.rs` still asserts the wording. No other reaching pose was
found: on JOIN-1's fix-pass-2 head, R1's reflex, declared, tube, seam and
bored-capsule batteries (`crates/sweep/tests/join1_r1_probes.rs`; 1152,
27000, 6900, 12150 and 2400 poses) reach no `RestZipUnsupported` at
all; on main 0abf909cb the reflex battery reached it on 8 poses
(`ChordBetweenIsolatedPierces` among them, `sqQ2` unions), which the
join now builds or refuses before the zip runs. All sixteen sub-frontiers
are reached by no row.

## Now seventeen (JOIN-2, PR 3880)

The zip reads the join's segments and realizes them outward from the
contact faces' boundary (`realize_seam`). Segments left whose ends are
all pierce-ring vertices joined to nothing refuse there as
`SegmentsBetweenIsolatedPierces`; `ChordBetweenIsolatedPierces` stays
`mint_chord`'s, reachable from `mirror_edges`. Neither is reached by the
topo or sweep suites, JOIN-2's reviewers' batteries
(`join2_r1_probes`, `join2_r2_probes`, `join2_d_probes`) or R1's grid.
