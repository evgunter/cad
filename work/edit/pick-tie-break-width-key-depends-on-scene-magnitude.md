---
id: pick-tie-break-width-key-depends-on-scene-magnitude
kind: issue
title: the width tie-break's key depends on where the scene sits, so an exact tie between identical faces is decided by coordinate magnitude
status: open
opened: 2026-09-16
needs_ev: true
---


Filed by EDIT-PICK3's fix pass, on an observation of review lane
`pick3-r2`. A class the ruling
`what-t-the-pick-door-answers-and-with-what-width` did not discuss;
not built, because deciding it is a design question and not a defect
the unit could settle.

## The observation

`pick_face`'s certified tie is decided by `TSpan::width`, and the width
is not a function of the two candidates' SHAPES alone. `t_span`'s
second term is

```
from_rounding = PROJECTION_ERROR_UNITS * EPSILON * m / (d·d)
```

with `m = Σ_i (|a_i| + |e1_i·u| + |e2_i·v| + |o_i|)·|d_i|` — the
MAGNITUDES of the triangle's anchor and the ray's origin, not just the
edges. So two congruent triangles, met by congruent rays at congruent
angles, carry different widths when one sits at the origin and the
other a kilometre away, and the door prefers the one nearer the
origin.

That is arithmetically correct — the far triangle's parameter really is
less well certified, because its coordinates carry more absolute
rounding — and it is also a rule the user did not ask for. Translate
the whole document and a tie can change its answer.

## Why it is a row and not a bug

The first term, `from_barycentrics`, is translation-invariant (it reads
`err_u`, `err_v`, `|e1|`, `|e2|`, `|d|`). The second is not, and it is
the smaller of the two on every candidate the corpus produced — the
class is reachable only where the first term TIES exactly, which is
where the two candidates are bit-identical in shape and differ only in
placement. `pick3_early_out::equal_widths_fall_to_the_earlier_target`
is that case built deliberately, with both triangles at the same
magnitudes so position decides; move one and width decides instead.

## What it would take to answer

Three shapes, none of them free:

- **Say it is right and state it.** The wider claim IS less certain,
  including for a reason the user thinks of as "where the model sits".
  One sentence at `TSpan::width` and the class is documented rather
  than latent.
- **Compare a translation-invariant width.** Drop `from_rounding` from
  the tie-break key while keeping it in the interval. Then the ORDER
  and the ENCLOSURE stop being the same number, which is the thing the
  ruling's (a)–(c) deliberately made one.
- **Anchor the arithmetic.** Evaluate the crossing relative to the
  triangle's own anchor so `m` loses its `|a_i|` and `|o_i|` terms.
  That is a change to `crossing` and `t_span` both, and it would move
  every measured width in the tree.

## Where the evidence is

`crates/editor-core/src/resolve/pick.rs`, `t_span` and
`PROJECTION_ERROR_UNITS` (the `m` sum is written out in both). The
corpus measurement that says the class is not currently reached is
`crates/viewer/tests/pick3_acceptance.rs` — `aim_lost == 0` over
441 126 rays with the widths as they stand.

## Question for Ev (2026-09-17, EDIT orchestrator) — on the fourth `[ev]` PR

Whether the certified tie's second key stays the full interval width
(arithmetically the better-certified claim, but a rule under which
translating the document can change a tie's answer), or becomes the
shape-only term (`from_barycentrics`, translation-invariant) while the
enclosure and the `precedes` order keep the full width. The
recommendation is on the PR; this row is parked on Ev's answer.
