---
id: certified-range-has-no-python-door
kind: issue
title: The certified locally-valid range query has no Python door
status: open
opened: 2026-09-13
refs: [1183]
---

Filed by DOCM-9, which built the kernel side and is forbidden by its
own fence from building either consumer door.

`editor_core::range::certified_range` proves, rather than samples, how
far one field can move before the document stops being the same build.
It is the on-demand counterpart to `viewer::bounds` (Ev's 2026-09-13
pacing ruling, `docs/DOCM-9-SPEC.md`), and `pncad` exposes no way to
ask it.

The signature to wrap, verbatim:

```rust
pub fn certified_range(
    doc: &Doc<ProfileProgram>,
    field: &RangeField,          // Param(ParamName) | Slot { node, slot }
    seed: Seed,                  // offsets from the field's nominal, lo <= 0 <= hi
    config: &DriveConfig,        // the caller's budget: the query is on demand
    tol: Tol,
) -> Result<CertifiedRange, RangeRefusal>;
```

Three things a binding has to carry rather than flatten:

- **The four arms, distinct.** `RangeSide` is `Certified { to }`,
  `NewFailure { certified_to, within, evidence }`,
  `DecisionFlip { .. }` and `Indeterminate { certified_to, within,
  reason }`. `Indeterminate` is NOT a bound — a binding that returns a
  pair of numbers per side would turn "the driver could not decide"
  into "the edge is here", which is the one reading the type exists to
  forbid. `RangeSide::is_bound()` is the discriminator.
- **Offsets, not absolute values.** `Seed` and every reported position
  are offsets from the field's own nominal, because the analyzed axis
  IS offsets and a subtraction would round.
  `CertifiedRange::absolute()` converts for display.
- **The typed refusal.** `RangeRefusal` names a structural slot, an
  expression-driven slot, an unknown node or slot, a seed that does
  not bracket the nominal, and the driver's own `DriveRefusal`. It is
  the shape LIB's existing refusal wrapping already takes.

`DriveConfig` reaches Python already wherever the driver does; if it
does not, that is this row's first piece of work.

## Home

LIB owns `crates/pncad-py` and the binding surface.
