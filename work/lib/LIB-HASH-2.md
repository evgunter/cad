---
id: LIB-HASH-2
kind: unit
title: the nine value classes are unhashable by design under the mirror rule, with the two-way derives table
status: closed
opened: 2026-09-09
branch: lib/hash-2
refs: [pncad-py-value-classes-compare-without-hashing]
pr: 2261
closed: 2026-09-09
---



Mechanical unit under the mirror rule Ev stated on `[ev]` PR #2233 and
ruled there as (B'): **a Python value class mirrors its Rust type's
derives**, and the kernel omits `Hash` on values in favour of the
funnel. LIB-ZERO applied it to `Length` and `Angle`; this unit applies
it one rung up, to the nine value classes
`pncad-py-value-classes-compare-without-hashing` left undecided, and
measures the rule in BOTH directions over the whole module.

Binding-only, and narrower than that: **no class gains or loses a
dunder.** The diff under `crates/pncad-py/src/` is empty. What changes
is the roster's prose in `crates/pncad-py/tests/test_hashability.py`.

## The nine, and why the mirror answers each

All nine derive `PartialEq` and no `Hash` on the Rust side, so all
nine compare and do not hash in Python. Three of the item's readings
pull the other way, and each is answered in the roster line:

- **The small records read like keys.** `MateFrame`, `MatePrimitive`,
  `Alignment`, `AnalysisPolicy` — "which alignments did this assembly
  use" is a set. But the kernel does not key on them: it SOLVES from a
  frame and an alignment, MATCHES on a primitive, and takes a policy
  as one run's argument. A Python `set` of them is a decision the
  kernel has not made.
- **Findings read like keys.** `CheckFinding`, `CheckEvidence`,
  `FlushFinding` — deduplicating findings across runs is the natural
  want. But the kernel reports findings in a deterministic ORDER
  rather than a set, and their evidence bottoms out in `f64`, so a
  hash would owe `Expr`'s float answer before it could exist.
- **Reports and configs hold collections.** `ChecksReport`,
  `ChecksConfig` — a hashable aggregate over a list or a `BTreeMap` is
  a decision with a cost, and the kernel has not made it either
  (`ChecksConfig` derives `Eq` and still no `Hash`).

## The two-way table

Every class the compiled module exposes, walked exactly as
`test_hashability.py` walks it (`vars(pncad)`, `inspect.isclass`):
**163 classes.** 105 of them define no `__eq__` at all — 36 exception
types and 69 handle/builder classes — so Python gives them object
identity for both dunders (`cls.__hash__ is object.__hash__` for all
105, and none defines its own). They mirror no Rust value semantics in
either direction and are accounted here as a group; the rule below is
about the 58 classes that compare BY VALUE.

| Python class | Rust type | its comparison/hash derives | Python hashes? | agree? |
| --- | --- | --- | --- | --- |
| `Advisory` | `editor_core::Advisory` | PartialEq, Eq | yes | Python hashes, Rust does not |
| `Alignment` | `editor_core::Alignment` | PartialEq | **no** | agree |
| `AnalysisPolicy` | `editor_core::AnalysisPolicy` | PartialEq | **no** | agree |
| `Angle` | `quantity::Angle` | PartialEq, PartialOrd | **no** | agree |
| `AngleUnit` | `quantity::AngleUnit` | PartialEq | yes | Python hashes, Rust does not |
| `ArcSide` | `profile::ArcSide` | PartialEq, Eq | yes | Python hashes, Rust does not |
| `ArcSweep` | `profile::ArcSweep` | PartialEq, Eq | yes | Python hashes, Rust does not |
| `AssertionDir` | `editor_core::AssertionDir` | PartialEq, Eq, Hash, PartialOrd, Ord | yes | agree |
| `AxisSense` | `editor_core::AxisSense` | PartialEq, Eq | yes | Python hashes, Rust does not |
| `BooleanOp` | `topo::BooleanOp` | PartialEq, Eq, Hash, PartialOrd, Ord | yes | agree |
| `CapEnd` | `editor_core::CapEnd` | PartialEq, Eq, Hash, PartialOrd, Ord | yes | agree |
| `CheckEvidence` | `editor_core::CheckEvidence` | PartialEq | **no** | agree |
| `CheckFinding` | `editor_core::CheckFinding` | PartialEq | **no** | agree |
| `CheckId` | `editor_core::CheckId` | PartialEq, Eq, PartialOrd, Ord | yes | Python hashes, Rust does not |
| `CheckKind` | `editor_core::CheckKind` | PartialEq, Eq | yes | Python hashes, Rust does not |
| `ChecksConfig` | `editor_core::ChecksConfig` | PartialEq, Eq | **no** | agree |
| `ChecksReport` | `editor_core::ChecksReport` | PartialEq | **no** | agree |
| `Cmp` | `editor_core::Cmp` | PartialEq, Eq, Hash, PartialOrd, Ord | yes | agree |
| `ContactClass` | `editor_core::ContactClass` | PartialEq, Eq, PartialOrd, Ord, Hash | yes | agree |
| `ContentPin` | `editor_core::ContentPin` | PartialEq, Eq, Hash, PartialOrd, Ord | yes | agree |
| `Count` | `quantity::Count` | PartialEq, Eq, PartialOrd, Ord, Hash | yes | agree |
| `CurveKind` | `editor_core::CurveKind` | PartialEq, Eq, Hash, PartialOrd, Ord | yes | agree |
| `Denotation` | `editor_core::Denotation` | PartialEq, Eq | yes | Python hashes, Rust does not |
| `Distribution` | `editor_core::Distribution` (plus the dimension the offsets were written in) | PartialEq | yes | Python hashes, Rust does not |
| `DocParam` | `editor_core::DocParam` | PartialEq | yes | Python hashes, Rust does not |
| `DocParamValue` | `editor_core::DocParamValue` | PartialEq | yes | Python hashes, Rust does not |
| `DocRef` | `editor_core::DocRef` | PartialEq, Eq, Hash, PartialOrd, Ord | yes | agree |
| `EntityKind` | `editor_core::EntityKind` | PartialEq, Eq, Hash, PartialOrd, Ord | yes | agree |
| `Expr` | `editor_core::Expr` | PartialEq | **no** | agree |
| `FaceCensus` | `step_import::FaceCensus` | PartialEq, Eq | yes | Python hashes, Rust does not |
| `FlushFinding` | `editor_core::FlushFinding` | PartialEq | **no** | agree |
| `FlushRung` | `editor_core::FlushRung` | PartialEq, Eq | yes | Python hashes, Rust does not |
| `Frame` | `editor_core::Frame` | PartialEq | yes | Python hashes, Rust does not |
| `Length` | `quantity::Length` | PartialEq, PartialOrd | **no** | agree |
| `LengthUnit` | `quantity::LengthUnit` | PartialEq | yes | Python hashes, Rust does not |
| `MateFrame` | `editor_core::MateFrame` | PartialEq | **no** | agree |
| `MatePrimitive` | `editor_core::MatePrimitive` | PartialEq | **no** | agree |
| `MateRole` | `editor_core::MateRole` | PartialEq, Eq | yes | Python hashes, Rust does not |
| `MateSide` | `editor_core::MateSide` | PartialEq, Eq | yes | Python hashes, Rust does not |
| `McAssertion` | `editor_core::McAssertion` | PartialEq | yes | Python hashes, Rust does not |
| `McConfig` | `editor_core::McConfig` | PartialEq, Eq | yes | Python hashes, Rust does not |
| `McMeasure` | `editor_core::McMeasure` | PartialEq | yes | Python hashes, Rust does not |
| `MeasureExpr` | `editor_core::MeasureExpr` | PartialEq | **no** | agree |
| `MeasurePrimitive` | `editor_core::MeasurePrimitive` | PartialEq, Eq, Hash | yes | agree |
| `MeridianEnd` | `editor_core::MeridianEnd` | PartialEq, Eq, Hash, PartialOrd, Ord | yes | agree |
| `NodeId` | `editor_core::RecipeNodeId` | PartialEq, Eq, Hash, PartialOrd, Ord | yes | agree |
| `OpGroup` | `editor_core::OpGroup` | PartialEq, Eq, Hash, PartialOrd, Ord | yes | agree |
| `ParamName` | `editor_core::ParamName` | PartialEq, Eq, Hash, PartialOrd, Ord | yes | agree |
| `PlaneRelation` | `topo::CarrierRelation` (`plane_eq` re-exports it as `PlaneRelation`) | PartialEq, Eq | yes | Python hashes, Rust does not |
| `RimSupport` | `editor_core::RimSupport` | PartialEq, Eq, Hash, PartialOrd, Ord | yes | agree |
| `SegTag` | `editor_core::SegTag` | PartialEq, Eq, Hash, PartialOrd, Ord | yes | agree |
| `Severity` | `editor_core::Severity` | PartialEq, Eq | yes | Python hashes, Rust does not |
| `SketchPlane` | `pncad::profile::SketchPlane<f64>` | (none of PartialEq/Eq/Hash) | yes | Python hashes, Rust does not |
| `SplitHalf` | `editor_core::SplitHalf` | PartialEq, Eq, Hash, PartialOrd, Ord | yes | agree |
| `SurfaceKind` | `geom_brep::SurfaceKind` | PartialEq, Eq | yes | Python hashes, Rust does not |
| `ValidationFinding` | the binding's own `validation::Finding` projection | PartialEq, Eq | yes | Python hashes, Rust does not |
| `WrittenAngle` | `quantity::WrittenAngle` | PartialEq | yes | Python hashes, Rust does not |
| `WrittenLength` | `quantity::WrittenLength` | PartialEq | yes | Python hashes, Rust does not |

`Debug`, `Clone`, `Copy`, `Default`, `Serialize` and `Deserialize` are
trimmed from the derive column — they say nothing about comparison or
hashing.

## The four cells

| cell | Python hashes | Rust derives `Hash` | count | classes |
| --- | --- | --- | --- | --- |
| **A** | yes | yes | 18 | the tag mirrors and id newtypes: `AssertionDir`, `BooleanOp`, `CapEnd`, `Cmp`, `ContactClass`, `ContentPin`, `Count`, `CurveKind`, `DocRef`, `EntityKind`, `MeasurePrimitive`, `MeridianEnd`, `NodeId`, `OpGroup`, `ParamName`, `RimSupport`, `SegTag`, `SplitHalf` |
| **B** | no | no | 13 | this unit's nine, plus `Expr`, `MeasureExpr` (stated already) and `Length`, `Angle` (LIB-ZERO) |
| **C** | **no** | **yes** | **0** | empty — no class refuses to hash over a type that derives `Hash` |
| **D** | yes | no | 27 | 3 ruled carve-outs, 2 already filed, 22 newly filed below |

**Cell C is the direction that would be a gap under the rule, and it
is empty.** The brief expected the 24 fieldless mirrors LIB-HASH made
hashable to sit over `Hash`-deriving enums; verified, and only PART of
that expectation holds — 12 of them do (cell A), and 12 do NOT (cell
D, below). None of them lands in C.

## Cell D, dispositioned

- **Ruled carve-out (3)**: `DocParam`, `WrittenLength`, `WrittenAngle`
  — (B') names them and keeps their hashes: recipe data past the
  funnel, already folding the zero.
- **Already filed (2)**: `LengthUnit`, `AngleUnit` —
  `the-unit-classes-hash-over-a-partialeq-only-newtype`.
- **Newly filed, three shapes, 22 classes** — filed rather than fixed,
  because each asks whether the Rust omission is a decision or a
  `#[derive]` list nobody needed, and that is the same question the
  unit-classes row already puts to Ev:
  - `the-tag-mirrors-hash-over-kernel-enums-that-derive-no-hash` — 12
    fieldless mirrors (`#[pyclass(eq, eq_int, hash)]`) over kernel
    enums deriving `PartialEq, Eq` and no `Hash`: `Advisory`,
    `ArcSide`, `ArcSweep`, `AxisSense`, `CheckId`, `CheckKind`,
    `FlushRung`, `MateRole`, `MateSide`, `PlaneRelation`, `Severity`,
    `SurfaceKind`.
  - `the-value-records-hash-by-hand-over-kernel-types-that-derive-no-hash`
    — 9 records with hand-written `__eq__`/`__hash__`: `Denotation`,
    `Distribution`, `DocParamValue`, `FaceCensus`, `Frame`,
    `McAssertion`, `McConfig`, `McMeasure`, `ValidationFinding`.
  - `sketchplane-compares-and-hashes-over-a-rust-type-that-derives-neither`
    — `SketchPlane` alone: `profile::SketchPlane<f64>` derives
    `Clone, Copy, Debug` and NOT `PartialEq`, so the Python class
    invents the equality too, not just the hash.

## Delivered

- **The nine `FILED` roster entries become nine stated reasons** in
  `crates/pncad-py/tests/test_hashability.py`, in the `Length`/`Angle`
  rows' voice: each names its Rust type and the derives it mirrors,
  and each answers the reading that pulls the other way. The `FILED`
  constant is gone with its last user.
- **No code change to any class.** `git diff` under
  `crates/pncad-py/src/` is empty; no `variant`/`kind` value moved; no
  `__hash__` or `__eq__` gained, lost or altered anywhere.
- **The roster's blind spot is stated at `UNHASHABLE`'s docstring**:
  every reason now names a derive list, which is a structurally
  checkable claim that nothing here checks — the binding census reads
  the façade's `pub use` LINES, so it sees a type's name and never its
  `#[derive(...)]`, and no other reader in the suite opens a `.rs`
  file. A derive list that changes in the kernel silently falsifies
  the prose beside it; only the two guards, which read `__hash__` off
  the class, stay true.
- **Both guards stay green** with the nine reasons rewritten:
  `test_every_unhashable_class_is_on_the_roster_with_a_reason` and
  `test_the_roster_carries_no_stale_entry`.

## The sweep, and what it could not match

The Python side is the module walk `test_hashability.py` does
(`vars(pncad)` + `inspect.isclass`), so it has no blind spot on this
module's classes. The Rust side is a source read: every `#[pyclass]`
item in `crates/pncad-py/src/`, its wrapped type (a newtype's field, a
mirror comment's `mirrors the documented ...` citation, or a
name match), resolved to a `pub struct`/`pub enum` in the workspace
and its `#[derive(...)]` attribute read. **What that could not match**:
a type reached through a `pub type` alias or a `pub use` rename is
resolved by hand, not by the reader — `FlushFinding`
(`editor_core::FlushFinding` is
`topo::flush::FlushFinding<(StableName, StableName)>`), `PlaneRelation`
(`topo::CarrierRelation`), `NodeId` (`RecipeNodeId`) and
`ValidationFinding` (the binding's own projection) are the four, each
verified by reading the definition. Two classes are macro-generated
(`PathPoint`, `PathDirectedPoint`) and land in the identity group, so
no derive claim rides on them.

## Closed

`pncad-py-value-classes-compare-without-hashing` closes here: its nine
undecided rows are decided by the mirror rule and each carries a
stated reason. Its two `by design` rows (`Expr`, `MeasureExpr`) were
never this unit's and are untouched. The residue this unit found is
the three files named under **Cell D** above, not a sentence in this
section.
