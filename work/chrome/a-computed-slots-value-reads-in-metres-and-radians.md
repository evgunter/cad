---
id: a-computed-slots-value-reads-in-metres-and-radians
kind: issue
title: viewer: a computed slot's value reads in metres and radians, whatever the document is written in
status: closed
opened: 2026-09-29
closed: 2026-10-01
pr: 3639
priority: P3
cost: M
branch: chrome/working-notation
refs: [a-driven-slots-field-draws-its-expression-source-at-any-width, the-gui-shows-no-measure-value-and-no-clearance]
---

## Question (answered by Ev, 2026-10-01)

In what notation should the viewer write a value no author wrote? This covers a driven slot's `= value`, the refusal affordance's "(currently …)", a measure's row, and both sides of an assertion's verdict. Today all of them read the canonical m/rad (`props::rendering_unit`), so `thickness * 2` over a 4 mm parameter reads `= 0.008 m`. The choice is where the notation comes from and who owns it: the document, the viewer per person, the expression itself, or canonical as now.


Found by the review of `chrome/slot-width` (PR 3478).

A driven slot's field shows the value its expression equals
(`crate::props::field_text`, through `crate::props::computed_text`),
and so does the refusal's affordance (`Refusal::affordance`,
"(currently …)"). Both write it in the notation
`crate::props::rendering_unit` gives a value with no remembered unit,
which is the CANONICAL one: `thickness * 2` over a parameter declared
`4 mm` reads `= 0.008 m`, and a driven revolve angle of 45° reads
`= 0.7853981634 rad`. Since PR 3478 the symbol is said, so nothing
misreads it, but a user who writes millimetres and degrees reads a
number in neither.

`rendering_unit`'s doc argues for the canonical choice: it is how
`unparse` renders such a value, so the panel and the text door agree
by construction. A different notation for a COMPUTED value (the unit
of the parameters the expression reads, the unit its literals were
written in, or a document-level preference) is a design choice, not a
bug fix, and would move that agreement.

**A third reader since AUTH-7 (PR 3528), 2026-09-30: a measure's tree
row.** `tree::Measured::Value` is spelled by the same
`props::computed_text`, so a 12.5 mm box's height reads `0.0125 m` and
a right angle reads `1.5707963268 rad`. The parent row's example
promised `12.500 mm`.

**The fix has to be crate-wide, not a measure special case.** AUTH-7's
two reviewers agreed on this. `rendering_unit` would consult a preferred
display unit per dimension, so driven slots, the affordance and measures
move together, and the agreement with `unparse` is decided once, in one
place. The per-measure alternatives do not survive:

* A measure records no notation. `MeasureExpr` carries a `dim`, and its
  primitives are never written by anyone.
* The measured geometry's slots cannot be an honest source. AUTH-7's
  correctness reviewer pointed out that there can be many of them and
  they can disagree: a distance between a face placed by a `4 mm` slot
  and one placed by a `0.2 in` slot has no single author's unit.
* The creation forms' unit picker (`widgets.rs`, "The creation forms'
  written-unit picker") is transient per-form state, not a preference.

What is open is where the preference lives: a document field, which is
persisted and diffable, or a viewer preference, which is per person
and not in the recipe. How it interacts with `unparse` is part of the
same question. That is a design fork.

**A fourth reader since AUTH-8, 2026-09-30: an assertion's verdict.**
`tree::Asserted` spells both the measured value and the bound through
`computed_text` in the measure's dimension, so an assertion authored as
`>= 0.5 mm` reads `>= 0.0005 m`. Here the bound is an authored `Expr`
that does carry a written unit, unlike the measure, and AUTH-8 still
did not spell it in that unit: the two numbers of one comparison would
then read in two notations. The crate-wide preference above moves both
together.

Inside ε the comparison reads false on its face: the kernel decides a
margin within ε as `Sign::Zero`, which a non-strict relation holds, so
a `Holds` row can print `0.0125 m >= 0.0125000005 m`. That is the
kernel's rule shown faithfully, and `Violated` never does the reverse.
It is notation evidence: the spelling shows a margin the decision
treats as zero.

## Ev's answer (2026-10-01, on PR 3602)

> getting rid of all hard-coded nonstandard sites would be great (idk if there are others than the two mm ones you mentioned); i think sticking with m and pi rad is fine, it's easy to change

The ruling has three parts:
- **One working notation.** There is one per-person working notation, a length unit and an angle unit, kept in the viewer's `Prefs` and never in the document. Every value no author wrote reads in it: a driven slot's field, the refusal affordance, a measure's row, and both sides of an assertion. The creation forms' unit pickers become its control, so `Drafts` loses its two unit fields.
- **No hard-coded notation.** Every site that hard-codes a notation reads the working notation instead. That includes `pane::view::camera_mm`, the properties pane's free-move probe, and any other site a census finds.
- **Default.** The default stays m and π·rad.

Two cleanups go with it:
- `unparse`'s dead "canonical unit when it remembers none" clause goes.
- `rendering_unit`'s canonical-choice argument goes.

## Closed

PR 3639: the working notation is `props::Notation` (a length unit and
an angle unit, default `m` and `pi rad`), held by
`session::DocSession::notation` and persisted in `prefs::Prefs` as
`[notation] length`/`angle` by unit symbol; an unknown symbol is
`prefs::Notice::UnknownUnit` and that unit falls back to the default.
The creation forms' unit pickers write it, and `Drafts` holds no unit.
A driven slot's `= value`, `Refusal::affordance`, `tree::Measured::Value`
and `tree::Asserted` (both numbers, carried as `props::Computed` and
spelled when drawn) read in it, and so do the camera readout, the
display δ (field, badge, budget sentence) and the free-move probe.
