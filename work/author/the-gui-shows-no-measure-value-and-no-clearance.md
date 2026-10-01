---
id: the-gui-shows-no-measure-value-and-no-clearance
kind: issue
title: the GUI shows a measure's existence and never its value
status: closed
opened: 2026-09-11
priority: P0
cost: D
closed: 2026-09-30
pr: 3528
---



Found while building the GUI montage lane (`demos/render-gui.sh`),
whose planned subject was *"clearance and measures, as the app shows
them"*. It could not be built as planned, and the reason is the
finding: **the app shows neither.**

**A measure is a tree row and nothing else.** `crates/viewer/src/tree.rs:190`
maps `Node::Measure { .. }` to the string `"Measure"`, and that label
plus a `RowStatus` is the whole of what reaches the screen. The node's
VALUE — the distance, the angle, the gap — appears nowhere in the
feature tree, nowhere in the properties panel, and nowhere in the
status line. A document can carry a measure that evaluated
successfully and a reader of the GUI cannot learn what it measured.

**Clearance is absent outright.** `clearance` does not appear anywhere
under `crates/viewer/src/`. The kernel's engine
(`crates/editor-core/src/clearance.rs`) has eleven typed refusal arms
and a certificate-carrying `Holds` verdict; none of that has a
consumer in the app.

**And the second half is why the first half may be harder than it
looks.** `MeasurePrimitive::min_clearance`'s own contract says:

> At the `f64` scalar this library evaluates at, the measure has NO
> VALUE: `Value.measure` raises `MeasureUnavailableAt` naming the door
> that could answer, and an assertion over it reports `Unevaluated`
> carrying the same reason. The node itself evaluates successfully —
> the absence is a value, not a failure.

The viewer is an `f64` build. So even a panel that displayed measure
values would display *nothing* for the one verb whose subject is
clearance, and would be right to. Showing clearance in the GUI is not
a panel change; it needs the app to reach an interval-scalar
evaluation, or to render the typed absence as the meaningful thing it
is rather than as a blank.

**What the montage ships instead**, stated so the substitution is not
silent: the assembly layer's own story — leaf parts with their named
document parameters, instantiation, patterning, and the mated bench —
with the status line's `at rest: certified (N declaration(s))` going
from 0 to 2 as the mates arrive. That IS a clearance-adjacent reading
and it is genuinely on screen, but it is a DECLARATION COUNT, not a
distance, and it is not what the planned cell was for.

**What a fix looks like**, cheapest first:

* **render the value beside the row.** A measure that has one is the
  common case (`distance`, `angle`, `gap` at `f64`), and a row reading
  `Measure  distance  12.500 mm` is most of the value of this issue
  for very little.
* **render the typed ABSENCE for the ones that have none** — a
  `min_clearance` row reading `unavailable at f64` with the door that
  could answer, which is what the kernel already hands back. That is
  more honest than a blank and costs a string.
* **a clearance consumer** — the verdict, its witness point, and the
  eleven refusals — which is a feature, not a repair, and wants its
  own design pass.

**Scope note.** This is a VIEW finding, not a kernel one. Nothing above
says the kernel is wrong; `min_clearance`'s f64 absence is deliberate
and documented. What is missing is a consumer.

Dispatched 2026-09-30 as **AUTH-7** (`docs/AUTH-7-SPEC.md`, branch `author/measure-value`): tiers 1 and 2, meaning the value, or the typed absence and its door. Tier 3, the clearance consumer, is split out as its own design row by the lane. Checked against the tree first: the label is at `tree.rs:385`; the value and the typed absence are `ValuePayload::Measure` / `MeasureUnavailable`, both reachable through `pncad::document`. The two `clearance` hits now in `crates/viewer/src/` are a comment and a parameter name, so no consumer exists.

AUTH-7 split tier 3 out as `the-gui-has-no-clearance-consumer` (design). Its sweep also filed `an-assertion-row-shows-no-verdict` and `a-mate-row-does-not-say-whether-it-placed-its-child`.

AUTH-7 landed the value as `Measure »  0.0125 m`. Two parts of the example `Measure  distance  12.500 mm` are scheduled elsewhere. The verb waits on a kernel door, `work/props/measureexpr-has-no-display-or-as-primitive`. The unit is the crate-wide computed-value notation question, `work/chrome/a-computed-slots-value-reads-in-metres-and-radians`, which now carries the measure case too.

## Closed 2026-09-30 — PR 3528 merged (`6e9523dd`)

**A measure shows its value, or says why it has none.** A measure with
a value shows it beside its tree row (`Measure »  0.0125 m`). A
`min_clearance` measure, which deliberately has no value at `f64`, shows
the kernel's own `MeasureUnavailableAt` sentence under its row, naming
the door that answers. A failed measure shows its badge and no value.
The value is keyed by the row's own node id and read from the landed
pair, as every badge is. The number goes through the crate's one rule
for computed values (`props::computed_text`), and the reason stays
typed until the pane renders it.

**Two premises in the spec were wrong, and both are recorded here.**
- *"In the author's units"* has no referent. A `MeasureExpr` carries a
  dimension, not a unit, and there is no preferred unit anywhere, so
  the value reads in canonical units, as a driven slot already does.
  The crate-wide fix is a preferred display unit per dimension. It is
  the existing CHROME design row
  `a-computed-slots-value-reads-in-metres-and-radians`, which now holds
  this evidence.
- *The verb* (`distance`) is not shown. `MeasureExpr` has no `Display`,
  and writing one in the viewer would mint a sentence; filed on PROPS
  as `measureexpr-has-no-display-or-as-primitive`.

**Tier 3, a clearance consumer,** is its own design row,
`the-gui-has-no-clearance-consumer`.

**Siblings filed:** `an-assertion-row-shows-no-verdict` (P0, the same
gap for assertion verdicts) and `a-mate-row-does-not-say-whether-it-placed-its-child`
(blocked on MSOLVE's `materole-has-no-display`).
