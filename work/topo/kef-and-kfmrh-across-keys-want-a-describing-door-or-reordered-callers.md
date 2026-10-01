---
id: kef-and-kfmrh-across-keys-want-a-describing-door-or-reordered-callers
kind: issue
title: kef and kfmrh move loops onto faces on other keys, eleven production call sites rely on the move, and their keys-only refusal waits on a design answer
status: open
opened: 2026-10-01
priority: P3
cost: M
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
| `kef` / `kef_minting` | 119,913 | 29,514 | 9 sites, 29,504 calls |
| `kfmrh` / `kfmrh_minting` | 20,370 | 3,041 | 2 sites, 3,010 calls |

The production sites, by call count that would refuse:

- `kef_minting`, `boolean/zip.rs` `zip_seam` (two sites): 8,835 + 4,985.
- `kef_minting`, `merge_faces.rs` `merge_group`: 7,947 of 7,966 (the
  cosurface merge across distinct keys is the door's whole purpose).
- `kef`, `chord_join.rs` `cut_core` (the sliver merge): 6,706.
- `kfmrh_minting`, `boolean/zip.rs` `zip_seam`: 2,815.
- `kef_minting`, `sweep/src/blend/surgery.rs`: 770.
- `kfmrh`, `shell.rs`'s rim glue: 195 (then `rename_loop_surface`
  re-describes the ring by hand).
- `kef_minting`, `boolean/rest.rs` (three sites): 81 + 80 + 98.
- `kef`, `editor-core/src/names/emit_topo.rs`: 2.

The rest are fixtures (`tests/loop_reparenting_pcurve_rows.rs`, sweep's
`verbs_shell.rs`, `verbs_sphsph_chart.rs` and `common/cert_corpus.rs`).

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
