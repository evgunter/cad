---
id: kef-and-kfmrh-across-keys-want-a-describing-door-or-reordered-callers
kind: issue
title: kef and kfmrh move loops onto faces on other keys, ten production call sites rely on the move, and their keys-only refusal waits on a design answer
status: open
opened: 2026-10-01
priority: P3
cost: M
design: true
needs_ev: true
refs: [mef-and-mfkrh-onto-a-new-chart-strand-the-edges-they-move, boundary-on-the-new-chart-has-two-homes-in-the-attach-doors, kevs-fan-merge-needs-a-re-describing-kill-door]
---

## What

`mef-and-mfkrh-onto-a-new-chart-strand-the-edges-they-move` gave
`mef`, `mfkrh` and `ring_move` the keys-only re-chart refusals
(`EulerOpError::RechartStrandsDescriptions`, then
`EulerOpError::RechartUnvouched`, from `Body::vouch_move` in
`crates/topo/src/attach.rs`). It did not give them to `kef` /
`kef_minting` (`kef_with`, `crates/topo/src/euler_kill.rs`) or
`kfmrh` / `kfmrh_minting` (`kfmrh_plan`, `crates/topo/src/euler_ring.rs`),
because production callers rely on the move. Both doors still move a
loop onto a face on another key without asking whether its edges'
descriptions name a key their faces wear afterwards, or whether a
certified edge names the key of the face it lands on.

## Measurement

Instrumented at the base of that unit: every call that moves half-edges
onto a face wearing another key, logged with its caller
(`#[track_caller]`) and what the two refusals would have said. Topo's
suite (1872 tests, slow set included) and the `ci` profiles of sweep,
mesh, step-import and editor-core (4739 tests).

| Door | Calls that move across keys | Would refuse | Production sites refusing |
|---|---|---|---|
| `kef` / `kef_minting` | 119,913 | 29,514 | 8 sites, 29,502 calls |
| `kfmrh` / `kfmrh_minting` | 20,370 | 3,041 | 2 sites, 3,010 calls |

The ten production sites, by call count that would refuse:

- `kef_minting`, `boolean/zip.rs` `zip_seam` (two sites): 8,835 + 4,985.
- `kef_minting`, `merge_faces.rs` `merge_group`: 7,947 of 7,966 (the
  cosurface merge across distinct keys is the door's whole purpose).
- `kef`, `chord_join.rs` `cut_core` (the sliver merge): 6,706.
- `kfmrh_minting`, `boolean/zip.rs` `zip_seam`: 2,815.
- `kef_minting`, `sweep/src/blend/surgery.rs`: 770.
- `kfmrh`, `shell.rs`'s rim glue: 195 (then `rename_loop_surface`
  re-describes the ring by hand).
- `kef_minting`, `boolean/rest.rs`, three of its four calls: `slit_zip`'s
  (81) and `zip_folded`'s two (80 + 98). The fourth, in `slit_zip`'s
  band-run arm, kills the face `mfkrh(.., Inherit)` has just minted on
  the folded face's own key, so it never moves across keys. Re-measured
  at the head of PR 3673's fix pass, topo's whole suite and sweep's `ci`
  profile: the same 81, 80 and 98, and no call of the fourth across keys.

The rest are fixtures (`tests/loop_reparenting_pcurve_rows.rs`, sweep's
`verbs_shell.rs`, `verbs_sphsph_chart.rs` and `common/cert_corpus.rs`,
and editor-core's `names/emit_topo.rs` test module, 2).

## Why it is not built

Each production caller strands edges (or leaves them unvouched) for
the length of a composing door that kills or re-describes them
afterwards: the zip kills the seam it fused, the merge and the sliver
cut re-describe what they absorbed, and the shell renames the ring.
Reordering them (attach first: move the dying face onto the surviving
face's key through `Body::set_face_surfaces_describing` before the
kill) is possible in principle at each site, but it reworks the boolean
pipeline's core and the blend, and it re-describes edges the zip is
about to kill. The other answer is a describing twin for each door
(Ev's PR 2527 ruling: a keys-only door refuses; a describing twin takes
a band and the re-descriptions, with no default), which is new public
API. Which of the two, per door, is a design question.

## Pinned

`attach::tests::kef_between_coplanar_faces_on_distinct_keys_strands_unasked`
pins `kef` as it stands: a kef between two coplanar faces on distinct
keys returns `Ok` and tier 3 reports `DescriptionNotAdjacent` on the
remnant's edges. The unit that gives `kef` its refusal flips that row.

## The question

What final state should `kef` and `kfmrh` reach for a move across keys?

Two designers, one round of reconciliation, agree on three parts. Each
stands whatever the answer:

1. **The merge door re-describes what it moves.**
   `merge_coplanar_faces[_declared]` re-describes its kept faces'
   boundaries before it returns, and `describe_minted_edges` loses that
   half of its worklist
   (`merge-coplanar-faces-returns-the-kept-boundary-described-against-the-absorbed-key`).
2. **Transient faces wear "no chart yet".** A face a composing door mints
   and then kills or re-charts before it returns wears the `mvfs`
   placeholder, which tier 3 refuses at rest (`UncertifiableSurface`),
   not a borrowed key. This covers the chord-join slivers, the null face
   and the boolean's section faces.
3. **A scope-close check.** A debug assertion at the outermost
   surgery-scope close that no certified edge names a key neither of its
   faces wears (D2 row 5). It lands after (1).

The split is on the kills themselves:

- **Refuse, with describing twins.** `kef` and `kfmrh` refuse a strand or
  an unvouched move keys-only, through `Body::vouch_move`, with an arm
  under which a chartless destination asks nothing.
  `kef_describing` and `kfmrh_describing` take the re-descriptions under a
  band and absorb the `_minting` twins. No ratified text changes. This
  follows the PR 2527 ruling that chart-relative facts are stated at the
  site.
- **Descriptions name sides, not keys.** An intrinsic description reads
  its two surfaces from its edge's faces, so a strand cannot be
  represented at any door. `set_edge_curve` keeps keys in the spec as the
  caller's claim, checked against adjacency. A chart-changing move leaves
  a certificate that tier 3 re-derives at rest. This edits D2's listing
  of `Intersection { s1, s2, witness }`. Its reach is geom-brep's
  `EdgeDescription` and about 54 spec constructors in 21 files. Until it
  lands, the kills stay as they are, with (3) as the backstop.
