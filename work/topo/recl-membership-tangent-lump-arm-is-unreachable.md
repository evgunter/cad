---
id: recl-membership-tangent-lump-arm-is-unreachable
kind: issue
title: topo: recl's membership reads a declared-Tangent flank pair's lump, but the tangent-flank short-circuit has already skipped every such pair
status: parked
opened: 2026-09-30
blocked_on: [3990]
---


(TOPO, PR 3513's third fix pass, found while constructing the
membership tie's declaration read.)

## What

`boolean::recl::resolve_edge_edge` runs the wedge membership only when
no flanking pair is declared `Tangent` (`tangent_flank`, over the four
combinations of `(fa_s | fa_e) × (fb_s | fb_e)`). Inside the membership,
the `SideCode::On` arm reads the pair `(own_secs[own_idx],
other_secs[oi])` and, where that pair is declared `Tangent`, lumps it
through `sectors::tangent_lump`. `own_idx` is one of A's flankers and
`oi` one of B's, so that pair is one of the four `tangent_flank` read:
the `Tangent` lump arm cannot be reached, and every pair reaching the
lump takes `require_same`.

## Repair shape

Remove the arm, or say why the short-circuit and the arm must both
stand (a flanking set the two readings would disagree on), with a row.
