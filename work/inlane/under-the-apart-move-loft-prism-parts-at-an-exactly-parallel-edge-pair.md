---
id: under-the-apart-move-loft-prism-parts-at-an-exactly-parallel-edge-pair
kind: issue
title: Under the apart move, loft_prism parts at an edge-pair census decision whose margin is exactly zero at the nominal
status: open
opened: 2026-11-02
---


Found by `reads/loft-section-ids` (REFS: every loft section carries a
minted `SectionId`, and the loft's names and content key spell it) through
`revalue_corpus::revalue_loft_prism`
(`crates/editor-core/tests/revalue_corpus.rs`).

**What happens.** Before that branch the row compared `loft_prism` whole
under `Move::Apart` (floor 259). With it, the `p0` and `p1` builds part
under `Apart`: a prefix comparison reads 242 decisions in step, 152
moved, and then flips `pm_census_ee_parallel` at decision 242, whose
margin is exactly `0.0` at the nominal and `2.02e-8` at `p1`; the builds
part there (`parted: (242, 1539, "pm_census_span_precision")`). In step
they part nowhere: 1539 compared, 857 moved, no flip.

The branch changes no geometry and no variable: the variables the move
offsets are the same, in the same order. What it changes is the minted
ids after the loft (its section ids, then its output variable), so
whatever orders the census's edge pairs by an id or a name now reaches
an exactly-parallel pair (two of the prism's walls' edges) at a point
where the apart move tips it. The zero margin is the defect; the id
change only exposed it.

**Pinned.** `revalue_loft_prism` leaves the `rows!` macro for an
explicit test under `pinned_red::row`, stopping on "compared under
another move than its pin" with `mode: Some(InStep)`, as
`revalue_heat_sink_fins` does.

**What closing it takes.** Find the edge pair the zero-margin decision
reads, and decide whether an exact parallel at the nominal should reach
a sign at all; then the row compares under `Apart` again and its floor
and owed flips are re-measured.
