---
id: a-hole-filleted-at-both-rims-in-one-fillet-panics-in-blend-surgery
kind: issue
title: A through-hole filleted at both rims in one Fillet panics in blend surgery: a birth row names an edge the source body does not carry
status: open
opened: 2026-10-05
priority: P1
cost: M
---


Found by RECIPE's third review of PR 3886 (2026-10-05), building a document to reach `RoleSeg::BandCross`/`BandSlit` pairs that differ only in `band` (two rims at the two ends of one meridian segment, `names/role.rs` ~1328–1358).

A through-hole whose two rims are both rounded by one `Fillet` panics during evaluation at `crates/sweep/src/blend/surgery.rs:924`: "a birth row names EdgeKey(20v1) as its source, which the source body does not carry (kernel bug)". A panic on a valid authored document is a kernel defect; the evaluation should build or refuse typed. The reviewer's probe is in `~/.local/share/cad-work/rev3886c/rev3886c.rs` (lane-private; the construction: a box, a cylinder subtracted through it, one fillet over both of the hole's rim edges).

## Findings (band-two-rim-panic)

- The panic is at the kernel door, not only through the document:
  `sweep::blend::build::fillet_edges` over both rims of a through-bore
  (extruded `bored_block_of_arcs`, N = 2 and 3, and a box minus a
  cylinder through `topo` booleans) panics in `blend_surgery`'s debug
  postcondition. Release builds returned the body with a birth row
  naming a minted key.
- Cause: the two rims are LADDER rims sharing the bore wall, whose seam
  lines run rim to rim. The first band splits each seam (`split_fragment`)
  and the second band's `rim_phase` splits the surviving piece, but
  recorded its `meridian_splits` row against the key it SPLIT (`m`, a
  fragment the first band minted) instead of `frag.source`. The annulus
  phase already named `f.source`; the ladder comment called the arm
  unreached.
- Fix: the ladder row names `frag.source`; the two splits of one seam
  are told apart by their band. The result is the sequential
  composition, and removes two plane–cylinder corner tori exactly.
