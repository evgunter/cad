---
id: m9-3-semantic-residues
kind: issue
title: M9-3 semantic residues - vtxfac/recl tangent-descent divergence, ring-count refusal shadowed, unpinned lane-desync arm
status: open
opened: 2026-08-23
github: 975
refs: [967, 971, 974]
priority: P1
cost: M
---

## From GitHub issue 975

Opened 2026-08-23; 0 comments.

(m9-3 lane)

Residues from the M9-3 unit (PRs #967/#971), collected at the fix-pass adjudication; none blocks the unit.

1. **vtxfac vs recl tangent-descent divergence.** `classify_vertex_on_face`'s coplanar-lump loop still lumps a declared-`Tangent` sector WHOLE-SECTOR through `tangent_lump` (transverse-direction verdict; exact-zero bridges to the Eq. 15.3 ⁻ posture), while `recl_sectors` descends PER BOUND (`tangent_relative_side`; a locus-riding bound stays `On` for the edge engine). The vtxfac site's pierced face is planar and no current fixture reaches a curved tangent sector there; when one does, the per-bound form is the measured-correct one (the whole-record lump misread the tangent-plane degeneracy on the rim fixture) and vtxfac should follow.
2. **Ring-count-mismatch refusal shadowed.** `glue_pair`'s "patch pair carries differing interior-boundary counts" refusal (rest.rs) is not reachable on the natural mismatch fixture: an unmatched interior boundary loses its vertex correspondents first and dies earlier as "seam chord between two isolated pierce points" (R1 NOTE-4) — the refusal text suggests a gate that other refusals shadow. Either construct the configuration that genuinely reaches it or fold the gate's text into the shadowing refusal's.
3. **Unpinned loud-desync arm.** rest.rs's "a seam segment edge did not survive" desync (the non-interior segment-death arm) appears in no test; a mutant that unions the R-interior set too widely would pass the suite. Related: `interior` (the killed-segment set) is unioned ACROSS all glue pairs rather than scoped per pair (R2 n1) — correct today (edge keys are arena-unique and dead keys never revive) but a tighter per-pair scope would make the desync arm's coverage meaningful.
4. **contfp's boundary pre-pass shape.** `contfp` (contain.rs) runs its vertex and edge passes PER LOOP, so on a ringed face an edge-interior hit on an earlier loop can shadow a vertex coincidence on a later loop — against its own stated invariant ("an edge-interior verdict can never shadow an endpoint coincidence"). `curved_boundary_containment` runs all-loops-vertex-first. [Fix-pass outcome to be recorded by the lane: either contfp was fixed with a red-then-green ringed-face row, or the configuration/ripple made it this issue's item.] The two pre-passes duplicate ~50 lines over the same four rows — one shared home when either next moves.
5. **`tangent_locus`'s home.** The DEV-1 witness lane lives in boolean/rest.rs for its consumers' sake; it is geometry, not zip machinery. Post-M9 movement candidate (with #974's circle arm, which would otherwise deepen the wrong-home investment). Do not move mid-milestone.

## Home

Every site named is `crates/topo/src/boolean/rest.rs` or its neighbours in S-MATE's `paths:` territory; the DEV-1 witness lane is its charter's declared-Rest ground.

## Outcome (TANG m9-3 residues lane, 2026-10-01)

1. **Live, not one rule — kept whole-sector, pinned.** A fixture now
   reaches `classify_vertex_on_face`'s declared-`Tangent` arm with a
   CURVED sector (`crates/sweep/tests/m9_3_zip.rs`,
   `a_tangent_curved_sector_on_a_face_lumps_whole`: the quarter round on
   a wide slab). Measured per bound, both of the wall sector's bounds
   read `On` (the ruling exactly; the arc at a 2.1e-8 m arm), and this
   door refuses two consecutive `On` entries — the per-bound form turned
   all three ops into `ClassificationInvariant`, where the whole-sector
   lump answers them right (difference = slab, intersection empty, union
   to the zip). `recl_sectors` can keep an `On` bound because its edge
   engine adjudicates one; this door has none. The reason is stated at
   the site. The 2.1e-8 arm is its own finding:
   `work/hone/an-arc-tangent-to-a-face-at-its-end-is-split-at-the-edge-of-the-band.md`.
2. **Folded.** `HoleCountsDiffer` is gone: a ring-count mismatch is
   holes that do not match one for one, refused as
   `HoleCyclesIncongruent`. Sweeping the family, `HoleVertexUnmatched`
   could not fire at all (the patch flood stops only at seam edges, so
   every ring vertex is a segment end or a paired patch face's outer
   vertex) and is now a lane desync. The family's other
   variants no row reaches: `work/zip/rest-zip-frontier-refusals-reached-by-no-row.md`.
3. **Fixed.** `interior` is scoped per glue pair: `rest::settle_glue`
   checks after each glue that every edge it reported interior is dead
   and every seam edge that died is one it reported, and a unit row
   reaches all three arms (including the "did not survive" desync).
4. **Already fixed** before this lane: `contain::boundary_pre_pass` is
   the one pre-pass `contfp` and `curved_boundary_containment` share,
   all-loops-vertex-first, with its red-then-green ringed-face row
   (`a_ring_vertex_is_never_shadowed_by_an_outer_edge`). Since CONTACT-4
   `contfp`'s walk is `splitting::containment::carrier_loop_side`, which
   trusts that pre-pass and runs none of its own; the single-loop
   `point_in_carrier_loop` reads its boundary through the same
   `LoopEdge::contact`. No duplication remains.
5. **Moved** to `crates/geom-brep/src/locus.rs` (`geom_brep::{TangentLocus,
   TangentLocusError, tangent_locus}`, still re-exported from `topo`).
   Pure move; the rows decide through geom-brep's funnel, which is the
   same `k_stats::decide`.
