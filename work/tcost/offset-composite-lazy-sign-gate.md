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
15.6 % of it after K2's weight hoist. On
`an_unreachable_tolerance_refuses_typed_at_the_budget` every cell is
sign-definite from round 0 and the lever pays nothing.

**The mixed rounds bound the saving, and sizing them is this unit's
own concern.** The 15.5 % above is the all-or-nothing case. One
station of the seven has a MIXED round 4 — 266 of 308 cells refuse at
the witness, 42 do not — where forming `X` and `Y` per cell saves
those 266 cells' share of the round and pays a per-cell entry point's
overhead on the other 42. A per-cell restructure is worth what the
census says across the corpus, not what the cleanest station says;
this unit owes that corpus figure, taken the same way, before it
quotes 15 %.

**What moving the poison channel actually costs.** `measure` (`:951`)
decides "misaligned or poisoned composite" by reading
`comp.x.cell_counts() == 0`, and `Composite::build` takes its
`breaks_u`/`breaks_v` off `x.breaks()` (`:1500`). Deferring `X`
therefore moves the channel a refusal decision is read from — it has
to move to another channel rather than simply going away. That
channel is already in the builder and the argument for it is one
sentence: `dd` is `dot_spans(&e, &m_tilde)`, formed whole-patch from
the same aligned operands `x` is formed from, and every channel here
is decomposed with the same `extra_u`/`extra_v`, so `cell_counts` and
`breaks` read off `dd` (or off `e`) are the same numbers `x` reports
and are poison in exactly the cases `x` is. So the obligation is not
a new soundness argument, it is the sentence above written at the
site plus one row that a poisoned or misaligned composite still
reports `+∞` through whichever channel replaces `x`.

**The precondition is already proved:** a product's cell coefficients
and the `cell_hull` the bound reads are bitwise identical formed
whole-patch or over that one cell alone —
`geom_core::spline::compose::patch::tests::a_products_cell_is_the_same_formed_whole_patch_or_alone`,
landed with K2.

Constraints carry over from `docs/TCOST-K2-SPEC.md` §4: bit-identical
certificates and refusal payloads, D9 (the branch must be the gate
`cell_bound` already evaluates, in the order it evaluates it), and
the digest instrument as the receipt.
