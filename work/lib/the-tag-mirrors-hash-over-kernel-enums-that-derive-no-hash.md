---
id: the-tag-mirrors-hash-over-kernel-enums-that-derive-no-hash
kind: issue
title: pncad-py: twelve fieldless tag mirrors hash over kernel enums that derive PartialEq and Eq and no Hash
status: open
opened: 2026-09-09
---



Found by LIB-HASH-2 while walking every class the compiled module
exposes against the rule Ev's (B') ruling states on
`the-quantity-boundary-compares-and-hashes-as-if-poison-and-signed-zero-cannot-arrive`:
**the Python class mirrors the Rust type's derives.** This row asks
rather than decides, exactly as
`the-unit-classes-hash-over-a-partialeq-only-newtype` does for the
unit views.

## What the mirror says

LIB-HASH made every fieldless enum mirror hashable
(`#[pyclass(eq, eq_int, hash)]`), which was the right repair for the
symptom it fixed — a tag that compares and cannot be a dict key. Read
against the mirror rule, though, the mirrors split in two, and only
half of them mirror anything:

| Python class | its kernel enum | that enum's derives |
| --- | --- | --- |
| `Advisory` (`crates/pncad-py/src/py/checks.rs:111`) | `editor_core::Advisory` (`crates/editor-core/src/checks.rs:161`) | `PartialEq, Eq` |
| `ArcSide` (`crates/pncad-py/src/py/path.rs:142`) | `profile::ArcSide` (`crates/profile/src/path/verbs.rs:148`) | `PartialEq, Eq` |
| `ArcSweep` (`crates/pncad-py/src/py/path.rs:121`) | `profile::ArcSweep` (`crates/profile/src/sugar.rs:38`) | `PartialEq, Eq` |
| `AxisSense` (`crates/pncad-py/src/py/mate.rs:163`) | `editor_core::AxisSense` (`crates/editor-core/src/mate.rs:144`) | `PartialEq, Eq` |
| `CheckId` (`crates/pncad-py/src/py/checks.rs:72`) | `editor_core::CheckId` (`crates/editor-core/src/checks.rs:51`) | `PartialEq, Eq, PartialOrd, Ord` |
| `CheckKind` (`crates/pncad-py/src/py/checks.rs:85`) | `editor_core::CheckKind` (`crates/editor-core/src/checks.rs:128`) | `PartialEq, Eq` |
| `FlushRung` (`crates/pncad-py/src/py/flush.rs:58`) | `editor_core::FlushRung` (`crates/topo/src/flush.rs:135`) | `PartialEq, Eq` |
| `MateRole` (`crates/pncad-py/src/py/mate.rs:472`) | `editor_core::MateRole` (`crates/editor-core/src/mate/solve.rs:43`) | `PartialEq, Eq` |
| `MateSide` (`crates/pncad-py/src/py/mate.rs:191`) | `editor_core::MateSide` (`crates/editor-core/src/mate.rs:84`) | `PartialEq, Eq` |
| `PlaneRelation` (`crates/pncad-py/src/py/flush.rs:33`) | `topo::CarrierRelation`, re-exported as `PlaneRelation` (`crates/topo/src/boolean/carrier_eq.rs:64`) | `PartialEq, Eq` |
| `Severity` (`crates/pncad-py/src/py/checks.rs:97`) | `editor_core::Severity` (`crates/editor-core/src/checks.rs:141`) | `PartialEq, Eq` |
| `SurfaceKind` (`crates/pncad-py/src/py/select.rs:385`) | `geom_brep::SurfaceKind` (`crates/geom-brep/src/intersect.rs:109`) | `PartialEq, Eq` |

Twelve hash where their kernel enum derives no `Hash`. The other
twelve mirrors — `AssertionDir`, `BooleanOp`, `CapEnd`, `Cmp`,
`ContactClass`, `CurveKind`, `EntityKind`, `MeridianEnd`, `OpGroup`,
`RimSupport`, `SegTag`, `SplitHalf` — sit over enums that DO derive
`Hash`, and are not this row. Twenty-four mirrors, split down the
middle.

## Why it is not obvious the rule reaches them

The same three counter-arguments the unit-classes row makes, and one
more:

- **A tag is a key, and that premise is asserted at the top of
  `crates/pncad-py/tests/test_hashability.py`**: "a comparable value
  is a KEY: every tag this module exposes goes in a set and comes back
  out of a dict". The ruling's reason for dropping a hash — "a
  quantity is a magnitude, not a key" — does not obviously transfer to
  a discriminant with no payload.
- **The hash is honest.** A fieldless enum's hash is its
  discriminant; there is no float in it and no `-0.0` question, so it
  agrees with the derived `PartialEq` it mirrors by construction.
- **It is pinned as a door.**
  `test_hashability.py::TestEveryMirrorIsAKey` tallies the WHOLE
  mirror surface through one set and one dict, and
  `test_a_mirror_read_off_a_door_keys_the_same_as_the_class_attribute`
  is the case that motivated LIB-HASH.
- **The Rust omission looks accidental here in a way it did not for
  `Length`.** `quantity::Length` omits `Hash` beside a documented
  funnel argument; `editor_core::Severity` omits it beside eleven
  sibling enums that derive it, which reads as a `#[derive]` list
  nobody needed rather than a statement.

If that last reading is the right one, the repair is UPWARD — add
`Hash` to the twelve kernel enums — and the Python side does not move
at all. If it is not, twelve mirrors lose `hash` and join
`UNHASHABLE`, and `TestEveryMirrorIsAKey` stops being a claim about
every mirror. Either way it is one decision, not twelve.
