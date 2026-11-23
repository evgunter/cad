---
id: certified-range-has-no-python-door
kind: issue
title: The certified locally-valid range query has no Python door
status: closed
opened: 2026-09-13
closed: 2026-11-22
refs: [1183]
priority: P3
cost: M
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
    seed: RangeSeed,                  // offsets from the field's nominal, lo <= 0 <= hi
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
- **Offsets, not absolute values.** `RangeSeed` and every reported position
  are offsets from the field's own nominal, because the analyzed axis
  IS offsets and a subtraction would round.
  `CertifiedRange::absolute()` converts for display.
- **The typed refusal.** `RangeRefusal` names a structural slot, an
  expression-driven slot, an unknown node or slot, a seed that does
  not bracket the nominal, and the driver's own `DriveRefusal`. It is
  the shape LIB's existing refusal wrapping already takes.

- **The condition the answer holds under.** `CertifiedRange::pinned()`
  names the parameters whose declared spread the derivation cleared so
  the drive had one axis. A binding that drops it hands a caller a
  range that reads as unconditional and is not.

`DriveConfig` reaches Python already wherever the driver does; if it
does not, that is this row's first piece of work.

## What this row also carries

**Removing the family from `NOT_CARRIED`.** The document-layer export
guard (`crates/pncad/tests/all.rs`,
`every_document_layer_root_export_is_carried_or_listed`) makes someone
decide about every new root export of `editor-core`. The seven names
above are listed there as deliberately interior with this row named as
the reason, so landing the door means carrying them through
`crate::analysis` (or `crate::document`) and deleting both the entries
and the paragraph that argues them. A promise living only in that
comment would go the moment someone edited it, which is why it is
written here.

## Cost, before anyone builds a door onto it

Measured on this tree at `docm/9-certified-range` (dev profile, one
machine): the query certifies NOTHING on the repo's corpus documents
at any budget a caller can afford — `corpus::plate_param`'s `hole_r`
costs ~3.4 s per leaf and certifies 0 of 64; `corpus::die`'s cube
`Distance` slot costs ~17 s per leaf. That is the driver's
certification width, not the query's arithmetic, and it is the reason
a Python door should expose the budget rather than choosing one:
`DriveConfig::default()`'s 65,536 leaves is days of wall clock.

## Home

LIB owns `crates/pncad-py` and the binding surface.

- 2026-11-03 — moved from LIB to PYDOOR by `git mv` when LIB closed, id and body unchanged; priced M (was legacy D) in the move (LIB had three P1 rows left behind 124 closed ones; its library-door rows went to PYDOOR, opened for them, and its tag, stub, census and projection rows to BIND; Ev, in chat).

## Closed

By OFFER's kernel certify unit (`work/offer/the-certified-range-refines-away-from-the-current-value`), which rebuilt the query as a resumable search and bound it in the same PR. The signature and the four arms quoted above are the query as it was; the door binds what replaced them.

- `crates/pncad-py/src/py/range.rs`: `RangeSearch` (`step()`, `steps`, `answer()`), the one-shot `certified_range`, the admission door `admit_range`, `RangeField`, `RangeSeed`, and the answer (`CertifiedRange`, `RangeSide`, `ProvenDifferent`, `NotDecided`), with the typed `RangeRefusal`. Undecided ground is its own object and never a number per side; offsets cross as quantities in the field's dimension; `pinned` is on the answer.
- The budget is the caller's: `max_depth` and `max_replays`, the latter required on the one-shot door. `DriveConfig` is not bound as a class; the rest of the certified half stays where `work/bind/certified-analysis-half-compiles-into-the-wheel-unbound` leaves it.
- The family left `NOT_CARRIED` and is carried through `pncad::analysis`.
