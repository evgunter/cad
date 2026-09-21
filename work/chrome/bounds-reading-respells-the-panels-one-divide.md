---
id: bounds-reading-respells-the-panels-one-divide
kind: issue
title: BoundsReading::wording re-spells props::shown_in by hand
status: open
opened: 2026-09-21
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
