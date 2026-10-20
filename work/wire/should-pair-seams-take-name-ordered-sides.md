---
id: should-pair-seams-take-name-ordered-sides
kind: issue
title: Should a pair boolean's seams take name-ordered sides, as FORK-SPANCHORD leaned, rather than Sided with a Merged parent opposite the operand face?
status: closed
opened: 2026-10-18
closed: 2026-10-20
pr: 5469
branch: wire/pair-seam-name-order
---


## Question

FORK-SPANCHORD (`docs/DESIGN-FORK-LOG.md` row 160) recommended that a
pair boolean's seam cite the parent of the face beside it on each side,
and both designers' answers leaned to putting the two sides in name order
("seam sides in name order (lean)"; "name-order sides simplest"). Ev
approved the recommendation "as recommended" on PR 5279, and the ratified
N3 sentence (`crates/editor-core/src/names/README.md`) says nothing about
order.

PR 5313 (`wire/seam-cites-parent`) keeps the pair's seams
`canonical::Seams::Sided`: `a` is the A side and `b` the B side, and a
merged face's `Merged` parent takes the side opposite the unmerged
operand face across the seam (`emit_topo`'s `name_boolean_edges`,
`beside`). `Seams::of_published` tells a pair seam from a union seam by
whether both sides are minted by the name's own node.

Whether to switch is an open question; nothing was switched in PR 5313.

## What a switch would move

- `canonical.rs`: `minted` would order pair seams `ByName`. The
  `Sided`/`ByName` split and `of_published` would go, or would be kept
  only for names already published. `RankRule` re-reads ranks wherever
  the order flips (the rank's line is oriented by the first side as
  written).
- Seam vertices in `emit_topo`'s `name_boolean_vertices`, which read
  `a_faces`/`b_faces` and `Crossing { face: b_faces[0] }` by slot, and
  `resolve/mod.rs`'s `SeamParent::across` (`a_side` decides which slot
  the parent is in).
- Every pair seam name whose operand sides are out of name order would
  be re-spelled, and every vertex rank along it with it. That is most
  of the corpus, so the four name pins move: `lib_g16_corpus_name_digests`,
  `perf2_name_keying_differential`, `seat4_verb_lowering` and
  `name_words_corpus`. `m4_pr3_names_*` and the goldened tables move as
  well. A persisted name of a pair seam would need a re-map.

Signed (WIRE lane `wire-seamparent`).
