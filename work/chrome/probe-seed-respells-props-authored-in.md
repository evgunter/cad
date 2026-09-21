---
id: probe-seed-respells-props-authored-in
kind: issue
title: probe_seed re-spells props::authored_in by hand
status: open
opened: 2026-09-21
priority: P1
cost: E
---



## Finding

`probe_seed` (`crates/viewer/src/session/probe.rs`) resolves an
`Option<UnitDef>` by hand:

```rust
fn probe_seed(unit: Option<UnitDef>) -> f64 {
    unit.map_or(1.0, |unit| props::from_written(1.0, unit))
}
```

That expression is `props::authored_in(unit, 1.0)`. `authored_in`
(`crates/viewer/src/props.rs`) is `from_written` over a field that may
name no unit, and its `None` arm returns the written number unchanged
— which for a written 1.0 is the `1.0` spelled here. The body already
calls `props::from_written`, so half the door is used and the other
half is written out.

This is the inverse direction of the defect
`work/chrome/bounds-reading-respells-the-panels-one-divide.md` records
on the reading side. `shown_in`'s own doc names the shape: *"a
hand-written `map_or` at each is the same identity written twice, free
to become two"*. What it costs is the same: the `None` arm — what a
field that names no unit means — has two homes, so a ruling on what a
count or a bare scalar steps by reaches `props` and not the probe.

Nothing is wrong with the number today; `authored_in`'s `None` arm and
the `1.0` here agree.

## Distinct from the residue already disclosed

`work/chrome/probe-rows-assert-in-one-direction-only.md` discloses a
residue about `probe_seed`: the reach ROW restates one written
millimetre rather than importing the seed, which that row argues is
deliberate. This finding is about `probe_seed`'s own BODY, not about
any row that reads it, and the two are independent: routing the body
through `authored_in` changes nothing a test restates.

## Home

CHROME. `crates/viewer/src/session/probe.rs` (`probe_seed`); the door
it should call is `crates/viewer/src/props.rs` (`props::authored_in`).
`work.py territory` reports the path as `chrome, view, vseam` — a
double claim, so CHROME can take it.

## Found by

CHROME-ONE-NUMBER's half-1 sweep, which looked for an
`Option<UnitDef>` resolved by a hand-written `map_or`/`match` rather
than by a `factor()` call, across `crates/viewer/src/` and
`crates/viewer/tests/`. That shape is what the `factor()` sweep in
`bounds-reading-respells-the-panels-one-divide` could not see here:
this site spells no `factor()` of its own, because the factor is
already inside the `props` call it makes.
