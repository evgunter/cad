---
id: renders-that-multiply-a-finite-guarded-length-spell-the-product-inf
kind: issue
title: three chrome renders multiply a finite-guarded length and spell the product inf
status: open
opened: 2026-09-16
refs: [render-mm-overflows-to-inf-for-a-delta-the-door-accepts, the-scientific-arm-rounds-out-of-the-type, viewer-substituted-value-class-is-crate-wide]
priority: P1
cost: E
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
