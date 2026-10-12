---
id: a-cube-less-a-pinched-operand-piercing-its-face-refuses-the-split-cones-re-pair
kind: issue
title: A cube less a pinched operand whose cones pierce its face at one point refuses ZipCorrespondence in split_cones' re-pair (36 dbl lines)
status: closed
opened: 2026-10-11
priority: P2
cost: M
branch: flush/pinch-residue
pr: 4777
closed: 2026-10-12
---


## What

Found by `the-pre-zip-pinch-weld-retires-once-coincident-pierces-split-per-cone`'s
lane (branch `flush/pinch-cluster`). The `sweep`
`join_pierce_runs_sweep::dbl_battery` (ignored, release) runs a pinched
operand against a cube: two corners touching only at `v`, the cube's
near face through `v`. Of its 2 160 lines, 36 refuse
`ZipCorrespondence { what: "a split section face does not pair one to
one" }`, raised by `crates/topo/src/boolean/zip.rs` `repair` after
`split_cones` has split the vertices at `v` per cone.

All 36 are `yx S`, the cube less the operand: 13 at `Ltop asym` seeds
2296 and 2959, and 23 at `asym asym` seeds 15, 225, 268 and 426
(directions 40 to 580). Replay one with
`DBL_POSE="asym 268 100 yx S" cargo test -p sweep --release --test all
dbl_one -- --ignored --nocapture`.

On the base (`weld_pinches` still welding the two pierces), the same 36
lines did not build right: the 23 `asym asym` lines built `OK BAD` with
three vertices at `v` for two cones (tier 3′ failing), and the 13
`Ltop asym` lines refused `ResultInvalid { RingMeetsOuter }`. So the typed refusal replaces a wrong body. It is
not a new failure of the split.

## Open

Not root-caused. The ring's root does not matter: the probe refused
the same way from every root `null::with_ring_root` sets (0 to 4) at
`asym 268 100`, `Ltop 2296 50` and `asym 15 40`. Read next: which of
the two sides' `split_side` merged section faces (`kef`) that the other
side's did not, at the cube's ring vertex or at the operand's vertices
at `v`.

## Measured (2026-10-12, main `300f7cefe`)

Reproduces: 36 of `dbl_battery`'s 2 160 lines refuse
`ZipCorrespondence` ("a split section face does not pair one to one"),
13 `Ltop asym yx S` and 23 `asym asym yx S`; every other line `SOUND`.

Read at `asym 268 100 yx S`, in `zip::split_cones`. At `v` the cube
(A) holds two vertices and the operand (B) two. A's hold corners
`{0, 10}` and `{8, 13}`; B's hold `{8, 0}` and `{10, 13}`, so each A
vertex corresponds to both B vertices. The cones (σ_B∘σ_A) are two,
`{A run 0, B run 10, A run 13, B run 8}` and
`{A run 8, B run 13, A run 10, B run 0}`, so every one of the four vertices is split. Each split's
`mev_null` and the `kef`/`kemr` that kills its null edge swap the
loop successors of the split's two corners. So after the splits A's
section loops are `(0 10)(8 13)∘next` and B's `(0 8)(10 13)∘next`: A's
three section faces merge into one, B's into three (one `kef`, one
`kemr`). The face pairing can be one to one only where both products
agree, and no two corners share a vertex on both sides. The refusal
was the face-pair zip's limit, not a wrong split.

## Fix (branch `flush/pinch-residue`)

The zip glues edges, not faces (`zip::glue`). Each fusion joint and
its `kev` hands the two joined half-edges' predecessors each other as
successor. A pair `(ob, r)` is glued once `r` precedes `ob` and `ob`
precedes `r`, a loop alone in its face, which `kef` then kills. A joint
across two loops is a `mekr`, across one a `mef`, across two faces a
`kfmrh` first. `zip_seam` drives it in `fusion_order`, which reproduces
the reassembly-oracle sequence joint for joint. `repair` now groups
the faces holding paired edges, and a group of more than one face of
either operand zips as one split seam (`zip_split`).

Release battery diff, base `300f7cefe` against the fix: the 36 lines go
to `OK SOUND` (volume equal to the oracle, one vertex per cone at `v`,
meshing) and nothing else moves. `pinch_runs` (3 024), `pierce_runs`
(4 326), `corner_pairs` (16 314), `four_pairs` and
`pinch_faces_tessellate` are byte-identical. Pinned by
`join_pierce_runs_sweep::a_cube_less_a_pinched_operand_whose_splits_cross_the_seams_faces_builds`.

## Fix pass (PR 4777's dual review)

- **The mirror case.** Read from the operand's side (several faces of
  the hosting side pairing with one of the other's), a joint between two
  loops could name the host's outer loop as `mekr`'s ring and refuse
  `Euler(RingIsOuter)`. The joint reads the same either way round, so
  `glue` makes the outer loop the target and collapses the joint onto
  `ob`'s vertex all the same. Pinned by running the pin's three poses
  in both roles (`topo::swap_split_roles`, `sweep-testing` only); with
  the swap on throughout, `dbl_battery` is 2 160 `SOUND`.
- `repair` refuses typed a split section edge whose partner lies off the
  other operand's section faces.
- Guard: `ops::tests::a_face_pair_zip_is_the_reassembly_oracles_sequence`
  zips every seam of three brick pairs under every op both ways
  (`zip_seam`, and the splitting oracle's `reglue_pair`) and compares
  the topology key for key.
- `zip_split`'s `kfmrh` onto a curved host is unexercised:
  `a-split-seams-kfmrh-onto-a-curved-host-is-unexercised`.
