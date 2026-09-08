---
id: LIB-B-DISTRIBUTIONS
kind: unit
title: binding census family B-DISTRIBUTIONS
status: review
branch: lib/b-distributions
opened: 2026-09-06
pr: 2192
---

Queued mechanical census family (the B-RESOLVE shape): sweep the
family's bindings against the census contract, construct the
previously unconstructible pins where the surface now allows, and
re-cut the census rows honestly. Families share the census/tags/test
files, so at most two run concurrently, staggered.

## Derived scope (stated before any code changed)

### The gate, measured first

`crate::analysis` is M10's ground and part of it is `interval`-gated,
so the first question is which of the three chartered read doors exist
on the DEFAULT build of `pncad-py` — the feature set the wheel is
made from. The façade states the split at one file:

- **UNGATED** (`crates/pncad/src/analysis.rs:49-52`):
  `AnalysisPolicy`, `AnalysisPolicyError`, `AnalyzedBox`,
  `AnalyzedParam`, `DEFAULT_QUANTILE_MASS`, `MeasureUnavailable`,
  `OffsetInterval`, `analyzed_box`, `box_mass`, `sample_offset`,
  `tail_mass`. `editor_core::analysis` itself carries no module-level
  `cfg` (`crates/editor-core/src/lib.rs:19`); only two scalar impls
  inside it are gated.
- **GATED** `#[cfg(feature = "interval")]` (`:55`, `:66`, `:83`,
  `:89`, `:104`): the E6 driver and its `ParamBox`/`BoxAxis`, the
  E4/E5 sensitivity and stackup, `assertion_at` and the verdict
  vocabulary, the E10 reporting layer.

**All three chartered doors are on the ungated line, so the whole of
this family closes on the default build and nothing is deferred to a
feature.** The gated half is not chartered by this family and is
reported as the limit rather than bound behind a feature the wheel
does not carry.

**No precedent for a feature-gated Python door**, checked rather than
assumed: the only `cfg(feature = …)` anywhere in `crates/pncad-py/src`
is `"python"` (`lib.rs:38`), which gates the whole PyO3 half, not any
individual door. So the question the brief left open has one answer —
there is no precedent, and this unit does not mint one.

### The census roster of this family is THREE entries, and the charter names four things

`test_binding_census.py` charters `B-DISTRIBUTIONS` in `FAMILIES`
(`:747`) and three `NOT_BOUND` entries cite it: `Distribution`
(`:1777`), `DistributionFault` (`:1797`), `DistributionField`
(`:1798`). All three are curated in `crates/pncad/src/document.rs:123`.

The charter's OTHER half — "the analysis doors that read them back:
the analyzed box, tail mass and leaf mass" — was never a row here and
could not become one. Those names are curated in
`crates/pncad/src/analysis.rs`, which is **not one of the three files
the census reads**; its docstring says so ("the three that curate the
document layer and the common surface"). So `analyzed_box`,
`tail_mass`, `box_mass`, `AnalysisPolicy`, `AnalyzedBox`,
`AnalyzedParam`, `MeasureUnavailable`, `OffsetInterval`,
`DEFAULT_QUANTILE_MASS` and `sample_offset` are invisible to the
census in both directions. This is the B-FACE-FRAME gap between a
charter and a roster in its widest form so far: three of the charter's
four things are outside the alphabet, and only `Distribution` made the
family dispatchable at all.

### What `pncad-py` can reach

`pncad-py` depends on `pncad` and `quantity` only, and everything is
inside that. Ground truth in Rust: `Distribution::{Band, Uniform,
Normal, TruncatedNormal}` (`crates/editor-core/src/distribution.rs:53`),
`DistributionField::{Sigma, Lo, Hi}` (`:88`),
`DistributionFault::{NonFinite, SigmaNotPositive,
NominalOutsideSupport}` (`:111`) with the ONE invariant statement
`Distribution::check` (`:172`), `DocParam::continuous_with`
(`crates/editor-core/src/doc.rs:189`) and `DocParam::distribution`
(`:200`).

### The sharp edge, and the shape the kernel offers for closing it

`DocEdit::SetDocParam` is create-or-replace: it takes a whole
`DocParam` and writes it (`crates/editor-core/src/edit.rs:1716` ->
`write_doc_param` -> `:1299`), so a `DocParam` rebuilt from a
dimension and a number replaces the declaration and the annotation is
gone with no refusal. `SetDocParamValue` reads the declaration off the
document and reuses it whole (`:1718-1731`), which is why it carries
the distribution forward.

**There is no distribution-only edit arm**, checked: `DocEdit` has 20
arms (`edit.rs:35-288`) and exactly two touch the parameter table,
both through `write_doc_param`. So the kernel's own shape for closing
this is the CONSTRUCTOR — `DocParam::continuous_with` — and this unit
invents no kernel edit. The deletion stays the kernel's semantics;
what changes is that a Python caller can now see the declaration and
restate it.

### Decisions honored, not relitigated

- `crates/pncad/tests/all.rs`'s `NOT_CARRIED` names the analysis
  lane's INTERIOR residue (`:3725`): `AxisScalar`, `SeedScalar`,
  `SeedError`, `seed_env`, `param_env_over`, `std_deviation`,
  `sensitivities`, `PairingViolation`. None of it is re-opened, and
  none of it is bound here.
- `Tol` stays the `float` epsilon, `StableName` stays text,
  `Dimension` stays what the `DocParam` constructors choose between.
- The tags `invalid_distribution` and `non_finite_doc_param` already
  exist (`src/tags.rs:232`, `:231`) and are already inventoried. This
  unit adds no arm to `edit_error_tag`; what it adds is the first
  CONSTRUCTION of the faults from Python, at a door that refuses
  earlier.

### What the census still cannot see after this unit

That `analyzed_box`, `AnalysisPolicy`, `AnalyzedBox`, `AnalyzedParam`,
`DocParam.distribution` and the two mass columns exist at all — all of them behind names the census's own file list
excludes. The positive form is
`crates/pncad-py/tests/test_distributions.py`.

## Outcome

The family closed. Bound at these spellings:

- `Distribution.band(lo, hi)` / `.uniform(lo, hi)` / `.normal(sigma)`
  / `.truncated_normal(sigma, lo, hi)` — a frozen value class of
  static constructors, `PatternKind`'s and `PartSelect`'s shape, plus
  two things those two do not have. The offsets are TYPED quantities
  (`Length | Angle | float`) and the class remembers which, because
  §L4's whole point is that `1e-6` must not mean microns to a reader
  and metres to the kernel; and construction runs the kernel's own
  `Distribution::check`, so an E2 invariant refuses where it is
  written rather than at the `Doc.apply` three lines later.
- `DocParam.length(value, distribution=None)` and its two siblings —
  the annotation beside the unit, on the same constructor. `count`
  takes none and cannot.
- `DocParam.distribution` — the read side. A keyed `Doc.doc_param`
  read door was written and then DELETED at the `origin/main` merge:
  LIB-B-NOTATION landed `Doc.params`, a whole-table snapshot whose own
  prose calls it "the only door that answers a whole parameter back",
  and `doc.params.get(name)` answers the same question. Two doors for
  one question is not worth the keyed lookup.
- `analyzed_box(doc, policy=None)` answering an `AnalyzedBox`, with
  `AnalysisPolicy`, `AnalyzedParam` and `DEFAULT_QUANTILE_MASS`.
- `AnalyzedBox.tail_mass(name)` and `AnalyzedBox.box_mass(name, lo,
  hi)` — the tail column and the leaf column, keyed BY THE BOX. The
  kernel's free `tail_mass`/`box_mass` deliberately do not cross:
  `AnalyzedBox::axis_tail_mass`'s own doc comment says the free doors
  let a caller pair one parameter's distribution with another's box
  and get a plausible number rather than a refusal, and Python has no
  compile step that would catch it. The kernel keeps them for the E6
  driver, which prices intervals that are deliberately not the
  analyzed ones — and that driver is gated and has no Python surface.
- Three typed exceptions, each keeping its Rust type's own name:
  `DistributionFault` (`variant` plus `field`/`sigma`/`lo`/`hi`,
  present on every arm), `MeasureUnavailable` (`variant`, `param`) and
  `AnalysisPolicyError` (`variant`, `mass`).

Census delta, exactly as the scope predicted: `Distribution` and
`DistributionFault` leave `NOT_BOUND` ENTIRELY under rule 1 (both are
top-level in `pncad.pyi` at those spellings), `DistributionField`
moves to `BOUND_AS` as `DistributionFault.field`, the charter leaves
`FAMILIES`, and the closure paragraph records the three-ways-out, the
charter/roster gap and the gate measurement.

`crates/pncad-py/tests/test_distributions.py` is the positive form:
36 tests mirroring `crates/editor-core/tests/m10_1_analysis.rs` and
the façade's end-to-end row, with every number an oracle a reader can
check (±3σ of a stated sigma, the standard normal's interquartile
range, a symmetric truncation's half, `1 - DEFAULT_QUANTILE_MASS`)
rather than a constant transcribed from the Rust rows. Two Rust
construction pins in `src/tests.rs` mint the four form words and the
three faults from real kernel values, on the default no-interpreter
row. Five ty fixture rows in each direction.

One finding banked on LIB's slate:
`work/lib/advisory-monte-carlo-lane-has-no-python-door.md` — the
E11.1 advisory estimator is ungated on the façade precisely so a
caller without the certified scalar can reach it, and the Python
caller (who is exactly that caller) cannot. Findings outside the fence
are in the PR body.

## Home

LIB's (the Python surface is outside M10's fence, the same routing
DOCM used for B-FACE-FRAME and B-PART). Filed 2026-09-06 at the
program's reactivation; `LIB-B-FACE-FRAME` and `LIB-B-PART` named it
as "unscheduled alongside".
