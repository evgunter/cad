---
id: the-fields-door-has-no-width-bound-at-all
kind: issue
title: number_text has no width bound at all: a large field value is spelled in hundreds of characters
status: closed
opened: 2026-09-16
priority: P0
cost: E
closed: 2026-09-21
branch: vgeom/p0-fields
---


(VIEW) `crates/viewer/src/widgets.rs`'s `number_text` is the fields'
door: `egui::Style::number_formatter` for every `DragValue` and `Slider`
in the chrome (`install_number_formatter`). It keeps the widget's own
spelling wherever `readout::reads_back` accepts it, and falls to
`readout::number` otherwise. **`readout::reads_back` judges truth and
says so** — *"Width is no part of reading back, which is why it is no
part of this"* — so the widget's spelling is taken at any width.

That is unbounded, and it is already asserted. The widget's spelling of
`1.0e9` in the millimetre range is `1000000000.0`, twelve characters
against a `readout::MAX_CHARS` of ten, and
`nothing_at_or_above_one_display_unit_renders_differently` sweeps to
`1.0e9` asserting exactly that text. At the top of `f64` it is far
worse: `emath::format_with_decimals_in_range` returns
`format!("{value:.1}")` on its first iteration (`almost_equal(inf, inf)`
is `a == b`, `emath-0.36.1/src/lib.rs:255-262`), which is the exact
decimal expansion — **311 characters** — and that parses back to the
value, so `number_text` returns it and `readout::number` is never
reached.

**So `MAX_CHARS` is not a crate-wide width guarantee and never was.** It
bounds one function's decimal search; the δ field's `FIELD_WIDTH` is the
only widget sized against it, and it is fed by `render_mm`, not by this
door. The chrome's parameter field also has no `.range()` —
`pane/properties.rs`'s three `number_field` sites are unranged, and only
`pane/create.rs`'s integer pattern count carries one — so the value
reaching this door is whatever a user typed.

**What a repair has to decide** is whether a *field* owes a width bound
when a *label* does not, which is the question
`the-scientific-arm-rounds-out-of-the-type` answered for `readout::number`
and left open here. Note the asymmetry that unit turned on: a clipped
draft still holds and commits the whole text, so width in a field costs
legibility rather than correctness — which is an argument for leaving
this alone and stating it, not only for fixing it. What is not
defensible is the current silence: `number_text`'s doc argues at length
about truth and says nothing about width, and the row over it asserts a
twelve-character text without remarking on it.

Not a regression: pre-dates the third arm and is untouched by it.

## Resolution (2026-09-21, `vgeom/p0-fields`)

**Bounded, and by the bound this crate already had.** `number_text`
keeps the widget's own spelling wherever it reads back **and fits
`readout::MAX_CHARS`**, and falls to `readout::number` otherwise. The
row's question — whether a field owes a width bound when a label does
not — is answered by not minting a second policy: `readout`'s own
header already calls this function *the fields' door* onto the one
rule, and `MAX_CHARS`' doc already says what a box narrower than it
costs. The door was applying the truth half of that rule and not the
width half.

**What the item said was not defensible is what changed.** The silence
is gone: `number_text`'s doc now argues width beside truth, and the row
that asserted a twelve-character text asserts the bound instead.

**The band, re-derived rather than quoted.** A millimetre field's range
is `1..=3` and its narrowest spelling is `{:.1}`, so the widget's text
passes ten characters at a magnitude of `1e8` — `1e7` once a sign is
spent — and not before. Measured: `f64::MAX` was **311 characters**
(`emath` compares in `f32`, and `almost_equal(inf, inf)` is `a == b`,
so its first candidate is accepted) and is now the render's own
twenty-two. `1e300` was 302 and is `1.000e300`. `1e8` was
`100000000.0` and is `100000000`.

**The drag is still not asked the question the rule avoids.** A
`DragValue` at `FIELD_DRAG_SPEED` moves half a millimetre per pixel, so
reaching `1e7` mm takes twenty million pixels of dragging: the band the
bound substitutes in is out of a drag's reach at the top exactly as the
sub-millimetre band is at the bottom.

**It is not a clip.** A `DragValue` renders its text through a
`TextWrapMode::Extend` button and its keyboard edit through a
`clip_text(false)` `TextEdit`, so the over-wide text pushed the panel
out rather than being cut off. No numeric field in the chrome carries a
`desired_width` at all — the two that do (`app.rs`, `pane/properties.rs`)
are name `TextEdit`s, and `pane/view.rs`'s `FIELD_WIDTH`, which IS sized
for `MAX_CHARS`, is fed by `render_mm` and not by this door. So nothing
needed widening to receive the bounded text; that was checked rather
than assumed.

**Rows**: `widgets::field_tests::a_field_spells_no_more_than_the_render_bound_covers`
(each value asserted to be a row only because the widget's own spelling
does not fit, then held to `readout::number`'s answer) and
`the_top_of_the_type_is_the_render_bounds_own_exception` (22 characters,
against the widget's 311). `nothing_at_or_above_one_display_unit_renders_differently`
is re-baselined to the new ceiling with the derivation written at it.

**Mutation**: dropping the `chars().count() <= MAX_CHARS` conjunct reds
those two rows and nothing else.

PR: `vgeom/p0-fields`.
