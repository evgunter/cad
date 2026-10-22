---
id: the-census-records-a-planar-area-contact-as-one-face-pair
kind: unit
title: the at-rest census records a planar area contact between arrivals as one face pair, a flush pair as its own row, and asks nothing of a confirmed pair's boundary cells
status: open
opened: 2026-10-22
priority: P1
cost: H
---


## What

`[assertions]` as ratified on PR 5555:

- **An area contact is one row.** Between two arrivals, the census
  decides each pair once, at the highest-dimensional cells that touch.
  Two faces meeting over an area are one pair. The census asks nothing
  of a cell bounding one of them against a cell bounding the other.
- **A flush pair is its own row.** Two faces on one carrier, facing the
  same way, whose boundaries meet while their interiors do not, are a
  flush pair. It is a finding unless it is established.

Today the census falls short in four ways:

- **No planar face pair.** `crates/topo/src/census.rs`
  `sweep_conformal_patches` is curved-only, so a planar contact between
  arrivals shows up only as its edge and vertex events: about 30
  findings for one seat.
- **No flush pair.** Nothing records it.
- **Backing reaches outside the overlap.** `vv_face_backed` and
  `ee_face_backed` back an event anywhere on a declared face. Only
  `vf_face_backed_confined` (via `pair_verified`) checks that the event
  is in the verified overlap.
- **A near-flush mated seat refuses the product.** `collinear_overlap`
  escalates an in-band near-flush edge before any backing is consulted,
  and `checks::at_rest::partition` sends the pair-less escalation to the
  gate.

## Owed

- **A planar face-pair arm.** Door 1 checks one carrier with opposed
  normals; Door 2 checks an overlap of positive area.
- **A flush arm.** Door 1 checks one carrier, aligned. Door 2 finds the
  overlap empty, and the shared boundary segment is the witness.
- **Confinement.** Every backing check goes through `pair_verified`.
- **No refusal at rest.** An in-band event inside a confirmed pair's
  boundary is asked nothing. Elsewhere, an in-band pair is that pair's
  "could not decide" finding, not a product refusal.
- **Quieting.** A flush finding is quieted by a measure that reads the
  signed offset between the two carriers along their shared normal.
- **Rows:**
  - a flush block on a block, unmated, flush on 1, 2 and 3 sides;
  - an overhang;
  - a near-flush pair in band;
  - two coplanar faces that meet only along a line (no face pair);
  - perf12's census goldens, re-blessed with their stated causes.
- **On stage 4 I's path:** `mates-declare-no-contact` waits on this.
