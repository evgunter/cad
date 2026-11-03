---
id: a-linear-patterns-direction-components-land-in-its-slides
kind: issue
title: the linear-pattern gesture writes a typed direction's components into slide formulas, unnormalized, so a diagonal row steps spacing·√2
status: closed
opened: 2026-11-02
priority: P1
cost: M
pr: 6394
branch: placing/values-direction
closed: 2026-11-03
---


Found by the PLACING sweep Ev asked for on PR 6281 (2026-11-02): "can you see if anything similar also got missed?" by the INTENT refactor. Read at `3bf15bef9c`; traced in code, not run unless said.

**Ruled.** `[variables.poses]`: "no slot holds a component of a direction". `[repetition]`: a row is "a slide of `scalar(k)·pitch`".

**Survives** (sure on the code; the pitch effect is traced, not run).
- `combine::row_steps` (`crates/viewer/src/combine.rs`) writes `Slide(axis_i) = spacing·d_i` for each nonzero component of the typed direction.
- Those components are `Formula::literal(…, Scalar)` from `drafts::scalars` (`crates/viewer/src/drafts.rs`), and `pncad::pattern::linear_pattern` multiplies each by `k`.
- So each direction component is a typed Scalar in a stored slot.
- A direction that is not a unit vector steps by `spacing·|d|`, so `(1,1,0)` steps `spacing·√2`. Every `combine_ops` row uses `(1,0,0)`, so nothing pins it.

**Owed.** First a row that reproduces it with a diagonal direction (if the pitch is right after all, close this as the prose half only). Then the row's direction as a pose read, a `Direction` or an `Axis`, with one slide of `scalar(k)·spacing` along it. The circular twin is `a-ring-about-a-tilted-axis-slides-each-member-along-it`, a different defect.

## Reproduced (2026-11-02, `placing/pattern-direction`)

Run, at `c3626bf54a`. `combine_ops::a_row_steps_its_spacing_along_an_oblique_direction`
(disabled red, naming this file) types `(1, 1, 0)` at spacing 50 mm:
member 1 stands at `(0.05, 0.05, 0)`, 70.7 mm away, not
`(0.0354, 0.0354, 0)`. `(0, 3, 4)` steps 250 mm. The axis-aligned row
and a pitch read from a variable stand right and are gated
(`combine_ops::a_row_along_an_axis_steps_its_spacing_and_reads_its_variable`).

## The spelling the row asks for has no door (a design question)

The repair asked for is one slide of `scalar(k)·spacing` along a read
direction. Today's vocabulary has two places a slide can live, and
neither holds it within the ratified text:

- **A placement value.** `Coord::Slide` runs along a target axis, as
  the least turn carries it beside a mate (`[placement.values]`,
  `crates/editor-core/src/mate/value.rs` module docs). A carried axis
  lands on the mated direction only "where the source's own mated
  direction is one of its axes"; for a direction mate or axis mate whose
  copy side reads the row's own direction `d` (so the member is not
  turned), the least turn is the identity and each slide measures the
  copy's origin along `x`, `y` or `z`, so `Slide(x) = scalar(k)·s`
  steps `s/d_x` along `d`: the same defect from the other side. This is
  the gap the circular twin names
  (`work/msolve/a-ring-about-a-tilted-axis-slides-each-member-along-it.md`,
  "a slide measured along the mated direction (the chart's own)").
- **A pose.** `PoseDef::Slide { pose, along, by }` is a slide along a
  `Direction`/`Axis` read, and a row written as one frame mate
  `F ≡ moved_frame(F, [Slide(z, scalar(k)·s)])`, `F` the typed direction
  read off the body as a frame's `z` (`DocSession::read_off`, as the
  ring's axis is), is admitted and stands each member exactly `k·s`
  along the unit direction (built and run on this branch, then taken
  out). But `[repetition]` says "no pose is constructed from an index"
  (FORK-PAT, `fa0815a177`): an index enters a placement as a value or
  through a mate onto another family's member. And the pitch leaves the
  placement's values, so the properties panel shows no per-member row
  for a row (`panel_display`'s four pattern rows go red:
  `per_member_row` finds none).

So the fix is one of: a value that slides along the mated direction (the
twin's first repair shape, which a ring needs too); or a ruling that a
slid pose may read an index, with the panel reading a mate target's
slide. Either is a revision of `[placement.values]` or `[repetition]`.

The typed numbers are a second, smaller question: the only door that
turns them into a direction is `DocSession::read_off`, itself open as
`work/held/the-viewers-numeric-gestures-read-a-frame-back-off-geometry.md`.

## Weighed by three designers (2026-11-02, fork row 244) and put to Ev

**Agreed by all three.** A placement value names the direction it runs along or the axis it turns about:
- a slide runs along the mated direction (`Slide(None)`), along a target axis carried by the least turn, or along a `Direction`/`Axis` read off a body in the target space;
- a spin may turn about a read `Axis`.

The copy stands at the sum of its slides. A row's source is its first copy `h0`, slid `scalar(k)·pitch` along the picked edge, with every other value `0`. A direction typed as numbers has no home: the gesture offers a pick or a target axis instead.

Only `[placement.values]` changes. The `[ev]` PR carries the diff and the three designers' texts. PR 6364 holds the disabled-red reproducer and the axis-aligned gate.

## Ev's answer (2026-11-02, `[ev]` PR 6369): "seems good!"

The `[placement.values]` change is ratified. Build it:
- the slide forms: mated (`Slide(None)`), a target axis, or a read Direction/Axis;
- spins about a read Axis;
- the sum reading;
- spans of read directions decided at evaluation;
- the pattern gesture taking a pick or a target axis, never typed components.

Un-ignore `combine_ops::a_row_steps_its_spacing_along_an_oblique_direction`. MSOLVE's `a-ring-about-a-tilted-axis-slides-each-member-along-it` closes with the same build.
