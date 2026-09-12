---
id: three-length-renders-outside-the-field-can-read-as-zero
kind: issue
title: the badge, the budget sentence and the probe bracket render a length at fixed precision, so each can still read as 0.000
status: open
opened: 2026-09-12
---


(VIEW) The three render-only siblings of
`delta-field-renders-a-sub-micrometre-delta-as-zero`, given a file of
their own now that the fix for that row exists and they do not use it.
Reported as siblings by PR 2366's lane, verified at the close of
`seeded-draft-is-the-commit-path-and-does-not-round-trip`, and
re-verified here against this tree.

## The three

- `crates/viewer/src/frame.rs:1376` — `delta_badge`,
  `format!("δ {:.3} mm chosen", fitted.delta.get() * 1.0e3)`. This one
  is the sharpest of the three: the δ it announces is the budget's own
  choice, `constant / TRIANGLE_BUDGET`, so a body whose cost constant
  is under a triangle·millimetre gets a badge reading *"δ 0.000 mm
  chosen"* — a number `DisplayTolerance::new` refuses, in the sentence
  whose whole job is to say which δ the picture is at.
- `crates/viewer/src/scene.rs:946` — `FittedDelta::wording`, the same
  δ and the requested one at `{:.3}` in the badge's detail. (Cited as
  `scene.rs:878` when the parent row filed it; the fix for the field
  moved it.)
- `crates/viewer/src/bounds.rs:216-221` — `Bounds::wording`'s
  `{written:.4}`, a probed bound divided by its display unit's factor.
  A bound under 0.1 µm written in mm reads `0.0000 mm`.

## What the field's fix does and does not give them

`DisplayTolerance::render_mm` (`crates/viewer/src/scene.rs`) is the δ
render that cannot read as a number δ cannot be: the shortest decimal
spelling that fits ten characters and reads back, through the
millimetre conversion, as a δ the door accepts within four significant
figures, and a scientific spelling when no decimal one does.

- The first two sites hold a `DisplayTolerance` and can call it
  directly. What stops that being this unit's work is not the call: it
  is that both sites render δ inside a sentence, and a sentence
  carrying `4.000e-4` where it used to carry `0.000` is a wording
  decision about a badge and a status line, not a field's width.
  `crates/viewer/tests/display_budget.rs`'s
  `a_coarsened_picture_says_so_in_both_numbers` builds its needles with
  `format!("{:.3}", …)` and would want re-pointing at the same time.
- `Bounds::wording` cannot use it at all: its subject is a probed
  length in the user's own display unit, not a δ in millimetres, so it
  needs the same RULE — prefer a spelling that reads back as the value
  it renders — at its own precision and unit. A shared helper over
  `(value, precision)` is the shape if all three land together; whether
  the probe bracket wants one at all is its module's call, since its
  own doc is careful that a bracket reads as *the furthest value found
  valid* rather than as an exact number.

Not swept with the field because the field was the only member an edit
could reach, and because a wording change to the badge and the status
line is a different review from a control's arithmetic.
