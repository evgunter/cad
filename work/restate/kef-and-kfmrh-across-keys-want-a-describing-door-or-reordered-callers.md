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
  twins do not absorb them yet. Their production callers, each kept
  for the reason below: the boolean zip (`zip.rs`: `fuse_by_joint`'s
  hole arm, `split_side`, and `zip_seam`'s fuse and its two retiring
  kills), the coplanar merge (`merge_group`), the chord join's sliver
  cut (`cut_core`) and the blend (`SourceFaces::kef_minted`).
- **`zip_seam`'s two retiring kills** (the loop over the seam and the
  final kill) move a seam edge onto a kept operand face that rests.
  The edge's stored description names an aux copy minted into the
  operand's arena (`bool_planar_chord_spec`, `chord_spec`), a
  different surface from the one the survivor wears, so a key swap
  cannot state it: its honest description is the one `describe_edges`
  (`crates/topo/src/boolean/ops.rs`) derives after the zip and merge,
  where the dihedral decides `Intersection`, `TangentIntersection` or
  a chart image. Deriving that at the kill is the restatement deriving
  kind, `restatement-derives-each-moved-edges-kind`, released from the D10 hold by INTENT stage 4 E (2026-10-09) for a designer pass.
- **The coplanar merge refuses the twin on curved survivors.**
  `merge_group` through `kef_describing` with
  `kef_carried_redescriptions` certifies every planar kill, and
  refuses `RechartUnvouched` on 115 calls in the `ci` profile, at the
  default eps, 1e-6 and 1e-12 alike, all onto a curved survivor
  (cylinder and torus stacks: `reach_continuation`,
  `reach_aligned_half_rods`, `curved_mergedoor`, `germ_torus_doors`,
  `verbs_cylcyl_probe`). The moved edges it does not vouch for are of
  two kinds. A chart image on the absorbed key whose other face, a
  neighbour cut from the same operand face, wears that key too (60
  calls): the kill leaves it coherent, so `carried_by_kill` lists
  nothing for it, and on the survivor's curved chart, a different key,
  no residual vouches for it. And an edge the zip's kills left
  incoherent before the merge runs (`TangentIntersection` or
  `Intersection` naming the absorbed key and a key neither face
  wears, or a chart image on such a key; 55 calls), which no
  restatement lists. Two coplanar faces carry no certified
  `Intersection` between them, so the key swap the planar kills need
  is honest; the curved ones need the first kind stated on the kept
  key by the caller, and the zip's strands described before the merge.
- **Transient faces cannot wear the placeholder while an intrinsic
  edge they border names their key.** A face that borders a kept face
  carries the shared edge as `Intersection(own key, kept key)`, and
  no description can name the placeholder, so making the face
  chartless strands the edge: `vouch_move` asks the strand before the
  chartless arm. Measured in the `ci` profile at the default eps
  (1e-6 and 1e-12 within 1%):
  - the chord join's null face, re-charted to the placeholder when
    `cut_core` completes it (`set_face_surface`): 8,684 of 43,830
    refuse `RechartStrandsDescriptions`. Its section faces inherit
    that surface, so the zip's fuse and `split_side`'s kill wait with
    it;
  - `cut_core`'s own kill precedes completion: it merges two slivers,
    and a sliver wears the operand face it was walled off from
    (`join`'s `mef(.., Inherit)`, its chord stated on that key by
    `running_from`). Of its 145,934 calls, 13,963 refuse the twin
    `RechartFalsifies` (`Intersection(sliver key, neighbour)`
    key-swapped onto the surviving sliver's surface, which the edge
    does not lie on) and 3,010 refuse residuals or vouching;
  - the blend's carve strips minted chartless: 7 of the 9 strip
    `mef`s refuse `RechartStrandsDescriptions` on every or nearly
    every call (`open/planar.rs`'s first chord 27,404 of 27,404); its
    kills refuse the twin on 9,715 of 40,575 calls, the same
    `Intersection` key swap.

  The strands are all `Intersection(old key, other)`, coherent now.
  Each wants either a description that does not name the transient
  face's key (a chart image on the other face: a change of kind, the
  D10-held restatement), or a chartless arm that also leaves a strand
  on a chartless face unasked mid-op. Which is a design question.
- **The open fork (PR 4562, 2026-10-10).** Every caller stays on
  `_minting` (option (c) of both questions below) until the fork is
  ruled. Both questions ask how an edge a kill moves gets described,
  so they go to the designers with `restatement-derives-each-moved-edges-kind`:
  - **F1, the merge on curved survivors.** (a) `merge_group` lists a
    re-derived `Chart(kept key)` for each same-key chart image, which
    settles 60 of the 115 calls. The other 55 wait on the zip's
    retiring kills being described. (b) Planar survivors take the twin
    and curved ones keep `_minting` as a named, counted residue.
    (c) It stays on `_minting`.
  - **F2, chartless transients against the strand rule.** (a) The
    chartless arm also leaves unasked a strand onto or off a chartless
    face mid-op, and agreed part 3's scope-close check catches it at
    rest. This reverses the ordering PR 4204 pinned and narrows ruling
    1's "refuse a strand". (b) While a face is transient, the edges it
    borders are described as chart images on the neighbour: a change
    of kind at mint, which is the restatement. (c) `cut_core`, the
    blend, the zip's fuse and `split_side` stay on `_minting`.
  - Two smaller moves are measured and not built. `split_side`
    certifies through the twin with carried restatements on 24 of 24
    calls. `fuse_by_joint`'s hole arm is a same-key `kfmrh` (no ci row
    reaches it), so `kfmrh_describing(face, divided, &[], tol)` would
    be the same move.
- Field privacy and the vouched-move module, and tier 1's naming check
  with D1's text, need every operator to refuse a strand, so they
  follow the absorption.

## An eleventh site: `ring_move_unasked` (2026-10-19)

PR 5334 (WIRE, `wire/split-maximal-halves`) adds an eleventh production
site of the same shape. `merge_faces.rs` `merge_group`'s ring drain
moved an absorbed face's rings onto the survivor with
`ring_move_minting`, which asks the move's keys; a split half whose two
section annuli on distinct plane keys merge (the revolved pentagon at
its apex circle) refused there `RechartStrandsDescriptions{RingMove}`.
The drain now calls `ring_move_unasked` (crate-private,
`euler_ring.rs`), `ring_move_minting` without the keys, as
`kef_minting` is `kef` without them. It is sound for the reason the
merge's `kef_minting` is: the door re-describes every boundary edge of
each kept face, rings included, before it returns
(`Body::merge_coplanar_faces_declared`, *The kept faces' boundaries are
re-described*), and its tier-2 gate runs on the result. It is
crate-private, so `pcurves.rs`'s posture census, which admits public
mutation paths only (`every_mutation_door_declares_its_pcurve_posture`
refuses an entry for any other), does not list it; it disposes of the
ring's rows as `ring_move_minting` does. `ring_move_minting` keeps its
callers outside production: the seqgen fuzz lane's mutator, the
row-walk proofs and the loop re-parenting fixtures.

Signed (wire-splitmax lane).
