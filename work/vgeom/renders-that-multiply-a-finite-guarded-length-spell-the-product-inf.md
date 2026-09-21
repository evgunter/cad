---
id: renders-that-multiply-a-finite-guarded-length-spell-the-product-inf
kind: issue
title: three chrome renders multiply a finite-guarded length and spell the product inf
status: closed
opened: 2026-09-16
refs: [render-mm-overflows-to-inf-for-a-delta-the-door-accepts, the-scientific-arm-rounds-out-of-the-type, viewer-substituted-value-class-is-crate-wide]
priority: P1
cost: E
branch: vgeom/render-spelling
pr: 3031
closed: 2026-09-21
---



(VIEW) The siblings of
`render-mm-overflows-to-inf-for-a-delta-the-door-accepts`, found by its
sweep and left for a unit of their own. The shape is the one
`the-scientific-arm-rounds-out-of-the-type` measured three times over:
**a guard that admits everything finite, then a multiplication UP, then
a render of the product** — so the text names a value the guard would
have refused, and `inf` is infinitely far from the one it was asked
about.

The δ door answered its own instance by refusing a δ whose millimetre
value is not an `f64` (`DisplayTolerance::new`, and
`MM_PER_METRE` is the factor both it and the render read). None of the
three below has a door to narrow: each renders a value some other type
owns, so the answer is that type's bound or the render's, and that is
the decision this row is for.

## The members

- **`crates/viewer/src/pane/view.rs`'s camera readout**, `mm = |metres|
  readout::number(metres * 1000.0)` over `Camera::distance`,
  `min_distance` and `max_distance`. `Camera::new` checks `finite` and
  `scene_radius >= MIN_SCENE_RADIUS` (`f64::MIN_POSITIVE`) and has **no
  upper bound**; `max_distance` is `scene_radius * MAX_DISTANCE_FACTOR`
  (100.0), so the render is `scene_radius * 1.0e5` and a scene radius
  above about `1.798e303` reads `band …–inf`. The multiplication to
  `inf` happens at `max_distance` itself one decade higher, so the
  value is `inf` before the render sees it — which is the half no
  bound inside the render can reach.
- **`crates/viewer/src/bounds.rs`'s `Bounds::wording`**,
  `readout::number(value / unit.factor())`. The smallest factor in
  `quantity`'s table is `MILLI`, so this multiplies up by a thousand,
  and a probed bound above about `1.798e305` written in millimetres
  reads `inf mm` — a bracket claiming the search reached infinity. The
  probe's own `f64` `number_field` sites carry no `.range()`, so the
  origin a bound is derived from is whatever a user typed.
- **`crates/viewer/src/props.rs`'s `field_text`**,
  `render_number(in_written(value.as_f64(), unit))` where `in_written`
  is `canonical / unit.factor()`. `render_number` is `{:?}`, which
  spells infinity `inf`, so a literal stored above about `1.798e305` m
  and displayed in millimetres shows `inf` in a field whose text is
  what an edit starts from.

## The sweep that produced them, and what it could not match

The population is **renders**, enumerated by call site rather than by
pattern: every call to `readout::number`, `widgets::number_text` and
`props::render_number` under `crates/viewer/src`, plus the `format!`
sites those three replaced. Each was then read for whether its argument
is a product or a quotient, and each such producer was read for its
validation.

**What that could not match**, in the order it matters:

- **No grep finds the reachability half.** The property is *a guard
  that admits everything finite followed by a multiplication up*, and
  nothing about a guard's text says what happens to its value three
  frames later. Every one of the three above was found by reading the
  producer, not by matching anything.
- **A product formed well above the render** arrives already `inf` and
  looks like an ordinary value at the site. `Camera::max_distance` is
  exactly that, and it is the reason the camera member cannot be fixed
  at the render.
- **A render outside this crate.** Other programs' ground.
- `widgets::number_text` renders its argument rather than a product of
  one, so it is not a member of this class; its own open row is
  `the-fields-door-has-no-width-bound-at-all`, about width.

`work/chrome/viewer-substituted-value-class-is-crate-wide` names *"a
division that yields `inf`, a norm that overflows"* as a blind spot of
its own greps rather than as members, so these three are not filed
there.

## The camera readout's factor is also a second spelling (2026-09-21, AUTH-2)

Evidence on the member already listed, not a second member. The camera
readout's `mm = |metres| readout::number(metres * 1000.0)`
(`crates/viewer/src/pane/view.rs`) writes the metre-to-millimetre
factor as a bare literal, where the same conversion has two named homes
in the crate: `scene::DisplayTolerance::render_mm`, which multiplies by
`MM_PER_METRE`, and `props::in_written` over the `mm` row of the closed
unit table. So whatever bound this row settles on for the multiply, the
fix has a third thing to do at this site — read the factor from one of
those rather than restate it — and a reader narrowing the guard at
`render_mm` would not reach this one.

Found by AUTH-2's Q1 sweep for a second unit vocabulary in the viewer
(`docs/AUTH-2-SPEC.md` C7): the pattern was a bare decimal scaling
beside a length, and this was its only hit outside `props`.

## The inverse factor is spelled at four production sites too (2026-09-21, CHROME-ONE-NUMBER)

More evidence on the camera-readout member above, and a second
spelling the section above it could not have found.

CHROME-ONE-NUMBER re-swept for the metre-to-millimetre factor while
routing `Bounds::wording` through `props::shown_in`, and reached the
same site by a different pattern — the literals `1000.0`, `1_000.0`,
`1.0e3` and the name `MM_PER_METRE` across `crates/viewer/`. It
confirms the finding above: `pane/view.rs`'s `mm` closure is the
pattern's only production hit outside `props` and `scene`.

**What that pattern is structurally blind to is the INVERSE**, and
nothing in either sweep said so. A field that parses millimetres
commits `mm * 1.0e-3`, and four production sites spell that
themselves:

- `crates/viewer/src/pane/view.rs`, the δ request commit
  (`*delta_request = Some(mm * 1.0e-3)`).
- `crates/viewer/src/pane/view.rs` again, the
  `DisplayTolerance::new(delta_mm * 1.0e-3)` beside it.
- `crates/viewer/src/pane/viewport.rs`, `DisplayTolerance::new(mm * 1.0e-3)`.
- `crates/viewer/src/widgets.rs`, `frame_of`'s `mm.map(|v| v * 1.0e-3)`.

So the camera readout is not one restatement of a factor with two
named homes — it is one of six sites, in two directions, around a
constant that names only one of them.

**The direction matters for whatever bound this row settles on.** The
factor above multiplies UP and is how the `inf` is reached; these four
multiply DOWN and cannot overflow. They are not new members of this
row's class. They are here because the fix to the camera member —
read the factor from a named home rather than restate it — has to
decide what the named home for the inverse is, and there is no
`MM_PER_METRE`-shaped answer today.

**Not a silent hazard, measured.** `scene::MM_PER_METRE` set to
`1.0e6` reddens four rows in `display_budget.rs`, including
`the_door_refuses_a_delta_whose_millimetre_value_is_not_one`, which
is the row that measures the two factors as inverses against its own
literals. So the commit sites diverging from the constant is caught
today; what is missing is a home, not a guard.

`scene::MM_PER_METRE`'s own doc claimed to be *"the one factor the δ
render and the δ door both read"*, which reads as a crate-wide claim
it never held; CHROME-ONE-NUMBER narrowed that sentence in the same
PR and pointed it at this row.

## Closed — the notation is asked whether it can name the value

**The decision, whose bound narrows a product: none of the three, and
that is the answer rather than a deferral.** Each member renders a
value some other type owns, and the row is right that the render
cannot narrow what it is handed. What a render CAN own is the
conversion it performs itself — and all three perform one. So the
repair is at the conversion, not at the value: `props::written` forms
`canonical / unit.factor()` and answers `None` when the result is not
a number, `props::no_reading` is the one spelling of that refusal, and
`props::written_text` is the two together over
`crate::readout::number`. `inf` is gone from all three sentences.

Per member, since the row asks for that:

- **`pane/view.rs`'s camera readout.** Now `camera_mm`, a named
  function rather than a closure, reading the factor from the unit
  table (`props::written_text(metres, MM.def())`) — which discharges
  the AUTH-2 half of this row: the metre-to-millimetre factor had two
  named homes and a bare `1000.0` here was a third.
  `the_camera_readout_writes_metres_in_the_tables_millimetre` holds
  the render to `scene::MM_PER_METRE`, the other home, so a third
  spelling reds. **The other half of this member is not a render
  question and is filed**: `Camera::max_distance` is
  `scene_radius * MAX_DISTANCE_FACTOR` and `Camera::new` admits every
  finite radius, so above `f64::MAX / 100.0` the band's top arrives
  here already `inf` —
  `camera-new-admits-a-scene-radius-whose-distance-band-is-not-finite`,
  P1/E, held out because `camera.rs` was another lane's this wave.
  **The inverse factor stays as it is**, and this lane's own sweep
  reached the same four sites CHROME-ONE-NUMBER lists above and
  disposed of them the same way: `mm * 1.0e-3` is a parse into
  canonical rather than a render, it multiplies DOWN and cannot
  overflow, and `DisplayTolerance::new`'s doc names that exact
  spelling as the thing its bound coincides with — so a home for the
  inverse is a question this row does not answer and does not need
  to. `* 1.0e-3` and `/ MM_PER_METRE` are not the same bits, which is
  the cost whoever takes it has to weigh.
- **`bounds.rs`'s `Bounds::wording`.** Through `props::written_text`,
  which also deletes the hand-written `value / u.factor()` this file
  carried — a second copy of `props::in_written`. The doc now says
  that no door upstream owns a bound here either: a probed bound is
  where a doubling search reached from a field with no `.range()`.
  `a_bound_with_no_millimetre_value_is_not_worded_as_infinity`.
- **`props.rs`'s `field_text`.** `written` over `render_number`, with
  `no_reading` where there is no number. `{:?}` stays the render for
  this one: a field's text is what an edit starts from, so it wants
  exact round-tripping digits, and a sentence does not.
  `a_literal_with_no_millimetre_value_does_not_show_one`.

**The other end of the same question, which the row did not have.**
The divide is a multiplication up for six of the closed table's eight
rows, and one row's factor is ABOVE one — `pi rad`, at π. So a
canonical angle of exactly `5e-324` rad written in `pi rad` divides to
`0.0`, and a text reading zero is the other thing
`readout::REL_TOLERANCE` refuses. Measured: `5e-324 / π == 0.0`,
`1e-323 / π == 5e-324`. `written` answers `None` for both directions
and the row pins both edges as products.

**The sweep, re-derived rather than quoted, and where it moved.** The
hit list is in the PR body. Two findings:

- `pane/view.rs`'s `{:.1}°` angle pair is NOT a member, and the reason
  is reachability rather than taste: `to_degrees` multiplies up by
  180/π, but `Camera::yaw` answers in `[−π, π)` and `Camera::pitch`
  inside `±(π/2 − margin)`. Stated at the site.
- **The row's population rule, applied to `number_text`'s call sites,
  concluded that door is not a member — and the rule applied to the
  `number_field(` call sites finds two more.** `widgets::named_field`
  and `widgets::value_field_ops` each bind a value that IS
  `in_written`'s quotient, so a field can show `inf` for a value the
  document holds. Filed as
  `a-field-bound-to-a-written-value-shows-a-product-that-overflowed`
  (P1/D): the repair is a door decision about what a field is when its
  notation cannot name its value, and it is not a string a render
  composes.

**And one false universal, corrected where it stood.**
`readout::number`'s doc said *"Nothing in the chrome hands this one"*
of a non-finite value. It now names its three callers and what each
guarantees, which is the sweep rule that produces that population.

## The second pass, shaped at the stated gap

`docs/prompts/implementer-discipline.md` §5 asks that a named blind
spot be looked into rather than only named. The sharpest one here is
this row's own: *no grep finds the reachability half.* Two passes
were shaped at it, from the two ends, and **neither found a fourth
member of the render class.**

**Backwards, from the arithmetic.** Every multiplication UP in
`crates/viewer/src` — a named screaming-snake factor, a literal
factor, a squaring (`powi(2)`, `x * x`), a `.sqrt()` over a sum of
squares, and `to_degrees` — read for whether anything bounds its
operand and whether its product reaches a text. Hits and disposition:

| producer | disposition |
|---|---|
| `scene.rs`'s `delta * MM_PER_METRE` (twice) | guarded at `DisplayTolerance::new`, on the product |
| `scene.rs`'s `PROBE_FACTOR * constant / budget` | goes through `DisplayTolerance::new` |
| `camera.rs`'s `scene_radius * MAX_DISTANCE_FACTOR` | the member filed as `camera-new-…-not-finite` |
| `camera.rs`'s `radius * FRAMING_MARGIN / half.sin()` | can be `inf` for a fov near zero, and `clamp_distance` clamps it back into `[r·0.05, r·100]` before anything reads it — bounded downstream, not a member |
| `camera.rs`'s `scene_radius * MIN_DISTANCE_FACTOR`, `distance * NEAR_FACTOR` | multiply DOWN; cannot overflow |
| `camera.rs`'s `sphere` radius | closed by `finite-bounds-yield-an-infinite-scene-radius` |
| `datums.rs`'s depth norm and `metres_per_pixel * TARGET_PITCH_PX`; `scene.rs`'s normal norm; `pane/viewport.rs`'s `at_offset`; `pickindex.rs`'s distances | all feed the PICTURE, never a text — outside this class by its own subject. `pane/viewport.rs`'s `at_offset` is the site `vgeom/deletions` disposed of as *"not a member"*, and this pass agrees, for the same reason from the other direction. `datums.rs`'s norm is `work/chrome/metres-per-pixel-at-hand-rolls-a-norm-the-file-already-calls` |
| `sketch.rs`'s two `hypot` calls | `hypot` is overflow-safe by construction; not the shape |
| `pane/features.rs`'s `depth.min(INDENT_MAX_DEPTH) as f32 * INDENT_STEP` | bounded by the `min` |
| `pane/view.rs`'s `to_degrees` pair | `yaw` is in `[−π, π)` and `pitch` inside `±(π/2 − margin)`; the product cannot leave the type |

**Forwards, from every text.** 98 production sites in
`crates/viewer/src` write a number into a text a person reads
(`ui.label`/`ui.weak`/`format!` with an interpolation). Filtered for
an argument that is ARITHMETIC rather than a value read as held,
exactly one remains: `frame.rs`'s `δ {} mm chosen`, which is
`DisplayTolerance::render_mm` — the guarded door. Every other
rendered number is a count, a name, or a value handed through one of
the three renders this unit bounded.

**What the second pass still could not reach.** A product formed in
another crate and handed here as an ordinary value: the forward pass
sees the render and the backward pass sees only this crate's
arithmetic, so a `pncad` door that multiplies up and returns a
plausible number is invisible to both. Nothing was found, and nothing
was looked for outside `crates/viewer/src` — that is other programs'
ground, and it is a gap rather than a negative result.

PR: `vgeom/render-spelling`, on `vgeom/p0-fields` (#3007) as its base.
