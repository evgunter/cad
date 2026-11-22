---
id: a-seamless-bands-trim-reads-a-point-inside-it-out
kind: issue
title: A seamless band's trim placement reads a point inside the band Out
status: open
opened: 2026-11-21
priority: P2
cost: M
---



Filed by the SECTCONE U1 lane (PR 7354), from a CI run of `topo`'s section-certificate rows. The finding is suspected: it rests on the guard's verdict and was not reproduced locally.

## What

PR 7354's first push gave plane × cylinder's tilted-ellipse component a witness point: `curve.eval(0)` on the closed ellipse, which lies on the wall. On that push, both of these went from `Err(Undecided)` to W3's `Out` on the band (CI run 44988689441, job `test (kernel)`):

- `section_cert_rows::w2_refuses_a_seamless_band_and_clears_a_banded_wall`: `[Ok([Out(F)]), Ok([Out(F)])]`;
- `w2_asks_the_partner_operands_face_for_a_g_claim`: `[Ok([Out(G)]), Ok([Out(G)])]`.

The face is `seamless_band(-1.0, 1.0)`: the unit wall about `z` over `z ∈ [−1, 1]`, closed on itself with no meridian edge. The partner is `tilted_slab()`, which cuts the wall in ellipses whose points satisfy `z ∈ [−0.1, 0.3]`. Every point of either ellipse is therefore inside the band. Yet `ops::place_witness` → `contain::placement_on_carrier` placed the witness `Out` of it.

A W3 that reads `Out` for a point inside the face clears a loop that may lie inside both faces. That is the interior loop the guard exists to refuse.

If the reading is real, the class is wider than plane × cylinder. W3 would clear an interior loop on every witnessed section loop on a seamless band, cylinder × sphere's ruling-sweep witnesses included: a soundness class of the interior-loop guard.

## Mitigation in PR 7354

The plane × cylinder loops name no witness, as the guard's hand classifier never did. They clear by W2 on a wall that describes and otherwise refuse, so these rows hold their old verdicts. A circle on a sphere keeps its witness.

## Done when

The trim placement of a point inside a seamless band reads `In`, or refuses. A row pins it: the witness above, placed on `seamless_band(-1.0, 1.0)`.
