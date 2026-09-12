---
id: delta-field-renders-a-sub-micrometre-delta-as-zero
kind: issue
title: the δ field renders a δ below 500 nm as 0.000 mm, and an edit-then-undo commits that render
status: closed
opened: 2026-09-11
closed: 2026-09-12
refs: [2366]
---


(VIEW) Residue of `seeded-draft-is-the-commit-path-and-does-not-round-trip`,
disclosed and filed in the PR that closed it.

## What is left

`crates/viewer/src/pane/view.rs`'s `delta_field` no longer seeds its
draft with the rendered δ, so a field nobody typed into commits
nothing. The render itself is unchanged and is still `{:.3}` over
millimetres, so:

- **It is wrong on screen below 500 nm.** δ = 0.4 µm reads `0.000` in
  the View pane's field, which is a number no δ may be
  (`crates/viewer/src/scene.rs:75`, `DisplayTolerance::new`, refuses
  `0.0`). The field is the one place a user reads the δ in force as a
  number they can act on.
- **And one path still commits it.** The render is the text an edit
  starts from, so a user who types a character into the field and
  deletes it again leaves the render in the box as their own draft,
  and that draft commits: δ = 1.6 µm becomes 2 µm. Narrower than the
  focus-and-leave the parent closed — it takes two keystrokes that
  cancel out — but it is the same quantisation, by the same render.

## Why it was not fixed with the parent

The parent's defect is that the SEED was the commit path; this is that
the RENDER is lossy, which is the family the parent files under
"siblings — same shape, prose only, lower stakes"
(`crates/viewer/src/bounds.rs:216-222`, `crates/viewer/src/frame.rs:1376`,
`crates/viewer/src/scene.rs:878`). Verified while closing the parent:
all three of those sink into `ui.weak` or a `Badge` and nothing parses
them back, so they are render-only as the parent says. This row is the
fourth member and the only one an edit can reach.

## What a fix cannot be

Not more decimals, and not the exact spelling either. A round-trip
seed does not close it: seeding the shortest round-tripping spelling of
`δ · 1e3` and parsing it back through `· 1e-3` returns a different
`f64` for **4,155 of 28,600** sampled δ in [1e-12, 1e-1] m (14.5%),
every one of them by exactly 1 ULP, because the lossy step is the unit
conversion and not the format. A preimage spelling exists for 97.7% of
those (within 1 ULP of the naive product) and for **669 of 28,600**
(2.3%) there is no `f64` millimetre value at all whose product with
`1e-3` is the δ in force. So the property to aim at is a render that
never reads as a number δ cannot be — not a render that round-trips.

## Closed

Both halves, by two changes that do not depend on each other.

**The render.** `DisplayTolerance::render_mm`
(`crates/viewer/src/scene.rs`) is the δ-as-millimetres text, and the
property it holds is the one this row asked for rather than a
precision: the shortest decimal spelling that fits ten characters AND
reads back — through the `mm * 1.0e-3` the field commits with — as a δ
`DisplayTolerance::new` accepts, within the render's own stated
accuracy; a `{:.3e}` spelling carries every δ no decimal one can. So
the choice of shape is made by reading the candidate back through the
door that refuses zero, not by a threshold that could go stale against
the format. δ = 0.4 µm now reads `0.0004`, δ = 1.6 µm reads `0.0016`,
δ = 1 pm reads `1.000e-9`, and no δ in `[5e-324, 1e3]` m renders as a
number the door refuses (`crates/viewer/tests/display_budget.rs`,
`no_delta_renders_as_a_number_a_delta_cannot_be`, which sweeps the
grid this row's arithmetic used).

The render is still a rounding — four significant figures is what ten
characters buy, and a budget δ is `constant / TRIANGLE_BUDGET`,
seventeen. That is the reason it stays a render and never a commit
path, and the reason the field is now 88 points rather than 56: a
ten-character render needs 73.3 points of text area and the old field
offered 48, so a render past six digits was clipped, and a clipped
render reads as a different δ (`the_field_shows_the_longest_render`
measures both numbers through egui's font metrics).

**The commit path.** A draft that reads as the render commits nothing.
The render is the text the box already held, so a field typed back to
it carries no number the render does not — and since the render is a
rounding of δ, committing one could only move δ to a coarser spelling
of itself, which is no δ's user intent. A user who means to re-assert
the displayed number types any other spelling of it (`0.050` for a
render of `0.05`) and that commits like any other draft. Both rows are
in `crates/viewer/src/pane/view.rs`'s test module:
`a_draft_typed_back_to_the_render_commits_nothing` (δ = 1/30 of the
starting δ, whose render is a rounding, so the guard is what stops
1.6666…e-6 becoming 1.667e-6) and
`another_spelling_of_the_rendered_delta_still_commits`.

What was NOT done: the three render-only siblings still spell their
own fixed precision, and have their own file,
`three-length-renders-outside-the-field-can-read-as-zero`.
