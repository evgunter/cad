---
id: bounds-reading-respells-the-panels-one-divide
kind: issue
title: BoundsReading::wording re-spells props::shown_in by hand
status: closed
opened: 2026-09-21
branch: chrome/one-number-one-home
cost: E
pr: 3022
closed: 2026-09-21
---



## Finding

`BoundsReading::wording` (`crates/viewer/src/bounds.rs`) writes the
canonical-to-written conversion by hand:

```rust
let written = crate::readout::number(unit.map_or(value, |u| value / u.factor()));
```

That expression is `crate::props::shown_in(unit, value)`, character for
character in meaning — `shown_in` is `in_written` over a field that may
name no unit, and `in_written` is `canonical / unit.factor()`. `props`'
module docs call the pair "the parser's own one-multiply semantics in
both directions" and say every value crossing the module goes through
them; `shown_in`'s own doc says it is "spelled once because both panel
fields need it and a hand-written `map_or` at each is the same identity
written twice, free to become two". This is the third site, and it is
the hand-written `map_or` that doc names.

Nothing is wrong with the number today. What it costs is that the
divide has two homes: a change to how a written value is derived from a
canonical one — the dimensionless row, a unit whose factor is not a
plain multiply — reaches the panel fields and not the range reading.

## Home

CHROME. `crates/viewer/src/bounds.rs` (`BoundsReading::wording`); the
door it should call is `crates/viewer/src/props.rs`
(`props::shown_in`). Separate from
`work/author/a-negative-extrude-distance-probes-as-valid.md`, which is
about what the probe ADMITS rather than how the reading is written.

## Found by

AUTH-2's Q1 sweep for a second unit vocabulary in the viewer
(`docs/AUTH-2-SPEC.md` C7). The sweep's pattern was `factor()` across
`crates/viewer/src/`; the hits were `props::in_written`,
`props::from_written` and this one.

## Measured

`BoundsReading::wording` now calls `props::shown_in`; the
hand-written `map_or` is gone. The rewrite is an identity arm for arm,
including the `None` arm, a zero factor and a NaN value.

The reading is wired to the door rather than agreeing with it by
coincidence, and **both halves of that were measured**: forcing
`props::in_written` to `canonical / (2.0 * unit.factor())` reddens
`valid_range::a_bound_too_fine_for_four_decimals_is_still_said`, and
the same mutation with `wording` reverted to the hand-written `map_or`
leaves it **green**. The second half is the one that matters — it is
what says the door was not already reaching this site.

The sweep this unit ran was shaped for the case a `factor()` pattern
cannot see — an `Option<UnitDef>` resolved by a hand-written
`map_or`/`match`, and a unit factor spelled as a bare literal — over
`crates/viewer/src/` and `crates/viewer/tests/`. Two further sites,
both outside this row:

- `session/probe.rs`'s `probe_seed` writes `props::authored_in` by
  hand, in the inverse direction
  (`work/chrome/probe-seed-respells-props-authored-in.md`).
- `pane/view.rs`'s camera readout spells the metre-to-millimetre
  factor as `1000.0` beside `scene::MM_PER_METRE`. **Already filed**,
  by AUTH-2's sweep on the same day, as a section of
  `work/vgeom/renders-that-multiply-a-finite-guarded-length-spell-the-product-inf.md`;
  this unit's evidence went onto that row rather than into a second
  one. What it added there is the INVERSE spelling, which both
  sweeps' patterns were structurally blind to: four production sites
  commit `mm * 1.0e-3` themselves.

**What the sweep could not match**: a conversion that computes its
factor rather than spelling one; a conversion performed in another
crate on the viewer's behalf; and `.map(|u| u.symbol())`, which
resolves the same `Option` but converts nothing, so the shape is
indistinguishable from the defect by pattern alone and was separated
by reading.
