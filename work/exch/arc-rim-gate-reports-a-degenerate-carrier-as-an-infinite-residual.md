---
id: arc-rim-gate-reports-a-degenerate-carrier-as-an-infinite-residual
kind: issue
title: step-import arc-rim gate reports a degenerate CIRCLE carrier as an infinite residual
status: open
opened: 2026-09-12
refs: [step-adopt-let-ok-iso-discards]
---

## What

`crates/step-import/src/adopt.rs`'s `arc_rim_on_wall_boundary` — the
import-side residual gate for an ARC rim against its NURBS wall —
opens by screening its own carrier:

```rust
if !(radius.is_finite() && radius > 0.0)
    || !(axis_norm.is_finite() && axis_norm > 0.0)
    || !(u_ref_norm.is_finite() && u_ref_norm > 0.0)
{
    return Err(ArcRimRefusal::Residual(f64::INFINITY));
}
```

(`crates/step-import/src/adopt.rs:890-895` at the head of
`door/step-adopt-iso-discards`; the same lines carried
`return Err(f64::INFINITY)` before it.)

The caller renders that as
`StepImportError::RimOffWallBoundary { id, residual: f64::INFINITY }`,
whose message reads *"the wall's own boundary column, sampled at the
certification schedule, deviates from the rim's circle by up to inf m
(ambient tolerance exceeded)"* (`crates/step-import/src/error.rs:399`).

Nothing was sampled and nothing deviated. The fact is that the file's
`CIRCLE` has a non-positive or non-finite radius, or a zero/non-finite
axis or reference direction — a **degenerate carrier**, which is a
statement about the parsed entity, not a verdict of the residual gate.
The reader has a vocabulary for that shape already
(`StepImportError::Topology { id, what }`, `MalformedRecord`), and D4 ¶3
wants the variant to name the subject as precisely as the failure has
one.

It is the same defect class as `step-adopt-let-ok-iso-discards`, which
this issue is residue of: there a **structural** fault about the wall
was dressed as the gate's own verdict; here a **carrier** fault is. That
row converted the two `boundary_iso_*` discards and deliberately did
not widen to this line.

## Why it matters

A `residual: inf` invites the reader to look for a rim that is far from
its wall. The recourse for a degenerate circle is different (the file's
`CIRCLE` entity), and the id the refusal names is the `EDGE_CURVE`, not
the carrier entity that is actually wrong.

## Measured, so the scope is known

**This is a diagnostics defect, not a soundness hole.** Executed on
`door/step-adopt-iso-discards`: `tests/fixtures/freecad/cylinder.step`
with `#24 = CIRCLE('',#25,0.5)` rewritten to radius `0.` and to `-0.5`,
imported at `Tol::witness()`. Both refuse, and neither reaches this
gate — the ladder gets there first:

```
step import: edge #21: no intensional description certifies —
intersection: geometry attachment gate: certification: the stored
parameter interval is not forward … and a degenerate zero-span interval
is refused by the same gate; tangent intersection: <the same>
```

So a degenerate `CIRCLE` cannot ride into a body; what it cannot do is
say the word *radius*. Note also that the parse arm has no screen at all
(`crates/step-import/src/entities.rs:918-930`, and `as_length` at `:369`
scales without metering) — unlike `CONICAL_SURFACE` five hundred lines
above it, which refuses `!(radius.is_finite() && radius >= 0.0)` in its
own arm (`:640`).

## Shape of the fix

Two candidates, and the choice is EXCH's:

- screen the radius (and the placement's axis/`u_ref` norms) in the
  `CIRCLE` parse arm, the way `CONICAL_SURFACE` already does, so the
  refusal names the offending entity instead of an edge three phases
  later — after which the gate's `INFINITY` screen is genuinely
  defensive and can say so;
- or leave the parse alone and give this screen its own refusal arm, so
  it stops borrowing `RimOffWallBoundary`'s sentence.

Either way the `Err(ArcRimRefusal::Residual(f64::INFINITY))` line stops
claiming a measurement it never made.
