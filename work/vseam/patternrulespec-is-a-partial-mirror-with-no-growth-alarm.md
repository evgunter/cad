---
id: patternrulespec-is-a-partial-mirror-with-no-growth-alarm
kind: issue
title: PatternRuleSpec mirrors two of PatternKind's three arms and nothing tells it when the kernel enum grows
status: open
opened: 2026-09-14
priority: P1
cost: E
---



Found by the sweep that closed
`two-partial-mirrors-in-the-viewer-have-no-growth-alarm`. That row
named two partial mirrors in the viewer with no alarm; the shape grep
it asks for turns up a third, which is not one of the two and so gets a
file rather than a sentence in a PR body.

## The finding

`session::author::PatternRuleSpec` (`crates/viewer/src/session/author.rs`,
`:95`) has `Linear` and `Circular`. The kernel enum it lowers to,
`pncad::document::PatternKind` (`crates/editor-core/src/node.rs`,
`:879`, re-exported through `crates/pncad/src/document.rs:77`), has a
third arm, `Explicit` (`:906`). The absence is deliberate and its
reason is written above the declaration — *"`Explicit` has no arm by
the plan's ruling: a list of absolute frames is not a form's job"* — so
this is a DELIBERATELY PARTIAL mirror, exactly the class the row above
names.

Nothing tells it when `PatternKind` grows. The only viewer match over
either type is `combine::pattern_node`'s (`crates/viewer/src/combine.rs`,
`fn pattern_node` at `:397`, the arms at `:440`), and it is exhaustive
over `PatternRuleSpec` — the direction that is already held. The
direction that is not is kernel-arm-to-spec: a FOURTH `PatternKind` arm
the pattern form should offer lands in another crate, compiles here,
and no row goes red.

That other-crate hop is what makes this the strongest of the three
instances rather than the weakest. `MatePrimitive` is the same case and
has an alarm; `Subject` and `DatumSpec` are the viewer's own types, and
a growth there at least reds a lowering match in the same crate. A
`PatternKind` arm added by a kernel lane reds nothing in the viewer at
all.

`forms::PatternKindChoice` (`crates/viewer/src/forms.rs`, `:54`) is
not a second site: it is TOTAL over `PatternRuleSpec`, and
`pane::create`'s lowering match holds it that way. The partiality is
one hop further out.

## Why the instrument that closed the other two does not just apply

`partial_mirror!`'s `onto` arm (`crates/viewer/src/vocab.rs`) names the
counterpart as a VALUE, so the offering has to be a fieldless enum —
which is what a form's choice enum is, and what `PatternRuleSpec` is
not: both its arms carry `Expr`s. Its own doc states that restriction
and names this site as the case it does not serve.

So whoever takes this decides between two answers, and the choice is
the point rather than a detail:

- **A fourth shape of `partial_mirror!`** that holds a counterpart's
  EXISTENCE by pattern rather than by value (an
  `#[allow(unreachable_patterns)]` match over the counterpart, or a
  `{ .. }` in the roster), which would serve any payload-carrying
  offering and not just this one.
- **Nothing, with the reason written down.** `Explicit` is ruled out by
  the plan and a fourth `PatternKind` arm is not scheduled anywhere on
  this board, so an alarm here may be a cost paid for a growth nobody
  will act on. If that is the answer it belongs in the declaration's
  doc, which today states the absence and not the exposure.

## What is not being claimed

That the partiality is wrong. The plan's ruling stands and this file
does not reopen it. The defect is that the mirror does not survive the
kernel enum growing.
