---
id: the-apothems-sign-is-a-value-read
kind: issue
title: the apothem's sign: six arc-family decisions on a parameter bulge are zero exactly where the apothem L(1-b^2)/(4b) is positive, which only a value read reaches
status: open
opened: 2026-09-25
priority: P2
cost: H
refs: [rule-d-reaches-the-unit-bulge-only, 3186]
parent: DECIDE-4
---

**Filed from DECIDE-4's fork, on Ev's ruling on `[ev]` #3186 (2026-09-25):**
this becomes its own item, not a widening of DECIDE-4.

## What stands

DECIDE-4's Phase 1 counted these on the `0.5` parameter control of R2's
D-tab (`r2_d_tab_parameter_dyadic`):

- `line_span`, `arc_span` and `contact_at_shared_vertex`, two each;
- `line_span`'s two on the `0.4` parameter D-tab.

None of them is frozen. The early form is
`u·(|2b|·2(1 − b²) − |4(b² − 1)|·b) / (|2b|·2b)`: the magnitude
`sqrt(r² − (L/2)²)` of the centre's offset against the signed offset
`L(1 − b²)/(4b)`. Evaluated at points in every sign region, it is zero
exactly on `0 < b < 1` and on `b < −1`, which is where the apothem is
positive. So these decisions stand on the apothem's sign, not on `b`'s.

The uncut forms are in `crates/editor-core/tests/m10_bulge_renders.txt`
(on `props/sign-hull` from DECIDE-4's merge), and the attribution is the
section "What stands on DECIDE-3's and SYM-9's tree (DECIDE-4)" of
`rule-d-reaches-the-unit-bulge-only`.

## Why neither sign route reaches it

- **The turn `σ` (route B, ruled for `b`'s sign)** relates `|b|` to `b`.
  The apothem's sign is `sign(1 − b²)·σ`, which the profile program does
  not decide.
- **Rule C on as the dial stands (route A)** does take all six. But it
  also re-labels theorems as `sign_gated` on every document, loses the
  door on `pcurve_map_residual`, and raises `numeric` everywhere. It was
  rejected on #3186.

## The candidate

A NARROWED rule C: a read that folds only `abs`/`sqrt` atoms whose
argument has a certified sign over the leaf's box, ordered behind the
door and behind every value-free fold, as the decision read is. Its
zeros are `sign_gated`, which is the honest label for a fold that holds
on the box and not identically. It is unmeasured. The first step is to
build it behind a dial and count what it takes and what it re-labels,
on the six documents and the bulge controls.

A structural alternative, owed a look before the read is built: whether
the sweep's arc construction already decides which side of the chord
the centre lies on. If it does, that decided sign could be stated the
way route B states the turn.
