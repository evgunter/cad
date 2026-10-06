---
id: kef-and-kfmrh-across-keys-want-a-describing-door-or-reordered-callers
kind: issue
title: kef and kfmrh move loops onto faces on other keys, ten production call sites rely on the move, and their keys-only refusal waits on a design answer
status: open
opened: 2026-10-01
priority: P3
cost: M
design: true
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

## Ruled (PR 3970, 2026-10-06)

Ev ruled over five design rounds; each round is recorded in `docs/DESIGN-FORK-LOG.md` row 76. The rulings in Ev's words:

1. **The kills' final state is B, built directly.** `kef` and `kfmrh` refuse a strand or an unvouched move keys-only, through `Body::vouch_move`. Describing twins take the move with its re-descriptions. `Loop.face`, `HalfEdge.parent_loop`, `Face.surface` and `Edge.curve` become private to one module that owns the vouch, so the compiler confines writes. There is no interim gate. (2026-10-04: "the final state should be B, and i also think it'd be best to go directly there rather than by way of A".)
2. **The naming check is in tier 1**: a certified edge's description names surfaces its faces wear. (2026-10-06: "i agree with the recommendation on choice 2".)
3. **Restatement is derived, in outline.** One restatement function re-expresses each moved edge's description on the keys its faces now wear. It chooses the edge's kind by one shared predicate (tier 3 check 4's reading) and re-certifies. The twins call it themselves. Callers list only what the door cannot derive. There is no debug assert in its place. (2026-10-06: "the overall idea of the final state sounds good, but the details here (incliding the ones the designers differ on) seem like they'll be changed by the `intent` refactor".)

**What waits on the intent refactor** (D10, HOLD of PR 3990; row `restatement-derives-each-moved-edges-kind`):
- choice 3's details, including whether a tangent edge stored as a chart is kept or turned intrinsic;
- D2's prefer-intrinsic authority question. Ev's 2026-07-19 rule says every definitely-transverse edge carries `Intersection`. An agent-written clause (`99cc678bfd`) exempts a *declared* conventional description. The code exempts a *derived* one.

**Buildable now:** field privacy and the vouch (1), and the tier-1 naming check (2). The three agreed parts in "The question" above stand.

## Built, and what waits

The first step of the ruled final state is built: `kef` and `kfmrh`
ask the move's keys through `Body::vouch_move` (`RechartDoor::Kef`,
`RechartDoor::Kfmrh`) and refuse a strand, then an unvouched landing;
`kef_describing` and `kfmrh_describing` take the move with its
re-descriptions under a band (`Body::vouch_described_move`,
`crates/topo/src/attach.rs`); `kef_carried_redescriptions` and
`kfmrh_carried_redescriptions` restate the stored descriptions there,
keeping the stored kind. A move onto a face wearing the "no chart yet"
placeholder asks nothing to vouch on that side (`Body::unvouched`,
`Landing::chartless`). The shell's rim glue takes `kfmrh_describing`
with `loop_rekeyed`'s specs, and `rename_loop_surface` is gone.

What waits:

- **`kef_minting` and `kfmrh_minting` still move unasked**, so the
  twins do not absorb them yet. Their production callers: the boolean
  zip (`zip_seam`), `boolean/rest.rs`, the coplanar merge
  (`merge_group`), the chord join's sliver cut (`cut_core`) and the
  blend (`SourceFaces::kef_minted`).
- **Four of those kills need a re-description no key swap gives.**
  `zip_seam`'s two retiring kills (`zip.rs`, the loop over the seam
  and the final kill) and `zip_folded`'s two (`rest.rs`) move a seam
  edge onto a kept operand face that rests. The edge's stored
  description names the dying side's key (the REST lane) or an aux
  copy minted into the operand's arena (`bool_planar_chord_spec`,
  `chord_spec`), a different surface from the one the survivor wears,
  so `carried_spec`, `remap_description` and `loop_rekeyed`, which
  rewrite a dead key to a live key on one surface, cannot state it.
  Its honest description is the one `describe_edges`
  (`crates/topo/src/boolean/ops.rs`) derives today after the zip and
  merge: the dihedral decides `Intersection`, `TangentIntersection` or
  a chart image. Deriving that at the kill is the restatement deriving
  kind, `restatement-derives-each-moved-edges-kind`, held on D10.
  Chartless section faces do not help these four: the survivor is a
  kept face.
- **The rest can move now**: the zip's fuse (`kfmrh_minting` in
  `zip_seam`) and `slit_zip`'s kill onto transient faces made
  chartless (agreed part 2); the merge by re-charting the absorbed
  face onto the kept key first (`set_face_surfaces_describing` with
  `carried_redescriptions`), or `kef_describing`; the chord join by
  re-charting the null face to the placeholder when it completes; the
  blend's carve strips chartless, its later band re-chart then taking
  the describing door.
- Field privacy and the vouched-move module, and tier 1's naming check
  with D1's text, need every operator to refuse a strand, so they
  follow the absorption.

