---
id: LIB-B-MEASURES
kind: unit
title: binding census family B-MEASURES
status: review
branch: lib/b-measures
opened: 2026-09-06
---

Queued mechanical census family (the B-RESOLVE shape): sweep the
family's bindings against the census contract, construct the
previously unconstructible pins where the surface now allows, and
re-cut the census rows honestly. Families share the census/tags/test
files, so at most two run concurrently, staggered.


### The gate, measured first — and it is the LANE, not the feature

All seven `NOT_BOUND` rows this family owns are curated in
`crates/pncad/src/document.rs:40` and `:60`, which carries **no
`cfg`** at all. So unlike B-DISTRIBUTIONS, where the question was
which façade line a door sat on, nothing here is behind `interval` on
the façade and the whole authoring vocabulary is on the default build
the wheel is made from.

The limit is one rung further in, and it is sharper. `min_clearance`
is answered by [`MinClearanceLane::min_separation`]
(`crates/editor-core/src/measure.rs:519`), a per-scalar CAPABILITY:

- `impl MinClearanceLane for f64` — `None`
  (`crates/editor-core/src/measure.rs:580`).
- `impl MinClearanceLane for geom_core::Probe` — `None`, `#[cfg(feature = "probe")]` (`:591`).
- `impl<T> MinClearanceLane for geom_core::Dual<T>` — `None` (`:606`).
- `impl<T: MinClearanceLane> MinClearanceLane for geom_core::Sym<T>` — `None` (`:668`).
- `impl MinClearanceLane for geom_core::Interval` — the ONLY `Some`,
  `#[cfg(feature = "interval")]` (`crates/editor-core/src/measure.rs:620`).

**The binding evaluates at `f64` alone** — `d::evaluate::<f64>` at
`crates/pncad-py/src/py/value.rs:1577`, and `PyEvaluation` holds a
`d::Evaluation<f64>` (`:765`). So:

- **`MeasureUnavailableAt` IS reachable on the default build**, and it
  is exactly what the `f64` lane answers a `min_clearance` with
  (`crates/editor-core/src/eval/wire.rs:2447-2457`). This family's
  fourth verb is authorable and its typed absence is readable, today,
  with no feature.
- **`MinClearanceRefusal` is NOT reachable from Python at any feature
  set.** Its one producer is the `interval` impl above and its one
  consumer arm is `NodeErrorKind::MeasureClearanceRefused`
  (`crates/editor-core/src/eval/mod.rs:1288`). Turning on
  `pncad-py`'s own `interval` feature does not reach it either: the
  feature forwards the scalar to `pncad`, and the binding still
  evaluates at `f64`. That is `profile_lift`'s sentence in the census
  docstring, arriving on the refusal side — the door starts answering
  differently exactly when Python gains a non-`f64` evaluation, and it
  should gain its spelling in the unit that brings one.

### The roster: seven entries, and where each goes

`test_binding_census.py` charters `B-MEASURES` in `FAMILIES` (`:746`)
and seven `NOT_BOUND` entries cite it (`:1785-1800`).

| row | after this unit | why |
| --- | --- | --- |
| `MeasureExpr` | off the roster (rule 1) | `pncad.pyi` declares a top-level `MeasureExpr` |
| `MeasurePrimitive` | off the roster (rule 1) | top-level, four static constructors |
| `AssertionDir` | off the roster (rule 1) | top-level, the `BooleanOp` mirror |
| `MeasureNodeFault` | off the roster (rule 1) | top-level exception, the Rust type's own name |
| `MeasureUnavailableAt` | off the roster (rule 1) | top-level exception, the Rust type's own name |
| `SitedRef` | stays `NOT_BOUND`, retagged `SHAPE` | never handed across: both doors that author one take a NODE and a NAME |
| `MinClearanceRefusal` | stays `NOT_BOUND`, retagged `SHAPE` | flattened to `EvaluationError.kind == "measure_clearance_refused"`, and unreachable at the `f64` lane |

Two of the seven therefore leave the `gap:` category without becoming
reach, and both say so at the entry. That is a THIRD way off a
charter beside B-DISTRIBUTIONS's two, and it is the honest one where
the census's own `sweep_body` precedent applies: "binding a door that
cannot succeed would move a name out of this list without moving
anything a caller can do".

### The Python spelling of `MeasureUnavailableAt`, and the name it must not collide with

LIB-B-DISTRIBUTIONS landed a Python exception class **`MeasureUnavailable`**
(`crates/pncad-py/src/errors.rs`, `ErrorClass::Measure`) for
`pncad::analysis::MeasureUnavailable` — the analysis lane's band
refusal, "I know the limits but not the shape". This family's
`MeasureUnavailableAt` is a DIFFERENT kernel type answering a
different question ("this scalar has nowhere to put an enclosure").
They are bound at **`MeasureUnavailableAt`**, the Rust type's own
name, which is both this taxonomy's stated convention ("the Python
class keeps the Rust type's own name") and what keeps the two apart
for a caller reading a traceback. Neither subclasses the other; both
subclass `PncadError`.

### Ground truth in Rust

- `Node::Measure { expr, refs }` (`crates/editor-core/src/node.rs:1902`)
  and `Node::Assertion { measure, bound, dir }` (`:1922`).
  `Node::measure(expr, refs) -> Result<_, MeasureNodeFault>` (`:2927`)
  is the ONE construction door; `Node::Assertion` has no smart
  constructor, because its check needs the document and runs at the
  edit door (`crates/editor-core/src/edit.rs:1366-1384`:
  `AssertionTarget`, `AssertionDimension`).
- `SitedRef { at, name }` (`node.rs:1021`), with `new` and `at_mint`.
- `MeasureNodeFault` (`node.rs:1110`) has exactly **one** arm,
  `RefIndexOutOfRange { verb, index, refs }`. It reaches the edit door
  as `EditError::MeasureMalformed` (tag `measure_malformed`,
  `crates/pncad-py/src/tags.rs:225`) and the load door as the same
  re-check.
- `MeasurePrimitive` (`measure.rs:61`): `Distance{a,b}`, `Angle{a,b}`,
  `MinClearance{a,b}`, `Gap{outer,inner}`, with `dim()`, `refs()` and
  `verb()`.
- `MeasureExpr` (`measure.rs:164`): `primitive`, `value`, `add`,
  `sub`, `neg`, `mul`, `div`, `min`, `max` — private fields, fallible
  binary constructors, dimension cached at construction.
- `AssertionDir` (`measure.rs:690`): fieldless `AtLeast`/`AtMost`
  with `symbol()`.
- `MeasureUnavailableAt::NeedsEnclosure { verb, scalar, door }`
  (`measure.rs:460`) and `MinClearanceRefusal { class, payload }`
  (`measure.rs:561`).

### The Rust rows that assert the numbers

`crates/editor-core/tests/m10_2_measure.rs` (evaluated) and
`m10_2_measure_wire.rs` (the wire). The `m10_5`/`m10_6`
`*_interval.rs` rows are `interval`-gated and out of Python's reach
for the lane reason above. **No constant is transcribed**: every
Python row re-derives its oracle from its own authoring — the plate's
holes at `x = ±0.30` with radius `0.2` make the web `0.2`, two slabs
2 m apart make the gap `+2`, a prism's opposed caps make the angle
`pi`, a coaxial bore and pin make the gap `r_bore - r_pin`.

### What `pncad-py` can reach, and the field-level names the census cannot see

`pncad-py` depends on `pncad` and `quantity`, and every name above is
inside that. What no roster in this file can see:

- **`Node::Measure` and `Node::Assertion` are ARMS of `Node`**, which
  rule 1 accounts WHOLE — `pncad.pyi` has declared a top-level `Node`
  since the beginning. Two of the twenty-five recipe node kinds have
  been unconstructible from Python for the life of the binding and
  nothing in either census said so. B-PART's "the entries an id owns
  are not always the entries a unit must move", one level in.
- **`Doc.node_kind` has answered `"measure"` and `"assertion"` since
  it was written** (`crates/pncad-py/src/node_kind.rs`), and both
  words were unreachable: no Python caller could author a node of
  either kind. A committed roster that is exhaustive over the kernel
  enum cannot tell a word a caller can reach from one it cannot.
- **`MeasureNodeFault`'s one arm is a FAULT ARM behind an edit tag.**
  `EditError.variant == "measure_malformed"` has been in the tag
  table and in the stub since M10-2 landed, for a node no Python
  caller could construct.
- **`NodeErrorKind`'s seven measurement arms** — `measure_ref_resolve`,
  `measure_ref_unreadable`, `measure_unsupported`,
  `measure_not_parallel`, `measure_non_finite`, `measure_malformed`,
  `measure_selection_kind`, plus `assertion_dimension` and
  `measure_clearance_refused` (`crates/pncad-py/src/tags.rs:429-444`)
  — are `EvaluationError.kind` words that no Python row could reach.
  `NodeErrorKind` is in `BOUND_AS` at that spelling, so the census
  accounts it whole and saw none of this.

### Decisions honored, not relitigated

- `crates/pncad/tests/all.rs`'s `NOT_CARRIED` names `MinClearanceLane`
  and `MinClearanceOperand` (`:3791`, `:3792`) as the analysis lane's
  interior residue — the third lane seam. Neither is re-opened and
  neither is bound; the seam is exactly what makes `MinClearanceRefusal`
  unreachable here, and that is the seam working.
- `AssertionVerdict` -> `Verdict` and `UnevaluatedReason` ->
  `Verdict.reason` are already in `BOUND_AS`, and `ASSERT_BOUND` is
  already `SHAPE`. The READ half (`Value.measure`, `Value.assertion`)
  ships and is not re-cut; what changes at one door is stated below.
- `StableName` stays opaque text (the ordinal-28 contract), so a
  `SitedRef` crosses as `(NodeId, str)` — the pair `Node.mate`
  already takes, flattened there because a mate has exactly two sides
  and a list here because a measure's arity is its reference list.

## Home

LIB's (the Python surface is outside M10's fence). Filed 2026-09-06
at the program's reactivation; `LIB-B-FACE-FRAME` and `LIB-B-PART`
named it as "unscheduled alongside". Sequenced after
B-DISTRIBUTIONS if both are taken: the census says this family's
asymmetry is B-DISTRIBUTIONS's without the sharp edge.
