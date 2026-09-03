---
id: offset-composite-lazy-sign-gate
kind: unit
title: offset_fit's composite forms X and Y on every cell, including the cells the sign witness refuses first
status: open
opened: 2026-09-03
---


TCOST-K2's L3, measured but not landed. `Composite::build`
(`crates/geom-brep/src/offset_fit.rs:1352`) forms `X` (`:1498`) and
`Y` (`:1499`) over the whole patch, for every cell of every round.
`cell_bound` (`:1534`) reads neither on a cell that refuses at the
sign witness `D` (`:1551`), and a whole round can be nothing but such
cells.

**Measured (TCOST-K2's Phase 1, merge base `bf81f0dfa`, dev, one
box).** On `offset_fit::a_patch_far_from_the_origin_certifies_as_well_as_one_at_it`,
station 1 of 7: rounds 0–3 send EVERY cell to `+∞` at `D` (1/1, 4/4,
16/16, 80/80 — never at `w`, `w̃` or the `‖E‖` floor), so `X` and `Y`
were built and never read on 101 cell measurements. That is 0.5707 s
of the station's 3.675 s: **15.5 %** of the call at the merge base,
15.6 % of it after K2's weight hoist. One station of the seven has a
MIXED round (266 of 308 cells refuse at the witness, 42 do not), so
the saving there is partial. On
`an_unreachable_tolerance_refuses_typed_at_the_budget` every cell is
sign-definite from round 0 and the lever pays nothing.

**Why K2 did not land it, and what this unit owes.** The K2 spec
called it "a mechanical per-cell restructure". It is not. `measure`
(`:951`) decides "misaligned or poisoned composite" by reading
`comp.x.cell_counts() == 0`, and `Composite::build` takes its
`breaks_u`/`breaks_v` off `x.breaks()`. Deferring `X` — per cell or
behind a `OnceCell` — moves the channel that makes a refusal
decision. This unit therefore owes an argument that the alignment
invariant `poison_like`'s doc states ("never produced by the
entry-point builders, which share one decomposition structure") is
the same fact read off `Ẽ` as off `X`, plus a row that a poisoned or
misaligned composite still reports the unbounded report from whatever
channel replaces `x`.

**The precondition is already proved:** a product's cell coefficients
and the `cell_hull` the bound reads are bitwise identical formed
whole-patch or over that one cell alone —
`geom_core::spline::compose::patch::tests::a_products_cell_is_the_same_formed_whole_patch_or_alone`,
landed with K2.

Constraints carry over from `docs/TCOST-K2-SPEC.md` §4: bit-identical
certificates and refusal payloads, D9 (the branch must be the gate
`cell_bound` already evaluates, in the order it evaluates it), and
the digest instrument as the receipt.
