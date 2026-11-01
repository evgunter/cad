---
id: the-tag-mirrors-hash-over-kernel-enums-that-derive-no-hash
kind: issue
title: pncad-py: twelve fieldless tag mirrors hash over kernel enums that derive PartialEq and Eq and no Hash
status: closed
opened: 2026-09-09
closed: 2026-09-09
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

## Question for Ev (2026-09-09, LIB orchestrator; `[ev]` PR) — how far the mirror rule reaches

One question over four items (this one, `the-value-records-hash-by-hand-over-kernel-types-that-derive-no-hash`, `sketchplane-compares-and-hashes-over-a-rust-type-that-derives-neither`, `the-unit-classes-hash-over-a-partialeq-only-newtype`). Your (B′) ruling on `[ev]` #2233 stated a rule — a Python value class mirrors its Rust type's derives; the kernel omits `Hash` on values in favour of the funnel — and gave its reason: a quantity is a MAGNITUDE, not a key. LIB-HASH-2's two-way walk over the 163 classes the module exposes found the reverse direction populated: 27 Python classes hash over Rust types that derive no `Hash` (3 are the ruled carve-out — `DocParam`, `WrittenLength`, `WrittenAngle` — and 24 are these four items: 12 fieldless tag mirrors, 9 hand-hashed records, `SketchPlane`, and the 2 unit views). Nothing refuses to hash over a `Hash`-deriving type. Three readings:

- **(A) The rule is about magnitudes, and the derive lists are corrected UPWARD where a type is a key.** A tag, a unit row and a payload-free record are keys — `test_hashability.py` opens on that premise and pins the mirror surface through one set and one dict — and the Rust omission beside them reads as a `#[derive]` list nobody needed (`Severity` omits `Hash` beside eleven siblings that derive it), not a statement. So Python moves nothing; LIB files one upward item per kernel program to add `Eq, Hash` where the type is a key (the 12 enums; `quantity`'s three unit views and `UnitDef`; the float-free records `Denotation`, `FaceCensus`, `McConfig`), and the mirror becomes exact from the Rust side. Records that bottom out in `f64` (`Distribution`, `DocParamValue`, `Frame`, `McAssertion`, `McMeasure`) are the (B′) carve-out generalised — recipe data folding the zero through the kernel's own fold — and keep their hashes as `WrittenLength` does; `SketchPlane`, whose Rust type declares no comparison at all, keeps its bit-equality pair as the one deliberate boundary invention (or gains `PartialEq/Eq/Hash` through `bit_eq` in `profile`, upward, if that program wants it). **Recommended.**
- **(B) The rule reaches every class.** 22 classes lose `__hash__` and join `UNHASHABLE`; `TestEveryMirrorIsAKey` stops being a claim about every mirror; `SketchPlane` compares by identity — the one row where the rule changes what a comparison ANSWERS, not whether a value keys. Mechanical, and it deletes the folds five records already do through the kernel.
- **(C) Split by kind**: tags and units keep hashing as keys; the nine records lose it; `SketchPlane` decided on its own. Two rules where (A) has one.

Recommendation: **(A)**. It is the reading under which (B′) and LIB-HASH's ruling (2016–2020: the mirrors carry `hash`) are both right, and it puts the repair where the drift is — in derive lists the kernel never needed to think about — rather than un-keying doors that are pinned as keys.

### The case-by-case reading, added 2026-09-09 after Ev leaned (A) with a refinement

Ev: hash absent where Rust omits it for funnel reasons; elsewhere
Rust may derive it and both sides always match; but no work adding
hashing where nothing plausibly keys. Measured: nothing in the tree
keys on any of the 24 today (no kernel map keyed by them; the only
Python key uses are the hashability pins), so the argument is from
shape. **Add `Hash` upward, Python unchanged**: the 12 tag mirrors
(fieldless, one derive word, a discriminant's natural use is a tally;
a split tag surface would be worse than either uniform answer),
`McConfig` (exact fields, the memo key for cached runs), `Denotation`
(a set of what selections denote), and the unit views + `UnitDef`
(the pinned tally-by-unit; a `Hash` by symbol under the seal, a few
lines in `quantity`). **Drop Python's `__hash__`, Rust unchanged**:
`Frame`, `DocParamValue`, `Distribution`, `McMeasure`, `McAssertion`
(magnitudes; their hand folds are the work (B′) declined),
`FaceCensus`, `ValidationFinding` (the binding's own projection).
**`SketchPlane`**: no hash either side; `impl PartialEq for
SketchPlane` in `profile` delegating to `bit_eq` so Rust means the
comparison Python already answers, and `==` keeps its answer. Net: 15
one-word upward derives plus the unit `Hash`, 8 Python hashes
removed — one mechanical unit spanning both sides, with the kernel
touches announced on their programs' trackers.

## Ruled (2026-09-09, Ev on `[ev]` PR #2265): (A), case by case

Both sides always match; `Hash` is absent where Rust omits it for
funnel reasons; elsewhere Rust may derive it, but no work is spent
adding hashing where nothing plausibly keys. For this item: the
twelve tag mirrors' kernel enums gain `Hash` upward (one derive word
each — `Advisory`, `ArcSide`, `ArcSweep`, `AxisSense`, `CheckId`,
`CheckKind`, `FlushRung`, `MateRole`, `MateSide`, `CarrierRelation`
(Python's `PlaneRelation`), `Severity`, `SurfaceKind`), the Python
mirrors unchanged. Mechanical unit LIB-MIRROR, one unit across both
sides for all four items.

## Closed (2026-09-09, LIB-MIRROR, PR #2271)

The twelve kernel enums gained `Hash`, one derive word each, at
exactly the `file:line` the table above cites; the Python mirrors did
not move and `TestEveryMirrorIsAKey` still tallies the whole mirror
surface. The twelve are now in cell A beside the twelve that were
already there, so the enum tag surface is uniform: twenty-four mirrors
over twenty-four `Hash`-deriving enums.
